//! Generated bindings owned by the GraphQL `ArtifactDimension` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "Lazy handle for GraphQL object `ArtifactDimension`."]
#[derive(Clone)]
pub struct ArtifactDimension {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for ArtifactDimension {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for ArtifactDimension {
    fn graphql_type() -> &'static str {
        "ArtifactDimension"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<ArtifactDimension> for crate::IdInput<ArtifactDimension> {
    fn from(value: ArtifactDimension) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<ArtifactDimension> for crate::IdInput<super::NodeClient> {
    fn from(value: ArtifactDimension) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl ArtifactDimension {
    #[doc = "The collection type, or null for a static dimension.\n\nSelects GraphQL field `collectionType` on `ArtifactDimension`."]
    pub async fn collection_type(&self) -> Result<Option<String>, crate::QueryError> {
        let query = self.selection.select("collectionType");
        query.execute(&self.session).await
    }
    #[doc = "A unique identifier for this ArtifactDimension.\n\nSelects GraphQL field `id` on `ArtifactDimension`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "Stable identifier: collection schema path, type:TypeName for an artifact type, or module.\n\nSelects GraphQL field `identifier` on `ArtifactDimension`."]
    pub async fn identifier(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("identifier");
        query.execute(&self.session).await
    }
    #[doc = "The collection item or artifact type name, or empty for the module dimension.\n\nSelects GraphQL field `itemType` on `ArtifactDimension`."]
    pub async fn item_type(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("itemType");
        query.execute(&self.session).await
    }
    #[doc = "The collection key argument description, or empty for a static dimension.\n\nSelects GraphQL field `keyDescription` on `ArtifactDimension`."]
    pub async fn key_description(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("keyDescription");
        query.execute(&self.session).await
    }
    #[doc = "The collection key argument name, or name for a static dimension.\n\nSelects GraphQL field `keyName` on `ArtifactDimension`."]
    pub async fn key_name(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("keyName");
        query.execute(&self.session).await
    }
    #[doc = "How this dimension gets its keys.\n\nSelects GraphQL field `kind` on `ArtifactDimension`."]
    pub async fn kind(&self) -> Result<super::ArtifactDimensionKind, crate::QueryError> {
        let query = self.selection.select("kind");
        query.execute(&self.session).await
    }
    #[doc = "Short name used to select this dimension.\n\nSelects GraphQL field `name` on `ArtifactDimension`."]
    pub async fn name(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("name");
        query.execute(&self.session).await
    }
    #[doc = "Qualified name used when the short name is ambiguous.\n\nSelects GraphQL field `qualifiedName` on `ArtifactDimension`."]
    pub async fn qualified_name(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("qualifiedName");
        query.execute(&self.session).await
    }
}
impl super::Node for ArtifactDimension {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
