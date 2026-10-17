//! Generated bindings owned by the GraphQL `GitRepository` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "A git repository."]
#[derive(Clone)]
pub struct GitRepository {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
#[doc = "Owned optional arguments for GraphQL operation `GitRepository.asWorkspace`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitRepositoryAsWorkspaceOpts {
    #[doc = "Current working directory inside the workspace root. Defaults to the workspace root.\n\n`None` omits GraphQL field `cwd` and preserves engine default `String(\"/\")`."]
    pub cwd: Option<String>,
}
impl GitRepositoryAsWorkspaceOpts {
    #[doc = "Sets GraphQL argument `cwd` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_cwd(mut self, value: impl Into<String>) -> Self {
        self.cwd = Some(value.into());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `GitRepository.branch`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitRepositoryBranchOpts {
    #[doc = "Ignore the workspace lockfile for this lookup.\n\n`None` omits GraphQL field `noLock` and preserves engine default `Boolean(false)`."]
    pub no_lock: Option<bool>,
}
impl GitRepositoryBranchOpts {
    #[doc = "Sets GraphQL argument `noLock` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_no_lock(mut self, value: bool) -> Self {
        self.no_lock = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `GitRepository.branches`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitRepositoryBranchesOpts {
    #[doc = "Glob patterns (e.g., \"refs/tags/v*\").\n\n`None` omits GraphQL field `patterns`."]
    pub patterns: Option<Vec<String>>,
}
impl GitRepositoryBranchesOpts {
    #[doc = "Sets GraphQL argument `patterns` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_patterns(mut self, value: Vec<impl Into<String>>) -> Self {
        self.patterns = Some(value.into_iter().map(Into::into).collect());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `GitRepository.bundle`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitRepositoryBundleOpts {
    #[doc = "A Git ref whose reachable objects are omitted and recorded as a prerequisite.\n\n`None` omits GraphQL field `base`."]
    pub base: Option<crate::IdInput<super::GitRef>>,
}
impl GitRepositoryBundleOpts {
    #[doc = "Sets GraphQL argument `base` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_base(mut self, value: crate::IdInput<super::GitRef>) -> Self {
        self.base = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `GitRepository.head`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitRepositoryHeadOpts {
    #[doc = "Ignore the workspace lockfile for this lookup.\n\n`None` omits GraphQL field `noLock` and preserves engine default `Boolean(false)`."]
    pub no_lock: Option<bool>,
}
impl GitRepositoryHeadOpts {
    #[doc = "Sets GraphQL argument `noLock` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_no_lock(mut self, value: bool) -> Self {
        self.no_lock = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `GitRepository.latest`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitRepositoryLatestOpts {
    #[doc = "Ignore the workspace lockfile for this lookup.\n\n`None` omits GraphQL field `noLock` and preserves engine default `Boolean(false)`."]
    pub no_lock: Option<bool>,
    #[doc = "Version query used to select the greatest matching release ref.\n\n`None` omits GraphQL field `version` and preserves engine default `String(\"\")`."]
    pub version: Option<String>,
}
impl GitRepositoryLatestOpts {
    #[doc = "Sets GraphQL argument `noLock` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_no_lock(mut self, value: bool) -> Self {
        self.no_lock = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `version` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `GitRepository.ref`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitRepositoryRefOpts {
    #[doc = "Ignore the workspace lockfile for this lookup.\n\n`None` omits GraphQL field `noLock` and preserves engine default `Boolean(false)`."]
    pub no_lock: Option<bool>,
}
impl GitRepositoryRefOpts {
    #[doc = "Sets GraphQL argument `noLock` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_no_lock(mut self, value: bool) -> Self {
        self.no_lock = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `GitRepository.tag`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitRepositoryTagOpts {
    #[doc = "Ignore the workspace lockfile for this lookup.\n\n`None` omits GraphQL field `noLock` and preserves engine default `Boolean(false)`."]
    pub no_lock: Option<bool>,
}
impl GitRepositoryTagOpts {
    #[doc = "Sets GraphQL argument `noLock` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_no_lock(mut self, value: bool) -> Self {
        self.no_lock = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `GitRepository.tags`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitRepositoryTagsOpts {
    #[doc = "Glob patterns (e.g., \"refs/tags/v*\").\n\n`None` omits GraphQL field `patterns`."]
    pub patterns: Option<Vec<String>>,
}
impl GitRepositoryTagsOpts {
    #[doc = "Sets GraphQL argument `patterns` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_patterns(mut self, value: Vec<impl Into<String>>) -> Self {
        self.patterns = Some(value.into_iter().map(Into::into).collect());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `GitRepository.withBundle`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitRepositoryWithBundleOpts {
    #[doc = "An optional remote ref hint for fetching a prerequisite when the remote does not allow fetches by object ID.\n\n`None` omits GraphQL field `prerequisiteRef` and preserves engine default `String(\"\")`."]
    pub prerequisite_ref: Option<String>,
}
impl GitRepositoryWithBundleOpts {
    #[doc = "Sets GraphQL argument `prerequisiteRef` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_prerequisite_ref(mut self, value: impl Into<String>) -> Self {
        self.prerequisite_ref = Some(value.into());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `GitRepository.withRemote`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitRepositoryWithRemoteOpts {
    #[doc = "Push destination, when pushes go somewhere other than url. Empty uses url.\n\n`None` omits GraphQL field `pushUrl` and preserves engine default `String(\"\")`."]
    pub push_url: Option<String>,
}
impl GitRepositoryWithRemoteOpts {
    #[doc = "Sets GraphQL argument `pushUrl` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_push_url(mut self, value: impl Into<String>) -> Self {
        self.push_url = Some(value.into());
        self
    }
}
impl crate::IntoID<crate::Id> for GitRepository {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for GitRepository {
    fn graphql_type() -> &'static str {
        "GitRepository"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<GitRepository> for crate::IdInput<GitRepository> {
    fn from(value: GitRepository) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<GitRepository> for crate::IdInput<super::NodeClient> {
    fn from(value: GitRepository) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl GitRepository {
    #[doc = "Creates a synthetic workspace from this repository's HEAD and uncommitted file changes.\n\nPending changes are applied at the repository root. The staging split is not preserved. The source repository is not modified.\n\nSelects GraphQL field `asWorkspace` on `GitRepository`."]
    #[must_use]
    pub fn as_workspace(&self) -> super::Workspace {
        let query = self.selection.select("asWorkspace");
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `asWorkspace` with a borrowed, reusable `GitRepositoryAsWorkspaceOpts` value."]
    #[must_use]
    pub fn as_workspace_opts(&self, opts: &GitRepositoryAsWorkspaceOpts) -> super::Workspace {
        let query = self.selection.select("asWorkspace");
        let query = if let Some(value) = &opts.cwd {
            query.arg("cwd", value)
        } else {
            query
        };
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Returns details of a branch.\n\nSelects GraphQL field `branch` on `GitRepository`."]
    #[must_use]
    pub fn branch(&self, name: impl Into<String>) -> super::GitRef {
        let query = self.selection.select("branch");
        let query = query.arg("name", name.into());
        super::GitRef {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `branch` with a borrowed, reusable `GitRepositoryBranchOpts` value."]
    #[must_use]
    pub fn branch_opts(
        &self,
        name: impl Into<String>,
        opts: &GitRepositoryBranchOpts,
    ) -> super::GitRef {
        let query = self.selection.select("branch");
        let query = query.arg("name", name.into());
        let query = if let Some(value) = &opts.no_lock {
            query.arg("noLock", value)
        } else {
            query
        };
        super::GitRef {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "branches that match any of the given glob patterns.\n\nSelects GraphQL field `branches` on `GitRepository`."]
    pub async fn branches(&self) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("branches");
        query.execute(&self.session).await
    }
    #[doc = "Executes GraphQL operation `branches` with a borrowed, reusable `GitRepositoryBranchesOpts` value."]
    pub async fn branches_opts(
        &self,
        opts: &GitRepositoryBranchesOpts,
    ) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("branches");
        let query = if let Some(value) = &opts.patterns {
            query.arg("patterns", value)
        } else {
            query
        };
        query.execute(&self.session).await
    }
    #[doc = "Pack the given refs and the objects needed to reconstruct them into a Git bundle.\n\nSelects GraphQL field `bundle` on `GitRepository`."]
    #[must_use]
    pub fn bundle(&self, refs: Vec<impl Into<String>>) -> super::GitBundle {
        let query = self.selection.select("bundle");
        let refs = refs.into_iter().map(Into::into).collect::<Vec<String>>();
        let query = query.arg("refs", refs);
        super::GitBundle {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `bundle` with a borrowed, reusable `GitRepositoryBundleOpts` value."]
    #[must_use]
    pub fn bundle_opts(
        &self,
        refs: Vec<impl Into<String>>,
        opts: &GitRepositoryBundleOpts,
    ) -> super::GitBundle {
        let query = self.selection.select("bundle");
        let query = if let Some(value) = &opts.base {
            query.arg_id_input("base", value.clone())
        } else {
            query
        };
        let refs = refs.into_iter().map(Into::into).collect::<Vec<String>>();
        let query = query.arg("refs", refs);
        super::GitBundle {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Returns details of a commit.\n\nSelects GraphQL field `commit` on `GitRepository`."]
    #[must_use]
    pub fn commit(&self, id: impl Into<String>) -> super::GitCommit {
        let query = self.selection.select("commit");
        let query = query.arg("id", id.into());
        super::GitCommit {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return the sole remote, otherwise origin, otherwise the selected branch's upstream remote, otherwise null.\n\nFrozen workspaces retain their captured upstream selection. Does not contact remote servers.\n\nSelects GraphQL field `defaultRemote` on `GitRepository`."]
    pub async fn default_remote(&self) -> Result<Option<super::GitRemote>, crate::QueryError> {
        let query = self.selection.select("defaultRemote");
        let query = query.select("id");
        query
            .execute_reentry::<super::GitRemote, Option<crate::Id>>(&self.session, "GitRemote")
            .await
    }
    #[doc = "Returns details for HEAD.\n\nSelects GraphQL field `head` on `GitRepository`."]
    #[must_use]
    pub fn head(&self) -> super::GitRef {
        let query = self.selection.select("head");
        super::GitRef {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `head` with a borrowed, reusable `GitRepositoryHeadOpts` value."]
    #[must_use]
    pub fn head_opts(&self, opts: &GitRepositoryHeadOpts) -> super::GitRef {
        let query = self.selection.select("head");
        let query = if let Some(value) = &opts.no_lock {
            query.arg("noLock", value)
        } else {
            query
        };
        super::GitRef {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "A unique identifier for this GitRepository.\n\nSelects GraphQL field `id` on `GitRepository`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "Return the latest stable release tag, falling back to HEAD when no release exists.\n\nRelease selection accepts an optional \"v\" prefix, incomplete versions, and zero-padded numeric components. This operation is pinned unless noLock is enabled.\n\nSelects GraphQL field `latest` on `GitRepository`."]
    #[must_use]
    pub fn latest(&self) -> super::GitRef {
        let query = self.selection.select("latest");
        super::GitRef {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `latest` with a borrowed, reusable `GitRepositoryLatestOpts` value."]
    #[must_use]
    pub fn latest_opts(&self, opts: &GitRepositoryLatestOpts) -> super::GitRef {
        let query = self.selection.select("latest");
        let query = if let Some(value) = &opts.no_lock {
            query.arg("noLock", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.version {
            query.arg("version", value)
        } else {
            query
        };
        super::GitRef {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Returns details of a ref.\n\nSelects GraphQL field `ref` on `GitRepository`."]
    #[must_use]
    pub fn r#ref(&self, name: impl Into<String>) -> super::GitRef {
        let query = self.selection.select("ref");
        let query = query.arg("name", name.into());
        super::GitRef {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `ref` with a borrowed, reusable `GitRepositoryRefOpts` value."]
    #[must_use]
    pub fn ref_opts(&self, name: impl Into<String>, opts: &GitRepositoryRefOpts) -> super::GitRef {
        let query = self.selection.select("ref");
        let query = query.arg("name", name.into());
        let query = if let Some(value) = &opts.no_lock {
            query.arg("noLock", value)
        } else {
            query
        };
        super::GitRef {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Look up a remote by name. Fails when the remote does not exist.\n\nSelects GraphQL field `remote` on `GitRepository`."]
    #[must_use]
    pub fn remote(&self, name: impl Into<String>) -> super::GitRemote {
        let query = self.selection.select("remote");
        let query = query.arg("name", name.into());
        super::GitRemote {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "List this repository's named remotes, with registered remotes overriding configured ones. Does not contact remote servers.\n\nSelects GraphQL field `remotes` on `GitRepository`."]
    pub async fn remotes(&self) -> Result<Vec<super::GitRemote>, crate::QueryError> {
        let query = self.selection.select("remotes");
        let query = query.select("id");
        query
            .execute_reentry::<super::GitRemote, Vec<crate::Id>>(&self.session, "GitRemote")
            .await
    }
    #[doc = "Returns details of a tag.\n\nSelects GraphQL field `tag` on `GitRepository`."]
    #[must_use]
    pub fn tag(&self, name: impl Into<String>) -> super::GitRef {
        let query = self.selection.select("tag");
        let query = query.arg("name", name.into());
        super::GitRef {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `tag` with a borrowed, reusable `GitRepositoryTagOpts` value."]
    #[must_use]
    pub fn tag_opts(&self, name: impl Into<String>, opts: &GitRepositoryTagOpts) -> super::GitRef {
        let query = self.selection.select("tag");
        let query = query.arg("name", name.into());
        let query = if let Some(value) = &opts.no_lock {
            query.arg("noLock", value)
        } else {
            query
        };
        super::GitRef {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "tags that match any of the given glob patterns.\n\nSelects GraphQL field `tags` on `GitRepository`."]
    pub async fn tags(&self) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("tags");
        query.execute(&self.session).await
    }
    #[doc = "Executes GraphQL operation `tags` with a borrowed, reusable `GitRepositoryTagsOpts` value."]
    pub async fn tags_opts(
        &self,
        opts: &GitRepositoryTagsOpts,
    ) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("tags");
        let query = if let Some(value) = &opts.patterns {
            query.arg("patterns", value)
        } else {
            query
        };
        query.execute(&self.session).await
    }
    #[doc = "Returns the changeset of uncommitted changes in the git repository.\n\nSelects GraphQL field `uncommitted` on `GitRepository`."]
    #[must_use]
    pub fn uncommitted(&self) -> super::Changeset {
        let query = self.selection.select("uncommitted");
        super::Changeset {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "The URL of the git repository.\n\nSelects GraphQL field `url` on `GitRepository`."]
    pub async fn url(&self) -> Result<Option<String>, crate::QueryError> {
        let query = self.selection.select("url");
        query.execute(&self.session).await
    }
    #[doc = "Import a Git bundle after fetching and verifying all of its prerequisites.\n\nSelects GraphQL field `withBundle` on `GitRepository`."]
    #[must_use]
    pub fn with_bundle(
        &self,
        bundle: impl Into<crate::IdInput<super::GitBundle>>,
    ) -> super::GitRepository {
        let query = self.selection.select("withBundle");
        let query = query.arg_id_input("bundle", bundle.into());
        super::GitRepository {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withBundle` with a borrowed, reusable `GitRepositoryWithBundleOpts` value."]
    #[must_use]
    pub fn with_bundle_opts(
        &self,
        bundle: impl Into<crate::IdInput<super::GitBundle>>,
        opts: &GitRepositoryWithBundleOpts,
    ) -> super::GitRepository {
        let query = self.selection.select("withBundle");
        let query = query.arg_id_input("bundle", bundle.into());
        let query = if let Some(value) = &opts.prerequisite_ref {
            query.arg("prerequisiteRef", value)
        } else {
            query
        };
        super::GitRepository {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Replace this repository's storage with the supplied self-contained Git repository, retaining its logical URL and push destinations.\n\nAccepts a whole checkout (including .git and pending file edits), .git contents, or a bare repository. Does not initialize a repository, merge histories, or modify either input.\n\nThe receiver's logical routing wins over the supplied Git configuration; that configuration is not rewritten. Use Directory.asGit to open the supplied repository without retaining the receiver's routing.\n\nWhen the receiver is a remote repository (or was derived from one), that remote is retained with its authentication: refs the supplied storage does not contain resolve through it.\n\nSelects GraphQL field `withContents` on `GitRepository`."]
    #[must_use]
    pub fn with_contents(
        &self,
        directory: impl Into<crate::IdInput<super::Directory>>,
    ) -> super::GitRepository {
        let query = self.selection.select("withContents");
        let query = query.arg_id_input("directory", directory.into());
        super::GitRepository {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Register a named remote on this repository, replacing any registered remote of the same name.\n\nRegistered remotes are recorded in checkouts materialized from this repository (GitRef.tree, Workspace.git.directory), so remote-aware tooling like gh can resolve and fetch from them. The origin remote also routes push when no explicit destination is passed: its push URL, or its URL, becomes the default destination.\n\nRouting metadata only, never a credential grant: pushes still authenticate with the caller's own credentials and require approval as usual.\n\nSelects GraphQL field `withRemote` on `GitRepository`."]
    #[must_use]
    pub fn with_remote(
        &self,
        name: impl Into<String>,
        url: impl Into<String>,
    ) -> super::GitRepository {
        let query = self.selection.select("withRemote");
        let query = query.arg("name", name.into());
        let query = query.arg("url", url.into());
        super::GitRepository {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withRemote` with a borrowed, reusable `GitRepositoryWithRemoteOpts` value."]
    #[must_use]
    pub fn with_remote_opts(
        &self,
        name: impl Into<String>,
        url: impl Into<String>,
        opts: &GitRepositoryWithRemoteOpts,
    ) -> super::GitRepository {
        let query = self.selection.select("withRemote");
        let query = query.arg("name", name.into());
        let query = if let Some(value) = &opts.push_url {
            query.arg("pushUrl", value)
        } else {
            query
        };
        let query = query.arg("url", url.into());
        super::GitRepository {
            session: self.session.clone(),
            selection: query,
        }
    }
}
impl super::Node for GitRepository {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
