//! Generated bindings owned by the GraphQL `LLMContent` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "An ordered run of text and media content for a model to read, built outside any conversation."]
#[derive(Clone)]
pub struct LlmContent {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
#[doc = "Owned optional arguments for GraphQL operation `LLMContent.withFile`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct LlmContentWithFileOpts {
    #[doc = "The media MIME type, e.g. \"image/png\". Inferred from the file's contents when omitted.\n\n`None` omits GraphQL field `mimeType` and preserves engine default `String(\"\")`."]
    pub mime_type: Option<String>,
}
impl LlmContentWithFileOpts {
    #[doc = "Sets GraphQL argument `mimeType` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }
}
impl crate::IntoID<crate::Id> for LlmContent {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for LlmContent {
    fn graphql_type() -> &'static str {
        "LLMContent"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<LlmContent> for crate::IdInput<LlmContent> {
    fn from(value: LlmContent) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<LlmContent> for crate::IdInput<super::NodeClient> {
    fn from(value: LlmContent) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl LlmContent {
    #[doc = "The ordered text and media blocks.\n\nSelects GraphQL field `blocks` on `LLMContent`."]
    pub async fn blocks(&self) -> Result<Vec<super::LlmContentBlock>, crate::QueryError> {
        let query = self.selection.select("blocks");
        let query = query.select("id");
        query
            .execute_reentry::<super::LlmContentBlock, Vec<crate::Id>>(
                &self.session,
                "LLMContentBlock",
            )
            .await
    }
    #[doc = "A unique identifier for this LLMContent.\n\nSelects GraphQL field `id` on `LLMContent`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "Append image, audio, or PDF bytes as an inline media block. The media kind follows the MIME type.\n\nPrefer withFile for anything but small payloads: the bytes become part of the content's identity, so they travel with every reference to it.\n\nSelects GraphQL field `withData` on `LLMContent`."]
    #[must_use]
    pub fn with_data(&self, data: super::Bytes, mime_type: impl Into<String>) -> super::LlmContent {
        let query = self.selection.select("withData");
        let query = query.arg("data", data);
        let query = query.arg("mimeType", mime_type.into());
        super::LlmContent {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Append an image, audio, or PDF file as an inline media block. The media kind follows the MIME type.\n\nSelects GraphQL field `withFile` on `LLMContent`."]
    #[must_use]
    pub fn with_file(&self, file: impl Into<crate::IdInput<super::File>>) -> super::LlmContent {
        let query = self.selection.select("withFile");
        let query = query.arg_id_input("file", file.into());
        super::LlmContent {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withFile` with a borrowed, reusable `LlmContentWithFileOpts` value."]
    #[must_use]
    pub fn with_file_opts(
        &self,
        file: impl Into<crate::IdInput<super::File>>,
        opts: &LlmContentWithFileOpts,
    ) -> super::LlmContent {
        let query = self.selection.select("withFile");
        let query = query.arg_id_input("file", file.into());
        let query = if let Some(value) = &opts.mime_type {
            query.arg("mimeType", value)
        } else {
            query
        };
        super::LlmContent {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Append a block of text.\n\nSelects GraphQL field `withText` on `LLMContent`."]
    #[must_use]
    pub fn with_text(&self, text: impl Into<String>) -> super::LlmContent {
        let query = self.selection.select("withText");
        let query = query.arg("text", text.into());
        super::LlmContent {
            session: self.session.clone(),
            selection: query,
        }
    }
}
impl super::Node for LlmContent {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
