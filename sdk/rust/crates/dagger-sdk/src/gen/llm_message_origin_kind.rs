//! Generated bindings owned by the GraphQL `LLMMessageOriginKind` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "EXPERIMENTAL: Agent APIs are likely to change.\n\nWho put a message on the conversation record."]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum LlmMessageOriginKind {
    #[doc = "Another agent: the message was sent from within that agent's turn."]
    #[serde(rename = "AGENT")]
    Agent,
    #[doc = "The engine, reporting a subscribed agent's lifecycle transition."]
    #[serde(rename = "EVENT")]
    Event,
    #[doc = "The user: a prompt submitted by a client rather than sent by an agent."]
    #[serde(rename = "USER")]
    User,
}
