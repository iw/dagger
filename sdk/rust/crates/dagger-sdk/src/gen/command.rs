//! Generated bindings owned by the GraphQL `Command` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "A command's arguments and execution settings."]
#[derive(Clone)]
pub struct Command {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for Command {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for Command {
    fn graphql_type() -> &'static str {
        "Command"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<Command> for crate::IdInput<Command> {
    fn from(value: Command) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<Command> for crate::IdInput<super::NodeClient> {
    fn from(value: Command) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl Command {
    #[doc = "The command arguments.\n\nSelects GraphQL field `args` on `Command`."]
    pub async fn args(&self) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("args");
        query.execute(&self.session).await
    }
    #[doc = "Environment variable overrides. Other variables come from the container.\n\nSelects GraphQL field `env` on `Command`."]
    pub async fn env(&self) -> Result<Vec<super::EnvVariable>, crate::QueryError> {
        let query = self.selection.select("env");
        let query = query.select("id");
        query
            .execute_reentry::<super::EnvVariable, Vec<crate::Id>>(&self.session, "EnvVariable")
            .await
    }
    #[doc = "A unique identifier for this Command.\n\nSelects GraphQL field `id` on `Command`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "Whether the command has all root capabilities.\n\nSelects GraphQL field `insecureRootCapabilities` on `Command`."]
    pub async fn insecure_root_capabilities(&self) -> Result<bool, crate::QueryError> {
        let query = self.selection.select("insecureRootCapabilities");
        query.execute(&self.session).await
    }
    #[doc = "Whether the command has access to Dagger.\n\nSelects GraphQL field `privilegedNesting` on `Command`."]
    pub async fn privileged_nesting(&self) -> Result<bool, crate::QueryError> {
        let query = self.selection.select("privilegedNesting");
        query.execute(&self.session).await
    }
    #[doc = "Working directory override. If unset, use the container's working directory.\n\nSelects GraphQL field `workdir` on `Command`."]
    pub async fn workdir(&self) -> Result<Option<String>, crate::QueryError> {
        let query = self.selection.select("workdir");
        query.execute(&self.session).await
    }
}
impl super::Node for Command {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
