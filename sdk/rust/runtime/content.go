package main

import (
	"context"
	"encoding/json"
	"fmt"
	"regexp"
	"strings"

	"rust-sdk/internal/dagger"
)

const (
	// rustSDKBuildImage matches the digest-pinned builder the engine-packaged content
	// uses, so a self-built content root compiles the same helper from the same inputs.
	rustSDKBuildImage = "rust:1.97.1-bookworm@sha256:705e294093973d7c10e83400393dce7b3611f8e03e55a80af7fff6d02ae1affb"
	// sdkDependencyPath, inside this runtime's own source, names the repository,
	// commit, and workspace path of the SDK crates generated projects depend on. A
	// commit cannot state its own hash, so a later commit records it.
	sdkDependencyPath = "assets/sdk-dependency.json"
)

var fullRevision = regexp.MustCompile(`^[0-9a-f]{40}$`)

type checkedTarget struct {
	DaggerRepository string `json:"dagger_repository"`
	DaggerRevision   string `json:"dagger_revision"`
	EngineVersion    string `json:"engine_version"`
	RustSDKVersion   string `json:"rust_sdk_version"`
	RustVersion      string `json:"rust_version"`
	SchemaDigest     string `json:"schema_digest"`
}

type sdkDependency struct {
	Repository string `json:"repository"`
	Revision   string `json:"revision"`
	Workspace  string `json:"workspace"`
}

// resolved returns this SDK with a complete content root. An engine that packages
// the Rust SDK passes one with dist/ already sealed. Installed as an ordinary
// module, the runtime cannot take its workspace from a default path: with a
// workspace bound into the call, engines resolve default paths against that
// workspace rather than this module's source. It instead reads its pinned SDK
// dependency from its own source and seals the content from that exact commit, so
// the helper is built from the crates generated projects depend on.
func (sdk *RustSDK) resolved(ctx context.Context) (*RustSDK, error) {
	if sdk.SDKSourceDir != nil {
		packaged, err := sdk.SDKSourceDir.Exists(ctx, engineDescriptorPath)
		if err != nil {
			return nil, fmt.Errorf("inspect Rust SDK content root: %w", err)
		}
		if packaged {
			return sdk, nil
		}
	}
	var dependency sdkDependency
	if err := readJSON(ctx, dag.CurrentModule().Source().File(sdkDependencyPath), &dependency); err != nil {
		return nil, fmt.Errorf("read Rust SDK dependency coordinates: %w", err)
	}
	if !strings.HasPrefix(dependency.Repository, "https://") || strings.Contains(dependency.Repository, "@") ||
		!fullRevision.MatchString(dependency.Revision) || dependency.Workspace == "" {
		return nil, fmt.Errorf("Rust SDK dependency must name a credential-free HTTPS repository, one full commit, and a workspace path")
	}
	workspace := dag.Git(dependency.Repository).
		Commit(dependency.Revision).
		Tree(dagger.GitCommitTreeOpts{DiscardGitDir: true}).
		Directory(dependency.Workspace)
	content, err := sealWorkspaceContent(ctx, workspace, dependency)
	if err != nil {
		return nil, err
	}
	return &RustSDK{SDKSourceDir: content}, nil
}

func sealWorkspaceContent(ctx context.Context, workspace *dagger.Directory, dependency sdkDependency) (*dagger.Directory, error) {
	var target checkedTarget
	if err := readJSON(ctx, workspace.File("codegen/target.json"), &target); err != nil {
		return nil, fmt.Errorf("read Rust SDK checked target: %w", err)
	}
	if target.DaggerRepository == "" || !fullRevision.MatchString(target.DaggerRevision) ||
		target.RustVersion == "" || target.RustSDKVersion == "" || target.SchemaDigest == "" {
		return nil, fmt.Errorf("Rust SDK checked target is incomplete")
	}
	platform, err := dag.DefaultPlatform(ctx)
	if err != nil {
		return nil, fmt.Errorf("resolve engine platform: %w", err)
	}
	rustTarget, err := rustTargetTriple(string(platform))
	if err != nil {
		return nil, err
	}
	rustfmtPath := fmt.Sprintf("/usr/local/rustup/toolchains/%s-%s/bin/rustfmt", target.RustVersion, rustTarget)

	rustWorkspace := workspace.Filter(dagger.DirectoryFilterOpts{
		Include: []string{
			"Cargo.toml", "Cargo.lock", "rust-toolchain.toml",
			"codegen/target.json", "codegen/schema.json", "codegen/generated.json",
			"crates/**/Cargo.toml", "crates/**/src/**/*.rs", "crates/**/assets/**",
		},
	})
	buildCtr := dag.Container(dagger.ContainerOpts{Platform: platform}).
		From(rustSDKBuildImage).
		WithDirectory("/src", rustWorkspace).
		WithWorkdir("/src").
		WithExec([]string{"rustup", "component", "add", "rustfmt", "--toolchain", target.RustVersion}).
		WithExec([]string{
			"cargo", "build", "--release", "--locked", "--package", "dagger-sdk-engine", "--bin", "dagger-rust-engine",
		})

	runtimeSource := workspace.Directory(sdkRuntimePath).Filter(dagger.DirectoryFilterOpts{
		Include: []string{
			"dagger-module.toml", "go.mod", "go.sum", "*.go", "internal/**/*.go",
		},
		Exclude: []string{"**/*_test.go"},
	})
	rootfs := dag.Directory().
		WithDirectory(sdkRuntimePath, runtimeSource).
		WithFile(engineToolPath, buildCtr.File("/src/target/release/dagger-rust-engine"), dagger.DirectoryWithFileOpts{Permissions: 0o755}).
		WithFile(formatterToolPath, buildCtr.File(rustfmtPath), dagger.DirectoryWithFileOpts{Permissions: 0o755}).
		WithFile(clientGenerationPath, workspace.File("crates/dagger-codegen/assets/client-generation.json")).
		WithFile(runtimePolicyPath, workspace.File("runtime/assets/runtime-policy.json")).
		WithFile("LICENSE", workspace.File("LICENSE"))

	sealed := buildCtr.
		// A scratch mount makes the package tool's writes available as a clean
		// directory output without retaining builder layers.
		WithMountedDirectory("/content", rootfs).
		WithExec([]string{
			"/src/target/release/dagger-rust-engine", "package-content",
			"--root", "/content",
			"--repository", "https://" + target.DaggerRepository,
			"--dagger-revision", target.DaggerRevision,
			"--engine-version", strings.TrimPrefix(target.EngineVersion, "v"),
			"--rust-sdk-version", target.RustSDKVersion,
			"--rust-toolchain", target.RustVersion,
			"--core-schema-digest", target.SchemaDigest,
			"--dependency-kind", "git",
			"--dependency-value", dependency.Revision,
			"--dependency-repository", dependency.Repository,
		})
	digest, err := sealed.Stdout(ctx)
	if err != nil {
		return nil, fmt.Errorf("seal Rust SDK content: %w", err)
	}
	if !strings.HasPrefix(strings.TrimSpace(digest), "sha256:") {
		return nil, fmt.Errorf("Rust SDK descriptor identity is absent or malformed")
	}
	return sealed.Directory("/content"), nil
}

func rustTargetTriple(platform string) (string, error) {
	switch {
	case strings.HasPrefix(platform, "linux/amd64"):
		return "x86_64-unknown-linux-gnu", nil
	case strings.HasPrefix(platform, "linux/arm64"):
		return "aarch64-unknown-linux-gnu", nil
	default:
		return "", fmt.Errorf("Rust SDK content does not support engine platform %q", platform)
	}
}

func readJSON(ctx context.Context, file *dagger.File, value any) error {
	contents, err := file.Contents(ctx)
	if err != nil {
		return err
	}
	return json.Unmarshal([]byte(contents), value)
}
