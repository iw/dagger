//! Generated bindings owned by the GraphQL `LLMMessageRole` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
#[doc = "The role that generated a message."]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum LlmMessageRole {
    #[doc = "A reply from the model."]
    #[serde(rename = "ASSISTANT")]
    Assistant,
    #[doc = "A system prompt."]
    #[serde(rename = "SYSTEM")]
    System,
    #[doc = "A user prompt or tool response."]
    #[serde(rename = "USER")]
    User,
}
