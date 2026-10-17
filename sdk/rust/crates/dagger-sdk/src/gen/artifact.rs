//! Generated bindings owned by the GraphQL `Artifact` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "One workspace value with a complete path and all required dimension keys. Reading metadata does not evaluate the value. Different addresses remain distinct even if they return the same object."]
#[derive(Clone)]
pub struct Artifact {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
#[doc = "Owned optional arguments for GraphQL operation `Artifact.uri`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct ArtifactUriOpts {
    #[doc = "Prefix the workspace's Git address and commit: dag://&lt;workspace&gt;@&lt;commit&gt;:&lt;path&gt;. Fails if the artifact has no workspace, or its workspace has no Git address.\n\n`None` omits GraphQL field `absolute` and preserves engine default `Boolean(false)`."]
    pub absolute: Option<bool>,
    #[doc = "Include the dimension keys as a query. Without them, the address is a path selector.\n\n`None` omits GraphQL field `dimensionKeys` and preserves engine default `Boolean(true)`."]
    pub dimension_keys: Option<bool>,
    #[doc = "Include the artifact type in the scheme: dag+container://.\n\n`None` omits GraphQL field `typeAssertion` and preserves engine default `Boolean(false)`."]
    pub type_assertion: Option<bool>,
}
impl ArtifactUriOpts {
    #[doc = "Sets GraphQL argument `absolute` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_absolute(mut self, value: bool) -> Self {
        self.absolute = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `dimensionKeys` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_dimension_keys(mut self, value: bool) -> Self {
        self.dimension_keys = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `typeAssertion` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_type_assertion(mut self, value: bool) -> Self {
        self.type_assertion = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Artifact.value`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct ArtifactValueOpts {
    #[doc = "Field arguments as a JSON object.\n\n`None` omits GraphQL field `arguments` and preserves engine default `String(\"{}\")`."]
    pub arguments: Option<crate::Json>,
}
impl ArtifactValueOpts {
    #[doc = "Sets GraphQL argument `arguments` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_arguments(mut self, value: crate::Json) -> Self {
        self.arguments = Some(value);
        self
    }
}
impl crate::IntoID<crate::Id> for Artifact {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for Artifact {
    fn graphql_type() -> &'static str {
        "Artifact"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<Artifact> for crate::IdInput<Artifact> {
    fn from(value: Artifact) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<Artifact> for crate::IdInput<super::NodeClient> {
    fn from(value: Artifact) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl Artifact {
    #[doc = "The arguments accepted by the artifact field.\n\nSelects GraphQL field `arguments` on `Artifact`."]
    pub async fn arguments(&self) -> Result<Vec<super::FunctionArg>, crate::QueryError> {
        let query = self.selection.select("arguments");
        let query = query.select("id");
        query
            .execute_reentry::<super::FunctionArg, Vec<crate::Id>>(&self.session, "FunctionArg")
            .await
    }
    #[doc = "The description of the field that supplies this artifact.\n\nSelects GraphQL field `description` on `Artifact`."]
    pub async fn description(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("description");
        query.execute(&self.session).await
    }
    #[doc = "The module name, collection keys, and full path key in the artifact type dimension.\n\nSelects GraphQL field `dimensionKeys` on `Artifact`."]
    pub async fn dimension_keys(
        &self,
    ) -> Result<Vec<super::ArtifactDimensionKey>, crate::QueryError> {
        let query = self.selection.select("dimensionKeys");
        let query = query.select("id");
        query
            .execute_reentry::<super::ArtifactDimensionKey, Vec<crate::Id>>(
                &self.session,
                "ArtifactDimensionKey",
            )
            .await
    }
    #[doc = "The directives carried by this artifact.\n\nSelects GraphQL field `directives` on `Artifact`."]
    pub async fn directives(&self) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("directives");
        query.execute(&self.session).await
    }
    #[doc = "A unique identifier for this Artifact.\n\nSelects GraphQL field `id` on `Artifact`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "A module load failure, or an empty string if discovery succeeded.\n\nSelects GraphQL field `loadError` on `Artifact`."]
    pub async fn load_error(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("loadError");
        query.execute(&self.session).await
    }
    #[doc = "The installed module name.\n\nSelects GraphQL field `moduleName` on `Artifact`."]
    pub async fn module_name(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("moduleName");
        query.execute(&self.session).await
    }
    #[doc = "Ordered, literal fields to follow. Entrypoint targets use their shorthand.\n\nSelects GraphQL field `path` on `Artifact`."]
    pub async fn path(&self) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("path");
        query.execute(&self.session).await
    }
    #[doc = "The artifact's DAG address, such as dag://engine-dev/playground.\n\nSelects GraphQL field `uri` on `Artifact`."]
    pub async fn uri(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("uri");
        query.execute(&self.session).await
    }
    #[doc = "Executes GraphQL operation `uri` with a borrowed, reusable `ArtifactUriOpts` value."]
    pub async fn uri_opts(&self, opts: &ArtifactUriOpts) -> Result<String, crate::QueryError> {
        let query = self.selection.select("uri");
        let query = if let Some(value) = &opts.absolute {
            query.arg("absolute", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.dimension_keys {
            query.arg("dimensionKeys", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.type_assertion {
            query.arg("typeAssertion", value)
        } else {
            query
        };
        query.execute(&self.session).await
    }
    #[doc = "Evaluate the target in the workspace that supplied this artifact.\n\nSelects GraphQL field `value` on `Artifact`."]
    #[must_use]
    pub fn value(&self) -> super::NodeClient {
        let query = self.selection.select("value");
        super::NodeClient {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `value` with a borrowed, reusable `ArtifactValueOpts` value."]
    #[must_use]
    pub fn value_opts(&self, opts: &ArtifactValueOpts) -> super::NodeClient {
        let query = self.selection.select("value");
        let query = if let Some(value) = &opts.arguments {
            query.arg("arguments", value)
        } else {
            query
        };
        super::NodeClient {
            session: self.session.clone(),
            selection: query,
        }
    }
}
impl super::Node for Artifact {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
