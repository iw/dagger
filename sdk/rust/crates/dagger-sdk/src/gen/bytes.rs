//! Generated bindings owned by the GraphQL `Bytes` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "Arbitrary binary data, represented as a base64-encoded string."]
#[derive(Clone, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(transparent)]
pub struct Bytes(
    #[doc = r" Exact string retained for GraphQL request and response encoding."] pub String,
);
impl From<String> for Bytes {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for Bytes {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}
