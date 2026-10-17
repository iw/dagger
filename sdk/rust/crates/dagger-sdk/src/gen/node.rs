//! Generated bindings owned by the GraphQL `Node` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "An object with a globally unique ID."]
pub trait Node: Clone + Send + Sync {
    #[doc = "Selects GraphQL field `id` on `Node`."]
    fn id(&self)
    -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send;
}
#[doc = "Lazy client handle for GraphQL interface `Node`."]
#[derive(Clone)]
pub struct NodeClient {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for NodeClient {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for NodeClient {
    fn graphql_type() -> &'static str {
        "Node"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<NodeClient> for crate::IdInput<NodeClient> {
    fn from(value: NodeClient) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl NodeClient {
    #[doc = "Selects GraphQL field `id` on `Node`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
}
impl super::Node for NodeClient {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
