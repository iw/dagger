//! Generated bindings owned by the GraphQL `Check` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "One deferred check. Reading pass, error, or sync runs it."]
#[derive(Clone)]
pub struct Check {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for Check {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for Check {
    fn graphql_type() -> &'static str {
        "Check"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<Check> for crate::IdInput<Check> {
    fn from(value: Check) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<Check> for crate::IdInput<super::NodeClient> {
    fn from(value: Check) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl Check {
    #[doc = "The assertion that is false when this check fails.\n\nSelects GraphQL field `assertion` on `Check`."]
    pub async fn assertion(&self) -> Result<Option<String>, crate::QueryError> {
        let query = self.selection.select("assertion");
        query.execute(&self.session).await
    }
    #[doc = "Run the check and return its failure, if any.\n\nSelects GraphQL field `error` on `Check`."]
    pub async fn error(&self) -> Result<Option<super::Error>, crate::QueryError> {
        let query = self.selection.select("error");
        let query = query.select("id");
        query
            .execute_reentry::<super::Error, Option<crate::Id>>(&self.session, "Error")
            .await
    }
    #[doc = "A unique identifier for this Check.\n\nSelects GraphQL field `id` on `Check`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "Run the check and return whether it passes.\n\nSelects GraphQL field `pass` on `Check`."]
    pub async fn pass(&self) -> Result<bool, crate::QueryError> {
        let query = self.selection.select("pass");
        query.execute(&self.session).await
    }
    #[doc = "An optional report produced by the check.\n\nSelects GraphQL field `report` on `Check`."]
    pub async fn report(&self) -> Result<Option<super::Directory>, crate::QueryError> {
        let query = self.selection.select("report");
        let query = query.select("id");
        query
            .execute_reentry::<super::Directory, Option<crate::Id>>(&self.session, "Directory")
            .await
    }
    #[doc = "Run the check and retain its result.\n\nSelects GraphQL field `sync` on `Check`."]
    #[must_use]
    pub fn sync(&self) -> super::Check {
        let query = self.selection.select("sync");
        super::Check {
            session: self.session.clone(),
            selection: query,
        }
    }
}
impl super::Node for Check {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
