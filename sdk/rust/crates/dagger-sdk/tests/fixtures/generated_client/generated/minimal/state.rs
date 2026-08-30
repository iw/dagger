//! Generated bindings owned by GraphQL coordinate `MinimalState`.
// @generated {"format":"dagger-rust-standalone-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:73ae0bb7d0ed95cb3e1b80aff4c2afad04fc7460fdae86c63d2ac8c7315b33b4","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
#[doc = "Client fixture enum MinimalState."]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    dagger_sdk::__private::serde::Deserialize,
    dagger_sdk::__private::serde::Serialize,
)]
#[serde(crate = "dagger_sdk::__private::serde")]
pub enum State {
    #[doc = "Client fixture value BUSY."]
    #[serde(rename = "BUSY")]
    Busy,
    #[doc = "Client fixture value READY."]
    #[serde(rename = "READY")]
    Ready,
}
