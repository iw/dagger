//! Generated bindings owned by the GraphQL `LLM` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "A conversation with a large language model (LLM): queue prompts, expose tools, and step the model until it completes its turn."]
#[derive(Clone)]
pub struct Llm {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
#[doc = "Owned optional arguments for GraphQL operation `LLM.artifacts`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct LlmArtifactsOpts {
    #[doc = "Only include artifacts matching these path patterns, as with Workspace.artifacts. A path selects that path and its children.\n\n`None` omits GraphQL field `include`."]
    pub include: Option<Vec<String>>,
}
impl LlmArtifactsOpts {
    #[doc = "Sets GraphQL argument `include` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_include(mut self, value: Vec<impl Into<String>>) -> Self {
        self.include = Some(value.into_iter().map(Into::into).collect());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `LLM.loop`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct LlmLoopOpts {
    #[doc = "Cap the number of steps. The loop fails if the cap is reached before the model ends its turn.\n\n`None` omits GraphQL field `maxSteps`."]
    pub max_steps: Option<i64>,
    #[doc = "Cap the model's output tokens on each step. Defaults to the model's maximum.\n\n`None` omits GraphQL field `maxTokens`."]
    pub max_tokens: Option<i64>,
}
impl LlmLoopOpts {
    #[doc = "Sets GraphQL argument `maxSteps` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_max_steps(mut self, value: i64) -> Self {
        self.max_steps = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `maxTokens` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_max_tokens(mut self, value: i64) -> Self {
        self.max_tokens = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `LLM.spawn`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct LlmSpawnOpts {
    #[doc = "The loop error to create the agent with, for state FAILED. Refused with any other state.\n\n`None` omits GraphQL field `error` and preserves engine default `String(\"\")`."]
    pub error: Option<String>,
    #[doc = "The runtime handle to restore the instance under, as published on its loop span as dagger.io/agent.id. Omit to mint a fresh instance.\n\n`None` omits GraphQL field `handle`."]
    pub handle: Option<String>,
    #[doc = "Display label for the agent — telemetry and error messages; carries no identity. Defaults to a short name derived from the conversation.\n\n`None` omits GraphQL field `name`."]
    pub name: Option<String>,
    #[doc = "Recorded parent handle when restoring an agent. Lineage does not install a notification subscription. Requires a supplied handle. The parent must already be restored in this session and cannot be the agent itself or its descendant.\n\n`None` omits GraphQL field `parentHandle`."]
    pub parent_handle: Option<String>,
    #[doc = "The lifecycle state to create the agent in, as facts on the entry: IDLE is ready to be prompted, PAUSED parks it, FAILED holds an error a resume retries past, STOPPED preserves a dormant snapshot that send or resume can relaunch.\n\nRUNNING and WAITING_INPUT are refused: they describe a loop, and a restored loop died with the session that published it — restore such an agent as IDLE, its interrupted turn's input still pending on the conversation.\n\n`None` omits GraphQL field `state` and preserves engine default `Enum(SchemaName(\"IDLE\"))`."]
    pub state: Option<super::AgentState>,
}
impl LlmSpawnOpts {
    #[doc = "Sets GraphQL argument `error` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `handle` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_handle(mut self, value: impl Into<String>) -> Self {
        self.handle = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `name` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `parentHandle` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_parent_handle(mut self, value: impl Into<String>) -> Self {
        self.parent_handle = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `state` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_state(mut self, value: super::AgentState) -> Self {
        self.state = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `LLM.step`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct LlmStepOpts {
    #[doc = "Cap the model's output tokens for this step. Defaults to the model's maximum.\n\n`None` omits GraphQL field `maxTokens`."]
    pub max_tokens: Option<i64>,
}
impl LlmStepOpts {
    #[doc = "Sets GraphQL argument `maxTokens` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_max_tokens(mut self, value: i64) -> Self {
        self.max_tokens = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `LLM.withContent`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct LlmWithContentOpts {
    #[doc = "The message's recorded provenance.\n\n`None` omits GraphQL field `origin`."]
    pub origin: Option<super::LlmMessageOriginInput>,
}
impl LlmWithContentOpts {
    #[doc = "Sets GraphQL argument `origin` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_origin(mut self, value: super::LlmMessageOriginInput) -> Self {
        self.origin = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `LLM.withContentFile`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct LlmWithContentFileOpts {
    #[doc = "The media MIME type; inferred from the file's contents when omitted.\n\n`None` omits GraphQL field `mimeType` and preserves engine default `String(\"\")`."]
    pub mime_type: Option<String>,
}
impl LlmWithContentFileOpts {
    #[doc = "Sets GraphQL argument `mimeType` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `LLM.withModel`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct LlmWithModelOpts {
    #[doc = "The provider serving the model, e.g. \"openai\". Overrides the provider otherwise inferred from the model name — useful when the name matches no known pattern (e.g. a fine-tune), or matches the wrong one.\n\n`None` omits GraphQL field `provider`."]
    pub provider: Option<String>,
}
impl LlmWithModelOpts {
    #[doc = "Sets GraphQL argument `provider` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `LLM.withPrompt`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct LlmWithPromptOpts {
    #[doc = "The message's recorded provenance, when it arrived through an agent mailbox rather than from the user. Rendered to the model as an attribution header at request-build time.\n\n`None` omits GraphQL field `origin`."]
    pub origin: Option<super::LlmMessageOriginInput>,
}
impl LlmWithPromptOpts {
    #[doc = "Sets GraphQL argument `origin` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_origin(mut self, value: super::LlmMessageOriginInput) -> Self {
        self.origin = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `LLM.withResponse`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct LlmWithResponseOpts {
    #[doc = "Cached input tokens read\n\n`None` omits GraphQL field `cachedTokenReads` and preserves engine default `Int(0)`."]
    pub cached_token_reads: Option<i64>,
    #[doc = "Cached input tokens written\n\n`None` omits GraphQL field `cachedTokenWrites` and preserves engine default `Int(0)`."]
    pub cached_token_writes: Option<i64>,
    #[doc = "Uncached input tokens sent\n\n`None` omits GraphQL field `inputTokens` and preserves engine default `Int(0)`."]
    pub input_tokens: Option<i64>,
    #[doc = "Tokens received from the model, including text and tool calls\n\n`None` omits GraphQL field `outputTokens` and preserves engine default `Int(0)`."]
    pub output_tokens: Option<i64>,
    #[doc = "Total tokens consumed by this response\n\n`None` omits GraphQL field `totalTokens` and preserves engine default `Int(0)`."]
    pub total_tokens: Option<i64>,
}
impl LlmWithResponseOpts {
    #[doc = "Sets GraphQL argument `cachedTokenReads` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_cached_token_reads(mut self, value: i64) -> Self {
        self.cached_token_reads = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `cachedTokenWrites` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_cached_token_writes(mut self, value: i64) -> Self {
        self.cached_token_writes = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `inputTokens` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_input_tokens(mut self, value: i64) -> Self {
        self.input_tokens = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `outputTokens` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_output_tokens(mut self, value: i64) -> Self {
        self.output_tokens = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `totalTokens` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_total_tokens(mut self, value: i64) -> Self {
        self.total_tokens = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `LLM.withToolResult`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct LlmWithToolResultOpts {
    #[doc = "Ordered text and media returned by the tool\n\n`None` omits GraphQL field `blocks` and preserves engine default `List(\\[\\])`."]
    pub blocks: Option<Vec<super::LlmContentBlockInput>>,
}
impl LlmWithToolResultOpts {
    #[doc = "Sets GraphQL argument `blocks` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_blocks(mut self, value: Vec<super::LlmContentBlockInput>) -> Self {
        self.blocks = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `LLM.withTools`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct LlmWithToolsOpts {
    #[doc = "Method names to exclude from the toolset (e.g. constructors, entrypoints).\n\n`None` omits GraphQL field `except` and preserves engine default `List(\\[\\])`."]
    pub except: Option<Vec<String>>,
    #[doc = "Version of this binding's state contract. Recomposition preserves compatible state when the version is unchanged and resets to the newly bound object's defaults when it differs. Change this when the state layout changes incompatibly. Same-type tool returns retain the version. Module identity and ownership checks still apply.\n\n`None` omits GraphQL field `version` and preserves engine default `Int(0)`."]
    pub version: Option<i64>,
}
impl LlmWithToolsOpts {
    #[doc = "Sets GraphQL argument `except` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_except(mut self, value: Vec<impl Into<String>>) -> Self {
        self.except = Some(value.into_iter().map(Into::into).collect());
        self
    }
    #[doc = "Sets GraphQL argument `version` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }
}
impl crate::IntoID<crate::Id> for Llm {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for Llm {
    fn graphql_type() -> &'static str {
        "LLM"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<Llm> for crate::IdInput<Llm> {
    fn from(value: Llm) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<Llm> for crate::IdInput<super::NodeClient> {
    fn from(value: Llm) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<Llm> for crate::IdInput<super::SyncerClient> {
    fn from(value: Llm) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl Llm {
    #[doc = "Reconstruct a spawned agent from its runtime handle.\n\nThis is the lookup spawn pins its result's identity through: the returned handle's ID is an honest, replayable chain denoting the one instance the spawn minted. It never creates an instance itself.\n\nSelects GraphQL field `agent` on `LLM`.\n\n**Experimental:** Agent APIs are likely to change."]
    #[must_use]
    pub fn agent(&self, handle: impl Into<String>, name: impl Into<String>) -> super::Agent {
        let query = self.selection.select("agent");
        let query = query.arg("handle", handle.into());
        let query = query.arg("name", name.into());
        super::Agent {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Discover every artifact this conversation can address, as one selection, without evaluating their values.\n\nTool objects bound with withTools contribute their modules' artifacts, rooted at their current values: evaluating one reads the live state of the bound tools, not a fresh construction. If a module's main object is bound, only its tree is included; otherwise each bound object of that module contributes its own tree. Addresses start with the module name. These artifacts have no workspace of their own: they evaluate in the LLM's bound workspace, if any, whoever evaluates them.\n\nThe workspace part is the artifacts of the workspace bound with withWorkspace, as returned by Workspace.artifacts; an LLM with no bound workspace has none. A workspace module with the same name as a module with bound tool objects is omitted: the bound tools shadow it. Unless they are only a plain construction of the module, which has no state of its own: then the workspace module's artifacts are kept instead.\n\nTool arguments that take an address resolve it here: a DAG address to one object, or, for Artifacts and Artifact arguments, a selection filtered like filterUri.\n\nSelects GraphQL field `artifacts` on `LLM`.\n\n**Experimental:** Agent APIs are likely to change."]
    #[must_use]
    pub fn artifacts(&self) -> super::Artifacts {
        let query = self.selection.select("artifacts");
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `artifacts` with a borrowed, reusable `LlmArtifactsOpts` value.\n\n**Experimental:** Agent APIs are likely to change."]
    #[must_use]
    pub fn artifacts_opts(&self, opts: &LlmArtifactsOpts) -> super::Artifacts {
        let query = self.selection.select("artifacts");
        let query = if let Some(value) = &opts.include {
            query.arg("include", value)
        } else {
            query
        };
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Run expertise in list order, passing this conversation through each function. Retain existing contributions.\n\nSelects GraphQL field `compose` on `LLM`."]
    #[must_use]
    pub fn compose(&self, expertise: Vec<crate::IdInput<super::Expertise>>) -> super::Llm {
        let query = self.selection.select("compose");
        let query = query.arg_id_input("expertise", expertise);
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "estimated number of tokens currently occupying the context window; unlike tokenUsage this is not cumulative over the session\n\nSelects GraphQL field `contextTokens` on `LLM`."]
    pub async fn context_tokens(&self) -> Result<i64, crate::QueryError> {
        let query = self.selection.select("contextTokens");
        query.execute(&self.session).await
    }
    #[doc = "The model's total context window in tokens, or null if unknown (e.g. a local or uncatalogued model).\n\nSelects GraphQL field `contextWindow` on `LLM`."]
    pub async fn context_window(&self) -> Result<Option<i64>, crate::QueryError> {
        let query = self.selection.select("contextWindow");
        query.execute(&self.session).await
    }
    #[doc = "Fork the conversation, so that otherwise-identical follow-ups evaluate independently instead of deduplicating to a single cached result.\n\nSelects GraphQL field `fork` on `LLM`."]
    #[must_use]
    pub fn fork(&self, label: impl Into<String>) -> super::Llm {
        let query = self.selection.select("fork");
        let query = query.arg("label", label.into());
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Report whether anything is queued to send to the model: an unsent prompt or unevaluated tool results. When true, another step will do work; when false, the turn is complete.\n\nSelects GraphQL field `hasPending` on `LLM`."]
    pub async fn has_pending(&self) -> Result<bool, crate::QueryError> {
        let query = self.selection.select("hasPending");
        query.execute(&self.session).await
    }
    #[doc = "A unique identifier for this LLM.\n\nSelects GraphQL field `id` on `LLM`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "The text of the model's most recent reply.\n\nSelects GraphQL field `lastReply` on `LLM`."]
    pub async fn last_reply(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("lastReply");
        query.execute(&self.session).await
    }
    #[doc = "Send the queued prompt and step the model against the available tools, until it ends its turn: a reply with no tool calls and nothing left queued.\n\nSelects GraphQL field `loop` on `LLM`."]
    #[must_use]
    pub fn r#loop(&self) -> super::Llm {
        let query = self.selection.select("loop");
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `loop` with a borrowed, reusable `LlmLoopOpts` value."]
    #[must_use]
    pub fn loop_opts(&self, opts: &LlmLoopOpts) -> super::Llm {
        let query = self.selection.select("loop");
        let query = if let Some(value) = &opts.max_steps {
            query.arg("maxSteps", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.max_tokens {
            query.arg("maxTokens", value)
        } else {
            query
        };
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "The full message history, as structured messages.\n\nSelects GraphQL field `messages` on `LLM`."]
    pub async fn messages(&self) -> Result<Vec<super::LlmMessage>, crate::QueryError> {
        let query = self.selection.select("messages");
        let query = query.select("id");
        query
            .execute_reentry::<super::LlmMessage, Vec<crate::Id>>(&self.session, "LLMMessage")
            .await
    }
    #[doc = "The model the conversation is running against, after resolving any configured default.\n\nSelects GraphQL field `model` on `LLM`."]
    pub async fn model(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("model");
        query.execute(&self.session).await
    }
    #[doc = "The provider serving the model, e.g. \"anthropic\", \"openai\", \"google\", or \"local\".\n\nSelects GraphQL field `provider` on `LLM`."]
    pub async fn provider(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("provider");
        query.execute(&self.session).await
    }
    #[doc = "The reasoning effort in use, e.g. \"low\", \"medium\", or \"high\". Empty or \"none\" when reasoning is disabled.\n\nSelects GraphQL field `reasoningEffort` on `LLM`."]
    pub async fn reasoning_effort(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("reasoningEffort");
        query.execute(&self.session).await
    }
    #[doc = "Run expertise in list order, replacing their modules' contributions and preserving compatible tool state.\n\nClear each selected module's contributions once before execution. Retain unowned contributions and contributions from other modules. Keep this LLM's workspace.\n\nA change to a tool binding's version resets its state. Removed bindings, changed identities, and incompatible state are errors.\n\nSelects GraphQL field `recompose` on `LLM`."]
    #[must_use]
    pub fn recompose(&self, expertise: Vec<crate::IdInput<super::Expertise>>) -> super::Llm {
        let query = self.selection.select("recompose");
        let query = query.arg_id_input("expertise", expertise);
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "The skills visible to the model, exactly as the ListSkills tool serves them: engine-embedded skills, skills installed with withSkills, and skills discovered in the workspace.\n\nSelects GraphQL field `skills` on `LLM`."]
    pub async fn skills(&self) -> Result<Vec<super::LlmSkill>, crate::QueryError> {
        let query = self.selection.select("skills");
        let query = query.select("id");
        query
            .execute_reentry::<super::LlmSkill, Vec<crate::Id>>(&self.session, "LLMSkill")
            .await
    }
    #[doc = "Spawn the conversation as an agent: a startable, addressable evaluation loop seeded with this conversation's state, tools, and workspace.\n\nEvery spawn mints a unique agent instance — two spawns of an identical conversation are two distinct agents, like two calls to a process spawn. The result is pinned to the instance (via the agent lookup field), so re-loading its ID re-addresses the same agent from any request in the session.\n\nThe loop is not started: the agent spends nothing until it is prompted or resumed, and any input pending on the conversation is stepped then.\n\nWith a handle, spawn restores an instance instead of minting one: this conversation becomes the committed history of the agent that handle names, so prompting it continues where it left off — rebuild a conversation's ID from a trace, load it, and spawn it under the handle it belonged to. Fails if that instance already has a runtime entry in this session: a restore must happen before anything else addresses the instance, since by then it may have stepped.\n\nSelects GraphQL field `spawn` on `LLM`.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn spawn(&self) -> Result<super::Agent, crate::QueryError> {
        let query = self.selection.select("spawn");
        let id: crate::Id = query.execute(&self.session).await?;
        Ok(crate::query::reenter::<super::Agent>(
            &self.session,
            id,
            "Agent",
        ))
    }
    #[doc = "Executes GraphQL operation `spawn` with a borrowed, reusable `LlmSpawnOpts` value.\n\n**Experimental:** Agent APIs are likely to change."]
    pub async fn spawn_opts(&self, opts: &LlmSpawnOpts) -> Result<super::Agent, crate::QueryError> {
        let query = self.selection.select("spawn");
        let query = if let Some(value) = &opts.error {
            query.arg("error", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.handle {
            query.arg("handle", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.name {
            query.arg("name", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.parent_handle {
            query.arg("parentHandle", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.state {
            query.arg("state", value)
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
    #[doc = "Advance the conversation by a single step: send the queued prompt or tool results to the model, evaluate any tool calls it makes, and queue their results. Use loop to step until the model ends its turn.\n\nSelects GraphQL field `step` on `LLM`."]
    #[must_use]
    pub fn step(&self) -> super::Llm {
        let query = self.selection.select("step");
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `step` with a borrowed, reusable `LlmStepOpts` value."]
    #[must_use]
    pub fn step_opts(&self, opts: &LlmStepOpts) -> super::Llm {
        let query = self.selection.select("step");
        let query = if let Some(value) = &opts.max_tokens {
            query.arg("maxTokens", value)
        } else {
            query
        };
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Force evaluation of the conversation's pending operations (prompts, steps, loops) in the engine.\n\nSelects GraphQL field `sync` on `LLM`."]
    pub async fn sync(&self) -> Result<super::Llm, crate::QueryError> {
        let query = self.selection.select("sync");
        let id: crate::Id = query.execute(&self.session).await?;
        Ok(crate::query::reenter::<super::Llm>(
            &self.session,
            id,
            "LLM",
        ))
    }
    #[doc = "The cumulative token usage, summed across every API call in the conversation.\n\nSelects GraphQL field `tokenUsage` on `LLM`."]
    #[must_use]
    pub fn token_usage(&self) -> super::LlmTokenUsage {
        let query = self.selection.select("tokenUsage");
        super::LlmTokenUsage {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Render documentation for the tools currently exposed to the model.\n\nSelects GraphQL field `tools` on `LLM`."]
    pub async fn tools(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("tools");
        query.execute(&self.session).await
    }
    #[doc = "The message history rendered as a plain-text transcript, suitable for feeding back to an LLM (e.g. for summarization).\n\nSelects GraphQL field `transcript` on `LLM`."]
    pub async fn transcript(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("transcript");
        query.execute(&self.session).await
    }
    #[doc = "Queue one user message containing ordered text and media blocks.\n\nSelects GraphQL field `withContent` on `LLM`."]
    #[must_use]
    pub fn with_content(&self, content: Vec<super::LlmContentBlockInput>) -> super::Llm {
        let query = self.selection.select("withContent");
        let query = query.arg("content", content);
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withContent` with a borrowed, reusable `LlmWithContentOpts` value."]
    #[must_use]
    pub fn with_content_opts(
        &self,
        content: Vec<super::LlmContentBlockInput>,
        opts: &LlmWithContentOpts,
    ) -> super::Llm {
        let query = self.selection.select("withContent");
        let query = query.arg("content", content);
        let query = if let Some(value) = &opts.origin {
            query.arg("origin", value)
        } else {
            query
        };
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Queue an image, audio, or PDF file as one user message. Media bytes are stored in the conversation.\n\nSelects GraphQL field `withContentFile` on `LLM`."]
    #[must_use]
    pub fn with_content_file(&self, file: impl Into<crate::IdInput<super::File>>) -> super::Llm {
        let query = self.selection.select("withContentFile");
        let query = query.arg_id_input("file", file.into());
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withContentFile` with a borrowed, reusable `LlmWithContentFileOpts` value."]
    #[must_use]
    pub fn with_content_file_opts(
        &self,
        file: impl Into<crate::IdInput<super::File>>,
        opts: &LlmWithContentFileOpts,
    ) -> super::Llm {
        let query = self.selection.select("withContentFile");
        let query = query.arg_id_input("file", file.into());
        let query = if let Some(value) = &opts.mime_type {
            query.arg("mimeType", value)
        } else {
            query
        };
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Add an external MCP server to the LLM\n\nSelects GraphQL field `withMCPServer` on `LLM`."]
    #[must_use]
    pub fn with_mcp_server(
        &self,
        name: impl Into<String>,
        service: impl Into<crate::IdInput<super::Service>>,
    ) -> super::Llm {
        let query = self.selection.select("withMCPServer");
        let query = query.arg("name", name.into());
        let query = query.arg_id_input("service", service.into());
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Change the model for the rest of the conversation. The message history is preserved; the new model takes effect on the next step.\n\nSelects GraphQL field `withModel` on `LLM`."]
    #[must_use]
    pub fn with_model(&self, model: impl Into<String>) -> super::Llm {
        let query = self.selection.select("withModel");
        let query = query.arg("model", model.into());
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withModel` with a borrowed, reusable `LlmWithModelOpts` value."]
    #[must_use]
    pub fn with_model_opts(&self, model: impl Into<String>, opts: &LlmWithModelOpts) -> super::Llm {
        let query = self.selection.select("withModel");
        let query = query.arg("model", model.into());
        let query = if let Some(value) = &opts.provider {
            query.arg("provider", value)
        } else {
            query
        };
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Queue a user prompt, to be sent to the model on the next step or loop.\n\nSelects GraphQL field `withPrompt` on `LLM`."]
    #[must_use]
    pub fn with_prompt(&self, prompt: impl Into<String>) -> super::Llm {
        let query = self.selection.select("withPrompt");
        let query = query.arg("prompt", prompt.into());
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withPrompt` with a borrowed, reusable `LlmWithPromptOpts` value."]
    #[must_use]
    pub fn with_prompt_opts(
        &self,
        prompt: impl Into<String>,
        opts: &LlmWithPromptOpts,
    ) -> super::Llm {
        let query = self.selection.select("withPrompt");
        let query = if let Some(value) = &opts.origin {
            query.arg("origin", value)
        } else {
            query
        };
        let query = query.arg("prompt", prompt.into());
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Queue a file's contents as a user prompt, like withPrompt.\n\nSelects GraphQL field `withPromptFile` on `LLM`."]
    #[must_use]
    pub fn with_prompt_file(&self, file: impl Into<crate::IdInput<super::File>>) -> super::Llm {
        let query = self.selection.select("withPromptFile");
        let query = query.arg_id_input("file", file.into());
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Change the reasoning effort for the rest of the conversation, overriding any configured default. The message history is preserved; the new effort takes effect on the next step.\n\nSelects GraphQL field `withReasoningEffort` on `LLM`."]
    #[must_use]
    pub fn with_reasoning_effort(&self, effort: impl Into<String>) -> super::Llm {
        let query = self.selection.select("withReasoningEffort");
        let query = query.arg("effort", effort.into());
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Append an assistant response to the message history without calling the model, e.g. to reconstruct a conversation from another source.\n\nSelects GraphQL field `withResponse` on `LLM`."]
    #[must_use]
    pub fn with_response(&self, content: Vec<super::LlmContentBlockInput>) -> super::Llm {
        let query = self.selection.select("withResponse");
        let query = query.arg("content", content);
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withResponse` with a borrowed, reusable `LlmWithResponseOpts` value."]
    #[must_use]
    pub fn with_response_opts(
        &self,
        content: Vec<super::LlmContentBlockInput>,
        opts: &LlmWithResponseOpts,
    ) -> super::Llm {
        let query = self.selection.select("withResponse");
        let query = if let Some(value) = &opts.cached_token_reads {
            query.arg("cachedTokenReads", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.cached_token_writes {
            query.arg("cachedTokenWrites", value)
        } else {
            query
        };
        let query = query.arg("content", content);
        let query = if let Some(value) = &opts.input_tokens {
            query.arg("inputTokens", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.output_tokens {
            query.arg("outputTokens", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.total_tokens {
            query.arg("totalTokens", value)
        } else {
            query
        };
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Install skills from a directory, adding them to the skills the model discovers with ListSkills and reads with ReadSkill. Each skill is a directory containing a SKILL.md with name and description frontmatter, discovered anywhere in the tree. Installed skills take precedence over skills discovered in the workspace, but cannot shadow the engine's built-in skills.\n\nSelects GraphQL field `withSkills` on `LLM`."]
    #[must_use]
    pub fn with_skills(
        &self,
        directory: impl Into<crate::IdInput<super::Directory>>,
    ) -> super::Llm {
        let query = self.selection.select("withSkills");
        let query = query.arg_id_input("directory", directory.into());
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Switch to the configured small model for the current provider, or that provider's recommended default. The message history is preserved; unknown providers without a small-model configuration keep their current model.\n\nSelects GraphQL field `withSmallModel` on `LLM`."]
    #[must_use]
    pub fn with_small_model(&self) -> super::Llm {
        let query = self.selection.select("withSmallModel");
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Add a system prompt, instructing the model across the whole conversation.\n\nSelects GraphQL field `withSystemPrompt` on `LLM`."]
    #[must_use]
    pub fn with_system_prompt(&self, prompt: impl Into<String>) -> super::Llm {
        let query = self.selection.select("withSystemPrompt");
        let query = query.arg("prompt", prompt.into());
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Append the result of a tool call to the message history.\n\nSelects GraphQL field `withToolResult` on `LLM`."]
    #[must_use]
    pub fn with_tool_result(
        &self,
        call_id: impl Into<String>,
        content: impl Into<String>,
        errored: bool,
    ) -> super::Llm {
        let query = self.selection.select("withToolResult");
        let query = query.arg("callId", call_id.into());
        let query = query.arg("content", content.into());
        let query = query.arg("errored", errored);
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withToolResult` with a borrowed, reusable `LlmWithToolResultOpts` value."]
    #[must_use]
    pub fn with_tool_result_opts(
        &self,
        call_id: impl Into<String>,
        content: impl Into<String>,
        errored: bool,
        opts: &LlmWithToolResultOpts,
    ) -> super::Llm {
        let query = self.selection.select("withToolResult");
        let query = if let Some(value) = &opts.blocks {
            query.arg("blocks", value)
        } else {
            query
        };
        let query = query.arg("callId", call_id.into());
        let query = query.arg("content", content.into());
        let query = query.arg("errored", errored);
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Expose an object's methods as tools. Every eligible method of the bound object becomes a tool; a tool that returns this object's own type replaces it as the new state. Repeatable to bind several objects.\n\nSelects GraphQL field `withTools` on `LLM`."]
    #[must_use]
    pub fn with_tools(&self, object: impl Into<crate::IdInput<super::NodeClient>>) -> super::Llm {
        let query = self.selection.select("withTools");
        let query = query.arg_id_input("object", object.into());
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withTools` with a borrowed, reusable `LlmWithToolsOpts` value."]
    #[must_use]
    pub fn with_tools_opts(
        &self,
        object: impl Into<crate::IdInput<super::NodeClient>>,
        opts: &LlmWithToolsOpts,
    ) -> super::Llm {
        let query = self.selection.select("withTools");
        let query = if let Some(value) = &opts.except {
            query.arg("except", value)
        } else {
            query
        };
        let query = query.arg_id_input("object", object.into());
        let query = if let Some(value) = &opts.version {
            query.arg("version", value)
        } else {
            query
        };
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Bind the LLM to a workspace, exposing its modules as tools exactly as the Dagger CLI would serve them for that workspace.\n\nSelects GraphQL field `withWorkspace` on `LLM`."]
    #[must_use]
    pub fn with_workspace(
        &self,
        workspace: impl Into<crate::IdInput<super::Workspace>>,
    ) -> super::Llm {
        let query = self.selection.select("withWorkspace");
        let query = query.arg_id_input("workspace", workspace.into());
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Disable the default system prompt\n\nSelects GraphQL field `withoutDefaultSystemPrompt` on `LLM`."]
    #[must_use]
    pub fn without_default_system_prompt(&self) -> super::Llm {
        let query = self.selection.select("withoutDefaultSystemPrompt");
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Clear the message history, keeping only the system prompts.\n\nSelects GraphQL field `withoutMessageHistory` on `LLM`."]
    #[must_use]
    pub fn without_message_history(&self) -> super::Llm {
        let query = self.selection.select("withoutMessageHistory");
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Clear the user-added system prompts, keeping only the default system prompt.\n\nSelects GraphQL field `withoutSystemPrompts` on `LLM`."]
    #[must_use]
    pub fn without_system_prompts(&self) -> super::Llm {
        let query = self.selection.select("withoutSystemPrompts");
        super::Llm {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return the workspace the LLM is bound to.\n\nSelects GraphQL field `workspace` on `LLM`."]
    #[must_use]
    pub fn workspace(&self) -> super::Workspace {
        let query = self.selection.select("workspace");
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
}
impl super::Node for Llm {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
impl super::Syncer for Llm {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
    fn sync(
        &self,
    ) -> impl core::future::Future<Output = Result<super::SyncerClient, crate::QueryError>> + Send
    {
        let query = self.selection.select("sync");
        let session = self.session.clone();
        async move {
            let id: crate::Id = query.execute(&session).await?;
            Ok(crate::query::reenter::<super::SyncerClient>(
                &session, id, "Syncer",
            ))
        }
    }
}
