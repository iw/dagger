//! Generated bindings owned by the GraphQL `CacheVolume` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
#[doc = "A directory whose contents persist across runs."]
#[derive(Clone)]
pub struct CacheVolume {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
impl crate::IntoID<crate::Id> for CacheVolume {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for CacheVolume {
    fn graphql_type() -> &'static str {
        "CacheVolume"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<CacheVolume> for crate::IdInput<CacheVolume> {
    fn from(value: CacheVolume) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<CacheVolume> for crate::IdInput<super::NodeClient> {
    fn from(value: CacheVolume) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl CacheVolume {
    #[doc = "A unique identifier for this CacheVolume.\n\nSelects GraphQL field `id` on `CacheVolume`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
}
impl super::Node for CacheVolume {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
