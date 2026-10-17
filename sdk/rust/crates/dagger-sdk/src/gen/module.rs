//! Generated bindings owned by the GraphQL `Module` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "A Dagger module."]
#[derive(Clone)]
pub struct Module {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
#[doc = "Owned optional arguments for GraphQL operation `Module.serve`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct ModuleServeOpts {
    #[doc = "Install the module as the entrypoint, promoting its main-object methods onto the Query root\n\n`None` omits GraphQL field `entrypoint`."]
    pub entrypoint: Option<bool>,
    #[doc = "Expose the dependencies of this module to the client\n\n`None` omits GraphQL field `includeDependencies`."]
    pub include_dependencies: Option<bool>,
}
impl ModuleServeOpts {
    #[doc = "Sets GraphQL argument `entrypoint` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_entrypoint(mut self, value: bool) -> Self {
        self.entrypoint = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `includeDependencies` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_include_dependencies(mut self, value: bool) -> Self {
        self.include_dependencies = Some(value);
        self
    }
}
impl crate::IntoID<crate::Id> for Module {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for Module {
    fn graphql_type() -> &'static str {
        "Module"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<Module> for crate::IdInput<Module> {
    fn from(value: Module) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<Module> for crate::IdInput<super::NodeClient> {
    fn from(value: Module) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<Module> for crate::IdInput<super::SyncerClient> {
    fn from(value: Module) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl Module {
    #[doc = "The source used to resolve contextual files and directories, when different from source.\n\nSelects GraphQL field `contextSource` on `Module`."]
    pub async fn context_source(&self) -> Result<Option<super::ModuleSource>, crate::QueryError> {
        let query = self.selection.select("contextSource");
        let query = query.select("id");
        query
            .execute_reentry::<super::ModuleSource, Option<crate::Id>>(
                &self.session,
                "ModuleSource",
            )
            .await
    }
    #[doc = "The dependencies of the module.\n\nSelects GraphQL field `dependencies` on `Module`."]
    pub async fn dependencies(&self) -> Result<Vec<super::Module>, crate::QueryError> {
        let query = self.selection.select("dependencies");
        let query = query.select("id");
        query
            .execute_reentry::<super::Module, Vec<crate::Id>>(&self.session, "Module")
            .await
    }
    #[doc = "The doc string of the module, if any\n\nSelects GraphQL field `description` on `Module`."]
    pub async fn description(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("description");
        query.execute(&self.session).await
    }
    #[doc = "Enumerations served by this module.\n\nSelects GraphQL field `enums` on `Module`."]
    pub async fn enums(&self) -> Result<Vec<super::TypeDef>, crate::QueryError> {
        let query = self.selection.select("enums");
        let query = query.select("id");
        query
            .execute_reentry::<super::TypeDef, Vec<crate::Id>>(&self.session, "TypeDef")
            .await
    }
    #[doc = "The generated files and directories made on top of the module source's context directory.\n\nSelects GraphQL field `generatedContextDirectory` on `Module`."]
    #[must_use]
    pub fn generated_context_directory(&self) -> super::Directory {
        let query = self.selection.select("generatedContextDirectory");
        super::Directory {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "A unique identifier for this Module.\n\nSelects GraphQL field `id` on `Module`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "Interfaces served by this module.\n\nSelects GraphQL field `interfaces` on `Module`."]
    pub async fn interfaces(&self) -> Result<Vec<super::TypeDef>, crate::QueryError> {
        let query = self.selection.select("interfaces");
        let query = query.select("id");
        query
            .execute_reentry::<super::TypeDef, Vec<crate::Id>>(&self.session, "TypeDef")
            .await
    }
    #[doc = "The introspection schema JSON file for this module.\n\nThis file represents the schema visible to the module's source code, including all core types and those from the dependencies.\n\nNote: this is in the context of a module, so some core types may be hidden.\n\nSelects GraphQL field `introspectionSchemaJSON` on `Module`."]
    #[must_use]
    pub fn introspection_schema_json(&self) -> super::File {
        let query = self.selection.select("introspectionSchemaJSON");
        super::File {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "The name of the module\n\nSelects GraphQL field `name` on `Module`."]
    pub async fn name(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("name");
        query.execute(&self.session).await
    }
    #[doc = "Objects served by this module.\n\nSelects GraphQL field `objects` on `Module`."]
    pub async fn objects(&self) -> Result<Vec<super::TypeDef>, crate::QueryError> {
        let query = self.selection.select("objects");
        let query = query.select("id");
        query
            .execute_reentry::<super::TypeDef, Vec<crate::Id>>(&self.session, "TypeDef")
            .await
    }
    #[doc = "The container that runs the module's entrypoint. It will fail to execute if the module doesn't compile.\n\nSelects GraphQL field `runtime` on `Module`."]
    pub async fn runtime(&self) -> Result<Option<super::Container>, crate::QueryError> {
        let query = self.selection.select("runtime");
        let query = query.select("id");
        query
            .execute_reentry::<super::Container, Option<crate::Id>>(&self.session, "Container")
            .await
    }
    #[doc = "The SDK config used by this module.\n\nSelects GraphQL field `sdk` on `Module`."]
    pub async fn sdk(&self) -> Result<Option<super::SdkConfig>, crate::QueryError> {
        let query = self.selection.select("sdk");
        let query = query.select("id");
        query
            .execute_reentry::<super::SdkConfig, Option<crate::Id>>(&self.session, "SDKConfig")
            .await
    }
    #[doc = "Serve a module's API in the current session.\n\nNote: this can only be called once per session. In the future, it could return a stream or service to remove the side effect.\n\nSelects GraphQL field `serve` on `Module`."]
    pub async fn serve(&self) -> Result<(), crate::QueryError> {
        let query = self.selection.select("serve");
        query.execute_void(&self.session).await
    }
    #[doc = "Executes GraphQL operation `serve` with a borrowed, reusable `ModuleServeOpts` value."]
    pub async fn serve_opts(&self, opts: &ModuleServeOpts) -> Result<(), crate::QueryError> {
        let query = self.selection.select("serve");
        let query = if let Some(value) = &opts.entrypoint {
            query.arg("entrypoint", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.include_dependencies {
            query.arg("includeDependencies", value)
        } else {
            query
        };
        query.execute_void(&self.session).await
    }
    #[doc = "The source for the module.\n\nSelects GraphQL field `source` on `Module`."]
    pub async fn source(&self) -> Result<Option<super::ModuleSource>, crate::QueryError> {
        let query = self.selection.select("source");
        let query = query.select("id");
        query
            .execute_reentry::<super::ModuleSource, Option<crate::Id>>(
                &self.session,
                "ModuleSource",
            )
            .await
    }
    #[doc = "Forces evaluation of the module, including any loading into the engine and associated validation.\n\nSelects GraphQL field `sync` on `Module`."]
    pub async fn sync(&self) -> Result<super::Module, crate::QueryError> {
        let query = self.selection.select("sync");
        let id: crate::Id = query.execute(&self.session).await?;
        Ok(crate::query::reenter::<super::Module>(
            &self.session,
            id,
            "Module",
        ))
    }
    #[doc = "User-defined default values, loaded from local .env files.\n\nSelects GraphQL field `userDefaults` on `Module`."]
    #[must_use]
    pub fn user_defaults(&self) -> super::EnvFile {
        let query = self.selection.select("userDefaults");
        super::EnvFile {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Retrieves the module with the given description\n\nSelects GraphQL field `withDescription` on `Module`."]
    #[must_use]
    pub fn with_description(&self, description: impl Into<String>) -> super::Module {
        let query = self.selection.select("withDescription");
        let query = query.arg("description", description.into());
        super::Module {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "This module plus the given Enum type and associated values\n\nSelects GraphQL field `withEnum` on `Module`."]
    #[must_use]
    pub fn with_enum(&self, r#enum: impl Into<crate::IdInput<super::TypeDef>>) -> super::Module {
        let query = self.selection.select("withEnum");
        let query = query.arg_id_input("enum", r#enum.into());
        super::Module {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "This module plus the given Interface type and associated functions\n\nSelects GraphQL field `withInterface` on `Module`."]
    #[must_use]
    pub fn with_interface(
        &self,
        iface: impl Into<crate::IdInput<super::TypeDef>>,
    ) -> super::Module {
        let query = self.selection.select("withInterface");
        let query = query.arg_id_input("iface", iface.into());
        super::Module {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "This module plus the given Object type and associated functions.\n\nSelects GraphQL field `withObject` on `Module`."]
    #[must_use]
    pub fn with_object(&self, object: impl Into<crate::IdInput<super::TypeDef>>) -> super::Module {
        let query = self.selection.select("withObject");
        let query = query.arg_id_input("object", object.into());
        super::Module {
            session: self.session.clone(),
            selection: query,
        }
    }
}
impl super::Node for Module {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
impl super::Syncer for Module {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
    fn sync(
        &self,
    ) -> impl core::future::Future<Output = Result<super::SyncerClient, crate::QueryError>> + Send
    {
        let query = self.selection.select("sync");
        let session = self.session.clone();
        async move {
            let id: crate::Id = query.execute(&session).await?;
            Ok(crate::query::reenter::<super::SyncerClient>(
                &session, id, "Syncer",
            ))
        }
    }
}
