//! Generated bindings owned by the GraphQL `LLMSkill` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
#[doc = "A skill available to a model: task-specific guidance discovered with ListSkills and read with ReadSkill."]
#[derive(Clone)]
pub struct LlmSkill {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for LlmSkill {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for LlmSkill {
    fn graphql_type() -> &'static str {
        "LLMSkill"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<LlmSkill> for crate::IdInput<LlmSkill> {
    fn from(value: LlmSkill) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<LlmSkill> for crate::IdInput<super::NodeClient> {
    fn from(value: LlmSkill) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl LlmSkill {
    #[doc = "The one-line description from the SKILL.md frontmatter.\n\nSelects GraphQL field `description` on `LLMSkill`."]
    pub async fn description(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("description");
        query.execute(&self.session).await
    }
    #[doc = "A unique identifier for this LLMSkill.\n\nSelects GraphQL field `id` on `LLMSkill`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "The skill name, as passed to ReadSkill.\n\nSelects GraphQL field `name` on `LLMSkill`."]
    pub async fn name(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("name");
        query.execute(&self.session).await
    }
}
impl super::Node for LlmSkill {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
