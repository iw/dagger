//! Generated bindings owned by the GraphQL `GitBundle` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "A Git bundle: a self-describing container of refs and the objects needed to reconstruct them, optionally rooted at prerequisite commits."]
#[derive(Clone)]
pub struct GitBundle {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for GitBundle {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for GitBundle {
    fn graphql_type() -> &'static str {
        "GitBundle"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<GitBundle> for crate::IdInput<GitBundle> {
    fn from(value: GitBundle) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<GitBundle> for crate::IdInput<super::NodeClient> {
    fn from(value: GitBundle) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl GitBundle {
    #[doc = "Return the bundle bytes as a File.\n\nSelects GraphQL field `asFile` on `GitBundle`."]
    #[must_use]
    pub fn as_file(&self) -> super::File {
        let query = self.selection.select("asFile");
        super::File {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "A unique identifier for this GitBundle.\n\nSelects GraphQL field `id` on `GitBundle`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "Object format capability: sha1 or sha256.\n\nSelects GraphQL field `objectFormat` on `GitBundle`."]
    pub async fn object_format(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("objectFormat");
        query.execute(&self.session).await
    }
    #[doc = "Commits that must already exist wherever this bundle is applied.\n\nSelects GraphQL field `prerequisiteSHAs` on `GitBundle`."]
    pub async fn prerequisite_sh_as(&self) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("prerequisiteSHAs");
        query.execute(&self.session).await
    }
    #[doc = "Refs advertised by the bundle and the object IDs they resolve to.\n\nSelects GraphQL field `refs` on `GitBundle`."]
    pub async fn refs(&self) -> Result<Vec<super::GitBundleRef>, crate::QueryError> {
        let query = self.selection.select("refs");
        let query = query.select("id");
        query
            .execute_reentry::<super::GitBundleRef, Vec<crate::Id>>(&self.session, "GitBundleRef")
            .await
    }
    #[doc = "Perform full structural verification of the bundle and error if it is malformed.\n\nSelects GraphQL field `validate` on `GitBundle`."]
    #[must_use]
    pub fn validate(&self) -> super::GitBundle {
        let query = self.selection.select("validate");
        super::GitBundle {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Bundle format version (2 or 3).\n\nSelects GraphQL field `version` on `GitBundle`."]
    pub async fn version(&self) -> Result<i64, crate::QueryError> {
        let query = self.selection.select("version");
        query.execute(&self.session).await
    }
}
impl super::Node for GitBundle {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
