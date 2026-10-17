//! Generated bindings owned by the GraphQL `AgentMessageDelivery` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "EXPERIMENTAL: Agent APIs are likely to change.\n\nHow a message landed in an agent's evaluation."]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum AgentMessageDelivery {
    #[doc = "The message is queued: the agent is paused or failed, and a resume will drain it."]
    #[serde(rename = "QUEUED")]
    Queued,
    #[doc = "The message opened a new turn: the agent was idle or newly started."]
    #[serde(rename = "STARTED")]
    Started,
    #[doc = "The message was absorbed into the in-flight turn at a step boundary, steering it."]
    #[serde(rename = "STEERED")]
    Steered,
}
