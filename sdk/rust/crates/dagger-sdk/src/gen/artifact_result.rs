//! Generated bindings owned by the GraphQL `ArtifactResult` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "Lazy handle for GraphQL object `ArtifactResult`."]
#[derive(Clone)]
pub struct ArtifactResult {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for ArtifactResult {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for ArtifactResult {
    fn graphql_type() -> &'static str {
        "ArtifactResult"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<ArtifactResult> for crate::IdInput<ArtifactResult> {
    fn from(value: ArtifactResult) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<ArtifactResult> for crate::IdInput<super::NodeClient> {
    fn from(value: ArtifactResult) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl ArtifactResult {
    #[doc = "The artifact that was evaluated.\n\nSelects GraphQL field `artifact` on `ArtifactResult`."]
    #[must_use]
    pub fn artifact(&self) -> super::Artifact {
        let query = self.selection.select("artifact");
        super::Artifact {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "The evaluation failure, if any.\n\nSelects GraphQL field `error` on `ArtifactResult`."]
    pub async fn error(&self) -> Result<Option<super::Error>, crate::QueryError> {
        let query = self.selection.select("error");
        let query = query.select("id");
        query
            .execute_reentry::<super::Error, Option<crate::Id>>(&self.session, "Error")
            .await
    }
    #[doc = "A unique identifier for this ArtifactResult.\n\nSelects GraphQL field `id` on `ArtifactResult`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "The evaluated value, or null when evaluation failed.\n\nSelects GraphQL field `value` on `ArtifactResult`."]
    pub async fn value(&self) -> Result<Option<super::NodeClient>, crate::QueryError> {
        let query = self.selection.select("value");
        let query = query.select("id");
        query
            .execute_reentry::<super::NodeClient, Option<crate::Id>>(&self.session, "Node")
            .await
    }
}
impl super::Node for ArtifactResult {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
