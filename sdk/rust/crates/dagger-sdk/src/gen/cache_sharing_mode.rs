//! Generated bindings owned by the GraphQL `CacheSharingMode` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
#[doc = "Sharing mode of the cache volume."]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CacheSharingMode {
    #[doc = "Shares the cache volume amongst many build pipelines, but will serialize the writes"]
    #[serde(rename = "LOCKED")]
    Locked,
    #[doc = "Keeps a cache volume for a single build pipeline"]
    #[serde(rename = "PRIVATE")]
    Private,
    #[doc = "Shares the cache volume amongst many build pipelines"]
    #[serde(rename = "SHARED")]
    Shared,
}
