//! Typed bindings for GraphQL module root `minimal`.
// @generated {"format":"dagger-rust-standalone-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:2313896b06186ada0116cc4af16aa742e847b856ead2a66135e65754a9af334c","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[path = "client.rs"]
mod client;
pub use client::*;
#[path = "config.rs"]
mod config;
pub use config::*;
#[path = "item.rs"]
mod item;
pub use item::*;
#[path = "minimal_client.rs"]
mod minimal_client;
pub use minimal_client::*;
#[path = "node.rs"]
mod node;
pub use node::*;
#[path = "state.rs"]
mod state;
pub use state::*;
#[path = "token.rs"]
mod token;
pub use token::*;
