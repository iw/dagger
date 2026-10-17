//! Generated bindings owned by the GraphQL `ModuleSourceKind` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "The kind of module source."]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ModuleSourceKind {
    #[doc = "GraphQL enum value `DIR_SOURCE`."]
    #[serde(rename = "DIR_SOURCE", alias = "DIR")]
    DirSource,
    #[doc = "GraphQL enum value `GIT_SOURCE`."]
    #[serde(rename = "GIT_SOURCE", alias = "GIT")]
    GitSource,
    #[doc = "GraphQL enum value `LOCAL_SOURCE`."]
    #[serde(rename = "LOCAL_SOURCE", alias = "LOCAL")]
    LocalSource,
}
