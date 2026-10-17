//! Generated bindings owned by the GraphQL `Agent` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "EXPERIMENTAL: Agent APIs are likely to change.\n\nA conversation loop running as an addressable, long-lived entity within the session. The conversation itself remains observable at any time as an immutable LLM value."]
#[derive(Clone)]
pub struct Agent {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
#[doc = "Owned optional arguments for GraphQL operation `Agent.notify`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct AgentNotifyOpts {
    #[doc = "The lifecycle states that fire an event. IDLE events carry the turn's final reply; FAILED events carry the loop error.\n\n`None` omits GraphQL field `on` and preserves engine default `List(\\[Enum(SchemaName(\"IDLE\")), Enum(SchemaName(\"FAILED\"))\\])`."]
    pub on: Option<Vec<super::AgentState>>,
}
impl AgentNotifyOpts {
    #[doc = "Sets GraphQL argument `on` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_on(mut self, value: Vec<super::AgentState>) -> Self {
        self.on = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Agent.pause`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct AgentPauseOpts {
    #[doc = "Preempt the in-flight step instead of letting it finish. All completed steps are kept and the interrupted turn stays open: messages it consumed remain pending, while unconsumed mailbox messages are discarded. Resume continues the turn from the last committed step. On an idle, never-started, or failed agent there is nothing to preempt, so this is a plain pause.\n\n`None` omits GraphQL field `interrupt` and preserves engine default `Boolean(false)`."]
    pub interrupt: Option<bool>,
}
impl AgentPauseOpts {
    #[doc = "Sets GraphQL argument `interrupt` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_interrupt(mut self, value: bool) -> Self {
        self.interrupt = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Agent.send`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct AgentSendOpts {
    #[doc = "Ordered TEXT, IMAGE, AUDIO, or DOCUMENT user content blocks. File inputs are resolved and all content is validated before enqueueing. Pass an empty message for media-only sends.\n\n`None` omits GraphQL field `content` and preserves engine default `List(\\[\\])`."]
    pub content: Option<Vec<super::LlmContentBlockInput>>,
    #[doc = "The ref of a message in the SENDER's own mailbox this send answers (e.g. \"#3\", from its attribution header). The recipient sees the two paired, and awaiters of the replied-to message resolve with this reply immediately instead of at the sender's turn end.\n\n`None` omits GraphQL field `replyTo` and preserves engine default `String(\"\")`."]
    pub reply_to: Option<String>,
}
impl AgentSendOpts {
    #[doc = "Sets GraphQL argument `content` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_content(mut self, value: Vec<super::LlmContentBlockInput>) -> Self {
        self.content = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `replyTo` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_reply_to(mut self, value: impl Into<String>) -> Self {
        self.reply_to = Some(value.into());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Agent.stop`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct AgentStopOpts {
    #[doc = "Cancel the loop immediately instead of letting an in-flight step finish. Either way the completed steps are preserved in the snapshot.\n\n`None` omits GraphQL field `kill` and preserves engine default `Boolean(false)`."]
    pub kill: Option<bool>,
}
impl AgentStopOpts {
    #[doc = "Sets GraphQL argument `kill` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_kill(mut self, value: bool) -> Self {
        self.kill = Some(value);
        self
    }
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
    #[doc = "Why the loop failed, for a FAILED agent; empty otherwise.\n\nThe snapshot holds the completed prefix — send or resume retries from it.\n\nSelects GraphQL field `error` on `Agent`.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn error(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("error");
        query.execute(&self.session).await
    }
    #[doc = "The opaque runtime handle minted by the spawn that created this agent.\n\nIt is the same value the agent's loop span publishes as dagger.io/agent.id, so a client can correlate the agent with what it discovers in the trace. Two spawns of an identical composition have different handles; a display name is shared freely.\n\nSelects GraphQL field `handle` on `Agent`.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn handle(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("handle");
        query.execute(&self.session).await
    }
    #[doc = "A unique identifier for this Agent.\n\nSelects GraphQL field `id` on `Agent`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "Look up a previously sent message by its ref.\n\nThis is the lookup send pins its result's identity through: the returned handle's ID is an honest, replayable chain, addressable from any request in the session (the cancel-and-request-again contract).\n\nFails if the agent has no runtime entry in this session, or no record of the given ref.\n\nSelects GraphQL field `message` on `Agent`.\n\n**Experimental:** Agent APIs are likely to change."]
    #[must_use]
    pub fn message(&self, r#ref: impl Into<String>) -> super::AgentMessage {
        let query = self.selection.select("message");
        let query = query.arg("ref", r#ref.into());
        super::AgentMessage {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Display label for the agent; carries no identity.\n\nSelects GraphQL field `name` on `Agent`.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn name(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("name");
        query.execute(&self.session).await
    }
    #[doc = "Subscribe another agent to this agent's lifecycle: each transition into one of the given states enqueues an event message to the subscriber — steering its open turn, or waking it if idle, like any other message.\n\nThis is how a supervisor hears every completion and failure without polling or blocking: subscribe at spawn time, keep working, and events arrive as attributed messages.\n\nEvents never relaunch a stopped subscriber, and an already-reached state fires immediately at subscribe time, so a fast agent settling before the subscription lands is not missed.\n\nA restored agent that nothing has sent to, started, or resumed yet is the exception: its state was reached in the session it was restored from, so subscribing to it announces nothing until it next transitions. This is how a restore reinstalls recorded subscriptions without waking their subscribers.\n\nIdempotent per subscriber; re-subscribing replaces the state set.\n\nSelects GraphQL field `notify` on `Agent`.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn notify(
        &self,
        subscriber: impl Into<crate::IdInput<super::Agent>>,
    ) -> Result<super::Agent, crate::QueryError> {
        let query = self.selection.select("notify");
        let query = query.arg_id_input("subscriber", subscriber.into());
        let id: crate::Id = query.execute(&self.session).await?;
        Ok(crate::query::reenter::<super::Agent>(
            &self.session,
            id,
            "Agent",
        ))
    }
    #[doc = "Executes GraphQL operation `notify` with a borrowed, reusable `AgentNotifyOpts` value.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn notify_opts(
        &self,
        subscriber: impl Into<crate::IdInput<super::Agent>>,
        opts: &AgentNotifyOpts,
    ) -> Result<super::Agent, crate::QueryError> {
        let query = self.selection.select("notify");
        let query = if let Some(value) = &opts.on {
            query.arg("on", value)
        } else {
            query
        };
        let query = query.arg_id_input("subscriber", subscriber.into());
        let id: crate::Id = query.execute(&self.session).await?;
        Ok(crate::query::reenter::<super::Agent>(
            &self.session,
            id,
            "Agent",
        ))
    }
    #[doc = "Stop draining the mailbox once the in-flight step completes, or immediately with interrupt.\n\nPause takes priority over pending work: a mid-turn pause suspends the turn, which resume continues. Messages sent while paused enqueue with QUEUED delivery until a resume.\n\nPausing a never-started agent leaves it paused for its eventual resume; pausing a failed agent is allowed (resume decides the retry); pausing a stopped agent fails.\n\nSelects GraphQL field `pause` on `Agent`.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn pause(&self) -> Result<super::Agent, crate::QueryError> {
        let query = self.selection.select("pause");
        let id: crate::Id = query.execute(&self.session).await?;
        Ok(crate::query::reenter::<super::Agent>(
            &self.session,
            id,
            "Agent",
        ))
    }
    #[doc = "Executes GraphQL operation `pause` with a borrowed, reusable `AgentPauseOpts` value.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn pause_opts(
        &self,
        opts: &AgentPauseOpts,
    ) -> Result<super::Agent, crate::QueryError> {
        let query = self.selection.select("pause");
        let query = if let Some(value) = &opts.interrupt {
            query.arg("interrupt", value)
        } else {
            query
        };
        let id: crate::Id = query.execute(&self.session).await?;
        Ok(crate::query::reenter::<super::Agent>(
            &self.session,
            id,
            "Agent",
        ))
    }
    #[doc = "Replace this instance's committed conversation with the given one, keeping the entry: identity, mailbox, and lifecycle state are untouched. A paused suspended turn is abandoned and its consumed messages are resolved before replacement.\n\nThis is the continuity verb. Compaction, a workspace rebind, a model change, or rewinding an interrupted prompt produce a new conversation value for the SAME agent; reseed swaps it in place, where a stop-and-respawn would mint a successor instance and split the agent across two roster entries. It is the client-facing form of what a continuation tool already does mid-turn: the agent adopts a new conversation without changing who it is.\n\nThe next turn continues from the reseeded conversation, and queued messages drain onto it. A FAILED agent keeps its error — resume retries from the new conversation.\n\nFails if the instance has no runtime entry in this session (only a spawned or re-hydrated instance holds a conversation to replace), if a step is in flight, or if the agent is stopped.\n\nSelects GraphQL field `reseed` on `Agent`.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn reseed(
        &self,
        conversation: impl Into<crate::IdInput<super::Llm>>,
    ) -> Result<super::Agent, crate::QueryError> {
        let query = self.selection.select("reseed");
        let query = query.arg_id_input("conversation", conversation.into());
        let id: crate::Id = query.execute(&self.session).await?;
        Ok(crate::query::reenter::<super::Agent>(
            &self.session,
            id,
            "Agent",
        ))
    }
    #[doc = "Resume draining the mailbox: a suspended turn continues from the last committed step, and queued messages drain.\n\nResuming a never-started agent starts its evaluation loop, detached from the calling request: it steps the conversation while input is pending, then idles awaiting further lifecycle operations. Resuming a FAILED agent retries its pending step. Resuming a STOPPED agent relaunches the same instance from its last committed snapshot.\n\nNo-op on a running or idle agent.\n\nSelects GraphQL field `resume` on `Agent`.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn resume(&self) -> Result<super::Agent, crate::QueryError> {
        let query = self.selection.select("resume");
        let id: crate::Id = query.execute(&self.session).await?;
        Ok(crate::query::reenter::<super::Agent>(
            &self.session,
            id,
            "Agent",
        ))
    }
    #[doc = "The conversation the agent was spawned from: the tools, workspace and message history its loop started with.\n\nFixed by the spawn; stepping does not move it (see snapshot for the latest committed step). Its workspace is the baseline the agent's own changes are measured from.\n\nFor an agent restored under a handle, this is the conversation it was restored from.\n\nSelects GraphQL field `seed` on `Agent`.\n\n**Experimental:** Agent APIs are likely to change."]
    #[must_use]
    pub fn seed(&self) -> super::Llm {
        let query = self.selection.select("seed");
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Enqueue a message, on the record: it is consumed at a step boundary, appends to the agent's history, and steers the running turn or opens a new one.\n\nNever blocks, never drops; concurrent sends queue in order.\n\nThe returned message is pinned through the message lookup field, so its handle is re-addressable from any request in the session: cancel a response request and request it again freely.\n\nSending to a never-started agent starts it (signal-with-start). Sending to a stopped agent restarts the same instance from its last committed snapshot. Sending to a paused or failed agent enqueues with QUEUED delivery, to be drained by a resume.\n\nSelects GraphQL field `send` on `Agent`.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn send(
        &self,
        message: impl Into<String>,
    ) -> Result<super::AgentMessage, crate::QueryError> {
        let query = self.selection.select("send");
        let query = query.arg("message", message.into());
        let id: crate::Id = query.execute(&self.session).await?;
        Ok(crate::query::reenter::<super::AgentMessage>(
            &self.session,
            id,
            "AgentMessage",
        ))
    }
    #[doc = "Executes GraphQL operation `send` with a borrowed, reusable `AgentSendOpts` value.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn send_opts(
        &self,
        message: impl Into<String>,
        opts: &AgentSendOpts,
    ) -> Result<super::AgentMessage, crate::QueryError> {
        let query = self.selection.select("send");
        let query = if let Some(value) = &opts.content {
            query.arg("content", value)
        } else {
            query
        };
        let query = query.arg("message", message.into());
        let query = if let Some(value) = &opts.reply_to {
            query.arg("replyTo", value)
        } else {
            query
        };
        let id: crate::Id = query.execute(&self.session).await?;
        Ok(crate::query::reenter::<super::AgentMessage>(
            &self.session,
            id,
            "AgentMessage",
        ))
    }
    #[doc = "The conversation as of the last committed step: immutable, branchable, persistable.\n\nThe seed conversation if the agent never stepped.\n\nBranching from it does not affect the agent.\n\nSelects GraphQL field `snapshot` on `Agent`.\n\n**Experimental:** Agent APIs are likely to change."]
    #[must_use]
    pub fn snapshot(&self) -> super::Llm {
        let query = self.selection.select("snapshot");
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Computed lifecycle state; never stored.\n\nAn agent that was never started reports IDLE: its mailbox is empty and no turn is open.\n\nSelects GraphQL field `state` on `Agent`.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn state(&self) -> Result<super::AgentState, crate::QueryError> {
        let query = self.selection.select("state");
        query.execute(&self.session).await
    }
    #[doc = "Release the agent's runtime. The tombstone (state, snapshot) stays readable for the rest of the session.\n\nSelects GraphQL field `stop` on `Agent`.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn stop(&self) -> Result<super::Agent, crate::QueryError> {
        let query = self.selection.select("stop");
        let id: crate::Id = query.execute(&self.session).await?;
        Ok(crate::query::reenter::<super::Agent>(
            &self.session,
            id,
            "Agent",
        ))
    }
    #[doc = "Executes GraphQL operation `stop` with a borrowed, reusable `AgentStopOpts` value.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn stop_opts(&self, opts: &AgentStopOpts) -> Result<super::Agent, crate::QueryError> {
        let query = self.selection.select("stop");
        let query = if let Some(value) = &opts.kill {
            query.arg("kill", value)
        } else {
            query
        };
        let id: crate::Id = query.execute(&self.session).await?;
        Ok(crate::query::reenter::<super::Agent>(
            &self.session,
            id,
            "Agent",
        ))
    }
    #[doc = "Block until the agent settles: IDLE, FAILED, or STOPPED. Read which from state afterwards.\n\nUnlike waiting for one exact state, this cannot hang merely because the agent settled in a different outcome.\n\nSelects GraphQL field `wait` on `Agent`.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn wait(&self) -> Result<super::Agent, crate::QueryError> {
        let query = self.selection.select("wait");
        let id: crate::Id = query.execute(&self.session).await?;
        Ok(crate::query::reenter::<super::Agent>(
            &self.session,
            id,
            "Agent",
        ))
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
