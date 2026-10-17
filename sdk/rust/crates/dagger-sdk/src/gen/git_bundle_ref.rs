//! Generated bindings owned by the GraphQL `GitBundleRef` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "A ref advertised by a Git bundle."]
#[derive(Clone)]
pub struct GitBundleRef {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for GitBundleRef {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for GitBundleRef {
    fn graphql_type() -> &'static str {
        "GitBundleRef"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<GitBundleRef> for crate::IdInput<GitBundleRef> {
    fn from(value: GitBundleRef) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<GitBundleRef> for crate::IdInput<super::NodeClient> {
    fn from(value: GitBundleRef) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl GitBundleRef {
    #[doc = "A unique identifier for this GitBundleRef.\n\nSelects GraphQL field `id` on `GitBundleRef`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "The advertised ref name.\n\nSelects GraphQL field `name` on `GitBundleRef`."]
    pub async fn name(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("name");
        query.execute(&self.session).await
    }
    #[doc = "The object ID the advertised ref resolves to.\n\nSelects GraphQL field `sha` on `GitBundleRef`."]
    pub async fn sha(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("sha");
        query.execute(&self.session).await
    }
}
impl super::Node for GitBundleRef {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
