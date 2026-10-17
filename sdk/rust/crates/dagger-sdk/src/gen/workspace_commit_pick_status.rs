//! Generated bindings owned by the GraphQL `WorkspaceCommitPickStatus` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "Whether a source commit can be pulled."]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum WorkspaceCommitPickStatus {
    #[doc = "The commit cannot be applied; see reason and conflictPaths."]
    #[serde(rename = "CONFLICT")]
    Conflict,
    #[doc = "The commit can be applied."]
    #[serde(rename = "PICKABLE")]
    Pickable,
    #[doc = "The commit is already present by hash or cherry-pick origin."]
    #[serde(rename = "PICKED")]
    Picked,
    #[doc = "The patch is already present, or applying it would be empty."]
    #[serde(rename = "REDUNDANT")]
    Redundant,
}
