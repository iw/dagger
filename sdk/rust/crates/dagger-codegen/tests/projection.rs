//! Exact-target semantic projection regressions.

use std::collections::BTreeMap;

use dagger_codegen::directive::{DirectiveApplicationPolicy, DirectivePolicy};
use dagger_codegen::projection::catalog::BindingKind;
use dagger_codegen::projection::fields::FieldStrategy;
use dagger_codegen::projection::types::TypeProjection;
use dagger_codegen::target::CodegenTarget;
use dagger_codegen::{CoreProjectionRequest, project_core, render_core};

const TARGET: &[u8] = include_bytes!("../../../codegen/target.json");
const SCHEMA: &[u8] = include_bytes!("../../../codegen/schema.json");

fn plan() -> dagger_codegen::ProjectionPlan {
    let target = CodegenTarget::decode_exact(TARGET).expect("checked target must decode");
    project_core(CoreProjectionRequest {
        target: &target,
        schema_json: SCHEMA,
    })
    .expect("checked target must project completely")
}

#[test]
fn exact_target_has_one_total_field_strategy() {
    let plan = plan();
    let mut strategies = BTreeMap::<&str, usize>::new();
    for field in plan.fields().values() {
        let name = match field.strategy {
            FieldStrategy::LazyHandle { .. } => "lazy",
            FieldStrategy::NullableHandle { .. } => "nullable",
            FieldStrategy::ReenterList { .. } => "reenter",
            FieldStrategy::ExecuteValue { .. } => "execute",
            FieldStrategy::ExpectedTypeSelf { .. } => "self",
            FieldStrategy::TargetPrivate => "private",
        };
        *strategies.entry(name).or_default() += 1;
    }

    assert_eq!(plan.fields().len(), 901);
    assert_eq!(strategies["lazy"], 375);
    assert_eq!(strategies["nullable"], 34);
    assert_eq!(strategies["reenter"], 60);
    assert_eq!(strategies["execute"], 412);
    assert_eq!(strategies["self"], 20);
    // The claimed surface no longer holds excluded-type references, so the
    // target-private strategy class is empty (schema refresh at v1.0.0-beta.11).
    assert!(!strategies.contains_key("private"));
}

#[test]
fn exact_target_named_types_and_edges_are_exhaustive() {
    let plan = plan();
    let mut kinds = BTreeMap::<&str, usize>::new();
    for projection in plan.named_types().values() {
        let kind = match projection {
            TypeProjection::Scalar(_) => "scalar",
            TypeProjection::Object(_) => "object",
            TypeProjection::Interface(_) => "interface",
            TypeProjection::Enum(_) => "enum",
            TypeProjection::InputObject(_) => "input",
            TypeProjection::TargetPrivate(_) => "private",
        };
        *kinds.entry(kind).or_default() += 1;
    }

    assert_eq!(plan.named_types().len(), 132);
    assert_eq!(kinds["scalar"], 9);
    assert_eq!(kinds["object"], 90);
    assert_eq!(kinds["interface"], 3);
    assert_eq!(kinds["enum"], 25);
    assert_eq!(kinds["input"], 5);
    // Same refresh: no named type projects as target-private any more.
    assert!(!kinds.contains_key("private"));
    assert_eq!(plan.implementations().len(), 105);
}

#[test]
fn directive_projection_accounts_for_active_aliases_and_inactive_definitions() {
    let plan = plan();
    let mut expected_types = 0;
    let mut deprecations = 0;
    let mut experimental = 0;
    let mut aliases = 0;
    let mut source_maps = 0;
    let mut reevaluations = 0;
    let mut workflow_roles = 0;
    let mut inactive = 0;
    for record in plan.directives().records().values() {
        if record.policy == DirectivePolicy::TargetInactive {
            inactive += 1;
            assert!(record.applications.is_empty());
            assert!(
                record
                    .definition_fingerprint
                    .as_str()
                    .starts_with("sha256:")
            );
        }
        for application in record.applications.values() {
            match application {
                DirectiveApplicationPolicy::ExpectedType { .. } => expected_types += 1,
                DirectiveApplicationPolicy::Deprecated { .. } => deprecations += 1,
                DirectiveApplicationPolicy::Experimental { .. } => experimental += 1,
                DirectiveApplicationPolicy::EnumValueAlias { .. } => aliases += 1,
                DirectiveApplicationPolicy::SourceMap { .. } => source_maps += 1,
                DirectiveApplicationPolicy::Reevaluate => reevaluations += 1,
                DirectiveApplicationPolicy::WorkflowRole { .. } => workflow_roles += 1,
            }
        }
    }

    assert_eq!(plan.directives().records().len(), 18);
    assert_eq!(expected_types, 123);
    assert_eq!(deprecations, 26);
    assert_eq!(experimental, 38);
    assert_eq!(aliases, 23);
    assert_eq!(source_maps, 0);
    assert_eq!(reevaluations, 40);
    assert_eq!(workflow_roles, 2);
    assert_eq!(inactive, 10);
}

#[test]
fn enums_account_for_every_coordinate_without_duplicate_alias_variants() {
    let plan = plan();
    let (variants, aliases) = plan
        .named_types()
        .values()
        .filter_map(|projection| {
            if let TypeProjection::Enum(enumeration) = projection {
                Some((enumeration.variants.len(), enumeration.aliases.len()))
            } else {
                None
            }
        })
        .fold((0, 0), |(variants, aliases), next| {
            (variants + next.0, aliases + next.1)
        });

    assert_eq!(variants, 90);
    assert_eq!(aliases, 23);
    assert_eq!(variants + aliases, 113);
}

#[test]
fn catalog_has_only_exact_semantic_keys() {
    let plan = plan();
    let mut kinds = BTreeMap::<BindingKind, usize>::new();
    for (key, descriptor) in plan.catalog().bindings() {
        assert_eq!(key, &descriptor.key);
        assert!(
            descriptor
                .implementation_fingerprint
                .as_str()
                .starts_with("sha256:")
        );
        *kinds.entry(key.binding_kind).or_default() += 1;
    }

    assert_eq!(plan.catalog().bindings().len(), 2_094);
    assert_eq!(kinds[&BindingKind::QueryRoot], 1);
    assert_eq!(kinds[&BindingKind::Scalar], 9);
    assert_eq!(kinds[&BindingKind::ObjectHandle], 90);
    assert_eq!(kinds[&BindingKind::InterfaceTrait], 3);
    assert_eq!(kinds[&BindingKind::InterfaceClient], 3);
    assert_eq!(kinds[&BindingKind::InterfaceImplementation], 105);
    assert_eq!(kinds[&BindingKind::Enum], 25);
    assert_eq!(kinds[&BindingKind::EnumVariant], 90);
    assert_eq!(kinds[&BindingKind::EnumAlias], 23);
    assert_eq!(kinds[&BindingKind::InputObject], 5);
    assert_eq!(kinds[&BindingKind::InputField], 22);
    assert_eq!(kinds[&BindingKind::FieldOperation], 901);
    assert!(!kinds.contains_key(&BindingKind::TargetPrivateType));
    assert!(!kinds.contains_key(&BindingKind::TargetPrivateField));
    assert_eq!(kinds[&BindingKind::Argument], 784);
    assert_eq!(kinds[&BindingKind::DirectivePolicy], 18);
    assert_eq!(kinds[&BindingKind::DirectiveArgument], 15);
}

#[test]
fn source_free_rendering_contains_only_generated_rust_artifacts() {
    let candidate = render_core(&plan()).expect("complete plan must render a client candidate");

    assert!(
        candidate
            .artifacts()
            .keys()
            .all(|path| path.ends_with(".rs"))
    );
    assert!(
        candidate
            .artifacts()
            .contains_key("crates/dagger-sdk/src/gen/mod.rs")
    );
    assert!(
        candidate
            .artifacts()
            .contains_key("crates/dagger-sdk/tests/core_reachability.rs")
    );
    assert!(
        candidate
            .artifacts()
            .contains_key("crates/dagger-sdk/tests/core_projection.rs")
    );
    assert!(!candidate.artifacts().contains_key("canonical-schema.json"));
    assert!(
        !candidate
            .artifacts()
            .contains_key("semantic-projection.json")
    );
}
