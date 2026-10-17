//! Generated bindings owned by the GraphQL `WorkspaceCommitPickReason` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "Why a source commit cannot be pulled."]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum WorkspaceCommitPickReason {
    #[doc = "The patch conflicts with committed content."]
    #[serde(rename = "CONTENT")]
    Content,
    #[doc = "The commit touches uncommitted paths in the receiving workspace."]
    #[serde(rename = "DIRTY")]
    Dirty,
    #[doc = "No conflict."]
    #[serde(rename = "NONE")]
    None,
}
