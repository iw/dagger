//! Generated bindings owned by GraphQL coordinate `MinimalToken`.
// @generated {"format":"dagger-rust-standalone-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:2313896b06186ada0116cc4af16aa742e847b856ead2a66135e65754a9af334c","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "Client fixture scalar MinimalToken."]
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    dagger_sdk::__private::serde::Deserialize,
    dagger_sdk::__private::serde::Serialize,
)]
#[serde(crate = "dagger_sdk::__private::serde", transparent)]
pub struct Token(
    /// Exact string retained for GraphQL request and response encoding.
    pub String,
);
impl From<String> for Token {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for Token {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}
