//! Generated bindings owned by the GraphQL `CurrentModuleAsSDKClient` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
#[doc = "A generated client the current SDK produces in the workspace."]
#[derive(Clone)]
pub struct CurrentModuleAsSdkClient {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for CurrentModuleAsSdkClient {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for CurrentModuleAsSdkClient {
    fn graphql_type() -> &'static str {
        "CurrentModuleAsSDKClient"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<CurrentModuleAsSdkClient> for crate::IdInput<CurrentModuleAsSdkClient> {
    fn from(value: CurrentModuleAsSdkClient) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<CurrentModuleAsSdkClient> for crate::IdInput<super::NodeClient> {
    fn from(value: CurrentModuleAsSdkClient) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl CurrentModuleAsSdkClient {
    #[doc = "A unique identifier for this CurrentModuleAsSDKClient.\n\nSelects GraphQL field `id` on `CurrentModuleAsSDKClient`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "The module the client is bound to (workspace-relative path or canonical ref).\n\nSelects GraphQL field `module` on `CurrentModuleAsSDKClient`."]
    pub async fn module(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("module");
        query.execute(&self.session).await
    }
    #[doc = "The resolved module source this client is bound to, including its dependency closure and pinned version.\n\nSelects GraphQL field `moduleSource` on `CurrentModuleAsSDKClient`."]
    #[must_use]
    pub fn module_source(&self) -> super::ModuleSource {
        let query = self.selection.select("moduleSource");
        super::ModuleSource {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Workspace-root-relative path of the generated client.\n\nSelects GraphQL field `path` on `CurrentModuleAsSDKClient`."]
    pub async fn path(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("path");
        query.execute(&self.session).await
    }
    #[doc = "The pinned version of the bound module, if any.\n\nSelects GraphQL field `pin` on `CurrentModuleAsSDKClient`."]
    pub async fn pin(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("pin");
        query.execute(&self.session).await
    }
}
impl super::Node for CurrentModuleAsSdkClient {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
