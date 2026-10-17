//! Generated bindings owned by the GraphQL `GitPushDisposition` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "How a Git push updated the remote ref."]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum GitPushDisposition {
    #[doc = "The remote ref was created."]
    #[serde(rename = "CREATED")]
    Created,
    #[doc = "The remote ref was fast-forwarded."]
    #[serde(rename = "FAST_FORWARD")]
    FastForward,
    #[doc = "The remote ref was replaced under an explicit lease."]
    #[serde(rename = "FORCED")]
    Forced,
    #[doc = "The remote ref already pointed to this commit."]
    #[serde(rename = "UP_TO_DATE")]
    UpToDate,
}
