//! Generated bindings owned by the GraphQL `PatchConflict` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "How to handle patch hunks that no longer apply to the target content."]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum PatchConflict {
    #[doc = "Fail the operation if any part of the patch does not apply."]
    #[serde(rename = "FAIL")]
    Fail,
    #[doc = "Apply the hunks that fit and insert conflict markers where hunks no longer match, instead of failing."]
    #[serde(rename = "LEAVE_CONFLICT_MARKERS")]
    LeaveConflictMarkers,
}
