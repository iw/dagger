//! Generated bindings owned by the GraphQL `FileType` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
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
