//! Generated bindings owned by the GraphQL `LLMContentBlockKind` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "The kind of content in a message block."]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum LlmContentBlockKind {
    #[doc = "Inline audio."]
    #[serde(rename = "AUDIO")]
    Audio,
    #[doc = "An inline PDF document."]
    #[serde(rename = "DOCUMENT")]
    Document,
    #[doc = "An inline image."]
    #[serde(rename = "IMAGE")]
    Image,
    #[doc = "Plain text content."]
    #[serde(rename = "TEXT")]
    Text,
    #[doc = "Model thinking/reasoning content (e.g. Anthropic extended thinking)."]
    #[serde(rename = "THINKING")]
    Thinking,
    #[doc = "A tool/function call from the model."]
    #[serde(rename = "TOOL_CALL")]
    ToolCall,
    #[doc = "A tool/function result."]
    #[serde(rename = "TOOL_RESULT")]
    ToolResult,
}
