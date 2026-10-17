//! Generated bindings owned by the GraphQL `ArtifactDimensionKey` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "Lazy handle for GraphQL object `ArtifactDimensionKey`."]
#[derive(Clone)]
pub struct ArtifactDimensionKey {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for ArtifactDimensionKey {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for ArtifactDimensionKey {
    fn graphql_type() -> &'static str {
        "ArtifactDimensionKey"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<ArtifactDimensionKey> for crate::IdInput<ArtifactDimensionKey> {
    fn from(value: ArtifactDimensionKey) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<ArtifactDimensionKey> for crate::IdInput<super::NodeClient> {
    fn from(value: ArtifactDimensionKey) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl ArtifactDimensionKey {
    #[doc = "The dimension identifier, fixed across the workspace schema.\n\nSelects GraphQL field `dimension` on `ArtifactDimensionKey`."]
    pub async fn dimension(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("dimension");
        query.execute(&self.session).await
    }
    #[doc = "A unique identifier for this ArtifactDimensionKey.\n\nSelects GraphQL field `id` on `ArtifactDimensionKey`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "The dimension item's key.\n\nSelects GraphQL field `key` on `ArtifactDimensionKey`."]
    pub async fn key(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("key");
        query.execute(&self.session).await
    }
}
impl super::Node for ArtifactDimensionKey {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
