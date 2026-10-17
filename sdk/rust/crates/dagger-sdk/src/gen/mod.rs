//! Generated Dagger core-schema bindings and stable public re-exports.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[path = "address.rs"]
mod address;
#[path = "agent.rs"]
mod agent;
#[path = "agent_message.rs"]
mod agent_message;
#[path = "agent_message_delivery.rs"]
mod agent_message_delivery;
#[path = "agent_state.rs"]
mod agent_state;
#[path = "artifact.rs"]
mod artifact;
#[path = "artifact_dimension.rs"]
mod artifact_dimension;
#[path = "artifact_dimension_key.rs"]
mod artifact_dimension_key;
#[path = "artifact_dimension_kind.rs"]
mod artifact_dimension_kind;
#[path = "artifact_path.rs"]
mod artifact_path;
#[path = "artifact_result.rs"]
mod artifact_result;
#[path = "artifacts.rs"]
mod artifacts;
#[path = "build_arg.rs"]
mod build_arg;
#[path = "bytes.rs"]
mod bytes;
#[path = "cache_sharing_mode.rs"]
mod cache_sharing_mode;
#[path = "cache_volume.rs"]
mod cache_volume;
#[path = "changeset.rs"]
mod changeset;
#[path = "changeset_merge_conflict.rs"]
mod changeset_merge_conflict;
#[path = "changesets_merge_conflict.rs"]
mod changesets_merge_conflict;
#[path = "check.rs"]
mod check;
#[path = "client_filesync_mirror.rs"]
mod client_filesync_mirror;
#[path = "cloud.rs"]
mod cloud;
#[path = "collection_delta.rs"]
mod collection_delta;
#[path = "collection_type_def.rs"]
mod collection_type_def;
#[path = "command.rs"]
mod command;
#[path = "container.rs"]
mod container;
#[path = "current_module.rs"]
mod current_module;
#[path = "diff_stat.rs"]
mod diff_stat;
#[path = "diff_stat_kind.rs"]
mod diff_stat_kind;
#[path = "directory.rs"]
mod directory;
#[path = "engine.rs"]
mod engine;
#[path = "engine_cache.rs"]
mod engine_cache;
#[path = "engine_cache_entry.rs"]
mod engine_cache_entry;
#[path = "engine_cache_entry_set.rs"]
mod engine_cache_entry_set;
#[path = "enum_type_def.rs"]
mod enum_type_def;
#[path = "enum_value_type_def.rs"]
mod enum_value_type_def;
#[path = "env_file.rs"]
mod env_file;
#[path = "env_variable.rs"]
mod env_variable;
#[path = "error.rs"]
mod error;
#[path = "error_value.rs"]
mod error_value;
#[path = "exists_type.rs"]
mod exists_type;
#[path = "expertise.rs"]
mod expertise;
#[path = "exportable.rs"]
mod exportable;
#[path = "field_type_def.rs"]
mod field_type_def;
#[path = "file.rs"]
mod file;
#[path = "file_type.rs"]
mod file_type;
#[path = "function.rs"]
mod function;
#[path = "function_arg.rs"]
mod function_arg;
#[path = "function_cache_policy.rs"]
mod function_cache_policy;
#[path = "function_call.rs"]
mod function_call;
#[path = "function_call_arg_value.rs"]
mod function_call_arg_value;
#[path = "generated_code.rs"]
mod generated_code;
#[path = "generator.rs"]
mod generator;
#[path = "git_bundle.rs"]
mod git_bundle;
#[path = "git_bundle_ref.rs"]
mod git_bundle_ref;
#[path = "git_commit.rs"]
mod git_commit;
#[path = "git_push_disposition.rs"]
mod git_push_disposition;
#[path = "git_push_result.rs"]
mod git_push_result;
#[path = "git_ref.rs"]
mod git_ref;
#[path = "git_remote.rs"]
mod git_remote;
#[path = "git_repository.rs"]
mod git_repository;
#[path = "healthcheck_config.rs"]
mod healthcheck_config;
#[path = "host.rs"]
mod host;
#[path = "http_state.rs"]
mod http_state;
#[path = "image_layer_compression.rs"]
mod image_layer_compression;
#[path = "image_media_types.rs"]
mod image_media_types;
#[path = "input_type_def.rs"]
mod input_type_def;
#[path = "interface_type_def.rs"]
mod interface_type_def;
#[path = "json_value.rs"]
mod json_value;
#[path = "label.rs"]
mod label;
#[path = "list_type_def.rs"]
mod list_type_def;
#[path = "llm.rs"]
mod llm;
#[path = "llm_content.rs"]
mod llm_content;
#[path = "llm_content_block.rs"]
mod llm_content_block;
#[path = "llm_content_block_input.rs"]
mod llm_content_block_input;
#[path = "llm_content_block_kind.rs"]
mod llm_content_block_kind;
#[path = "llm_message.rs"]
mod llm_message;
#[path = "llm_message_origin.rs"]
mod llm_message_origin;
#[path = "llm_message_origin_input.rs"]
mod llm_message_origin_input;
#[path = "llm_message_origin_kind.rs"]
mod llm_message_origin_kind;
#[path = "llm_message_role.rs"]
mod llm_message_role;
#[path = "llm_skill.rs"]
mod llm_skill;
#[path = "llm_token_usage.rs"]
mod llm_token_usage;
#[path = "module.rs"]
mod module;
#[path = "module_config_client.rs"]
mod module_config_client;
#[path = "module_source.rs"]
mod module_source;
#[path = "module_source_experimental_feature.rs"]
mod module_source_experimental_feature;
#[path = "module_source_kind.rs"]
mod module_source_kind;
#[path = "network_protocol.rs"]
mod network_protocol;
#[path = "node.rs"]
mod node;
#[path = "object_type_def.rs"]
mod object_type_def;
#[path = "patch_conflict.rs"]
mod patch_conflict;
#[path = "pipeline_label.rs"]
mod pipeline_label;
#[path = "port.rs"]
mod port;
#[path = "port_forward.rs"]
mod port_forward;
#[path = "query.rs"]
mod query;
#[path = "registry_protocol.rs"]
mod registry_protocol;
#[path = "remote_git_mirror.rs"]
mod remote_git_mirror;
#[path = "return_type.rs"]
mod return_type;
#[path = "scalar_type_def.rs"]
mod scalar_type_def;
#[path = "schema.rs"]
mod schema;
#[path = "sdk_config.rs"]
mod sdk_config;
#[path = "search_result.rs"]
mod search_result;
#[path = "search_submatch.rs"]
mod search_submatch;
#[path = "secret.rs"]
mod secret;
#[path = "service.rs"]
mod service;
#[path = "socket.rs"]
mod socket;
#[path = "source_map.rs"]
mod source_map;
#[path = "stat.rs"]
mod stat;
#[path = "syncer.rs"]
mod syncer;
#[path = "terminal.rs"]
mod terminal;
#[path = "type_def.rs"]
mod type_def;
#[path = "type_def_kind.rs"]
mod type_def_kind;
#[path = "volume.rs"]
mod volume;
#[path = "workspace.rs"]
mod workspace;
#[path = "workspace_commit_pick.rs"]
mod workspace_commit_pick;
#[path = "workspace_commit_pick_reason.rs"]
mod workspace_commit_pick_reason;
#[path = "workspace_commit_pick_status.rs"]
mod workspace_commit_pick_status;
#[path = "workspace_git.rs"]
mod workspace_git;
#[path = "workspace_migration.rs"]
mod workspace_migration;
#[path = "workspace_migration_step.rs"]
mod workspace_migration_step;
#[path = "workspace_module.rs"]
mod workspace_module;
#[path = "workspace_module_setting.rs"]
mod workspace_module_setting;
#[path = "workspace_sdk.rs"]
mod workspace_sdk;
pub use address::*;
pub use agent::*;
pub use agent_message::*;
pub use agent_message_delivery::*;
pub use agent_state::*;
pub use artifact::*;
pub use artifact_dimension::*;
pub use artifact_dimension_key::*;
pub use artifact_dimension_kind::*;
pub use artifact_path::*;
pub use artifact_result::*;
pub use artifacts::*;
pub use build_arg::*;
pub use bytes::*;
pub use cache_sharing_mode::*;
pub use cache_volume::*;
pub use changeset::*;
pub use changeset_merge_conflict::*;
pub use changesets_merge_conflict::*;
pub use check::*;
pub use client_filesync_mirror::*;
pub use cloud::*;
pub use collection_delta::*;
pub use collection_type_def::*;
pub use command::*;
pub use container::*;
pub use current_module::*;
pub use diff_stat::*;
pub use diff_stat_kind::*;
pub use directory::*;
pub use engine::*;
pub use engine_cache::*;
pub use engine_cache_entry::*;
pub use engine_cache_entry_set::*;
pub use enum_type_def::*;
pub use enum_value_type_def::*;
pub use env_file::*;
pub use env_variable::*;
pub use error::*;
pub use error_value::*;
pub use exists_type::*;
pub use expertise::*;
pub use exportable::*;
pub use field_type_def::*;
pub use file::*;
pub use file_type::*;
pub use function::*;
pub use function_arg::*;
pub use function_cache_policy::*;
pub use function_call::*;
pub use function_call_arg_value::*;
pub use generated_code::*;
pub use generator::*;
pub use git_bundle::*;
pub use git_bundle_ref::*;
pub use git_commit::*;
pub use git_push_disposition::*;
pub use git_push_result::*;
pub use git_ref::*;
pub use git_remote::*;
pub use git_repository::*;
pub use healthcheck_config::*;
pub use host::*;
pub use http_state::*;
pub use image_layer_compression::*;
pub use image_media_types::*;
pub use input_type_def::*;
pub use interface_type_def::*;
pub use json_value::*;
pub use label::*;
pub use list_type_def::*;
pub use llm::*;
pub use llm_content::*;
pub use llm_content_block::*;
pub use llm_content_block_input::*;
pub use llm_content_block_kind::*;
pub use llm_message::*;
pub use llm_message_origin::*;
pub use llm_message_origin_input::*;
pub use llm_message_origin_kind::*;
pub use llm_message_role::*;
pub use llm_skill::*;
pub use llm_token_usage::*;
pub use module::*;
pub use module_config_client::*;
pub use module_source::*;
pub use module_source_experimental_feature::*;
pub use module_source_kind::*;
pub use network_protocol::*;
pub use node::*;
pub use object_type_def::*;
pub use patch_conflict::*;
pub use pipeline_label::*;
pub use port::*;
pub use port_forward::*;
pub use query::*;
pub use registry_protocol::*;
pub use remote_git_mirror::*;
pub use return_type::*;
pub use scalar_type_def::*;
pub use schema::*;
pub use sdk_config::*;
pub use search_result::*;
pub use search_submatch::*;
pub use secret::*;
pub use service::*;
pub use socket::*;
pub use source_map::*;
pub use stat::*;
pub use syncer::*;
pub use terminal::*;
pub use type_def::*;
pub use type_def_kind::*;
pub use volume::*;
pub use workspace::*;
pub use workspace_commit_pick::*;
pub use workspace_commit_pick_reason::*;
pub use workspace_commit_pick_status::*;
pub use workspace_git::*;
pub use workspace_migration::*;
pub use workspace_migration_step::*;
pub use workspace_module::*;
pub use workspace_module_setting::*;
pub use workspace_sdk::*;
