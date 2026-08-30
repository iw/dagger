//! Generated bindings owned by the GraphQL `DiffStatKind` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
#[doc = "The type of change for a diff stat entry."]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum DiffStatKind {
    #[doc = "A file or directory was added."]
    #[serde(rename = "ADDED")]
    Added,
    #[doc = "A file was modified."]
    #[serde(rename = "MODIFIED")]
    Modified,
    #[doc = "A file or directory was removed."]
    #[serde(rename = "REMOVED")]
    Removed,
    #[doc = "A file was renamed."]
    #[serde(rename = "RENAMED")]
    Renamed,
}
