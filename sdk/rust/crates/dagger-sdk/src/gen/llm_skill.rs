//! Generated bindings owned by the GraphQL `LLMSkill` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
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
