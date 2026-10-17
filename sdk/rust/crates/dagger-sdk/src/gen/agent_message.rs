//! Generated bindings owned by the GraphQL `AgentMessage` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "EXPERIMENTAL: Agent APIs are likely to change.\n\nA message delivered to an agent's mailbox."]
#[derive(Clone)]
pub struct AgentMessage {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for AgentMessage {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for AgentMessage {
    fn graphql_type() -> &'static str {
        "AgentMessage"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<AgentMessage> for crate::IdInput<AgentMessage> {
    fn from(value: AgentMessage) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<AgentMessage> for crate::IdInput<super::NodeClient> {
    fn from(value: AgentMessage) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl AgentMessage {
    #[doc = "How the message conclusively landed: opened a new turn (STARTED), was absorbed into the running turn at a step boundary (STEERED), or queued behind it (QUEUED).\n\nBlocks until provider or native lifecycle evidence is conclusive. Once recorded, the result or cancellation error is immutable.\n\nSelects GraphQL field `delivery` on `AgentMessage`.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn delivery(&self) -> Result<super::AgentMessageDelivery, crate::QueryError> {
        let query = self.selection.select("delivery");
        query.execute(&self.session).await
    }
    #[doc = "A unique identifier for this AgentMessage.\n\nSelects GraphQL field `id` on `AgentMessage`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "The message's short ref within the receiving agent's runtime, e.g. \"#3\".\n\nThis is the deterministic token the recipient's attribution header shows and a reply's replyTo names — quote it when telling the recipient what to answer.\n\nSelects GraphQL field `ref` on `AgentMessage`.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn r#ref(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("ref");
        query.execute(&self.session).await
    }
    #[doc = "Block until this message is answered, and return the answer: an explicit reply (a send whose replyTo names this message), or the final reply of the turn that consumed it, whichever comes first.\n\nIdempotent: cancel and request the response again freely; concurrent waiters share the result.\n\nFails if the agent stops before the message resolves. On a failed agent it projects the failure — but the message stays pending, so after a resume consumes it, requesting the response again returns the real reply.\n\nRefused when called from inside an agent turn whose wait would deadlock: turns should not block on other agents — send without awaiting, and the reply arrives as a message.\n\nSelects GraphQL field `response` on `AgentMessage`.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn response(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("response");
        query.execute(&self.session).await
    }
}
impl super::Node for AgentMessage {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
