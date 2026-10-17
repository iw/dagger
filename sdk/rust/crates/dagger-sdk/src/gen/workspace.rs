//! Generated bindings owned by the GraphQL `Workspace` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "A Dagger workspace detected from the current working directory or constructed from a Directory."]
#[derive(Clone)]
pub struct Workspace {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.artifacts`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceArtifactsOpts {
    #[doc = "Only include artifacts matching these path patterns, as with checks and services. A path selects that path and its children.\n\n`None` omits GraphQL field `include`."]
    pub include: Option<Vec<String>>,
}
impl WorkspaceArtifactsOpts {
    #[doc = "Sets GraphQL argument `include` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_include(mut self, value: Vec<impl Into<String>>) -> Self {
        self.include = Some(value.into_iter().map(Into::into).collect());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.changes`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceChangesOpts {
    #[doc = "An earlier workspace state to compare against.\n\n`None` omits GraphQL field `from`."]
    pub from: Option<crate::IdInput<super::Workspace>>,
}
impl WorkspaceChangesOpts {
    #[doc = "Sets GraphQL argument `from` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_from(mut self, value: crate::IdInput<super::Workspace>) -> Self {
        self.from = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.compareCommitsFrom`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceCompareCommitsFromOpts {
    #[doc = "Full lowercase commit hashes or unambiguous lowercase hex prefixes (4-40 characters) to select, in any order. Prefixes resolve against the frozen source's Git objects and are recorded as full hashes; duplicate selections after resolution are rejected. Empty selects all new source commits. Selected commits must be within the source's latest 10000 commits.\n\n`None` omits GraphQL field `commits` and preserves engine default `List(\\[\\])`."]
    pub commits: Option<Vec<String>>,
    #[doc = "Maximum commits in either differing history, from 1 to 1000. Exceeding the limit fails; nothing is silently omitted.\n\n`None` omits GraphQL field `maxCommits` and preserves engine default `Int(100)`."]
    pub max_commits: Option<i64>,
}
impl WorkspaceCompareCommitsFromOpts {
    #[doc = "Sets GraphQL argument `commits` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_commits(mut self, value: Vec<impl Into<String>>) -> Self {
        self.commits = Some(value.into_iter().map(Into::into).collect());
        self
    }
    #[doc = "Sets GraphQL argument `maxCommits` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_max_commits(mut self, value: i64) -> Self {
        self.max_commits = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.configRead`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceConfigReadOpts {
    #[doc = "Include the selected environment, user overrides, and legacy workspace settings.\n\n`None` omits GraphQL field `effective` and preserves engine default `Boolean(false)`."]
    pub effective: Option<bool>,
    #[doc = "Dotted key path (e.g. modules.greeter.source). Empty for full config.\n\n`None` omits GraphQL field `key` and preserves engine default `String(\"\")`."]
    pub key: Option<String>,
}
impl WorkspaceConfigReadOpts {
    #[doc = "Sets GraphQL argument `effective` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_effective(mut self, value: bool) -> Self {
        self.effective = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `key` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.directory`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceDirectoryOpts {
    #[doc = "Exclude artifacts that match the given pattern (e.g., \\[\"node_modules/\", \".git*\"\\]).\n\n`None` omits GraphQL field `exclude` and preserves engine default `List(\\[\\])`."]
    pub exclude: Option<Vec<String>>,
    #[doc = "Apply .gitignore filter rules inside the directory.\n\n`None` omits GraphQL field `gitignore` and preserves engine default `Boolean(false)`."]
    pub gitignore: Option<bool>,
    #[doc = "Include only artifacts that match the given pattern (e.g., \\[\"app/\", \"package.*\"\\]).\n\n`None` omits GraphQL field `include` and preserves engine default `List(\\[\\])`."]
    pub include: Option<Vec<String>>,
}
impl WorkspaceDirectoryOpts {
    #[doc = "Sets GraphQL argument `exclude` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_exclude(mut self, value: Vec<impl Into<String>>) -> Self {
        self.exclude = Some(value.into_iter().map(Into::into).collect());
        self
    }
    #[doc = "Sets GraphQL argument `gitignore` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_gitignore(mut self, value: bool) -> Self {
        self.gitignore = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `include` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_include(mut self, value: Vec<impl Into<String>>) -> Self {
        self.include = Some(value.into_iter().map(Into::into).collect());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.export`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceExportOpts {
    #[doc = "Earlier workspace state to compare against. For Git integration, live inputs are snapshotted at export time; use a snapshot to retain the baseline of a previous export.\n\n`None` omits GraphQL field `from`."]
    pub from: Option<crate::IdInput<super::Workspace>>,
    #[doc = "Destination checkout path on the calling client. Relative paths start at the client's working directory. Omit to use the calling client's current local workspace root.\n\n`None` omits GraphQL field `path` and preserves engine default `String(\"\")`."]
    pub path: Option<String>,
}
impl WorkspaceExportOpts {
    #[doc = "Sets GraphQL argument `from` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_from(mut self, value: crate::IdInput<super::Workspace>) -> Self {
        self.from = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `path` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.findRoots`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceFindRootsOpts {
    #[doc = "Glob patterns pruning the walk below start (e.g. \\[\"**/node_modules/**\"\\]).\n\n`None` omits GraphQL field `exclude` and preserves engine default `List(\\[\\])`."]
    pub exclude: Option<Vec<String>>,
    #[doc = "Directory to start from. Relative paths resolve from the workspace cwd.\n\n`None` omits GraphQL field `start` and preserves engine default `String(\".\")`."]
    pub start: Option<String>,
}
impl WorkspaceFindRootsOpts {
    #[doc = "Sets GraphQL argument `exclude` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_exclude(mut self, value: Vec<impl Into<String>>) -> Self {
        self.exclude = Some(value.into_iter().map(Into::into).collect());
        self
    }
    #[doc = "Sets GraphQL argument `start` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_start(mut self, value: impl Into<String>) -> Self {
        self.start = Some(value.into());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.findUp`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceFindUpOpts {
    #[doc = "Path to start the search from. Relative paths resolve from the workspace cwd; absolute paths resolve from the workspace root.\n\n`None` omits GraphQL field `from` and preserves engine default `String(\".\")`."]
    pub from: Option<String>,
}
impl WorkspaceFindUpOpts {
    #[doc = "Sets GraphQL argument `from` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.migrate`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceMigrateOpts {
    #[doc = "Additional local modules to migrate. Relative paths start at the workspace cwd; absolute paths start at the workspace root.\n\n`None` omits GraphQL field `modules` and preserves engine default `List(\\[\\])`."]
    pub modules: Option<Vec<String>>,
}
impl WorkspaceMigrateOpts {
    #[doc = "Sets GraphQL argument `modules` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_modules(mut self, value: Vec<impl Into<String>>) -> Self {
        self.modules = Some(value.into_iter().map(Into::into).collect());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.migrateModule`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceMigrateModuleOpts {
    #[doc = "Module directory. Relative paths start at the workspace cwd; absolute paths start at the workspace root.\n\n`None` omits GraphQL field `path` and preserves engine default `String(\".\")`."]
    pub path: Option<String>,
}
impl WorkspaceMigrateModuleOpts {
    #[doc = "Sets GraphQL argument `path` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.search`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceSearchOpts {
    #[doc = "Allow the . pattern to match newlines in multiline mode.\n\n`None` omits GraphQL field `dotall` and preserves engine default `Boolean(false)`."]
    pub dotall: Option<bool>,
    #[doc = "Only return matching files, not lines and content\n\n`None` omits GraphQL field `filesOnly` and preserves engine default `Boolean(false)`."]
    pub files_only: Option<bool>,
    #[doc = "Glob patterns to match (e.g., \"*.md\")\n\n`None` omits GraphQL field `globs` and preserves engine default `List(\\[\\])`."]
    pub globs: Option<Vec<String>>,
    #[doc = "Enable case-insensitive matching.\n\n`None` omits GraphQL field `insensitive` and preserves engine default `Boolean(false)`."]
    pub insensitive: Option<bool>,
    #[doc = "Limit the number of results to return\n\n`None` omits GraphQL field `limit`."]
    pub limit: Option<i64>,
    #[doc = "Interpret the pattern as a literal string instead of a regular expression.\n\n`None` omits GraphQL field `literal` and preserves engine default `Boolean(false)`."]
    pub literal: Option<bool>,
    #[doc = "Enable searching across multiple lines.\n\n`None` omits GraphQL field `multiline` and preserves engine default `Boolean(false)`."]
    pub multiline: Option<bool>,
    #[doc = "Directory or file paths to search\n\n`None` omits GraphQL field `paths` and preserves engine default `List(\\[\\])`."]
    pub paths: Option<Vec<String>>,
    #[doc = "Skip hidden files (files starting with .).\n\n`None` omits GraphQL field `skipHidden` and preserves engine default `Boolean(false)`."]
    pub skip_hidden: Option<bool>,
    #[doc = "Honor .gitignore, .ignore, and .rgignore files.\n\n`None` omits GraphQL field `skipIgnored` and preserves engine default `Boolean(false)`."]
    pub skip_ignored: Option<bool>,
}
impl WorkspaceSearchOpts {
    #[doc = "Sets GraphQL argument `dotall` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_dotall(mut self, value: bool) -> Self {
        self.dotall = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `filesOnly` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_files_only(mut self, value: bool) -> Self {
        self.files_only = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `globs` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_globs(mut self, value: Vec<impl Into<String>>) -> Self {
        self.globs = Some(value.into_iter().map(Into::into).collect());
        self
    }
    #[doc = "Sets GraphQL argument `insensitive` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_insensitive(mut self, value: bool) -> Self {
        self.insensitive = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `limit` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `literal` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_literal(mut self, value: bool) -> Self {
        self.literal = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `multiline` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_multiline(mut self, value: bool) -> Self {
        self.multiline = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `paths` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_paths(mut self, value: Vec<impl Into<String>>) -> Self {
        self.paths = Some(value.into_iter().map(Into::into).collect());
        self
    }
    #[doc = "Sets GraphQL argument `skipHidden` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_skip_hidden(mut self, value: bool) -> Self {
        self.skip_hidden = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `skipIgnored` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_skip_ignored(mut self, value: bool) -> Self {
        self.skip_ignored = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withClient`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithClientOpts {
    #[doc = "Optional SDK name. Inspect all installed SDKs when omitted.\n\n`None` omits GraphQL field `sdk` and preserves engine default `String(\"\")`."]
    pub sdk: Option<String>,
    #[doc = "Explicit SDK-module constructor setting overrides for this scope. Requires an explicit SDK name.\n\n`None` omits GraphQL field `settings`."]
    pub settings: Option<crate::Json>,
}
impl WorkspaceWithClientOpts {
    #[doc = "Sets GraphQL argument `sdk` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_sdk(mut self, value: impl Into<String>) -> Self {
        self.sdk = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `settings` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_settings(mut self, value: crate::Json) -> Self {
        self.settings = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withCommit`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithCommitOpts {
    #[doc = "Author and committer email. Defaults to git config user.email in the calling client's working directory, otherwise dagger@localhost.\n\n`None` omits GraphQL field `authorEmail`."]
    pub author_email: Option<String>,
    #[doc = "Author and committer name. Defaults to git config user.name in the calling client's working directory, otherwise Dagger.\n\n`None` omits GraphQL field `authorName`."]
    pub author_name: Option<String>,
    #[doc = "Add a Signed-off-by trailer using the commit author's name and email.\n\n`None` omits GraphQL field `signoff` and preserves engine default `Boolean(false)`."]
    pub signoff: Option<bool>,
}
impl WorkspaceWithCommitOpts {
    #[doc = "Sets GraphQL argument `authorEmail` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_author_email(mut self, value: impl Into<String>) -> Self {
        self.author_email = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `authorName` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_author_name(mut self, value: impl Into<String>) -> Self {
        self.author_name = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `signoff` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_signoff(mut self, value: bool) -> Self {
        self.signoff = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withCommitsFrom`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithCommitsFromOpts {
    #[doc = "Full lowercase commit hashes or unambiguous lowercase hex prefixes (4-40 characters) to select, in any order. Prefixes resolve against the frozen source's Git objects and are recorded as full hashes; duplicate selections after resolution are rejected. Empty selects all new source commits. Selected commits must be within the source's latest 10000 commits.\n\n`None` omits GraphQL field `commits` and preserves engine default `List(\\[\\])`."]
    pub commits: Option<Vec<String>>,
    #[doc = "Maximum commits in either differing history, from 1 to 1000. Exceeding the limit fails; nothing is silently omitted.\n\n`None` omits GraphQL field `maxCommits` and preserves engine default `Int(100)`."]
    pub max_commits: Option<i64>,
}
impl WorkspaceWithCommitsFromOpts {
    #[doc = "Sets GraphQL argument `commits` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_commits(mut self, value: Vec<impl Into<String>>) -> Self {
        self.commits = Some(value.into_iter().map(Into::into).collect());
        self
    }
    #[doc = "Sets GraphQL argument `maxCommits` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_max_commits(mut self, value: i64) -> Self {
        self.max_commits = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withConfigEnv`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithConfigEnvOpts {
    #[doc = "Write to the workspace config directory at the workspace cwd.\n\n`None` omits GraphQL field `here` and preserves engine default `Boolean(false)`."]
    pub here: Option<bool>,
}
impl WorkspaceWithConfigEnvOpts {
    #[doc = "Sets GraphQL argument `here` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_here(mut self, value: bool) -> Self {
        self.here = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withConfigValue`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithConfigValueOpts {
    #[doc = "Write to the workspace config directory at the workspace cwd.\n\n`None` omits GraphQL field `here` and preserves engine default `Boolean(false)`."]
    pub here: Option<bool>,
    #[doc = "List value to set. Elements are stored verbatim, with no auto-detection. Mutually exclusive with value.\n\n`None` omits GraphQL field `values`."]
    pub values: Option<Vec<String>>,
}
impl WorkspaceWithConfigValueOpts {
    #[doc = "Sets GraphQL argument `here` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_here(mut self, value: bool) -> Self {
        self.here = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `values` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_values(mut self, value: Vec<impl Into<String>>) -> Self {
        self.values = Some(value.into_iter().map(Into::into).collect());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withFile`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithFileOpts {
    #[doc = "Permissions of the added file. Defaults to the source file permissions.\n\n`None` omits GraphQL field `permissions`."]
    pub permissions: Option<i64>,
}
impl WorkspaceWithFileOpts {
    #[doc = "Sets GraphQL argument `permissions` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_permissions(mut self, value: i64) -> Self {
        self.permissions = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withInitModule`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithInitModuleOpts {
    #[doc = "Select this module as the entrypoint and install it. False prevents automatic selection. When omitted, select only if both path and name are omitted and the module is installed.\n\n`None` omits GraphQL field `entrypoint`."]
    pub entrypoint: Option<bool>,
    #[doc = "Install the module. When omitted, install only if path is omitted.\n\n`None` omits GraphQL field `install`."]
    pub install: Option<bool>,
    #[doc = "Module name. The engine infers it from path, the active config file, or the workspace root when omitted.\n\n`None` omits GraphQL field `name` and preserves engine default `String(\"\")`."]
    pub name: Option<String>,
    #[doc = "Module path relative to the workspace cwd, or an absolute workspace path. Defaults to .dagger/modules/&lt;name&gt; beside the active workspace config.\n\n`None` omits GraphQL field `path` and preserves engine default `String(\"\")`."]
    pub path: Option<String>,
    #[doc = "Explicit SDK-module constructor setting overrides for this scope.\n\n`None` omits GraphQL field `settings`."]
    pub settings: Option<crate::Json>,
}
impl WorkspaceWithInitModuleOpts {
    #[doc = "Sets GraphQL argument `entrypoint` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_entrypoint(mut self, value: bool) -> Self {
        self.entrypoint = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `install` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_install(mut self, value: bool) -> Self {
        self.install = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `name` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `path` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `settings` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_settings(mut self, value: crate::Json) -> Self {
        self.settings = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withModule`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithModuleOpts {
    #[doc = "Write to the workspace config directory at the workspace cwd.\n\n`None` omits GraphQL field `here` and preserves engine default `Boolean(false)`."]
    pub here: Option<bool>,
    #[doc = "Override name for the installed module entry.\n\n`None` omits GraphQL field `name` and preserves engine default `String(\"\")`."]
    pub name: Option<String>,
}
impl WorkspaceWithModuleOpts {
    #[doc = "Sets GraphQL argument `here` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_here(mut self, value: bool) -> Self {
        self.here = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `name` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withNewFile`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithNewFileOpts {
    #[doc = "Permissions of the new file.\n\n`None` omits GraphQL field `permissions` and preserves engine default `Int(420)`."]
    pub permissions: Option<i64>,
}
impl WorkspaceWithNewFileOpts {
    #[doc = "Sets GraphQL argument `permissions` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_permissions(mut self, value: i64) -> Self {
        self.permissions = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withPatchFile`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithPatchFileOpts {
    #[doc = "How to handle hunks that no longer apply to the target content: fail (default), or apply what fits and leave git-style conflict markers where it doesn't.\n\n`None` omits GraphQL field `onConflict` and preserves engine default `Enum(SchemaName(\"FAIL\"))`."]
    pub on_conflict: Option<super::PatchConflict>,
}
impl WorkspaceWithPatchFileOpts {
    #[doc = "Sets GraphQL argument `onConflict` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_on_conflict(mut self, value: super::PatchConflict) -> Self {
        self.on_conflict = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withReset`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithResetOpts {
    #[doc = "Discard uncommitted changes, resetting the working tree to the commit.\n\n`None` omits GraphQL field `hard` and preserves engine default `Boolean(false)`."]
    pub hard: Option<bool>,
}
impl WorkspaceWithResetOpts {
    #[doc = "Sets GraphQL argument `hard` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_hard(mut self, value: bool) -> Self {
        self.hard = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withSDK`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithSdkOpts {
    #[doc = "Optional override for the SDK name conventionally derived from the installed module name.\n\n`None` omits GraphQL field `asSdkName` and preserves engine default `String(\"\")`."]
    pub as_sdk_name: Option<String>,
    #[doc = "Write to the workspace config directory at the workspace cwd.\n\n`None` omits GraphQL field `here` and preserves engine default `Boolean(false)`."]
    pub here: Option<bool>,
    #[doc = "Override name for the installed SDK entry.\n\n`None` omits GraphQL field `name` and preserves engine default `String(\"\")`."]
    pub name: Option<String>,
}
impl WorkspaceWithSdkOpts {
    #[doc = "Sets GraphQL argument `asSdkName` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_as_sdk_name(mut self, value: impl Into<String>) -> Self {
        self.as_sdk_name = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `here` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_here(mut self, value: bool) -> Self {
        self.here = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `name` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withUpdatedClients`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithUpdatedClientsOpts {
    #[doc = "Select clients in every scope instead of only the scopes containing the workspace cwd.\n\n`None` omits GraphQL field `all` and preserves engine default `Boolean(false)`."]
    pub all: Option<bool>,
    #[doc = "Recorded client targets to update. All targets in the selected scopes are updated when omitted.\n\n`None` omits GraphQL field `modules` and preserves engine default `List(\\[\\])`."]
    pub modules: Option<Vec<String>>,
    #[doc = "Optional SDK name. All installed SDK modules are selected when omitted.\n\n`None` omits GraphQL field `sdk` and preserves engine default `String(\"\")`."]
    pub sdk: Option<String>,
}
impl WorkspaceWithUpdatedClientsOpts {
    #[doc = "Sets GraphQL argument `all` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_all(mut self, value: bool) -> Self {
        self.all = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `modules` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_modules(mut self, value: Vec<impl Into<String>>) -> Self {
        self.modules = Some(value.into_iter().map(Into::into).collect());
        self
    }
    #[doc = "Sets GraphQL argument `sdk` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_sdk(mut self, value: impl Into<String>) -> Self {
        self.sdk = Some(value.into());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withUpdatedLock`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithUpdatedLockOpts {
    #[doc = "Do not regenerate SDK client scopes.\n\n`None` omits GraphQL field `noGenerate` and preserves engine default `Boolean(false)`."]
    pub no_generate: Option<bool>,
}
impl WorkspaceWithUpdatedLockOpts {
    #[doc = "Sets GraphQL argument `noGenerate` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_no_generate(mut self, value: bool) -> Self {
        self.no_generate = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withUpdatedModules`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithUpdatedModulesOpts {
    #[doc = "Installed module names or sources. A version suffix sets a new request. An empty list refreshes all installed modules.\n\n`None` omits GraphQL field `names` and preserves engine default `List(\\[\\])`."]
    pub names: Option<Vec<String>>,
    #[doc = "New source for exactly one selected module. Resolved like an install source. Cannot be combined with a version or a version suffix.\n\n`None` omits GraphQL field `source` and preserves engine default `String(\"\")`."]
    pub source: Option<String>,
    #[doc = "New version request for exactly one selected module. Cannot be combined with a version suffix.\n\n`None` omits GraphQL field `version` and preserves engine default `String(\"\")`."]
    pub version: Option<String>,
}
impl WorkspaceWithUpdatedModulesOpts {
    #[doc = "Sets GraphQL argument `names` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_names(mut self, value: Vec<impl Into<String>>) -> Self {
        self.names = Some(value.into_iter().map(Into::into).collect());
        self
    }
    #[doc = "Sets GraphQL argument `source` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `version` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withoutClient`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithoutClientOpts {
    #[doc = "Optional SDK name. Search all installed SDKs when omitted.\n\n`None` omits GraphQL field `sdk` and preserves engine default `String(\"\")`."]
    pub sdk: Option<String>,
}
impl WorkspaceWithoutClientOpts {
    #[doc = "Sets GraphQL argument `sdk` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_sdk(mut self, value: impl Into<String>) -> Self {
        self.sdk = Some(value.into());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withoutConfigEnv`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithoutConfigEnvOpts {
    #[doc = "Write to the workspace config directory at the workspace cwd.\n\n`None` omits GraphQL field `here` and preserves engine default `Boolean(false)`."]
    pub here: Option<bool>,
}
impl WorkspaceWithoutConfigEnvOpts {
    #[doc = "Sets GraphQL argument `here` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_here(mut self, value: bool) -> Self {
        self.here = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withoutConfigValue`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithoutConfigValueOpts {
    #[doc = "Write to the workspace config directory at the workspace cwd.\n\n`None` omits GraphQL field `here` and preserves engine default `Boolean(false)`."]
    pub here: Option<bool>,
}
impl WorkspaceWithoutConfigValueOpts {
    #[doc = "Sets GraphQL argument `here` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_here(mut self, value: bool) -> Self {
        self.here = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withoutModule`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithoutModuleOpts {
    #[doc = "Write to the workspace config directory at the workspace cwd.\n\n`None` omits GraphQL field `here` and preserves engine default `Boolean(false)`."]
    pub here: Option<bool>,
}
impl WorkspaceWithoutModuleOpts {
    #[doc = "Sets GraphQL argument `here` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_here(mut self, value: bool) -> Self {
        self.here = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `Workspace.withoutSDK`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WorkspaceWithoutSdkOpts {
    #[doc = "Write to the workspace config directory at the workspace cwd.\n\n`None` omits GraphQL field `here` and preserves engine default `Boolean(false)`."]
    pub here: Option<bool>,
}
impl WorkspaceWithoutSdkOpts {
    #[doc = "Sets GraphQL argument `here` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_here(mut self, value: bool) -> Self {
        self.here = Some(value);
        self
    }
}
impl crate::IntoID<crate::Id> for Workspace {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for Workspace {
    fn graphql_type() -> &'static str {
        "Workspace"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<Workspace> for crate::IdInput<Workspace> {
    fn from(value: Workspace) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<Workspace> for crate::IdInput<super::NodeClient> {
    fn from(value: Workspace) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl Workspace {
    #[doc = "Canonical Dagger address of the workspace location, or an opaque identity for synthetic workspaces.\n\nSelects GraphQL field `address` on `Workspace`."]
    pub async fn address(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("address");
        query.execute(&self.session).await
    }
    #[doc = "Discover static object artifacts from workspace modules without evaluating their values.\n\nSelects GraphQL field `artifacts` on `Workspace`."]
    #[must_use]
    pub fn artifacts(&self) -> super::Artifacts {
        let query = self.selection.select("artifacts");
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `artifacts` with a borrowed, reusable `WorkspaceArtifactsOpts` value."]
    #[must_use]
    pub fn artifacts_opts(&self, opts: &WorkspaceArtifactsOpts) -> super::Artifacts {
        let query = self.selection.select("artifacts");
        let query = if let Some(value) = &opts.include {
            query.arg("include", value)
        } else {
            query
        };
        super::Artifacts {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace's changes, with paths relative to its working directory.\n\nPass from to compare against an earlier workspace state. Omitting it preserves the cumulative behavior used by clients from before this argument was added.\n\nSelects GraphQL field `changes` on `Workspace`."]
    #[must_use]
    pub fn changes(&self) -> super::Changeset {
        let query = self.selection.select("changes");
        super::Changeset {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `changes` with a borrowed, reusable `WorkspaceChangesOpts` value."]
    #[must_use]
    pub fn changes_opts(&self, opts: &WorkspaceChangesOpts) -> super::Changeset {
        let query = self.selection.select("changes");
        let query = if let Some(value) = &opts.from {
            query.arg_id_input("from", value.clone())
        } else {
            query
        };
        super::Changeset {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Preview which source commits withCommitsFrom would apply, skip, or report as conflicting.\n\nResults are ordered oldest first and account for earlier applicable commits in the same preview. The preview does not apply commits or write to the checkout.\n\nA local receiver is snapshotted automatically; untracked files require interactive approval. Source uncommitted changes are ignored. Exceeding maxCommits fails rather than returning a partial preview. Divergent merge commits require manual integration.\n\nSelects GraphQL field `compareCommitsFrom` on `Workspace`."]
    pub async fn compare_commits_from(
        &self,
        source: impl Into<crate::IdInput<super::Workspace>>,
    ) -> Result<Vec<super::WorkspaceCommitPick>, crate::QueryError> {
        let query = self.selection.select("compareCommitsFrom");
        let query = query.arg_id_input("source", source.into());
        let query = query.select("id");
        query
            .execute_reentry::<super::WorkspaceCommitPick, Vec<crate::Id>>(
                &self.session,
                "WorkspaceCommitPick",
            )
            .await
    }
    #[doc = "Executes GraphQL operation `compareCommitsFrom` with a borrowed, reusable `WorkspaceCompareCommitsFromOpts` value."]
    pub async fn compare_commits_from_opts(
        &self,
        source: impl Into<crate::IdInput<super::Workspace>>,
        opts: &WorkspaceCompareCommitsFromOpts,
    ) -> Result<Vec<super::WorkspaceCommitPick>, crate::QueryError> {
        let query = self.selection.select("compareCommitsFrom");
        let query = if let Some(value) = &opts.commits {
            query.arg("commits", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.max_commits {
            query.arg("maxCommits", value)
        } else {
            query
        };
        let query = query.arg_id_input("source", source.into());
        let query = query.select("id");
        query
            .execute_reentry::<super::WorkspaceCommitPick, Vec<crate::Id>>(
                &self.session,
                "WorkspaceCommitPick",
            )
            .await
    }
    #[doc = "Selected native workspace config file relative to the workspace cwd, if any.\n\nSelects GraphQL field `configFile` on `Workspace`."]
    pub async fn config_file(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("configFile");
        query.execute(&self.session).await
    }
    #[doc = "Read a configuration value from dagger.toml.\n\nIf key is empty, returns the full config.\n\nIf key points to a scalar, returns the value.\n\nIf key points to a table, returns flattened dotted-key output.\n\nSelects GraphQL field `configRead` on `Workspace`."]
    pub async fn config_read(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("configRead");
        query.execute(&self.session).await
    }
    #[doc = "Executes GraphQL operation `configRead` with a borrowed, reusable `WorkspaceConfigReadOpts` value."]
    pub async fn config_read_opts(
        &self,
        opts: &WorkspaceConfigReadOpts,
    ) -> Result<String, crate::QueryError> {
        let query = self.selection.select("configRead");
        let query = if let Some(value) = &opts.effective {
            query.arg("effective", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.key {
            query.arg("key", value)
        } else {
            query
        };
        query.execute(&self.session).await
    }
    #[doc = "Current location within the workspace root.\n\nThe workspace root is returned as \"/\".\n\nRelative paths in workspace APIs resolve from here.\n\nSelects GraphQL field `cwd` on `Workspace`."]
    pub async fn cwd(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("cwd");
        query.execute(&self.session).await
    }
    #[doc = "Return the selected SDK module's current scope at this workspace location.\n\nSelects GraphQL field `detectScope` on `Workspace`."]
    pub async fn detect_scope(&self, sdk: impl Into<String>) -> Result<String, crate::QueryError> {
        let query = self.selection.select("detectScope");
        let query = query.arg("sdk", sdk.into());
        query.execute(&self.session).await
    }
    #[doc = "Returns a Directory from the workspace.\n\nRelative paths resolve from the workspace cwd. Absolute paths resolve from the workspace root.\n\nSelects GraphQL field `directory` on `Workspace`."]
    #[must_use]
    pub fn directory(&self, path: impl Into<String>) -> super::Directory {
        let query = self.selection.select("directory");
        let query = query.arg("path", path.into());
        super::Directory {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `directory` with a borrowed, reusable `WorkspaceDirectoryOpts` value."]
    #[must_use]
    pub fn directory_opts(
        &self,
        path: impl Into<String>,
        opts: &WorkspaceDirectoryOpts,
    ) -> super::Directory {
        let query = self.selection.select("directory");
        let query = if let Some(value) = &opts.exclude {
            query.arg("exclude", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.gitignore {
            query.arg("gitignore", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.include {
            query.arg("include", value)
        } else {
            query
        };
        let query = query.arg("path", path.into());
        super::Directory {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Installed name of the module selected as the workspace entrypoint, or an empty string when none is selected.\n\nReflects the selected env's effective view. Fails if several modules are selected.\n\nSelects GraphQL field `entrypoint` on `Workspace`."]
    pub async fn entrypoint(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("entrypoint");
        query.execute(&self.session).await
    }
    #[doc = "List named environments defined in the workspace configuration.\n\nSelects GraphQL field `envList` on `Workspace`."]
    pub async fn env_list(&self) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("envList");
        query.execute(&self.session).await
    }
    #[doc = "Write this workspace's commits and pending changes to a checkout on the calling client.\n\nPath selects the destination; omitting it uses the calling client's current local workspace root, including when exporting a snapshot or committed workspace. Exported file paths are relative to the workspace root regardless of its working directory. This writes only to the client making the call, never the source's client.\n\nA live workspace exported to its own checkout applies only its overlay edits, without capturing the whole checkout. This also applies with an explicit path. Pass from with the same live base to apply only changes since that overlay state.\n\nOther exports integrate divergent commits by cherry-picking, preserve unrelated checkout edits, and refuse conflicts. Live inputs are snapshotted automatically; capturing untracked source files requires interactive approval. Stable inputs retain their baseline. Pass from to save only work since an earlier source value, including previously saved pending edits that are now committed.\n\nSelects GraphQL field `export` on `Workspace`."]
    pub async fn export(&self) -> Result<(), crate::QueryError> {
        let query = self.selection.select("export");
        query.execute_void(&self.session).await
    }
    #[doc = "Executes GraphQL operation `export` with a borrowed, reusable `WorkspaceExportOpts` value."]
    pub async fn export_opts(&self, opts: &WorkspaceExportOpts) -> Result<(), crate::QueryError> {
        let query = self.selection.select("export");
        let query = if let Some(value) = &opts.from {
            query.arg_id_input("from", value.clone())
        } else {
            query
        };
        let query = if let Some(value) = &opts.path {
            query.arg("path", value)
        } else {
            query
        };
        query.execute_void(&self.session).await
    }
    #[doc = "Returns a File from the workspace.\n\nRelative paths resolve from the workspace cwd. Absolute paths resolve from the workspace root.\n\nSelects GraphQL field `file` on `Workspace`."]
    #[must_use]
    pub fn file(&self, path: impl Into<String>) -> super::File {
        let query = self.selection.select("file");
        let query = query.arg("path", path.into());
        super::File {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Find project roots marked by any of the given filenames, starting from a path relative to the workspace cwd.\n\nReturns cwd-relative directory paths for every marked directory at or below start, plus the nearest marked ancestor when start itself is not marked.\n\nEach returned path is usable as-is with other workspace APIs, e.g. directory(path).\n\nSelects GraphQL field `findRoots` on `Workspace`."]
    pub async fn find_roots(
        &self,
        markers: Vec<impl Into<String>>,
    ) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("findRoots");
        let markers = markers.into_iter().map(Into::into).collect::<Vec<String>>();
        let query = query.arg("markers", markers);
        query.execute(&self.session).await
    }
    #[doc = "Executes GraphQL operation `findRoots` with a borrowed, reusable `WorkspaceFindRootsOpts` value."]
    pub async fn find_roots_opts(
        &self,
        markers: Vec<impl Into<String>>,
        opts: &WorkspaceFindRootsOpts,
    ) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("findRoots");
        let query = if let Some(value) = &opts.exclude {
            query.arg("exclude", value)
        } else {
            query
        };
        let markers = markers.into_iter().map(Into::into).collect::<Vec<String>>();
        let query = query.arg("markers", markers);
        let query = if let Some(value) = &opts.start {
            query.arg("start", value)
        } else {
            query
        };
        query.execute(&self.session).await
    }
    #[doc = "Search for a file or directory by walking up from the start path within the workspace.\n\nReturns the absolute workspace path if found, or null if not found.\n\nRelative start paths resolve from the workspace cwd.\n\nThe search stops at the workspace root and will not traverse above it.\n\nSelects GraphQL field `findUp` on `Workspace`."]
    pub async fn find_up(
        &self,
        name: impl Into<String>,
    ) -> Result<Option<String>, crate::QueryError> {
        let query = self.selection.select("findUp");
        let query = query.arg("name", name.into());
        query.execute(&self.session).await
    }
    #[doc = "Executes GraphQL operation `findUp` with a borrowed, reusable `WorkspaceFindUpOpts` value."]
    pub async fn find_up_opts(
        &self,
        name: impl Into<String>,
        opts: &WorkspaceFindUpOpts,
    ) -> Result<Option<String>, crate::QueryError> {
        let query = self.selection.select("findUp");
        let query = if let Some(value) = &opts.from {
            query.arg("from", value)
        } else {
            query
        };
        let query = query.arg("name", name.into());
        query.execute(&self.session).await
    }
    #[doc = "Git state for this workspace. Errors if the workspace is not in a git repository.\n\nSelects GraphQL field `git` on `Workspace`."]
    #[must_use]
    pub fn git(&self) -> super::WorkspaceGit {
        let query = self.selection.select("git");
        super::WorkspaceGit {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Returns a list of files and directories that match the given pattern.\n\nPatterns match paths relative to the workspace root.\n\nSelects GraphQL field `glob` on `Workspace`."]
    pub async fn glob(&self, pattern: impl Into<String>) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("glob");
        let query = query.arg("pattern", pattern.into());
        query.execute(&self.session).await
    }
    #[doc = "A unique identifier for this Workspace.\n\nSelects GraphQL field `id` on `Workspace`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "Plan the explicit migration needed for the current workspace.\n\nInclude installed local modules and their local dependencies. Other module candidates remain unchanged unless selected.\n\nThe returned plan has an empty changeset and no steps when no migration is needed.\n\nSelects GraphQL field `migrate` on `Workspace`."]
    #[must_use]
    pub fn migrate(&self) -> super::WorkspaceMigration {
        let query = self.selection.select("migrate");
        super::WorkspaceMigration {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `migrate` with a borrowed, reusable `WorkspaceMigrateOpts` value."]
    #[must_use]
    pub fn migrate_opts(&self, opts: &WorkspaceMigrateOpts) -> super::WorkspaceMigration {
        let query = self.selection.select("migrate");
        let query = if let Some(value) = &opts.modules {
            query.arg("modules", value)
        } else {
            query
        };
        super::WorkspaceMigration {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Plan migration of one local module without migrating its dependencies or creating a workspace configuration.\n\nInclude SDK registration when a workspace configuration exists and remove obsolete generated-file ignore rules.\n\nSelects GraphQL field `migrateModule` on `Workspace`."]
    #[must_use]
    pub fn migrate_module(&self) -> super::WorkspaceMigration {
        let query = self.selection.select("migrateModule");
        super::WorkspaceMigration {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `migrateModule` with a borrowed, reusable `WorkspaceMigrateModuleOpts` value."]
    #[must_use]
    pub fn migrate_module_opts(
        &self,
        opts: &WorkspaceMigrateModuleOpts,
    ) -> super::WorkspaceMigration {
        let query = self.selection.select("migrateModule");
        let query = if let Some(value) = &opts.path {
            query.arg("path", value)
        } else {
            query
        };
        super::WorkspaceMigration {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return a module defined in the workspace configuration.\n\nReflects the selected env's effective view.\n\nSelects GraphQL field `module` on `Workspace`."]
    #[must_use]
    pub fn module(&self, name: impl Into<String>) -> super::WorkspaceModule {
        let query = self.selection.select("module");
        let query = query.arg("name", name.into());
        super::WorkspaceModule {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Load a module source from a path within the workspace.\n\nRelative paths (e.g., \"foo\") resolve from the workspace cwd; absolute paths (e.g., \"/foo\") resolve from the workspace root.\n\nFails if the path does not point to an initialized module.\n\nSelects GraphQL field `moduleSource` on `Workspace`."]
    #[must_use]
    pub fn module_source(&self, path: impl Into<String>) -> super::ModuleSource {
        let query = self.selection.select("moduleSource");
        let query = query.arg("path", path.into());
        super::ModuleSource {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "List modules defined in the workspace configuration.\n\nReflects the selected env's effective view.\n\nSelects GraphQL field `modules` on `Workspace`."]
    pub async fn modules(&self) -> Result<Vec<super::WorkspaceModule>, crate::QueryError> {
        let query = self.selection.select("modules");
        let query = query.select("id");
        query
            .execute_reentry::<super::WorkspaceModule, Vec<crate::Id>>(
                &self.session,
                "WorkspaceModule",
            )
            .await
    }
    #[doc = "Resolve an address in this workspace.\n\nA DAG address (dag://&lt;path&gt;) selects exactly one workspace artifact: artifacts.filterUri(value).one(). Its typed loaders use that artifact and never fall back to external resolution.\n\nA value without the dag:// scheme keeps its external meaning, such as a container image reference.\n\nThe Address retains this workspace across module calls and ID reloads.\n\nSelects GraphQL field `resolve` on `Workspace`."]
    #[must_use]
    pub fn resolve(&self, value: impl Into<String>) -> super::Address {
        let query = self.selection.select("resolve");
        let query = query.arg("value", value.into());
        super::Address {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "An installed SDK, by name.\n\nSelects GraphQL field `sdk` on `Workspace`."]
    #[must_use]
    pub fn sdk(&self, name: impl Into<String>) -> super::WorkspaceSdk {
        let query = self.selection.select("sdk");
        let query = query.arg("name", name.into());
        super::WorkspaceSdk {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Installed SDKs.\n\nSelects GraphQL field `sdks` on `Workspace`."]
    pub async fn sdks(&self) -> Result<Vec<super::WorkspaceSdk>, crate::QueryError> {
        let query = self.selection.select("sdks");
        let query = query.select("id");
        query
            .execute_reentry::<super::WorkspaceSdk, Vec<crate::Id>>(&self.session, "WorkspaceSDK")
            .await
    }
    #[doc = "Searches for content matching the given regular expression or literal string.\n\nUses Rust regex syntax; escape literal ., \\[, \\], {, }, | with backslashes.\n\nRuns ripgrep on the client host, falling back to grep if unavailable.\n\nSelects GraphQL field `search` on `Workspace`."]
    pub async fn search(
        &self,
        pattern: impl Into<String>,
    ) -> Result<Vec<super::SearchResult>, crate::QueryError> {
        let query = self.selection.select("search");
        let query = query.arg("pattern", pattern.into());
        let query = query.select("id");
        query
            .execute_reentry::<super::SearchResult, Vec<crate::Id>>(&self.session, "SearchResult")
            .await
    }
    #[doc = "Executes GraphQL operation `search` with a borrowed, reusable `WorkspaceSearchOpts` value."]
    pub async fn search_opts(
        &self,
        pattern: impl Into<String>,
        opts: &WorkspaceSearchOpts,
    ) -> Result<Vec<super::SearchResult>, crate::QueryError> {
        let query = self.selection.select("search");
        let query = if let Some(value) = &opts.dotall {
            query.arg("dotall", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.files_only {
            query.arg("filesOnly", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.globs {
            query.arg("globs", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.insensitive {
            query.arg("insensitive", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.limit {
            query.arg("limit", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.literal {
            query.arg("literal", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.multiline {
            query.arg("multiline", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.paths {
            query.arg("paths", value)
        } else {
            query
        };
        let query = query.arg("pattern", pattern.into());
        let query = if let Some(value) = &opts.skip_hidden {
            query.arg("skipHidden", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.skip_ignored {
            query.arg("skipIgnored", value)
        } else {
            query
        };
        let query = query.select("id");
        query
            .execute_reentry::<super::SearchResult, Vec<crate::Id>>(&self.session, "SearchResult")
            .await
    }
    #[doc = "Return a snapshot of this workspace as a stable value.\n\nGit capture is a progressive enhancement: if the workspace has no Git repository or commits, or the client cannot capture Git, return this workspace unchanged. Approval rejections and capture failures remain errors.\n\nUse the returned workspace for subsequent reads, edits, and module loading against the captured baseline. Snapshotting an existing stable value preserves its baseline; snapshot currentWorkspace again to capture later checkout changes.\n\nOnly the owning client can capture a local checkout. Tracked changes are captured automatically; untracked files require interactive approval. Remote Git refs are pinned to their resolved commits. Capturing leaves the checkout unchanged.\n\nThe recipe is portable when a remote can serve its base; otherwise it is frozen for this session only.\n\nSelects GraphQL field `snapshot` on `Workspace`.\n\n**Experimental:** Best-effort capture for resumable sessions; capture and fallback behavior may change."]
    #[must_use]
    pub fn snapshot(&self) -> super::Workspace {
        let query = self.selection.select("snapshot");
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a changeset applied, without mutating the source.\n\nSelects GraphQL field `withChanges` on `Workspace`."]
    #[must_use]
    pub fn with_changes(
        &self,
        changes: impl Into<crate::IdInput<super::Changeset>>,
    ) -> super::Workspace {
        let query = self.selection.select("withChanges");
        let query = query.arg_id_input("changes", changes.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a generated module client added to one SDK scope.\n\nSelect the deepest detected or registered scope. Fail if several SDKs have that deepest scope.\n\nSelects GraphQL field `withClient` on `Workspace`."]
    #[must_use]
    pub fn with_client(&self, module: impl Into<String>) -> super::Workspace {
        let query = self.selection.select("withClient");
        let query = query.arg("module", module.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withClient` with a borrowed, reusable `WorkspaceWithClientOpts` value."]
    #[must_use]
    pub fn with_client_opts(
        &self,
        module: impl Into<String>,
        opts: &WorkspaceWithClientOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withClient");
        let query = query.arg("module", module.into());
        let query = if let Some(value) = &opts.sdk {
            query.arg("sdk", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.settings {
            query.arg("settings", value)
        } else {
            query
        };
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Create a Git commit from a changeset and return a stable workspace with HEAD advanced.\n\nThe changeset is three-way merged into both HEAD and the frozen working tree. Compatible unselected edits remain uncommitted; incoming changes need not already be in the working tree. Conflicts with either tree fail without modifying the workspace. Empty changesets, or changes already present in HEAD, fail with nothing to commit.\n\nA local workspace is snapshotted automatically before committing; untracked files require interactive approval. The host checkout is not modified.\n\nMissing author fields are resolved from Git config in the calling client's working directory at commit time, then recorded explicitly for reproducible commits. Unconfigured fields default to Dagger and dagger@localhost.\n\nSelects GraphQL field `withCommit` on `Workspace`."]
    #[must_use]
    pub fn with_commit(
        &self,
        changes: impl Into<crate::IdInput<super::Changeset>>,
        date: impl Into<String>,
        message: impl Into<String>,
    ) -> super::Workspace {
        let query = self.selection.select("withCommit");
        let query = query.arg_id_input("changes", changes.into());
        let query = query.arg("date", date.into());
        let query = query.arg("message", message.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withCommit` with a borrowed, reusable `WorkspaceWithCommitOpts` value."]
    #[must_use]
    pub fn with_commit_opts(
        &self,
        changes: impl Into<crate::IdInput<super::Changeset>>,
        date: impl Into<String>,
        message: impl Into<String>,
        opts: &WorkspaceWithCommitOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withCommit");
        let query = if let Some(value) = &opts.author_email {
            query.arg("authorEmail", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.author_name {
            query.arg("authorName", value)
        } else {
            query
        };
        let query = query.arg_id_input("changes", changes.into());
        let query = query.arg("date", date.into());
        let query = query.arg("message", message.into());
        let query = if let Some(value) = &opts.signoff {
            query.arg("signoff", value)
        } else {
            query
        };
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Integrate source commits into this workspace and return the result, preserving this workspace's uncommitted changes and metadata.\n\nFast-forward when the selected commits include all new ancestors of their tip; otherwise cherry-pick them oldest first. Already integrated commits and patches already present are skipped. Any conflict fails the operation. Source uncommitted changes are not transferred; merge them explicitly if needed. Use compareCommitsFrom to preview the integration.\n\nA local receiver is snapshotted automatically; untracked files require interactive approval. The checkout is not modified. Export the result with an explicit path to write it to a checkout.\n\nCherry-picks preserve the source author and author date, use the calling client's Git config for committer identity, and reuse the source committer date for reproducible hashes. Origin trailers track cherry-picked commits. Divergent merge commits require manual integration.\n\nSelects GraphQL field `withCommitsFrom` on `Workspace`."]
    #[must_use]
    pub fn with_commits_from(
        &self,
        source: impl Into<crate::IdInput<super::Workspace>>,
    ) -> super::Workspace {
        let query = self.selection.select("withCommitsFrom");
        let query = query.arg_id_input("source", source.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withCommitsFrom` with a borrowed, reusable `WorkspaceWithCommitsFromOpts` value."]
    #[must_use]
    pub fn with_commits_from_opts(
        &self,
        source: impl Into<crate::IdInput<super::Workspace>>,
        opts: &WorkspaceWithCommitsFromOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withCommitsFrom");
        let query = if let Some(value) = &opts.commits {
            query.arg("commits", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.max_commits {
            query.arg("maxCommits", value)
        } else {
            query
        };
        let query = query.arg_id_input("source", source.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a named config environment created.\n\nSelects GraphQL field `withConfigEnv` on `Workspace`."]
    #[must_use]
    pub fn with_config_env(&self, name: impl Into<String>) -> super::Workspace {
        let query = self.selection.select("withConfigEnv");
        let query = query.arg("name", name.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withConfigEnv` with a borrowed, reusable `WorkspaceWithConfigEnvOpts` value."]
    #[must_use]
    pub fn with_config_env_opts(
        &self,
        name: impl Into<String>,
        opts: &WorkspaceWithConfigEnvOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withConfigEnv");
        let query = if let Some(value) = &opts.here {
            query.arg("here", value)
        } else {
            query
        };
        let query = query.arg("name", name.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Select the config environment carried by this workspace.\n\nSelects GraphQL field `withConfigEnvironment` on `Workspace`."]
    #[must_use]
    pub fn with_config_environment(&self, name: impl Into<String>) -> super::Workspace {
        let query = self.selection.select("withConfigEnvironment");
        let query = query.arg("name", name.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Select workspace-root-relative config and lockfile paths. Empty paths clear the selection.\n\nSelects GraphQL field `withConfigPaths` on `Workspace`."]
    #[must_use]
    pub fn with_config_paths(
        &self,
        config_file: impl Into<String>,
        lock_file: impl Into<String>,
    ) -> super::Workspace {
        let query = self.selection.select("withConfigPaths");
        let query = query.arg("configFile", config_file.into());
        let query = query.arg("lockFile", lock_file.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a configuration value written.\n\nWhen the session selects an env, the key is scoped to that env's overlay and the env is created if missing.\n\nSelects GraphQL field `withConfigValue` on `Workspace`."]
    #[must_use]
    pub fn with_config_value(
        &self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> super::Workspace {
        let query = self.selection.select("withConfigValue");
        let query = query.arg("key", key.into());
        let query = query.arg("value", value.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withConfigValue` with a borrowed, reusable `WorkspaceWithConfigValueOpts` value."]
    #[must_use]
    pub fn with_config_value_opts(
        &self,
        key: impl Into<String>,
        value: impl Into<String>,
        opts: &WorkspaceWithConfigValueOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withConfigValue");
        let query = if let Some(value) = &opts.here {
            query.arg("here", value)
        } else {
            query
        };
        let query = query.arg("key", key.into());
        let query = query.arg("value", value.into());
        let query = if let Some(value) = &opts.values {
            query.arg("values", value)
        } else {
            query
        };
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a directory merged into the given path, without mutating the source.\n\nAnything already at the path stays, and files the source carries win, as with Directory.withDirectory. Use withNewDirectory to replace the path instead.\n\nSelects GraphQL field `withDirectory` on `Workspace`."]
    #[must_use]
    pub fn with_directory(
        &self,
        path: impl Into<String>,
        source: impl Into<crate::IdInput<super::Directory>>,
    ) -> super::Workspace {
        let query = self.selection.select("withDirectory");
        let query = query.arg("path", path.into());
        let query = query.arg_id_input("source", source.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with an installed module selected as its entrypoint.\n\nEvery other entrypoint selection is cleared. Entrypoints live in the base workspace config.\n\nSelects GraphQL field `withEntrypoint` on `Workspace`."]
    #[must_use]
    pub fn with_entrypoint(&self, name: impl Into<String>) -> super::Workspace {
        let query = self.selection.select("withEntrypoint");
        let query = query.arg("name", name.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a file added or replaced, without mutating the source.\n\nSelects GraphQL field `withFile` on `Workspace`."]
    #[must_use]
    pub fn with_file(
        &self,
        path: impl Into<String>,
        source: impl Into<crate::IdInput<super::File>>,
    ) -> super::Workspace {
        let query = self.selection.select("withFile");
        let query = query.arg("path", path.into());
        let query = query.arg_id_input("source", source.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withFile` with a borrowed, reusable `WorkspaceWithFileOpts` value."]
    #[must_use]
    pub fn with_file_opts(
        &self,
        path: impl Into<String>,
        source: impl Into<crate::IdInput<super::File>>,
        opts: &WorkspaceWithFileOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withFile");
        let query = query.arg("path", path.into());
        let query = if let Some(value) = &opts.permissions {
            query.arg("permissions", value)
        } else {
            query
        };
        let query = query.arg_id_input("source", source.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a location initialized as a module scope.\n\nThe selected SDK module records the scope and generates the module source.\n\nSelects GraphQL field `withInitModule` on `Workspace`."]
    #[must_use]
    pub fn with_init_module(&self, sdk: impl Into<String>) -> super::Workspace {
        let query = self.selection.select("withInitModule");
        let query = query.arg("sdk", sdk.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withInitModule` with a borrowed, reusable `WorkspaceWithInitModuleOpts` value."]
    #[must_use]
    pub fn with_init_module_opts(
        &self,
        sdk: impl Into<String>,
        opts: &WorkspaceWithInitModuleOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withInitModule");
        let query = if let Some(value) = &opts.entrypoint {
            query.arg("entrypoint", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.install {
            query.arg("install", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.name {
            query.arg("name", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.path {
            query.arg("path", value)
        } else {
            query
        };
        let query = query.arg("sdk", sdk.into());
        let query = if let Some(value) = &opts.settings {
            query.arg("settings", value)
        } else {
            query
        };
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a native configuration, without changing an existing configuration.\n\nFail if legacy configuration needs workspace migration.\n\nSelects GraphQL field `withInitialized` on `Workspace`."]
    #[must_use]
    pub fn with_initialized(&self) -> super::Workspace {
        let query = self.selection.select("withInitialized");
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a module installed in its config.\n\nWhen the session selects an env, the module is recorded in that env's overlay and the env is created if missing.\n\nSelects GraphQL field `withModule` on `Workspace`."]
    #[must_use]
    pub fn with_module(&self, r#ref: impl Into<String>) -> super::Workspace {
        let query = self.selection.select("withModule");
        let query = query.arg("ref", r#ref.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withModule` with a borrowed, reusable `WorkspaceWithModuleOpts` value."]
    #[must_use]
    pub fn with_module_opts(
        &self,
        r#ref: impl Into<String>,
        opts: &WorkspaceWithModuleOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withModule");
        let query = if let Some(value) = &opts.here {
            query.arg("here", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.name {
            query.arg("name", value)
        } else {
            query
        };
        let query = query.arg("ref", r#ref.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a directory mounted read-only at the given path, without mutating the source.\n\nMounted content is readable through the normal workspace file tools but shadows the source at the mount path and stays out of the pending changeset: it never appears in changes, is never exported, and cannot be modified.\n\nSelects GraphQL field `withMountedDirectory` on `Workspace`."]
    #[must_use]
    pub fn with_mounted_directory(
        &self,
        path: impl Into<String>,
        source: impl Into<crate::IdInput<super::Directory>>,
    ) -> super::Workspace {
        let query = self.selection.select("withMountedDirectory");
        let query = query.arg("path", path.into());
        let query = query.arg_id_input("source", source.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a file mounted read-only at the given path, without mutating the source.\n\nMounted content is readable through the normal workspace file tools but shadows the source at the mount path and stays out of the pending changeset: it never appears in changes, is never exported, and cannot be modified.\n\nSelects GraphQL field `withMountedFile` on `Workspace`."]
    #[must_use]
    pub fn with_mounted_file(
        &self,
        path: impl Into<String>,
        source: impl Into<crate::IdInput<super::File>>,
    ) -> super::Workspace {
        let query = self.selection.select("withMountedFile");
        let query = query.arg("path", path.into());
        let query = query.arg_id_input("source", source.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with the given path replaced by a directory, without mutating the source.\n\nThe source becomes the entire contents of the path: anything already there that the source does not carry is removed. Use withDirectory to keep it instead.\n\nSelects GraphQL field `withNewDirectory` on `Workspace`."]
    #[must_use]
    pub fn with_new_directory(
        &self,
        path: impl Into<String>,
        source: impl Into<crate::IdInput<super::Directory>>,
    ) -> super::Workspace {
        let query = self.selection.select("withNewDirectory");
        let query = query.arg("path", path.into());
        let query = query.arg_id_input("source", source.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a new or replaced file, without mutating the source.\n\nSelects GraphQL field `withNewFile` on `Workspace`."]
    #[must_use]
    pub fn with_new_file(
        &self,
        contents: impl Into<String>,
        path: impl Into<String>,
    ) -> super::Workspace {
        let query = self.selection.select("withNewFile");
        let query = query.arg("contents", contents.into());
        let query = query.arg("path", path.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withNewFile` with a borrowed, reusable `WorkspaceWithNewFileOpts` value."]
    #[must_use]
    pub fn with_new_file_opts(
        &self,
        contents: impl Into<String>,
        path: impl Into<String>,
        opts: &WorkspaceWithNewFileOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withNewFile");
        let query = query.arg("contents", contents.into());
        let query = query.arg("path", path.into());
        let query = if let Some(value) = &opts.permissions {
            query.arg("permissions", value)
        } else {
            query
        };
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with the given Git-compatible patch file applied, without mutating the source.\n\nPaths in the patch are relative to the workspace root, whatever its cwd, as `git diff` writes them. Patching a path at or under a mount is an error.\n\nSelects GraphQL field `withPatchFile` on `Workspace`.\n\n**Experimental:** This API is highly experimental and may be removed or replaced entirely."]
    #[must_use]
    pub fn with_patch_file(
        &self,
        patch: impl Into<crate::IdInput<super::File>>,
    ) -> super::Workspace {
        let query = self.selection.select("withPatchFile");
        let query = query.arg_id_input("patch", patch.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withPatchFile` with a borrowed, reusable `WorkspaceWithPatchFileOpts` value.\n\n**Experimental:** This API is highly experimental and may be removed or replaced entirely."]
    #[must_use]
    pub fn with_patch_file_opts(
        &self,
        patch: impl Into<crate::IdInput<super::File>>,
        opts: &WorkspaceWithPatchFileOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withPatchFile");
        let query = if let Some(value) = &opts.on_conflict {
            query.arg("onConflict", value)
        } else {
            query
        };
        let query = query.arg_id_input("patch", patch.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Move this workspace's Git HEAD to a commit and return the resulting stable workspace.\n\nA local workspace is snapshotted automatically before resetting; untracked files require interactive approval. The host checkout is not modified. By default the difference between the previous working tree and the target commit stays uncommitted, as with git reset --mixed, so history can be reworked and reapplied with withCommit — e.g. to amend the latest commit message, reset to its parent and commit again.\n\nWith hard, the working tree is reset to the commit and every uncommitted change is discarded.\n\nCommits orphaned by the reset are not preserved: the frozen repository keeps reachable history only, so a reset cannot be undone by resetting forward again.\n\nSelects GraphQL field `withReset` on `Workspace`."]
    #[must_use]
    pub fn with_reset(&self, commit: impl Into<String>) -> super::Workspace {
        let query = self.selection.select("withReset");
        let query = query.arg("commit", commit.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withReset` with a borrowed, reusable `WorkspaceWithResetOpts` value."]
    #[must_use]
    pub fn with_reset_opts(
        &self,
        commit: impl Into<String>,
        opts: &WorkspaceWithResetOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withReset");
        let query = query.arg("commit", commit.into());
        let query = if let Some(value) = &opts.hard {
            query.arg("hard", value)
        } else {
            query
        };
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with an SDK installed in its config.\n\nSelects GraphQL field `withSDK` on `Workspace`."]
    #[must_use]
    pub fn with_sdk(&self, r#ref: impl Into<String>) -> super::Workspace {
        let query = self.selection.select("withSDK");
        let query = query.arg("ref", r#ref.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withSDK` with a borrowed, reusable `WorkspaceWithSdkOpts` value."]
    #[must_use]
    pub fn with_sdk_opts(
        &self,
        r#ref: impl Into<String>,
        opts: &WorkspaceWithSdkOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withSDK");
        let query = if let Some(value) = &opts.as_sdk_name {
            query.arg("asSdkName", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.here {
            query.arg("here", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.name {
            query.arg("name", value)
        } else {
            query
        };
        let query = query.arg("ref", r#ref.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with the selected module clients updated.\n\nThe engine re-reads the source of each selected client target and writes the lock entries that those targets reach.\n\nThe selected SDK module then regenerates every scope that owns one of the targets.\n\nSelects GraphQL field `withUpdatedClients` on `Workspace`."]
    #[must_use]
    pub fn with_updated_clients(&self) -> super::Workspace {
        let query = self.selection.select("withUpdatedClients");
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withUpdatedClients` with a borrowed, reusable `WorkspaceWithUpdatedClientsOpts` value."]
    #[must_use]
    pub fn with_updated_clients_opts(
        &self,
        opts: &WorkspaceWithUpdatedClientsOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withUpdatedClients");
        let query = if let Some(value) = &opts.all {
            query.arg("all", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.modules {
            query.arg("modules", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.sdk {
            query.arg("sdk", value)
        } else {
            query
        };
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with refreshed lockfile state.\n\nSDK client scopes are regenerated unless noGenerate is true.\n\nSelects GraphQL field `withUpdatedLock` on `Workspace`."]
    #[must_use]
    pub fn with_updated_lock(&self) -> super::Workspace {
        let query = self.selection.select("withUpdatedLock");
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withUpdatedLock` with a borrowed, reusable `WorkspaceWithUpdatedLockOpts` value."]
    #[must_use]
    pub fn with_updated_lock_opts(&self, opts: &WorkspaceWithUpdatedLockOpts) -> super::Workspace {
        let query = self.selection.select("withUpdatedLock");
        let query = if let Some(value) = &opts.no_generate {
            query.arg("noGenerate", value)
        } else {
            query
        };
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with updated module sources, versions and lockfile state.\n\nAn SDK client scope is regenerated when it targets an updated module.\n\nSelects GraphQL field `withUpdatedModules` on `Workspace`."]
    #[must_use]
    pub fn with_updated_modules(&self) -> super::Workspace {
        let query = self.selection.select("withUpdatedModules");
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withUpdatedModules` with a borrowed, reusable `WorkspaceWithUpdatedModulesOpts` value."]
    #[must_use]
    pub fn with_updated_modules_opts(
        &self,
        opts: &WorkspaceWithUpdatedModulesOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withUpdatedModules");
        let query = if let Some(value) = &opts.names {
            query.arg("names", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.source {
            query.arg("source", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.version {
            query.arg("version", value)
        } else {
            query
        };
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with the calling client's user-level config re-read and applied.\n\nUser-level config (the \\[workspaces.*\\] section of the Dagger config file) is read when a session loads its workspace, and snapshots keep that configuration. Call this to pick up edits made since, for example when an agent reloads its modules.\n\nThe entry is matched by the workspace's git origin remote. A workspace without one, or without a matching entry, gets no user-level config.\n\nSelects GraphQL field `withUserConfig` on `Workspace`."]
    #[must_use]
    pub fn with_user_config(&self) -> super::Workspace {
        let query = self.selection.select("withUserConfig");
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with its working directory pointed at the given workspace-relative path.\n\nSelects GraphQL field `withWorkdir` on `Workspace`."]
    #[must_use]
    pub fn with_workdir(&self, path: impl Into<String>) -> super::Workspace {
        let query = self.selection.select("withWorkdir");
        let query = query.arg("path", path.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a module client removed from the deepest matching recorded scope.\n\nFail if several SDKs have that deepest scope. The selected SDK module regenerates the complete scope.\n\nIf invalid client targets remain, save the removal and skip generation until those targets are corrected or removed.\n\nSelects GraphQL field `withoutClient` on `Workspace`."]
    #[must_use]
    pub fn without_client(&self, module: impl Into<String>) -> super::Workspace {
        let query = self.selection.select("withoutClient");
        let query = query.arg("module", module.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withoutClient` with a borrowed, reusable `WorkspaceWithoutClientOpts` value."]
    #[must_use]
    pub fn without_client_opts(
        &self,
        module: impl Into<String>,
        opts: &WorkspaceWithoutClientOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withoutClient");
        let query = query.arg("module", module.into());
        let query = if let Some(value) = &opts.sdk {
            query.arg("sdk", value)
        } else {
            query
        };
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a named config environment removed.\n\nSelects GraphQL field `withoutConfigEnv` on `Workspace`."]
    #[must_use]
    pub fn without_config_env(&self, name: impl Into<String>) -> super::Workspace {
        let query = self.selection.select("withoutConfigEnv");
        let query = query.arg("name", name.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withoutConfigEnv` with a borrowed, reusable `WorkspaceWithoutConfigEnvOpts` value."]
    #[must_use]
    pub fn without_config_env_opts(
        &self,
        name: impl Into<String>,
        opts: &WorkspaceWithoutConfigEnvOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withoutConfigEnv");
        let query = if let Some(value) = &opts.here {
            query.arg("here", value)
        } else {
            query
        };
        let query = query.arg("name", name.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a configuration value removed.\n\nErrors when the key is not currently set.\n\nWhen the session selects an env, the key is scoped to that env's overlay.\n\nSelects GraphQL field `withoutConfigValue` on `Workspace`."]
    #[must_use]
    pub fn without_config_value(&self, key: impl Into<String>) -> super::Workspace {
        let query = self.selection.select("withoutConfigValue");
        let query = query.arg("key", key.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withoutConfigValue` with a borrowed, reusable `WorkspaceWithoutConfigValueOpts` value."]
    #[must_use]
    pub fn without_config_value_opts(
        &self,
        key: impl Into<String>,
        opts: &WorkspaceWithoutConfigValueOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withoutConfigValue");
        let query = if let Some(value) = &opts.here {
            query.arg("here", value)
        } else {
            query
        };
        let query = query.arg("key", key.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a directory removed, without mutating the source.\n\nSelects GraphQL field `withoutDirectory` on `Workspace`."]
    #[must_use]
    pub fn without_directory(&self, path: impl Into<String>) -> super::Workspace {
        let query = self.selection.select("withoutDirectory");
        let query = query.arg("path", path.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with no module selected as its entrypoint.\n\nSelects GraphQL field `withoutEntrypoint` on `Workspace`."]
    #[must_use]
    pub fn without_entrypoint(&self) -> super::Workspace {
        let query = self.selection.select("withoutEntrypoint");
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a file removed, without mutating the source.\n\nSelects GraphQL field `withoutFile` on `Workspace`."]
    #[must_use]
    pub fn without_file(&self, path: impl Into<String>) -> super::Workspace {
        let query = self.selection.select("withoutFile");
        let query = query.arg("path", path.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with files removed, without mutating the source.\n\nSelects GraphQL field `withoutFiles` on `Workspace`."]
    #[must_use]
    pub fn without_files(&self, paths: Vec<impl Into<String>>) -> super::Workspace {
        let query = self.selection.select("withoutFiles");
        let paths = paths.into_iter().map(Into::into).collect::<Vec<String>>();
        let query = query.arg("paths", paths);
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with a module removed from its config.\n\nWhen the session selects an env, only that env's overlay entry is removed.\n\nSelects GraphQL field `withoutModule` on `Workspace`."]
    #[must_use]
    pub fn without_module(&self, name: impl Into<String>) -> super::Workspace {
        let query = self.selection.select("withoutModule");
        let query = query.arg("name", name.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withoutModule` with a borrowed, reusable `WorkspaceWithoutModuleOpts` value."]
    #[must_use]
    pub fn without_module_opts(
        &self,
        name: impl Into<String>,
        opts: &WorkspaceWithoutModuleOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withoutModule");
        let query = if let Some(value) = &opts.here {
            query.arg("here", value)
        } else {
            query
        };
        let query = query.arg("name", name.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with the content mounted at the given path unmounted.\n\nRemoves directory and file mounts at or below the path, revealing the underlying workspace content. Other mounts and pending changes are preserved.\n\nSelects GraphQL field `withoutMount` on `Workspace`."]
    #[must_use]
    pub fn without_mount(&self, path: impl Into<String>) -> super::Workspace {
        let query = self.selection.select("withoutMount");
        let query = query.arg("path", path.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return this workspace with an SDK removed from its config.\n\nSelects GraphQL field `withoutSDK` on `Workspace`."]
    #[must_use]
    pub fn without_sdk(&self, name: impl Into<String>) -> super::Workspace {
        let query = self.selection.select("withoutSDK");
        let query = query.arg("name", name.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withoutSDK` with a borrowed, reusable `WorkspaceWithoutSdkOpts` value."]
    #[must_use]
    pub fn without_sdk_opts(
        &self,
        name: impl Into<String>,
        opts: &WorkspaceWithoutSdkOpts,
    ) -> super::Workspace {
        let query = self.selection.select("withoutSDK");
        let query = if let Some(value) = &opts.here {
            query.arg("here", value)
        } else {
            query
        };
        let query = query.arg("name", name.into());
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
}
impl super::Node for Workspace {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
