//! Generated bindings owned by the GraphQL `CurrentModule` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "Reflective module API provided to functions at runtime."]
#[derive(Clone)]
pub struct CurrentModule {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
#[doc = "Owned optional arguments for GraphQL operation `CurrentModule.workdir`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct CurrentModuleWorkdirOpts {
    #[doc = "Exclude artifacts that match the given pattern (e.g., \\[\"node_modules/\", \".git*\"\\]).\n\n`None` omits GraphQL field `exclude` and preserves engine default `List(\\[\\])`."]
    pub exclude: Option<Vec<String>>,
    #[doc = "Apply .gitignore filter rules inside the directory\n\n`None` omits GraphQL field `gitignore` and preserves engine default `Boolean(false)`."]
    pub gitignore: Option<bool>,
    #[doc = "Include only artifacts that match the given pattern (e.g., \\[\"app/\", \"package.*\"\\]).\n\n`None` omits GraphQL field `include` and preserves engine default `List(\\[\\])`."]
    pub include: Option<Vec<String>>,
}
impl CurrentModuleWorkdirOpts {
    #[doc = "Sets GraphQL argument `exclude` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_exclude(mut self, value: Vec<impl Into<String>>) -> Self {
        self.exclude = Some(value.into_iter().map(Into::into).collect());
        self
    }
    #[doc = "Sets GraphQL argument `gitignore` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_gitignore(mut self, value: bool) -> Self {
        self.gitignore = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `include` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_include(mut self, value: Vec<impl Into<String>>) -> Self {
        self.include = Some(value.into_iter().map(Into::into).collect());
        self
    }
}
impl crate::IntoID<crate::Id> for CurrentModule {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for CurrentModule {
    fn graphql_type() -> &'static str {
        "CurrentModule"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<CurrentModule> for crate::IdInput<CurrentModule> {
    fn from(value: CurrentModule) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<CurrentModule> for crate::IdInput<super::NodeClient> {
    fn from(value: CurrentModule) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl CurrentModule {
    #[doc = "The module's context directory loaded into the engine (plus any generated code that may have been created).\n\nIt holds the module's source code and every other file the engine loaded for the module, at their paths relative to the context root. Use sourceSubpath to locate the source code in it.\n\nSelects GraphQL field `contextDirectory` on `CurrentModule`."]
    #[must_use]
    pub fn context_directory(&self) -> super::Directory {
        let query = self.selection.select("contextDirectory");
        super::Directory {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "The dependencies of the module.\n\nSelects GraphQL field `dependencies` on `CurrentModule`."]
    pub async fn dependencies(&self) -> Result<Vec<super::Module>, crate::QueryError> {
        let query = self.selection.select("dependencies");
        let query = query.select("id");
        query
            .execute_reentry::<super::Module, Vec<crate::Id>>(&self.session, "Module")
            .await
    }
    #[doc = "The generated files and directories made on top of the module source's context directory.\n\nSelects GraphQL field `generatedContextDirectory` on `CurrentModule`."]
    #[must_use]
    pub fn generated_context_directory(&self) -> super::Directory {
        let query = self.selection.select("generatedContextDirectory");
        super::Directory {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "A unique identifier for this CurrentModule.\n\nSelects GraphQL field `id` on `CurrentModule`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "The name of the module being executed in\n\nSelects GraphQL field `name` on `CurrentModule`."]
    pub async fn name(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("name");
        query.execute(&self.session).await
    }
    #[doc = "The directory containing the module's source code loaded into the engine (plus any generated code that may have been created).\n\nSelects GraphQL field `source` on `CurrentModule`."]
    #[must_use]
    pub fn source(&self) -> super::Directory {
        let query = self.selection.select("source");
        super::Directory {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "The path to the directory containing the module's source code, relative to the context directory.\n\nSelects GraphQL field `sourceSubpath` on `CurrentModule`."]
    pub async fn source_subpath(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("sourceSubpath");
        query.execute(&self.session).await
    }
    #[doc = "Load a directory from the module's scratch working directory, including any changes that may have been made to it during module function execution.\n\nSelects GraphQL field `workdir` on `CurrentModule`."]
    #[must_use]
    pub fn workdir(&self, path: impl Into<String>) -> super::Directory {
        let query = self.selection.select("workdir");
        let query = query.arg("path", path.into());
        super::Directory {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `workdir` with a borrowed, reusable `CurrentModuleWorkdirOpts` value."]
    #[must_use]
    pub fn workdir_opts(
        &self,
        path: impl Into<String>,
        opts: &CurrentModuleWorkdirOpts,
    ) -> super::Directory {
        let query = self.selection.select("workdir");
        let query = if let Some(value) = &opts.exclude {
            query.arg("exclude", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.gitignore {
            query.arg("gitignore", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.include {
            query.arg("include", value)
        } else {
            query
        };
        let query = query.arg("path", path.into());
        super::Directory {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Load a file from the module's scratch working directory, including any changes that may have been made to it during module function execution.Load a file from the module's scratch working directory, including any changes that may have been made to it during module function execution.\n\nSelects GraphQL field `workdirFile` on `CurrentModule`."]
    #[must_use]
    pub fn workdir_file(&self, path: impl Into<String>) -> super::File {
        let query = self.selection.select("workdirFile");
        let query = query.arg("path", path.into());
        super::File {
            session: self.session.clone(),
            selection: query,
        }
    }
}
impl super::Node for CurrentModule {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
