//! Generated bindings owned by the GraphQL `LLMContentBlockInput` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "A content block within an LLM message."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[non_exhaustive]
pub struct LlmContentBlockInput {
    #[doc = "The arguments to pass to the tool (for TOOL_CALL kind)."]
    #[serde(rename = "arguments", default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<crate::Json>,
    #[doc = "The unique ID of a tool call (for TOOL_CALL or TOOL_RESULT kinds)."]
    #[serde(rename = "callId", default, skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    #[doc = "Ordered TEXT or media blocks returned by a tool."]
    #[serde(rename = "content", default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<super::LlmContentBlockInput>>,
    #[doc = "Media bytes. Supply exactly one of data or file for media."]
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<super::Bytes>,
    #[doc = "Whether the tool call resulted in an error (for TOOL_RESULT kind)."]
    #[serde(rename = "errored", default, skip_serializing_if = "Option::is_none")]
    pub errored: Option<bool>,
    #[doc = "A media file to resolve to inline bytes."]
    #[serde(rename = "file", default, skip_serializing_if = "Option::is_none")]
    pub file: Option<crate::Id>,
    #[doc = "The kind of content block."]
    #[serde(rename = "kind")]
    pub kind: super::LlmContentBlockKind,
    #[doc = "Media MIME type; required for inline data, inferred for a file."]
    #[serde(rename = "mimeType", default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[doc = "Provider-specific opaque data (e.g. Anthropic thinking signature)."]
    #[serde(rename = "signature", default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[doc = "Text content (for TEXT, THINKING, or TOOL_RESULT kinds)."]
    #[serde(rename = "text", default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = "The name of the tool to call (for TOOL_CALL kind)."]
    #[serde(rename = "toolName", default, skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
}
impl LlmContentBlockInput {
    #[doc = "Creates `LlmContentBlockInput` with every required GraphQL input field."]
    #[must_use]
    pub fn new(kind: super::LlmContentBlockKind) -> Self {
        Self {
            arguments: None,
            call_id: None,
            content: None,
            data: None,
            errored: None,
            file: None,
            kind,
            mime_type: None,
            signature: None,
            text: None,
            tool_name: None,
        }
    }
    #[doc = "Sets GraphQL input field `arguments`; the field is omitted until this method is called."]
    #[must_use]
    pub fn with_arguments(mut self, value: crate::Json) -> Self {
        self.arguments = Some(value);
        self
    }
    #[doc = "Sets GraphQL input field `callId`; the field is omitted until this method is called."]
    #[must_use]
    pub fn with_call_id(mut self, value: String) -> Self {
        self.call_id = Some(value);
        self
    }
    #[doc = "Sets GraphQL input field `content`; the field is omitted until this method is called."]
    #[must_use]
    pub fn with_content(mut self, value: Vec<super::LlmContentBlockInput>) -> Self {
        self.content = Some(value);
        self
    }
    #[doc = "Sets GraphQL input field `data`; the field is omitted until this method is called."]
    #[must_use]
    pub fn with_data(mut self, value: super::Bytes) -> Self {
        self.data = Some(value);
        self
    }
    #[doc = "Sets GraphQL input field `errored`; the field is omitted until this method is called."]
    #[must_use]
    pub fn with_errored(mut self, value: bool) -> Self {
        self.errored = Some(value);
        self
    }
    #[doc = "Sets GraphQL input field `file`; the field is omitted until this method is called."]
    #[must_use]
    pub fn with_file(mut self, value: crate::Id) -> Self {
        self.file = Some(value);
        self
    }
    #[doc = "Sets GraphQL input field `mimeType`; the field is omitted until this method is called."]
    #[must_use]
    pub fn with_mime_type(mut self, value: String) -> Self {
        self.mime_type = Some(value);
        self
    }
    #[doc = "Sets GraphQL input field `signature`; the field is omitted until this method is called."]
    #[must_use]
    pub fn with_signature(mut self, value: String) -> Self {
        self.signature = Some(value);
        self
    }
    #[doc = "Sets GraphQL input field `text`; the field is omitted until this method is called."]
    #[must_use]
    pub fn with_text(mut self, value: String) -> Self {
        self.text = Some(value);
        self
    }
    #[doc = "Sets GraphQL input field `toolName`; the field is omitted until this method is called."]
    #[must_use]
    pub fn with_tool_name(mut self, value: String) -> Self {
        self.tool_name = Some(value);
        self
    }
}
