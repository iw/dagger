//! Generated bindings owned by GraphQL coordinate `MinimalConfig`.
// @generated {"format":"dagger-rust-standalone-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:73ae0bb7d0ed95cb3e1b80aff4c2afad04fc7460fdae86c63d2ac8c7315b33b4","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
#[doc = "Client fixture input MinimalConfig."]
#[derive(Clone, Debug, dagger_sdk::__private::serde::Serialize)]
#[serde(crate = "dagger_sdk::__private::serde")]
#[non_exhaustive]
pub struct Config {
    #[doc = "Client fixture argument enabled."]
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Option<bool>>,
}
impl Config {
    /// Creates `Config` with every required GraphQL input.
    #[must_use]
    pub fn new() -> Self {
        Self { enabled: None }
    }
    /// Supplies GraphQL input `enabled`; calling this method preserves explicit null and zero values.
    #[must_use]
    pub fn with_enabled(mut self, value: bool) -> Self {
        self.enabled = Some(Some(value));
        self
    }
    /// Supplies an explicit GraphQL null for input `enabled` rather than omitting it.
    #[must_use]
    pub fn with_enabled_null(mut self) -> Self {
        self.enabled = Some(None);
        self
    }
}
impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
