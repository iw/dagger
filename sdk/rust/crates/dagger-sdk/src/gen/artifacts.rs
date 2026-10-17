//! Generated bindings owned by the GraphQL `Artifacts` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "An immutable selection of workspace artifacts. Listed types, dimensions, and keys use OR; chained filters use AND. Empty alternatives and unknown names match nothing. Filters never change addresses or dimension identifiers."]
#[derive(Clone)]
pub struct Artifacts {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
#[doc = "Owned optional arguments for GraphQL operation `Artifacts.filterDirectives`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct ArtifactsFilterDirectivesOpts {
    #[doc = "Remove the matching artifacts instead.\n\n`None` omits GraphQL field `exclude` and preserves engine default `Boolean(false)`."]
    pub exclude: Option<bool>,
}
impl ArtifactsFilterDirectivesOpts {
    #[doc = "Sets GraphQL argument `exclude` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_exclude(mut self, value: bool) -> Self {
        self.exclude = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Artifacts.filterParentDirectives`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct ArtifactsFilterParentDirectivesOpts {
    #[doc = "Remove the matching artifacts instead.\n\n`None` omits GraphQL field `exclude` and preserves engine default `Boolean(false)`."]
    pub exclude: Option<bool>,
}
impl ArtifactsFilterParentDirectivesOpts {
    #[doc = "Sets GraphQL argument `exclude` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_exclude(mut self, value: bool) -> Self {
        self.exclude = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Artifacts.filterParentTypes`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct ArtifactsFilterParentTypesOpts {
    #[doc = "Remove the matching artifacts instead.\n\n`None` omits GraphQL field `exclude` and preserves engine default `Boolean(false)`."]
    pub exclude: Option<bool>,
}
impl ArtifactsFilterParentTypesOpts {
    #[doc = "Sets GraphQL argument `exclude` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_exclude(mut self, value: bool) -> Self {
        self.exclude = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Artifacts.filterTypes`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct ArtifactsFilterTypesOpts {
    #[doc = "Remove the matching artifacts instead.\n\n`None` omits GraphQL field `exclude` and preserves engine default `Boolean(false)`."]
    pub exclude: Option<bool>,
}
impl ArtifactsFilterTypesOpts {
    #[doc = "Sets GraphQL argument `exclude` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_exclude(mut self, value: bool) -> Self {
        self.exclude = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Artifacts.pathDefinitions`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct ArtifactsPathDefinitionsOpts {
    #[doc = "Prefix each address with the workspace's Git address and commit.\n\n`None` omits GraphQL field `absolute` and preserves engine default `Boolean(false)`."]
    pub absolute: Option<bool>,
    #[doc = "Project paths to the items of this collection dimension. Preserve parent dimensions and remove descendant dimensions.\n\n`None` omits GraphQL field `dimension`."]
    pub dimension: Option<String>,
    #[doc = "Include the artifact type in each address scheme.\n\n`None` omits GraphQL field `typeAssertion` and preserves engine default `Boolean(false)`."]
    pub type_assertion: Option<bool>,
}
impl ArtifactsPathDefinitionsOpts {
    #[doc = "Sets GraphQL argument `absolute` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_absolute(mut self, value: bool) -> Self {
        self.absolute = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `dimension` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_dimension(mut self, value: impl Into<String>) -> Self {
        self.dimension = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `typeAssertion` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_type_assertion(mut self, value: bool) -> Self {
        self.type_assertion = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Artifacts.values`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct ArtifactsValuesOpts {
    #[doc = "Field arguments applied to each artifact, as a JSON object.\n\n`None` omits GraphQL field `arguments` and preserves engine default `String(\"{}\")`."]
    pub arguments: Option<crate::Json>,
    #[doc = "Cancel remaining work after the first failure.\n\n`None` omits GraphQL field `failFast` and preserves engine default `Boolean(false)`."]
    pub fail_fast: Option<bool>,
    #[doc = "Evaluate at most this many artifacts at once on this engine; the rest wait. Checks that scale out to cloud engines are not counted. 0 means no limit.\n\n`None` omits GraphQL field `maxConcurrency` and preserves engine default `Int(0)`."]
    pub max_concurrency: Option<i64>,
}
impl ArtifactsValuesOpts {
    #[doc = "Sets GraphQL argument `arguments` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_arguments(mut self, value: crate::Json) -> Self {
        self.arguments = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `failFast` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_fail_fast(mut self, value: bool) -> Self {
        self.fail_fast = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `maxConcurrency` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_max_concurrency(mut self, value: i64) -> Self {
        self.max_concurrency = Some(value);
        self
    }
}
impl crate::IntoID<crate::Id> for Artifacts {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for Artifacts {
    fn graphql_type() -> &'static str {
        "Artifacts"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<Artifacts> for crate::IdInput<Artifacts> {
    fn from(value: Artifacts) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<Artifacts> for crate::IdInput<super::NodeClient> {
    fn from(value: Artifacts) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl Artifacts {
    #[doc = "Convert the selection to Changesets. Fail if any artifact is not a Changeset. Does not apply command filters.\n\nSelects GraphQL field `asChangesets` on `Artifacts`."]
    pub async fn as_changesets(&self) -> Result<Vec<super::Changeset>, crate::QueryError> {
        let query = self.selection.select("asChangesets");
        let query = query.select("id");
        query
            .execute_reentry::<super::Changeset, Vec<crate::Id>>(&self.session, "Changeset")
            .await
    }
    #[doc = "Convert the selection to Checks. Fail if any artifact is not a Check. Does not apply command filters or run the checks.\n\nSelects GraphQL field `asChecks` on `Artifacts`."]
    pub async fn as_checks(&self) -> Result<Vec<super::Check>, crate::QueryError> {
        let query = self.selection.select("asChecks");
        let query = query.select("id");
        query
            .execute_reentry::<super::Check, Vec<crate::Id>>(&self.session, "Check")
            .await
    }
    #[doc = "Convert the selection to expertise without running the functions. Fail if any artifact is not a source of expertise.\n\nSelects GraphQL field `asExpertise` on `Artifacts`."]
    pub async fn as_expertise(&self) -> Result<Vec<super::Expertise>, crate::QueryError> {
        let query = self.selection.select("asExpertise");
        let query = query.select("id");
        query
            .execute_reentry::<super::Expertise, Vec<crate::Id>>(&self.session, "Expertise")
            .await
    }
    #[doc = "Convert the selection to Generators without running them. Fail if any artifact is not a Generator.\n\nSelects GraphQL field `asGenerators` on `Artifacts`."]
    pub async fn as_generators(&self) -> Result<Vec<super::Generator>, crate::QueryError> {
        let query = self.selection.select("asGenerators");
        let query = query.select("id");
        query
            .execute_reentry::<super::Generator, Vec<crate::Id>>(&self.session, "Generator")
            .await
    }
    #[doc = "Convert the selection to Services. Fail if any artifact is not a Service. Does not apply command filters or start the services.\n\nSelects GraphQL field `asServices` on `Artifacts`."]
    pub async fn as_services(&self) -> Result<Vec<super::Service>, crate::QueryError> {
        let query = self.selection.select("asServices");
        let query = query.select("id");
        query
            .execute_reentry::<super::Service, Vec<crate::Id>>(&self.session, "Service")
            .await
    }
    #[doc = "List dimensions on the selected schema paths, including empty collections. Does not read runtime values.\n\nSelects GraphQL field `dimensionDefinitions` on `Artifacts`."]
    pub async fn dimension_definitions(
        &self,
    ) -> Result<Vec<super::ArtifactDimension>, crate::QueryError> {
        let query = self.selection.select("dimensionDefinitions");
        let query = query.select("id");
        query
            .execute_reentry::<super::ArtifactDimension, Vec<crate::Id>>(
                &self.session,
                "ArtifactDimension",
            )
            .await
    }
    #[doc = "List collection items represented in this selection for the given dimension. Preserve parent keys and remove duplicate item addresses. Does not evaluate item values.\n\nSelects GraphQL field `dimensionItems` on `Artifacts`."]
    pub async fn dimension_items(
        &self,
        dimension: impl Into<String>,
    ) -> Result<Vec<super::Artifact>, crate::QueryError> {
        let query = self.selection.select("dimensionItems");
        let query = query.arg("dimension", dimension.into());
        let query = query.select("id");
        query
            .execute_reentry::<super::Artifact, Vec<crate::Id>>(&self.session, "Artifact")
            .await
    }
    #[doc = "List keys represented in this selection for the given dimension, sorted with no duplicates.\n\nSelects GraphQL field `dimensionKeys` on `Artifacts`."]
    pub async fn dimension_keys(
        &self,
        dimension: impl Into<String>,
    ) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("dimensionKeys");
        let query = query.arg("dimension", dimension.into());
        query.execute(&self.session).await
    }
    #[doc = "List dimension identifiers represented in this selection, sorted with no duplicates.\n\nSelects GraphQL field `dimensions` on `Artifacts`."]
    pub async fn dimensions(&self) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("dimensions");
        query.execute(&self.session).await
    }
    #[doc = "Keep artifacts with any listed key in this dimension.\n\nSelects GraphQL field `filterDimensionKeys` on `Artifacts`."]
    #[must_use]
    pub fn filter_dimension_keys(
        &self,
        dimension: impl Into<String>,
        keys: Vec<impl Into<String>>,
    ) -> super::Artifacts {
        let query = self.selection.select("filterDimensionKeys");
        let query = query.arg("dimension", dimension.into());
        let keys = keys.into_iter().map(Into::into).collect::<Vec<String>>();
        let query = query.arg("keys", keys);
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Keep artifacts selected through any listed dimension.\n\nSelects GraphQL field `filterDimensions` on `Artifacts`."]
    #[must_use]
    pub fn filter_dimensions(&self, dimensions: Vec<impl Into<String>>) -> super::Artifacts {
        let query = self.selection.select("filterDimensions");
        let dimensions = dimensions
            .into_iter()
            .map(Into::into)
            .collect::<Vec<String>>();
        let query = query.arg("dimensions", dimensions);
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Keep artifacts with any listed directive. Does not filter by type or workspace settings.\n\nSelects GraphQL field `filterDirectives` on `Artifacts`."]
    #[must_use]
    pub fn filter_directives(&self, directives: Vec<impl Into<String>>) -> super::Artifacts {
        let query = self.selection.select("filterDirectives");
        let directives = directives
            .into_iter()
            .map(Into::into)
            .collect::<Vec<String>>();
        let query = query.arg("directives", directives);
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `filterDirectives` with a borrowed, reusable `ArtifactsFilterDirectivesOpts` value."]
    #[must_use]
    pub fn filter_directives_opts(
        &self,
        directives: Vec<impl Into<String>>,
        opts: &ArtifactsFilterDirectivesOpts,
    ) -> super::Artifacts {
        let query = self.selection.select("filterDirectives");
        let directives = directives
            .into_iter()
            .map(Into::into)
            .collect::<Vec<String>>();
        let query = query.arg("directives", directives);
        let query = if let Some(value) = &opts.exclude {
            query.arg("exclude", value)
        } else {
            query
        };
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Keep artifacts whose immediate parent has any listed directive. Artifacts without a parent do not match.\n\nSelects GraphQL field `filterParentDirectives` on `Artifacts`."]
    #[must_use]
    pub fn filter_parent_directives(&self, directives: Vec<impl Into<String>>) -> super::Artifacts {
        let query = self.selection.select("filterParentDirectives");
        let directives = directives
            .into_iter()
            .map(Into::into)
            .collect::<Vec<String>>();
        let query = query.arg("directives", directives);
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `filterParentDirectives` with a borrowed, reusable `ArtifactsFilterParentDirectivesOpts` value."]
    #[must_use]
    pub fn filter_parent_directives_opts(
        &self,
        directives: Vec<impl Into<String>>,
        opts: &ArtifactsFilterParentDirectivesOpts,
    ) -> super::Artifacts {
        let query = self.selection.select("filterParentDirectives");
        let directives = directives
            .into_iter()
            .map(Into::into)
            .collect::<Vec<String>>();
        let query = query.arg("directives", directives);
        let query = if let Some(value) = &opts.exclude {
            query.arg("exclude", value)
        } else {
            query
        };
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Keep artifacts whose immediate parent has any listed object type. Artifacts without a typed parent do not match.\n\nSelects GraphQL field `filterParentTypes` on `Artifacts`."]
    #[must_use]
    pub fn filter_parent_types(&self, types: Vec<impl Into<String>>) -> super::Artifacts {
        let query = self.selection.select("filterParentTypes");
        let types = types.into_iter().map(Into::into).collect::<Vec<String>>();
        let query = query.arg("types", types);
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `filterParentTypes` with a borrowed, reusable `ArtifactsFilterParentTypesOpts` value."]
    #[must_use]
    pub fn filter_parent_types_opts(
        &self,
        types: Vec<impl Into<String>>,
        opts: &ArtifactsFilterParentTypesOpts,
    ) -> super::Artifacts {
        let query = self.selection.select("filterParentTypes");
        let query = if let Some(value) = &opts.exclude {
            query.arg("exclude", value)
        } else {
            query
        };
        let types = types.into_iter().map(Into::into).collect::<Vec<String>>();
        let query = query.arg("types", types);
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Match one complete, ordered field sequence exactly.\n\nSelects GraphQL field `filterPath` on `Artifacts`."]
    #[must_use]
    pub fn filter_path(&self, path: Vec<impl Into<String>>) -> super::Artifacts {
        let query = self.selection.select("filterPath");
        let path = path.into_iter().map(Into::into).collect::<Vec<String>>();
        let query = query.arg("path", path);
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Keep paths that match a glob pattern. A literal path matches exactly. Both module-qualified and entrypoint paths match.\n\nSelects GraphQL field `filterPathPattern` on `Artifacts`."]
    #[must_use]
    pub fn filter_path_pattern(&self, pattern: impl Into<String>) -> super::Artifacts {
        let query = self.selection.select("filterPathPattern");
        let query = query.arg("pattern", pattern.into());
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Keep artifacts of any listed concrete GraphQL type.\n\nSelects GraphQL field `filterTypes` on `Artifacts`."]
    #[must_use]
    pub fn filter_types(&self, types: Vec<impl Into<String>>) -> super::Artifacts {
        let query = self.selection.select("filterTypes");
        let types = types.into_iter().map(Into::into).collect::<Vec<String>>();
        let query = query.arg("types", types);
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `filterTypes` with a borrowed, reusable `ArtifactsFilterTypesOpts` value."]
    #[must_use]
    pub fn filter_types_opts(
        &self,
        types: Vec<impl Into<String>>,
        opts: &ArtifactsFilterTypesOpts,
    ) -> super::Artifacts {
        let query = self.selection.select("filterTypes");
        let query = if let Some(value) = &opts.exclude {
            query.arg("exclude", value)
        } else {
            query
        };
        let types = types.into_iter().map(Into::into).collect::<Vec<String>>();
        let query = query.arg("types", types);
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Apply a DAG address as one filter: the chain of path, type, and dimension-key filters it encodes.\n\nThe scheme is optional. The path may be a pattern; an empty path selects all artifacts.\n\nSelects GraphQL field `filterUri` on `Artifacts`."]
    #[must_use]
    pub fn filter_uri(&self, uri: impl Into<String>) -> super::Artifacts {
        let query = self.selection.select("filterUri");
        let query = query.arg("uri", uri.into());
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "A unique identifier for this Artifacts.\n\nSelects GraphQL field `id` on `Artifacts`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "Enumerate complete artifacts without evaluating their values.\n\nSelects GraphQL field `items` on `Artifacts`."]
    pub async fn items(&self) -> Result<Vec<super::Artifact>, crate::QueryError> {
        let query = self.selection.select("items");
        let query = query.select("id");
        query
            .execute_reentry::<super::Artifact, Vec<crate::Id>>(&self.session, "Artifact")
            .await
    }
    #[doc = "List the modules represented in this selection without evaluating artifact values.\n\nSelects GraphQL field `modules` on `Artifacts`."]
    pub async fn modules(&self) -> Result<Vec<super::Module>, crate::QueryError> {
        let query = self.selection.select("modules");
        let query = query.select("id");
        query
            .execute_reentry::<super::Module, Vec<crate::Id>>(&self.session, "Module")
            .await
    }
    #[doc = "Require exactly one artifact; fail if there are zero or multiple matches. Several matches are listed, one address per line.\n\nSelects GraphQL field `one` on `Artifacts`."]
    #[must_use]
    pub fn one(&self) -> super::Artifact {
        let query = self.selection.select("one");
        super::Artifact {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "List selected schema paths, including empty collections. Does not read runtime values. Applies type keys and collection presence; collection key values require items.\n\nSelects GraphQL field `pathDefinitions` on `Artifacts`."]
    pub async fn path_definitions(&self) -> Result<Vec<super::ArtifactPath>, crate::QueryError> {
        let query = self.selection.select("pathDefinitions");
        let query = query.select("id");
        query
            .execute_reentry::<super::ArtifactPath, Vec<crate::Id>>(&self.session, "ArtifactPath")
            .await
    }
    #[doc = "Executes GraphQL operation `pathDefinitions` with a borrowed, reusable `ArtifactsPathDefinitionsOpts` value."]
    pub async fn path_definitions_opts(
        &self,
        opts: &ArtifactsPathDefinitionsOpts,
    ) -> Result<Vec<super::ArtifactPath>, crate::QueryError> {
        let query = self.selection.select("pathDefinitions");
        let query = if let Some(value) = &opts.absolute {
            query.arg("absolute", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.dimension {
            query.arg("dimension", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.type_assertion {
            query.arg("typeAssertion", value)
        } else {
            query
        };
        let query = query.select("id");
        query
            .execute_reentry::<super::ArtifactPath, Vec<crate::Id>>(&self.session, "ArtifactPath")
            .await
    }
    #[doc = "List concrete type definitions represented in this selection, sorted by name with no duplicates.\n\nSelects GraphQL field `types` on `Artifacts`."]
    pub async fn types(&self) -> Result<Vec<super::TypeDef>, crate::QueryError> {
        let query = self.selection.select("types");
        let query = query.select("id");
        query
            .execute_reentry::<super::TypeDef, Vec<crate::Id>>(&self.session, "TypeDef")
            .await
    }
    #[doc = "The DAG address that selects this whole selection: filterUri(uri) selects the same set.\n\nSelects GraphQL field `uri` on `Artifacts`."]
    pub async fn uri(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("uri");
        query.execute(&self.session).await
    }
    #[doc = "Evaluate the selection in parallel, retaining each result and error.\n\nSelects GraphQL field `values` on `Artifacts`."]
    pub async fn values(&self) -> Result<Vec<super::ArtifactResult>, crate::QueryError> {
        let query = self.selection.select("values");
        let query = query.select("id");
        query
            .execute_reentry::<super::ArtifactResult, Vec<crate::Id>>(
                &self.session,
                "ArtifactResult",
            )
            .await
    }
    #[doc = "Executes GraphQL operation `values` with a borrowed, reusable `ArtifactsValuesOpts` value."]
    pub async fn values_opts(
        &self,
        opts: &ArtifactsValuesOpts,
    ) -> Result<Vec<super::ArtifactResult>, crate::QueryError> {
        let query = self.selection.select("values");
        let query = if let Some(value) = &opts.arguments {
            query.arg("arguments", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.fail_fast {
            query.arg("failFast", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.max_concurrency {
            query.arg("maxConcurrency", value)
        } else {
            query
        };
        let query = query.select("id");
        query
            .execute_reentry::<super::ArtifactResult, Vec<crate::Id>>(
                &self.session,
                "ArtifactResult",
            )
            .await
    }
    #[doc = "Combine two selections, keeping each workspace address once. Different addresses remain distinct even if they return the same object.\n\nSelects GraphQL field `withArtifacts` on `Artifacts`."]
    #[must_use]
    pub fn with_artifacts(
        &self,
        artifacts: impl Into<crate::IdInput<super::Artifacts>>,
    ) -> super::Artifacts {
        let query = self.selection.select("withArtifacts");
        let query = query.arg_id_input("artifacts", artifacts.into());
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Remove artifacts selected by a DAG address.\n\nSelects GraphQL field `withoutUri` on `Artifacts`."]
    #[must_use]
    pub fn without_uri(&self, uri: impl Into<String>) -> super::Artifacts {
        let query = self.selection.select("withoutUri");
        let query = query.arg("uri", uri.into());
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
}
impl super::Node for Artifacts {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
