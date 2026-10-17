//! Generated bindings owned by the GraphQL `AgentState` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "EXPERIMENTAL: Agent APIs are likely to change.\n\nComputed lifecycle state of an agent."]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum AgentState {
    #[doc = "The loop failed; snapshot holds the completed prefix. Resume retries."]
    #[serde(rename = "FAILED")]
    Failed,
    #[doc = "Mailbox empty, turn complete; blocked in receive."]
    #[serde(rename = "IDLE")]
    Idle,
    #[doc = "Mailbox accepting but not draining, until resume."]
    #[serde(rename = "PAUSED")]
    Paused,
    #[doc = "A model request or tool evaluation is in flight."]
    #[serde(rename = "RUNNING")]
    Running,
    #[doc = "Runtime released; snapshot remains readable."]
    #[serde(rename = "STOPPED")]
    Stopped,
    #[doc = "Blocked on input from the user."]
    #[serde(rename = "WAITING_INPUT")]
    WaitingInput,
}
