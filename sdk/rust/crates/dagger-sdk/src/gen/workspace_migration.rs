//! Generated bindings owned by the GraphQL `WorkspaceMigration` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "A planned workspace migration."]
#[derive(Clone)]
pub struct WorkspaceMigration {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for WorkspaceMigration {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for WorkspaceMigration {
    fn graphql_type() -> &'static str {
        "WorkspaceMigration"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<WorkspaceMigration> for crate::IdInput<WorkspaceMigration> {
    fn from(value: WorkspaceMigration) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<WorkspaceMigration> for crate::IdInput<super::NodeClient> {
    fn from(value: WorkspaceMigration) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl WorkspaceMigration {
    #[doc = "Filesystem changes for the full migration plan.\n\nSelects GraphQL field `changes` on `WorkspaceMigration`."]
    #[must_use]
    pub fn changes(&self) -> super::Changeset {
        let query = self.selection.select("changes");
        super::Changeset {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Native workspace config path after migration, relative to the workspace root. Empty if no workspace config exists.\n\nSelects GraphQL field `configFile` on `WorkspaceMigration`."]
    pub async fn config_file(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("configFile");
        query.execute(&self.session).await
    }
    #[doc = "A unique identifier for this WorkspaceMigration.\n\nSelects GraphQL field `id` on `WorkspaceMigration`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "Unselected legacy module directories relative to the workspace root. Candidates can include fixtures.\n\nSelects GraphQL field `moduleCandidates` on `WorkspaceMigration`."]
    pub async fn module_candidates(&self) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("moduleCandidates");
        query.execute(&self.session).await
    }
    #[doc = "Logical migration steps, each identified by a stable code.\n\nSelects GraphQL field `steps` on `WorkspaceMigration`."]
    pub async fn steps(&self) -> Result<Vec<super::WorkspaceMigrationStep>, crate::QueryError> {
        let query = self.selection.select("steps");
        let query = query.select("id");
        query
            .execute_reentry::<super::WorkspaceMigrationStep, Vec<crate::Id>>(
                &self.session,
                "WorkspaceMigrationStep",
            )
            .await
    }
}
impl super::Node for WorkspaceMigration {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
