//! Generated bindings owned by the GraphQL `Port` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
#[doc = "A port exposed by a container."]
#[derive(Clone)]
pub struct Port {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for Port {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for Port {
    fn graphql_type() -> &'static str {
        "Port"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<Port> for crate::IdInput<Port> {
    fn from(value: Port) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<Port> for crate::IdInput<super::NodeClient> {
    fn from(value: Port) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl Port {
    #[doc = "The port description.\n\nSelects GraphQL field `description` on `Port`."]
    pub async fn description(&self) -> Result<Option<String>, crate::QueryError> {
        let query = self.selection.select("description");
        query.execute(&self.session).await
    }
    #[doc = "Skip the health check when run as a service.\n\nSelects GraphQL field `experimentalSkipHealthcheck` on `Port`."]
    pub async fn experimental_skip_healthcheck(&self) -> Result<bool, crate::QueryError> {
        let query = self.selection.select("experimentalSkipHealthcheck");
        query.execute(&self.session).await
    }
    #[doc = "A unique identifier for this Port.\n\nSelects GraphQL field `id` on `Port`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "The port number.\n\nSelects GraphQL field `port` on `Port`."]
    pub async fn port(&self) -> Result<i64, crate::QueryError> {
        let query = self.selection.select("port");
        query.execute(&self.session).await
    }
    #[doc = "The transport layer protocol.\n\nSelects GraphQL field `protocol` on `Port`."]
    pub async fn protocol(&self) -> Result<super::NetworkProtocol, crate::QueryError> {
        let query = self.selection.select("protocol");
        query.execute(&self.session).await
    }
}
impl super::Node for Port {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
