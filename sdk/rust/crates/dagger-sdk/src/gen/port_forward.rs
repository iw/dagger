//! Generated bindings owned by the GraphQL `PortForward` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "Port forwarding rules for tunneling network traffic."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[non_exhaustive]
pub struct PortForward {
    #[doc = "Destination port for traffic."]
    #[serde(rename = "backend")]
    pub backend: i64,
    #[doc = "Port to expose to clients. If unspecified, a default will be chosen."]
    #[serde(rename = "frontend", default, skip_serializing_if = "Option::is_none")]
    pub frontend: Option<i64>,
    #[doc = "Transport layer protocol to use for traffic."]
    #[serde(rename = "protocol", default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<super::NetworkProtocol>,
}
impl PortForward {
    #[doc = "Creates `PortForward` with every required GraphQL input field."]
    #[must_use]
    pub fn new(backend: i64) -> Self {
        Self {
            backend,
            frontend: None,
            protocol: None,
        }
    }
    #[doc = "Sets GraphQL input field `frontend`; the field is omitted until this method is called."]
    #[must_use]
    pub fn with_frontend(mut self, value: i64) -> Self {
        self.frontend = Some(value);
        self
    }
    #[doc = "Sets GraphQL input field `protocol`; the field is omitted until this method is called."]
    #[must_use]
    pub fn with_protocol(mut self, value: super::NetworkProtocol) -> Self {
        self.protocol = Some(value);
        self
    }
}
