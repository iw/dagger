//! Generated bindings owned by the GraphQL `PatchConflict` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
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
