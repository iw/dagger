//! Generated bindings owned by the GraphQL `FileType` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
#[doc = "File type."]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum FileType {
    #[doc = "directory file type"]
    #[serde(rename = "DIRECTORY", alias = "DIRECTORY_TYPE")]
    Directory,
    #[doc = "regular file type"]
    #[serde(rename = "REGULAR", alias = "REGULAR_TYPE")]
    Regular,
    #[doc = "symlink file type"]
    #[serde(rename = "SYMLINK", alias = "SYMLINK_TYPE")]
    Symlink,
    #[doc = "unknown file type"]
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
