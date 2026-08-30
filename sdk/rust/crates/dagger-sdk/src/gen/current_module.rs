//! Generated bindings owned by the GraphQL `CurrentModule` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
#[doc = "Reflective module API provided to functions at runtime."]
#[derive(Clone)]
pub struct CurrentModule {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
#[doc = "Owned optional arguments for GraphQL operation `CurrentModule.generators`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct CurrentModuleGeneratorsOpts {
    #[doc = "Only include generators matching the specified patterns\n\n`None` omits GraphQL field `include`."]
    pub include: Option<Vec<String>>,
}
impl CurrentModuleGeneratorsOpts {
    #[doc = "Sets GraphQL argument `include` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_include(mut self, value: Vec<impl Into<String>>) -> Self {
        self.include = Some(value.into_iter().map(Into::into).collect());
        self
    }
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
    #[doc = "Treat the currently executing module as an SDK installed in the given workspace, exposing the modules and clients it manages.\n\nErrors if the current module is not installed as an SDK in this workspace.\n\nSelects GraphQL field `asSDK` on `CurrentModule`."]
    #[must_use]
    pub fn as_sdk(
        &self,
        workspace: impl Into<crate::IdInput<super::Workspace>>,
    ) -> super::CurrentModuleAsSdk {
        let query = self.selection.select("asSDK");
        let query = query.arg_id_input("workspace", workspace.into());
        super::CurrentModuleAsSdk {
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
    #[doc = "Return all generators defined by the module\n\nSelects GraphQL field `generators` on `CurrentModule`.\n\n**Experimental:** This API is highly experimental and may be removed or replaced entirely."]
    #[must_use]
    pub fn generators(&self) -> super::GeneratorGroup {
        let query = self.selection.select("generators");
        super::GeneratorGroup {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `generators` with a borrowed, reusable `CurrentModuleGeneratorsOpts` value.\n\n**Experimental:** This API is highly experimental and may be removed or replaced entirely."]
    #[must_use]
    pub fn generators_opts(&self, opts: &CurrentModuleGeneratorsOpts) -> super::GeneratorGroup {
        let query = self.selection.select("generators");
        let query = if let Some(value) = &opts.include {
            query.arg("include", value)
        } else {
            query
        };
        super::GeneratorGroup {
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
