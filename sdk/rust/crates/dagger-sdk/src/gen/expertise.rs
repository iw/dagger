//! Generated bindings owned by the GraphQL `Expertise` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "An agent function that can modify a conversation."]
#[derive(Clone)]
pub struct Expertise {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for Expertise {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for Expertise {
    fn graphql_type() -> &'static str {
        "Expertise"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<Expertise> for crate::IdInput<Expertise> {
    fn from(value: Expertise) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<Expertise> for crate::IdInput<super::NodeClient> {
    fn from(value: Expertise) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl Expertise {
    #[doc = "The agent function's description.\n\nSelects GraphQL field `description` on `Expertise`."]
    pub async fn description(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("description");
        query.execute(&self.session).await
    }
    #[doc = "A unique identifier for this Expertise.\n\nSelects GraphQL field `id` on `Expertise`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "The agent function's name.\n\nSelects GraphQL field `name` on `Expertise`."]
    pub async fn name(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("name");
        query.execute(&self.session).await
    }
    #[doc = "The module that defines the agent function.\n\nSelects GraphQL field `originalModule` on `Expertise`."]
    #[must_use]
    pub fn original_module(&self) -> super::Module {
        let query = self.selection.select("originalModule");
        super::Module {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "The agent function's path within its module.\n\nSelects GraphQL field `path` on `Expertise`."]
    pub async fn path(&self) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("path");
        query.execute(&self.session).await
    }
}
impl super::Node for Expertise {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
