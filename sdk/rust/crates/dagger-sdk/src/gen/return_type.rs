//! Generated bindings owned by the GraphQL `ReturnType` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
#[doc = "Expected return type of an execution"]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ReturnType {
    #[doc = "Any execution (exit codes 0-127 and 192-255)"]
    #[serde(rename = "ANY")]
    Any,
    #[doc = "A failed execution (exit codes 1-127 and 192-255)"]
    #[serde(rename = "FAILURE")]
    Failure,
    #[doc = "A successful execution (exit code 0)"]
    #[serde(rename = "SUCCESS")]
    Success,
}
