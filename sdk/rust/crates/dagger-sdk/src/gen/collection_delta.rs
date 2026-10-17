//! Generated bindings owned by the GraphQL `CollectionDelta` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "Lazy handle for GraphQL object `CollectionDelta`."]
#[derive(Clone)]
pub struct CollectionDelta {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for CollectionDelta {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for CollectionDelta {
    fn graphql_type() -> &'static str {
        "CollectionDelta"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<CollectionDelta> for crate::IdInput<CollectionDelta> {
    fn from(value: CollectionDelta) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<CollectionDelta> for crate::IdInput<super::NodeClient> {
    fn from(value: CollectionDelta) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl CollectionDelta {
    #[doc = "Current keys absent from the original collection, in current order.\n\nSelects GraphQL field `addedKeys` on `CollectionDelta`."]
    pub async fn added_keys(&self) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("addedKeys");
        query.execute(&self.session).await
    }
    #[doc = "A unique identifier for this CollectionDelta.\n\nSelects GraphQL field `id` on `CollectionDelta`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "Original keys absent from the current collection, in original order.\n\nSelects GraphQL field `removedKeys` on `CollectionDelta`."]
    pub async fn removed_keys(&self) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("removedKeys");
        query.execute(&self.session).await
    }
}
impl super::Node for CollectionDelta {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
