//! Generated bindings owned by the GraphQL `AgentGroup` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
#[doc = "Lazy handle for GraphQL object `AgentGroup`."]
#[derive(Clone)]
pub struct AgentGroup {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
#[doc = "Owned optional arguments for GraphQL operation `AgentGroup.compose`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct AgentGroupComposeOpts {
    #[doc = "The base LLM to compose onto. Defaults to a fresh workspace-bound LLM.\n\n`None` omits GraphQL field `base`."]
    pub base: Option<crate::IdInput<super::Llm>>,
}
impl AgentGroupComposeOpts {
    #[doc = "Sets GraphQL argument `base` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_base(mut self, value: crate::IdInput<super::Llm>) -> Self {
        self.base = Some(value);
        self
    }
}
impl crate::IntoID<crate::Id> for AgentGroup {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for AgentGroup {
    fn graphql_type() -> &'static str {
        "AgentGroup"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<AgentGroup> for crate::IdInput<AgentGroup> {
    fn from(value: AgentGroup) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<AgentGroup> for crate::IdInput<super::NodeClient> {
    fn from(value: AgentGroup) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl AgentGroup {
    #[doc = "Compose all selected agent middlewares onto a base LLM, in alphabetical module:fn order, and return the composed LLM.\n\nSelects GraphQL field `compose` on `AgentGroup`."]
    #[must_use]
    pub fn compose(&self) -> super::Llm {
        let query = self.selection.select("compose");
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `compose` with a borrowed, reusable `AgentGroupComposeOpts` value."]
    #[must_use]
    pub fn compose_opts(&self, opts: &AgentGroupComposeOpts) -> super::Llm {
        let query = self.selection.select("compose");
        let query = if let Some(value) = &opts.base {
            query.arg_id_input("base", value.clone())
        } else {
            query
        };
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "A unique identifier for this AgentGroup.\n\nSelects GraphQL field `id` on `AgentGroup`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "Return a list of individual agents and their details\n\nSelects GraphQL field `list` on `AgentGroup`."]
    pub async fn list(&self) -> Result<Vec<super::Agent>, crate::QueryError> {
        let query = self.selection.select("list");
        let query = query.select("id");
        query
            .execute_reentry::<super::Agent, Vec<crate::Id>>(&self.session, "Agent")
            .await
    }
}
impl super::Node for AgentGroup {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
