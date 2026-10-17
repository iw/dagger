//! Generated bindings owned by the GraphQL `WorkspaceModuleSetting` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "A constructor-backed module setting."]
#[derive(Clone)]
pub struct WorkspaceModuleSetting {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for WorkspaceModuleSetting {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for WorkspaceModuleSetting {
    fn graphql_type() -> &'static str {
        "WorkspaceModuleSetting"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<WorkspaceModuleSetting> for crate::IdInput<WorkspaceModuleSetting> {
    fn from(value: WorkspaceModuleSetting) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<WorkspaceModuleSetting> for crate::IdInput<super::NodeClient> {
    fn from(value: WorkspaceModuleSetting) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl WorkspaceModuleSetting {
    #[doc = "The constructor argument's declared default, formatted like value, or empty when the argument has no default.\n\nSelects GraphQL field `defaultValue` on `WorkspaceModuleSetting`."]
    pub async fn default_value(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("defaultValue");
        query.execute(&self.session).await
    }
    #[doc = "The constructor argument description.\n\nSelects GraphQL field `description` on `WorkspaceModuleSetting`."]
    pub async fn description(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("description");
        query.execute(&self.session).await
    }
    #[doc = "A unique identifier for this WorkspaceModuleSetting.\n\nSelects GraphQL field `id` on `WorkspaceModuleSetting`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "Whether the setting accepts a list of values.\n\nSelects GraphQL field `isList` on `WorkspaceModuleSetting`."]
    pub async fn is_list(&self) -> Result<bool, crate::QueryError> {
        let query = self.selection.select("isList");
        query.execute(&self.session).await
    }
    #[doc = "Whether the setting is an object type resolved from an address string (Container, Directory, File, Secret, Service, ...), which may be a module reference.\n\nSelects GraphQL field `isObject` on `WorkspaceModuleSetting`."]
    pub async fn is_object(&self) -> Result<bool, crate::QueryError> {
        let query = self.selection.select("isObject");
        query.execute(&self.session).await
    }
    #[doc = "Whether the setting is a string argument, stored as a TOML string even when the value reads as a number or boolean.\n\nSelects GraphQL field `isString` on `WorkspaceModuleSetting`."]
    pub async fn is_string(&self) -> Result<bool, crate::QueryError> {
        let query = self.selection.select("isString");
        query.execute(&self.session).await
    }
    #[doc = "The setting key.\n\nSelects GraphQL field `key` on `WorkspaceModuleSetting`."]
    pub async fn key(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("key");
        query.execute(&self.session).await
    }
    #[doc = "The value stored in workspace config after applying the selected workspace environment, or empty when unset.\n\nSelects GraphQL field `value` on `WorkspaceModuleSetting`."]
    pub async fn value(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("value");
        query.execute(&self.session).await
    }
}
impl super::Node for WorkspaceModuleSetting {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
