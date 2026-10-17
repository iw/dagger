//! Engine-free production-stack regeneration properties for initialized modules.

// Shared schema-fixture DSL, compiled into this crate a second time: its
// include_bytes! paths are file-relative and its proptest regression paths resolve
// per consuming crate, both intended. Only one item is consumed here, so the rest of
// the DSL is expectedly unused.
#[path = "../../dagger-codegen/tests/support/mod.rs"]
mod codegen_support;

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use codegen_support::module_visible_schema;
use dagger_sdk_engine::initialization::{InitializationInputs, plan_initialization};
use dagger_sdk_engine::post_work::{Cancellation, ProcessOutcome};
use dagger_sdk_engine::{
    CanonicalRegistry, CanonicalRepositoryUrl, EngineSourceDescriptor, ExactRustToolchain,
    ExactVersion, FormatVersion, FullRevision, ModuleConfigFormat, ModuleOperationInput,
    OperationKind, OperationPostWork, OperationRequest, OperationRoot, PostWorkPlan,
    PublishedSdkDependency, RelativeOperationPath, SchemaInput, SdkPackageName, Sha256Digest,
    StableCoordinate, TargetIdentity, ToolchainSelection,
};
use sha2::{Digest as _, Sha256};

const REVISION: &str = "a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94";
const CORE_SCHEMA_DIGEST: &str =
    "sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe";
const MODULE_ROOT: &str = "workspace/greeter";

#[derive(Clone, Copy, Debug)]
struct HostRustfmt;

impl OperationPostWork for HostRustfmt {
    async fn execute(
        &self,
        root: &OperationRoot,
        plan: &PostWorkPlan,
        _environment: &BTreeMap<String, String>,
        cancel: &Cancellation,
    ) -> Result<ProcessOutcome, dagger_sdk_engine::EngineDiagnostic> {
        assert!(!cancel.is_cancelled());
        let PostWorkPlan::FormatRust { files, .. } = plan else {
            panic!("module rendering admits only formatter post-work")
        };
        let mut command = Command::new("rustup");
        command.args(["run", "1.97.1", "rustfmt", "--edition", "2024"]);
        for file in files {
            command.arg(root.regular_file(file).unwrap());
        }
        let output = command.output().unwrap();
        Ok(ProcessOutcome {
            success: output.status.success(),
            stdout: output.stdout,
            stderr: output.stderr,
            truncated: false,
        })
    }
}

// Identical authored input regenerates identical bytes, and an authored edit
// regenerates once and then converges again: generated output never feeds back into
// the authoring identity it is rendered from.
#[tokio::test]
async fn production_stack_regenerates_initialized_modules_idempotently() {
    let temporary = tempfile::tempdir().unwrap();
    let module_root = RelativeOperationPath::parse(MODULE_ROOT).unwrap();
    let toolchain = ToolchainSelection::TargetDefault {
        toolchain: "1.97.1".parse().unwrap(),
    };
    let plan = plan_initialization(InitializationInputs {
        module_root: &module_root,
        package_name: "greeter",
        manifest: None,
        starter_source: None,
        gitignore: None,
        gitattributes: None,
        dependency: &dependency(),
        toolchain: &toolchain,
        dependency_resolved: true,
        lockfile_present: false,
    })
    .unwrap();
    for (path, bytes) in &plan.files {
        let destination = temporary.path().join(path.as_str());
        fs::create_dir_all(destination.parent().unwrap()).unwrap();
        fs::write(destination, bytes).unwrap();
    }
    let library = temporary.path().join(MODULE_ROOT).join("src/lib.rs");
    let mut authored = fs::read_to_string(&library).unwrap();
    authored.push_str(
        "\n#[dagger_sdk::methods]\nimpl ModuleRoot {\n    #[dagger(function)]\n    pub fn tree(&self, source: dagger_sdk::Directory) -> dagger_sdk::Directory {\n        source\n    }\n}\n",
    );
    fs::write(&library, &authored).unwrap();

    let root = OperationRoot::open(temporary.path()).unwrap();
    let schema = module_visible_schema(0);
    let generate = |source: &[u8]| request(&schema, source);

    let first = execute(&root, &generate(authored.as_bytes()), &schema).await;
    assert!(!first.is_empty());
    let generated = snapshot(temporary.path());
    let replay = execute(&root, &generate(authored.as_bytes()), &schema).await;
    assert!(replay.is_empty(), "replay touched {replay:?}");
    assert_eq!(snapshot(temporary.path()), generated);

    authored.push_str(
        "\n#[dagger_sdk::methods]\nimpl ModuleRoot {\n    #[dagger(function)]\n    pub fn greeting(&self) -> String {\n        \"hello\".to_owned()\n    }\n}\n",
    );
    fs::write(&library, &authored).unwrap();
    let edited = execute(&root, &generate(authored.as_bytes()), &schema).await;
    assert!(!edited.is_empty());
    let regenerated = snapshot(temporary.path());
    let converged = execute(&root, &generate(authored.as_bytes()), &schema).await;
    assert!(
        converged.is_empty(),
        "converged replay touched {converged:?}"
    );
    assert_eq!(snapshot(temporary.path()), regenerated);
}

async fn execute(root: &OperationRoot, request: &OperationRequest, schema: &[u8]) -> Vec<String> {
    dagger_sdk_engine::execute_operation_with_post_work(
        root,
        request,
        schema,
        &descriptor(),
        &Cancellation::default(),
        &HostRustfmt,
    )
    .await
    .unwrap()
    .touched_paths
    .iter()
    .map(ToString::to_string)
    .collect()
}

// The adapter's semantic source digest changes exactly when authored source changes;
// hashing the authored library models that identity for this single-file module.
fn request(schema: &[u8], authored_source: &[u8]) -> OperationRequest {
    OperationRequest {
        format_version: FormatVersion,
        operation: OperationKind::GenerateModule,
        target: target(),
        visible_schema: SchemaInput {
            path: RelativeOperationPath::parse("schema.json").unwrap(),
            digest: hash(schema),
        },
        module: Some(ModuleOperationInput {
            name: StableCoordinate::new("greeter").unwrap(),
            original_name: StableCoordinate::new("greeter").unwrap(),
            source_subpath: RelativeOperationPath::parse(MODULE_ROOT).unwrap(),
            config_format: ModuleConfigFormat::Current,
            source_digest: hash(authored_source),
            resolved_pin: None,
        }),
        sdk_dependency: dependency(),
        output_root: RelativeOperationPath::parse(MODULE_ROOT).unwrap(),
    }
}

fn descriptor() -> EngineSourceDescriptor {
    let target = target();
    EngineSourceDescriptor {
        format_version: FormatVersion,
        repository: target.repository,
        dagger_revision: target.dagger_revision,
        engine_version: target.engine_version,
        rust_sdk_version: target.rust_sdk_version,
        rust_toolchain: target.rust_toolchain,
        sdk_dependency: dependency(),
        core_schema_digest: target.core_schema_digest,
        packaged_asset_manifest_digest: hash(b"module generation fixture"),
    }
}

fn target() -> TargetIdentity {
    TargetIdentity {
        format_version: FormatVersion,
        repository: CanonicalRepositoryUrl::new("https://github.com/dagger/dagger").unwrap(),
        dagger_revision: FullRevision::new(REVISION).unwrap(),
        engine_version: ExactVersion::new("1.0.0-beta.11.rust.5").unwrap(),
        rust_sdk_version: ExactVersion::new("1.0.0-beta.11.rust.5").unwrap(),
        rust_toolchain: ExactRustToolchain::new("1.97.1").unwrap(),
        core_schema_digest: Sha256Digest::new(CORE_SCHEMA_DIGEST).unwrap(),
    }
}

fn dependency() -> PublishedSdkDependency {
    PublishedSdkDependency::Registry {
        registry: CanonicalRegistry::new("crates-io").unwrap(),
        package: SdkPackageName::new("dagger-sdk").unwrap(),
        exact_version: ExactVersion::new("1.0.0-beta.11.rust.5").unwrap(),
    }
}

fn hash(bytes: &[u8]) -> Sha256Digest {
    Sha256Digest::new(format!("sha256:{:x}", Sha256::digest(bytes))).unwrap()
}

fn snapshot(root: &Path) -> Vec<(String, Vec<u8>)> {
    fn visit(root: &Path, at: &Path, output: &mut Vec<(String, Vec<u8>)>) {
        let mut entries = fs::read_dir(at)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        entries.sort_by_key(fs::DirEntry::file_name);
        for entry in entries {
            if entry.file_type().unwrap().is_dir() {
                visit(root, &entry.path(), output);
            } else {
                output.push((
                    entry
                        .path()
                        .strip_prefix(root)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/"),
                    fs::read(entry.path()).unwrap(),
                ));
            }
        }
    }
    let mut output = Vec::new();
    visit(root, root, &mut output);
    output
}
