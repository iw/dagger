//! Generated bindings owned by the GraphQL `NetworkProtocol` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "Transport layer network protocol associated to a port."]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum NetworkProtocol {
    #[doc = "GraphQL enum value `TCP`."]
    #[serde(rename = "TCP")]
    Tcp,
    #[doc = "GraphQL enum value `UDP`."]
    #[serde(rename = "UDP")]
    Udp,
}
