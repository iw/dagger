//! Generated bindings owned by GraphQL coordinate `MinimalState`.
// @generated {"format":"dagger-rust-standalone-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:2313896b06186ada0116cc4af16aa742e847b856ead2a66135e65754a9af334c","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
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
