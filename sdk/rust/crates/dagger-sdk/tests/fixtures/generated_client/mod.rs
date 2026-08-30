//! Standalone Dagger client composed over the shared public Rust SDK runtime.
// @generated {"format":"dagger-rust-standalone-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:73ae0bb7d0ed95cb3e1b80aff4c2afad04fc7460fdae86c63d2ac8c7315b33b4","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
pub use dagger_sdk as core;
pub use dagger_sdk::{Client, ClientConfig, connect, connect_with};
mod generated;
/// Typed bindings for selected GraphQL module `minimal`.
#[path = "generated/minimal/mod.rs"]
pub mod minimal;
/// Adds the selected GraphQL module root to an existing shared client.
pub trait MinimalExt {
    /// Selects GraphQL root field `minimal` without opening another session.
    fn minimal(&self) -> minimal::Client;
}
impl MinimalExt for dagger_sdk::Client {
    fn minimal(&self) -> minimal::Client {
        minimal::Client::from_query(self.query_builder().select("minimal"))
    }
}
impl MinimalExt for dagger_sdk::QueryBuilder {
    fn minimal(&self) -> minimal::Client {
        minimal::Client::from_query(self.select("minimal"))
    }
}
/// Imports the selected module extension trait for method resolution.
pub mod prelude {
    pub use super::MinimalExt as _;
}
