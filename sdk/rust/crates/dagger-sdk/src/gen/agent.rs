//! Generated bindings owned by the GraphQL `Agent` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
#[doc = "Lazy handle for GraphQL object `Agent`."]
#[derive(Clone)]
pub struct Agent {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for Agent {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for Agent {
    fn graphql_type() -> &'static str {
        "Agent"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<Agent> for crate::IdInput<Agent> {
    fn from(value: Agent) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<Agent> for crate::IdInput<super::NodeClient> {
    fn from(value: Agent) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl Agent {
    #[doc = "The description of the agent\n\nSelects GraphQL field `description` on `Agent`."]
    pub async fn description(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("description");
        query.execute(&self.session).await
    }
    #[doc = "A unique identifier for this Agent.\n\nSelects GraphQL field `id` on `Agent`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "Return the fully qualified name of the agent\n\nSelects GraphQL field `name` on `Agent`."]
    pub async fn name(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("name");
        query.execute(&self.session).await
    }
    #[doc = "The original module in which the agent has been defined\n\nSelects GraphQL field `originalModule` on `Agent`."]
    #[must_use]
    pub fn original_module(&self) -> super::Module {
        let query = self.selection.select("originalModule");
        super::Module {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "The path of the agent within its module\n\nSelects GraphQL field `path` on `Agent`."]
    pub async fn path(&self) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("path");
        query.execute(&self.session).await
    }
}
impl super::Node for Agent {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
