// Package main exposes the module-backed ABI adapter for the built-in Rust SDK.
package main

import (
	"context"
	"fmt"
	"path"
	"sort"
	"strings"

	"rust-sdk/internal/dagger"
	"rust-sdk/internal/metadata"
)

const (
	sdkRuntimePath          = "runtime"
	engineToolPath          = "dist/dagger-rust-engine"
	formatterToolPath       = "dist/rustfmt"
	engineDescriptorPath    = "dist/engine-source.json"
	clientGenerationPath    = "dist/client-generation.json"
	runtimePolicyPath       = "dist/runtime-policy.json"
	operationRequestPath    = "/var/lib/dagger/rust/request.json"
	operationSchemaPath     = "/var/lib/dagger/rust/schema.json"
	operationProjectPath    = "/var/lib/dagger/rust/project"
	operationResultPath     = "/var/lib/dagger/rust/result.json"
	runtimePolicyMountPath  = "/usr/local/share/dagger/rust/runtime-policy.json"
	runtimePlanPath         = "/var/lib/dagger/rust/runtime-plan.json"
	runtimeProvenancePath   = "/var/lib/dagger/rust/runtime-provenance.json"
	runtimeDiagnosticPath   = "/var/lib/dagger/rust/runtime-diagnostic.json"
	operationDescriptorPath = "/usr/local/share/dagger/rust/engine-source.json"
	operationToolPath       = "/usr/local/bin/dagger-rust-engine"
	operationWorkspaceRoot  = "workspace"
)

// RustSDK is immutable packaged state. Operation methods build fresh Dagger object
// graphs from this source instead of retaining mutable container or project handles.
type RustSDK struct {
	SDKSourceDir *dagger.Directory // +private
}

// New constructs the built-in adapter from the complete packaged SDK content root.
func New(
	// Complete engine-packaged Rust SDK content, including runtime and dist metadata.
	// Absent when the runtime is installed as an ordinary module; operations then
	// seal the content from the pinned SDK dependency.
	// +optional
	sdkSourceDir *dagger.Directory,
) (*RustSDK, error) {
	return &RustSDK{SDKSourceDir: sdkSourceDir}, nil
}

// FindClientRoot returns the nearest Cargo package root at or above the
// workspace's current location, or null when no Cargo package contains it.
func (sdk *RustSDK) FindClientRoot(ctx context.Context, ws *dagger.Workspace) (*string, error) {
	cwd, err := workspaceCwd(ctx, ws)
	if err != nil {
		return nil, err
	}
	// findUp stats a host-backed workspace, so a manifest that exists only in
	// pending workspace edits is visible solely through the current directory.
	present, err := workspaceHasFile(ctx, ws, "Cargo.toml")
	if err != nil {
		return nil, err
	}
	if present {
		return &cwd, nil
	}
	found, err := ws.FindUp(ctx, "Cargo.toml")
	if err != nil {
		return nil, fmt.Errorf("find enclosing Cargo package: %w", err)
	}
	if found == "" {
		return nil, nil
	}
	root := normalizeWorkspacePath(path.Dir("/" + strings.TrimPrefix(found, "/")))
	return &root, nil
}

// GenerateScope reconciles the Rust SDK state of one workspace scope. A module
// scope is scaffolded and given its module configuration on first use, then its
// generated code is refreshed through the engine's module code generation, which
// calls back into Codegen with the module's visible schema.
func (sdk *RustSDK) GenerateScope(
	ctx context.Context,
	ws *dagger.Workspace,
	isModule bool,
	name string,
	clients []*dagger.ModuleSource,
) (*dagger.Workspace, error) {
	scope, err := workspaceCwd(ctx, ws)
	if err != nil {
		return nil, err
	}
	if !isConfinedWorkspacePath(scope) {
		return nil, fmt.Errorf("SDK scope is not confined to the workspace")
	}
	// The engine schedules a scope without a module only to generate the clients
	// recorded for it, and records a module scope's dependencies as its clients.
	// Neither has a Rust generation contract under the SDK-module interface, so
	// both are refused before any workspace edit.
	if !isModule {
		return nil, fmt.Errorf("the Rust SDK does not generate module clients; remove the clients recorded for scope %q", scope)
	}
	if len(clients) > 0 {
		return nil, fmt.Errorf("the Rust SDK does not generate module dependencies; remove the clients recorded for module scope %q", scope)
	}

	configured := ws
	hasManifest, err := workspaceHasAnyFile(ctx, ws, "dagger-module.toml", "dagger.json")
	if err != nil {
		return nil, err
	}
	if !hasManifest {
		// Only scaffolding runs a Rust operation here; regeneration reaches Codegen
		// through the engine, which resolves the content root in that call.
		sdk, err = sdk.resolved(ctx)
		if err != nil {
			return nil, err
		}
		configured, err = sdk.initializedModuleScope(ctx, ws, scope, name)
		if err != nil {
			return nil, err
		}
	}
	generated, err := generatedModuleCode(ctx, configured, scope)
	if err != nil {
		return nil, err
	}
	return configured.WithDirectory(".", generated), nil
}

// initializedModuleScope scaffolds a new Rust module in the scope and writes its
// module configuration. The SDK-module interface leaves module configuration to
// the SDK, and the Rust initialization plan refuses to write it, so the adapter
// authors it.
func (sdk *RustSDK) initializedModuleScope(
	ctx context.Context,
	ws *dagger.Workspace,
	scope string,
	name string,
) (*dagger.Workspace, error) {
	scaffold, err := sdk.moduleScaffold(ctx, ws, scope, name)
	if err != nil {
		return nil, err
	}
	engineVersion, err := dag.Version(ctx)
	if err != nil {
		return nil, fmt.Errorf("read engine version: %w", err)
	}
	runtime, err := installedRuntimeSource(ctx, ws)
	if err != nil {
		return nil, err
	}
	manifest := moduleManifest(name, engineVersion, runtime)
	return ws.WithDirectory(".", scaffold).
		WithNewFile("dagger-module.toml", manifest, dagger.WorkspaceWithNewFileOpts{Permissions: 0o644}), nil
}

// moduleScaffold returns the scope directory after the Rust initialization plan.
// The plan sees the whole workspace so it can adopt an existing Cargo project, but
// it writes only beneath the module root.
func (sdk *RustSDK) moduleScaffold(
	ctx context.Context,
	ws *dagger.Workspace,
	scope string,
	name string,
) (*dagger.Directory, error) {
	moduleRoot, err := rebasePath(scope)
	if err != nil {
		return nil, fmt.Errorf("module path: %w", err)
	}
	descriptor, err := sdk.descriptorMetadata(ctx)
	if err != nil {
		return nil, err
	}
	request := map[string]any{
		"request_kind": "initialize-module",
		"request": map[string]any{
			"format_version": 1,
			"target":         targetIdentity(descriptor),
			"module": map[string]any{
				"name":           name,
				"original_name":  name,
				"source_subpath": moduleRoot,
				"config_format":  "current",
				"source_digest":  metadata.DigestModuleSource("initialization:" + scope),
			},
			"package_name":   name,
			"sdk_dependency": descriptor.SDKDependency,
		},
	}
	execution, err := sdk.executeOperation(ctx, request, nil, ws.Directory("/"), "initialization")
	if err != nil {
		return nil, err
	}
	initialized := execution.project.Directory(operationWorkspaceRoot)
	if scope == "." {
		return initialized, nil
	}
	return initialized.Directory(scope), nil
}

// installedRuntimeSource names the runtime a new module loads: the reference this
// SDK module was installed from. Modules resolve a local runtime path against
// their own directory, so only a remote reference keeps its meaning when copied
// into a module configuration.
func installedRuntimeSource(ctx context.Context, ws *dagger.Workspace) (string, error) {
	moduleName, err := dag.CurrentModule().Name(ctx)
	if err != nil {
		return "", fmt.Errorf("read Rust SDK module name: %w", err)
	}
	source, err := ws.Module(moduleName).Source(ctx)
	if err != nil {
		return "", fmt.Errorf("read the workspace installation of %q: %w", moduleName, err)
	}
	if !isRemoteModuleReference(source) {
		return "", fmt.Errorf("Rust SDK %q is installed from a local path; install it from a Git reference to initialize modules", moduleName)
	}
	return source, nil
}

// isRemoteModuleReference follows the engine's classification of a module source
// (core/gitref.FastKindCheck and core/workspace.IsLocalRef @
// 249edbd646e63c5181053c8af72623ed9bdf030d): a leading path marker or a dot-free
// source is a workspace path, and a Git scheme or a host ahead of the first
// separator names a remote reference.
func isRemoteModuleReference(source string) bool {
	switch {
	case strings.HasPrefix(source, "/"), strings.HasPrefix(source, "."):
		return false
	case strings.HasPrefix(source, "http://"), strings.HasPrefix(source, "https://"),
		strings.HasPrefix(source, "git://"), strings.HasPrefix(source, "ssh://"):
		return true
	case !strings.Contains(source, "."):
		return false
	}
	host, _, _ := strings.Cut(strings.ReplaceAll(source, `\`, "/"), "/")
	if _, afterUser, scpLike := strings.Cut(host, "@"); scpLike {
		host = afterUser
	}
	if strings.Contains(host, ".") || strings.Contains(host, ":") {
		return true
	}
	// Azure DevOps Server references name a dot-free host with a _git segment.
	return strings.Contains(source, "/_git/")
}

// moduleManifest renders the configuration of a new Rust module in the layout
// the engine wrote when it authored module configuration. The engine version is
// the running engine's release without build metadata.
func moduleManifest(name string, engineVersion string, runtime string) string {
	release, _, _ := strings.Cut(engineVersion, "+")
	return "name = " + tomlString(name) + "\n" +
		"engineVersion = " + tomlString(release) + "\n\n" +
		"[runtime]\n" +
		"  source = " + tomlString(runtime) + "\n"
}

// tomlString encodes a TOML basic string, escaping every character the format
// does not admit literally.
func tomlString(value string) string {
	var encoded strings.Builder
	encoded.WriteByte('"')
	for _, character := range value {
		switch {
		case character == '"':
			encoded.WriteString(`\"`)
		case character == '\\':
			encoded.WriteString(`\\`)
		case character < 0x20 || character == 0x7f:
			fmt.Fprintf(&encoded, `\u%04X`, character)
		default:
			encoded.WriteRune(character)
		}
	}
	encoded.WriteByte('"')
	return encoded.String()
}

// generatedModuleCode returns the scope directory after the engine's module code
// generation, without the module configuration this SDK owns in the scope.
func generatedModuleCode(ctx context.Context, ws *dagger.Workspace, scope string) (*dagger.Directory, error) {
	sourcePath := "/"
	if scope != "." {
		sourcePath = "/" + scope
	}
	generated := ws.WithWorkdir(".").ModuleSource(sourcePath).GeneratedContextDirectory()
	if scope != "." {
		generated = generated.Directory(scope)
	}
	for _, manifest := range []string{"dagger-module.toml", "dagger.json"} {
		present, err := generated.Exists(ctx, manifest)
		if err != nil {
			return nil, fmt.Errorf("inspect generated module scope: %w", err)
		}
		if present {
			generated = generated.WithoutFile(manifest)
		}
	}
	return generated, nil
}

func workspaceHasFile(ctx context.Context, ws *dagger.Workspace, name string) (bool, error) {
	present, err := ws.Directory(".", dagger.WorkspaceDirectoryOpts{Include: []string{name}}).Exists(ctx, name)
	if err != nil {
		return false, fmt.Errorf("inspect workspace scope for %s: %w", name, err)
	}
	return present, nil
}

func workspaceHasAnyFile(ctx context.Context, ws *dagger.Workspace, names ...string) (bool, error) {
	for _, name := range names {
		present, err := workspaceHasFile(ctx, ws, name)
		if err != nil || present {
			return present, err
		}
	}
	return false, nil
}

func workspaceCwd(ctx context.Context, ws *dagger.Workspace) (string, error) {
	if ws == nil {
		return "", fmt.Errorf("workspace not provided")
	}
	cwd, err := ws.Cwd(ctx)
	if err != nil {
		return "", fmt.Errorf("read workspace cwd: %w", err)
	}
	return normalizeWorkspacePath(cwd), nil
}

func normalizeWorkspacePath(candidate string) string {
	trimmed := strings.Trim(candidate, "/")
	if trimmed == "" {
		return "."
	}
	return path.Clean(trimmed)
}

func isConfinedWorkspacePath(candidate string) bool {
	if path.IsAbs(candidate) {
		return false
	}
	normalized := normalizeWorkspacePath(candidate)
	return normalized != ".." && !strings.HasPrefix(normalized, "../") && !strings.Contains(candidate, "\\")
}

// engineDescriptor returns the opaque Rust-owned descriptor. Go forwards these bytes
// to the packaged tool and deliberately does not reinterpret dependency provenance.
func (sdk *RustSDK) engineDescriptor() *dagger.File {
	return sdk.SDKSourceDir.File(engineDescriptorPath)
}

func (sdk *RustSDK) engineTool() *dagger.File {
	return sdk.SDKSourceDir.File(engineToolPath)
}

func (sdk *RustSDK) clientGenerationMetadata(ctx context.Context) (metadata.ClientGeneration, error) {
	contents, err := sdk.SDKSourceDir.File(clientGenerationPath).Contents(ctx)
	if err != nil {
		return metadata.ClientGeneration{}, fmt.Errorf("read packaged client-generation metadata: %w", err)
	}
	return metadata.DecodeClientGeneration([]byte(contents))
}

// Codegen compiles the engine-visible schema into the scoped module context and
// returns the Rust-owned VCS policy emitted by the same operation plan. The engine
// writes that policy into the module's own .gitattributes and .gitignore, so its
// paths are relative to the module source root rather than the context.
func (sdk *RustSDK) Codegen(
	ctx context.Context,
	modSource *dagger.ModuleSource,
	introspectionJSON *dagger.File,
) (*dagger.GeneratedCode, error) {
	sdk, resolveErr := sdk.resolved(ctx)
	if resolveErr != nil {
		return nil, resolveErr
	}
	if introspectionJSON == nil {
		return nil, fmt.Errorf("introspection JSON not provided")
	}
	identity, err := moduleIdentity(ctx, modSource, "current")
	if err != nil {
		return nil, err
	}
	request, _, err := sdk.generationRequest(ctx, "generate-module", identity, introspectionJSON, identity.SourceSubpath)
	if err != nil {
		return nil, err
	}
	execution, err := sdk.executeOperation(ctx, request, introspectionJSON, modSource.ContextDirectory(), "generation")
	if err != nil {
		return nil, err
	}
	generated, err := metadata.StripOperationRoot(execution.result.VCSGenerated, identity.SourceSubpath)
	if err != nil {
		return nil, err
	}
	ignored, err := metadata.StripOperationRoot(execution.result.VCSIgnored, identity.SourceSubpath)
	if err != nil {
		return nil, err
	}
	return dag.GeneratedCode(execution.project.Directory(operationWorkspaceRoot)).
		WithVCSGeneratedPaths(generated).
		WithVCSIgnoredPaths(ignored), nil
}

// RequiredClientGenerationFiles returns the renderer-owned finite host input set.
func (sdk *RustSDK) RequiredClientGenerationFiles(ctx context.Context) ([]string, error) {
	sdk, resolveErr := sdk.resolved(ctx)
	if resolveErr != nil {
		return nil, resolveErr
	}
	metadata, err := sdk.clientGenerationMetadata(ctx)
	if err != nil {
		return nil, err
	}
	return metadata.RequiredHostFiles, nil
}

// GenerateClient renders the standalone client only within the requested output
// subtree of the scoped module context.
func (sdk *RustSDK) GenerateClient(
	ctx context.Context,
	modSource *dagger.ModuleSource,
	introspectionJSON *dagger.File,
	outputDir string,
) (*dagger.Directory, error) {
	sdk, resolveErr := sdk.resolved(ctx)
	if resolveErr != nil {
		return nil, resolveErr
	}
	if introspectionJSON == nil {
		return nil, fmt.Errorf("introspection JSON not provided")
	}
	identity, err := moduleIdentity(ctx, modSource, "current")
	if err != nil {
		return nil, err
	}
	outputRoot, err := rebasePath(outputDir)
	if err != nil {
		return nil, fmt.Errorf("client output directory: %w", err)
	}
	request, _, err := sdk.generationRequest(ctx, "generate-client", identity, introspectionJSON, outputRoot)
	if err != nil {
		return nil, err
	}
	execution, err := sdk.executeOperation(ctx, request, introspectionJSON, modSource.ContextDirectory(), "generation")
	if err != nil {
		return nil, err
	}
	return execution.project.Directory(operationWorkspaceRoot), nil
}

type scopedModuleIdentity struct {
	Name          string
	OriginalName  string
	SourceSubpath string
	SourceDigest  string
	ConfigFormat  string
	ResolvedPin   string
}

type operationExecution struct {
	project *dagger.Directory
	result  metadata.ExecutionResult
}

func (sdk *RustSDK) executeOperation(
	ctx context.Context,
	request any,
	schema *dagger.File,
	project *dagger.Directory,
	expectedKind string,
) (operationExecution, error) {
	requestBytes, err := metadata.CanonicalJSON(request)
	if err != nil {
		return operationExecution{}, fmt.Errorf("encode Rust operation request: %w", err)
	}
	policy, err := sdk.runtimePolicyMetadata(ctx)
	if err != nil {
		return operationExecution{}, err
	}
	descriptor, err := sdk.descriptorMetadata(ctx)
	if err != nil {
		return operationExecution{}, err
	}
	rustTarget, err := runtimeTarget(ctx, policy)
	if err != nil {
		return operationExecution{}, err
	}
	formatterInstallPath := path.Join(
		"/usr/local/rustup/toolchains",
		descriptor.RustToolchain+"-"+rustTarget,
		"bin/rustfmt",
	)
	container := dag.Container().From(policy.BuildImage).
		WithFile(operationToolPath, sdk.engineTool(), dagger.ContainerWithFileOpts{Permissions: 0o755}).
		// rustfmt resolves private toolchain libraries relative to its executable.
		// Retaining the rustup layout keeps that lookup valid without an ambient proxy.
		WithFile(formatterInstallPath, sdk.SDKSourceDir.File(formatterToolPath), dagger.ContainerWithFileOpts{Permissions: 0o755}).
		WithFile(operationDescriptorPath, sdk.engineDescriptor()).
		WithDirectory(operationProjectPath, dag.Directory().WithDirectory(operationWorkspaceRoot, project)).
		WithNewFile(operationRequestPath, string(requestBytes))
	args := []string{
		operationToolPath, "execute",
		"--request", operationRequestPath,
		"--descriptor", operationDescriptorPath,
		"--project", operationProjectPath,
		"--result", operationResultPath,
	}
	if schema != nil {
		container = container.WithFile(operationSchemaPath, schema)
		args = append(args, "--schema", operationSchemaPath)
	}
	executed, err := container.WithExec(args).Sync(ctx)
	if err != nil {
		return operationExecution{}, fmt.Errorf("execute Rust SDK operation: %w", err)
	}
	resultBytes, err := executed.File(operationResultPath).Contents(ctx)
	if err != nil {
		return operationExecution{}, fmt.Errorf("read Rust SDK operation result: %w", err)
	}
	result, err := metadata.DecodeExecutionResult([]byte(resultBytes), expectedKind)
	if err != nil {
		return operationExecution{}, err
	}
	return operationExecution{project: executed.Directory(operationProjectPath), result: result}, nil
}

func (sdk *RustSDK) generationRequest(
	ctx context.Context,
	operation string,
	module scopedModuleIdentity,
	schema *dagger.File,
	outputRoot string,
) (map[string]any, []byte, error) {
	descriptor, err := sdk.descriptorMetadata(ctx)
	if err != nil {
		return nil, nil, err
	}
	contents, err := schema.Contents(ctx)
	if err != nil {
		return nil, nil, fmt.Errorf("read visible schema: %w", err)
	}
	schemaBytes := []byte(contents)
	return generationRequestDocument(descriptor, operation, module, schemaBytes, outputRoot), schemaBytes, nil
}

// generationRequestDocument is deliberately data-only: Rust owns source discovery,
// descriptor construction, registration, codecs, dispatch, and evidence semantics.
func generationRequestDocument(
	descriptor metadata.EngineSource,
	operation string,
	module scopedModuleIdentity,
	schemaBytes []byte,
	outputRoot string,
) map[string]any {
	moduleDocument := map[string]any{
		"name":           module.Name,
		"original_name":  module.OriginalName,
		"source_subpath": module.SourceSubpath,
		"config_format":  module.ConfigFormat,
		"source_digest":  module.SourceDigest,
	}
	if module.ResolvedPin != "" {
		moduleDocument["resolved_pin"] = module.ResolvedPin
	}
	return map[string]any{
		"request_kind": "generate",
		"request": map[string]any{
			"format_version": 1,
			"operation":      operation,
			"target":         targetIdentity(descriptor),
			"visible_schema": map[string]any{
				"path":   "schema.json",
				"digest": metadata.DigestBytes(schemaBytes),
			},
			"module":         moduleDocument,
			"sdk_dependency": descriptor.SDKDependency,
			"output_root":    outputRoot,
		},
	}
}

func (sdk *RustSDK) descriptorMetadata(ctx context.Context) (metadata.EngineSource, error) {
	contents, err := sdk.engineDescriptor().Contents(ctx)
	if err != nil {
		return metadata.EngineSource{}, fmt.Errorf("read packaged engine descriptor: %w", err)
	}
	return metadata.DecodeEngineSource([]byte(contents))
}

func (sdk *RustSDK) runtimePolicyMetadata(ctx context.Context) (metadata.RuntimePolicy, error) {
	contents, err := sdk.SDKSourceDir.File(runtimePolicyPath).Contents(ctx)
	if err != nil {
		return metadata.RuntimePolicy{}, fmt.Errorf("read packaged runtime policy: %w", err)
	}
	return metadata.DecodeRuntimePolicy([]byte(contents))
}

func targetIdentity(descriptor metadata.EngineSource) map[string]any {
	return map[string]any{
		"format_version":     descriptor.FormatVersion,
		"repository":         descriptor.Repository,
		"dagger_revision":    descriptor.DaggerRevision,
		"engine_version":     descriptor.EngineVersion,
		"rust_sdk_version":   descriptor.RustSDKVersion,
		"rust_toolchain":     descriptor.RustToolchain,
		"core_schema_digest": descriptor.CoreSchemaDigest,
	}
}

func moduleIdentity(ctx context.Context, source *dagger.ModuleSource, configFormat string) (scopedModuleIdentity, error) {
	if source == nil {
		return scopedModuleIdentity{}, fmt.Errorf("module source not provided")
	}
	name, err := source.ModuleName(ctx)
	if err != nil {
		return scopedModuleIdentity{}, fmt.Errorf("read module name: %w", err)
	}
	originalName, err := source.ModuleOriginalName(ctx)
	if err != nil {
		return scopedModuleIdentity{}, fmt.Errorf("read original module name: %w", err)
	}
	subpath, err := source.SourceSubpath(ctx)
	if err != nil {
		return scopedModuleIdentity{}, fmt.Errorf("read module source subpath: %w", err)
	}
	rebased, err := rebasePath(subpath)
	if err != nil {
		return scopedModuleIdentity{}, fmt.Errorf("module source subpath: %w", err)
	}
	// Dagger's directory digest retains empty parent directories created solely by
	// generated paths. Hashing a canonical leaf-file inventory keeps every authored
	// input while preventing that incidental scaffolding from making a fresh manifest
	// stale. The operation manifest separately authenticates the excluded products.
	digest, err := semanticModuleSourceDigest(ctx, source, subpath)
	if err != nil {
		return scopedModuleIdentity{}, fmt.Errorf("read semantic module source digest: %w", err)
	}
	kind, err := source.Kind(ctx)
	if err != nil {
		return scopedModuleIdentity{}, fmt.Errorf("read module source kind: %w", err)
	}
	requiresPin, err := moduleSourceRequiresPin(kind)
	if err != nil {
		return scopedModuleIdentity{}, err
	}
	resolvedPin := ""
	if requiresPin {
		resolvedPin, err = source.Pin(ctx)
		if err != nil {
			return scopedModuleIdentity{}, fmt.Errorf("read resolved module pin: %w", err)
		}
		if err := validateOptionalRevision(resolvedPin); err != nil || resolvedPin == "" {
			return scopedModuleIdentity{}, fmt.Errorf("resolved remote module pin is invalid")
		}
	}
	return scopedModuleIdentity{
		Name: name, OriginalName: originalName, SourceSubpath: rebased,
		SourceDigest: digest, ConfigFormat: configFormat, ResolvedPin: resolvedPin,
	}, nil
}

// Directory sources are immutable Dagger snapshots and local sources are bound by
// the semantic leaf digest above. Only Git sources need a separately authenticated
// resolved commit in the operation request.
func moduleSourceRequiresPin(kind dagger.ModuleSourceKind) (bool, error) {
	switch kind {
	case dagger.ModuleSourceKindGit:
		return true, nil
	case dagger.ModuleSourceKindLocal, dagger.ModuleSourceKindDir:
		return false, nil
	default:
		return false, fmt.Errorf("module source kind is unsupported")
	}
}

func validateOptionalRevision(value string) error {
	if value == "" {
		return nil
	}
	if len(value) != 40 {
		return fmt.Errorf("revision must be a full lowercase commit")
	}
	for _, character := range value {
		if (character < '0' || character > '9') && (character < 'a' || character > 'f') {
			return fmt.Errorf("revision must be a full lowercase commit")
		}
	}
	return nil
}

func semanticModuleSourceDigest(
	ctx context.Context,
	source *dagger.ModuleSource,
	subpath string,
) (string, error) {
	// The engine writes its canonical config representation after codegen. Overlaying
	// that representation before enumeration keeps config semantics in the identity
	// without coupling the manifest to pre- versus post-write serialization.
	normalizedContext := source.ContextDirectory().WithDirectory("/", source.UpdatedConfigDirectory())
	semantic := normalizedContext.Directory(subpath).Filter(dagger.DirectoryFilterOpts{
		Exclude: []string{
			".dagger/rust", ".dagger/rust/**",
			"src/dagger_generated", "src/dagger_generated/**",
			"src/bin/dagger-module.rs",
			".gitattributes", ".gitignore",
		},
	})
	paths, err := semantic.Glob(ctx, "**")
	if err != nil {
		return "", fmt.Errorf("enumerate semantic module source: %w", err)
	}
	sort.Strings(paths)
	files := make([]metadata.ModuleSourceFile, 0, len(paths))
	for _, candidate := range paths {
		if strings.HasSuffix(candidate, "/") {
			continue
		}
		digest, err := semantic.File(candidate).Digest(ctx, dagger.FileDigestOpts{ExcludeMetadata: true})
		if err != nil {
			return "", fmt.Errorf("digest semantic module source path %s: %w", candidate, err)
		}
		files = append(files, metadata.ModuleSourceFile{Path: candidate, Digest: digest})
	}
	return metadata.DigestModuleSourceFiles(files)
}

// The private operation root always has a named first component because Rust rejects
// an empty capability path. Rebasing does not change caller-visible result paths.
func rebasePath(candidate string) (string, error) {
	return metadata.RebaseOperationPath(candidate)
}
