//! Generated bindings owned by the GraphQL `RegistryProtocol` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "Transport protocol to use for registry operations."]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RegistryProtocol {
    #[doc = "GraphQL enum value `HTTP`."]
    #[serde(rename = "HTTP")]
    Http,
    #[doc = "GraphQL enum value `HTTPS`."]
    #[serde(rename = "HTTPS")]
    Https,
}
