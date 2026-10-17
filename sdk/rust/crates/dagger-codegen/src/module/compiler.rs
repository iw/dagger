//! All-or-nothing module-authoring compilation.
//!
//! This facade composes the pure discovery, type, descriptor, projection, and render
//! phases over one immutable request. No phase publishes bytes, and an error discards
//! every intermediate value so callers cannot observe a partially projected module.

use std::collections::{BTreeMap, BTreeSet};

use super::descriptor::{DescriptorBuilder, DescriptorInput};
use super::diagnostic::{ModuleDiagnostic, ModuleDiagnosticCode, ModuleDiagnosticSet};
use super::metadata::{CompiledFunction, FunctionCompiler};
use super::model::{
    GeneratedAssetPath, ModuleDescriptor, ModuleIntrospection, ModuleSourceSnapshot, ModuleTarget,
    RegistrationProjection, RustSymbol, Sha256Digest,
};
use super::projection::ProjectionCompiler;
use super::render::{ModuleRenderRequest, ModuleRenderer, RenderedModuleAssets};
use super::source::{GeneratedTypeRegistry, ModuleDiscovery, SourceDiscovery};
use super::types::{TypeCatalog, TypeResolver};

/// Immutable inputs to one complete engine-free compilation.
pub struct ModuleCompilationRequest<'a> {
    /// Exact engine, SDK, schema, and Rust target identity.
    pub target: &'a ModuleTarget,
    /// Canonical selected package source.
    pub source: &'a ModuleSourceSnapshot,
    /// Checked generated-type registry for the visible schema.
    pub generated_types: &'a GeneratedTypeRegistry,
    /// Core and dependency type names already occupying the visible schema.
    pub visible_type_names: &'a BTreeSet<super::model::WireName>,
    /// Immutable module compiler and renderer identity.
    pub generator_digest: &'a Sha256Digest,
    /// Actual Cargo alias through which generated support reaches `dagger-sdk`.
    pub sdk_dependency_alias: &'a str,
    /// Checked visible bindings copied without invoking Core generation.
    pub checked_bindings: &'a BTreeMap<GeneratedAssetPath, Vec<u8>>,
}

/// Complete successful pure compilation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleCompilation {
    /// Sole canonical semantic model.
    pub descriptor: ModuleDescriptor,
    /// Engine registration view derived from the descriptor.
    pub registration: RegistrationProjection,
    /// Structurally equivalent checked introspection.
    pub introspection: ModuleIntrospection,
    /// Complete in-memory generated candidate tree.
    pub assets: RenderedModuleAssets,
}

/// Pure compiler facade for the authoring pipeline.
pub struct ModuleCompiler;

impl ModuleCompiler {
    /// Compiles one immutable request without filesystem, process, network, or engine I/O.
    pub fn compile(
        request: ModuleCompilationRequest<'_>,
    ) -> Result<ModuleCompilation, ModuleDiagnosticSet> {
        if request.generated_types.expected_digest() != &request.target.visible_schema_digest {
            return Err(singleton(diag(
                ModuleDiagnosticCode::GeneratedTypeStale,
                "the generated-type registry does not match the selected visible schema",
                "load checked bindings from the exact target visible schema",
            )));
        }

        let discovery = SourceDiscovery::discover(request.source, request.generated_types)?;
        let catalog = TypeCatalog::compile(&discovery)?;
        let resolver = TypeResolver::new(&discovery);
        let compiler = FunctionCompiler::new(resolver, &catalog);
        let mut functions = BTreeMap::<RustSymbol, Vec<CompiledFunction>>::new();
        for declaration in discovery.declarations.values() {
            functions.insert(
                declaration.rust_symbol.clone(),
                compiler.compile_all(
                    &declaration.rust_symbol,
                    &discovery.root,
                    &declaration.functions,
                )?,
            );
        }
        let descriptor = DescriptorBuilder::build(DescriptorInput {
            target: request.target,
            source: request.source,
            discovery: &discovery,
            catalog: &catalog,
            functions: &functions,
            generator_digest: request.generator_digest,
        })?;
        let projections = ProjectionCompiler::project(&descriptor, request.visible_type_names)?;
        let generated_type_paths = generated_type_paths(&discovery)?;
        let assets = ModuleRenderer::render(ModuleRenderRequest {
            descriptor: &descriptor,
            registration: &projections.registration,
            introspection: &projections.introspection,
            sdk_dependency_alias: request.sdk_dependency_alias,
            checked_bindings: request.checked_bindings,
            generated_type_paths: &generated_type_paths,
        })?;
        Ok(ModuleCompilation {
            descriptor,
            registration: projections.registration,
            introspection: projections.introspection,
            assets,
        })
    }
}

// Every generated type in the descriptor was resolved from one checked binding in
// the discovery closure, so its binding path is the spelling generated code must use.
fn generated_type_paths(
    discovery: &ModuleDiscovery,
) -> Result<BTreeMap<String, String>, ModuleDiagnosticSet> {
    let mut paths = BTreeMap::new();
    for binding in discovery.generated_types.values() {
        if paths
            .insert(binding.target_name.clone(), binding.rust_path.clone())
            .is_some_and(|previous| previous != binding.rust_path)
        {
            return Err(singleton(diag(
                ModuleDiagnosticCode::RustPathInvalid,
                "one target type resolved through more than one generated Rust path",
                "name each checked generated type through one Rust path",
            )));
        }
    }
    Ok(paths)
}

fn diag(
    code: ModuleDiagnosticCode,
    message: &'static str,
    remediation: &'static str,
) -> ModuleDiagnostic {
    ModuleDiagnostic::new(code, None, message, remediation)
        .expect("static compiler diagnostics satisfy the safe renderer policy")
}

fn singleton(diagnostic: ModuleDiagnostic) -> ModuleDiagnosticSet {
    ModuleDiagnosticSet::new([diagnostic])
        .expect("a singleton compiler diagnostic set is non-empty")
}
