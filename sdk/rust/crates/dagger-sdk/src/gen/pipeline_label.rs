//! Generated bindings owned by the GraphQL `PipelineLabel` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
#[doc = "Key value object that represents a pipeline label."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[non_exhaustive]
pub struct PipelineLabel {
    #[doc = "Label name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Label value."]
    #[serde(rename = "value")]
    pub value: String,
}
impl PipelineLabel {
    #[doc = "Creates `PipelineLabel` with every required GraphQL input field."]
    #[must_use]
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
