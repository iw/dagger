//! Generated bindings owned by the GraphQL `GitRef` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:8cbd134fc17405aa0ae54b45640f4bd6b93a022bd4a2c4594375f3658962bf7f","target_revision":"249edbd646e63c5181053c8af72623ed9bdf030d"}
#[doc = "A git ref (tag, branch, or commit)."]
#[derive(Clone)]
pub struct GitRef {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
#[doc = "Owned optional arguments for GraphQL operation `GitRef.asWorkspace`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitRefAsWorkspaceOpts {
    #[doc = "Current working directory inside the workspace root. Defaults to the workspace root.\n\n`None` omits GraphQL field `cwd` and preserves engine default `String(\"/\")`."]
    pub cwd: Option<String>,
}
impl GitRefAsWorkspaceOpts {
    #[doc = "Sets GraphQL argument `cwd` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_cwd(mut self, value: impl Into<String>) -> Self {
        self.cwd = Some(value.into());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `GitRef.log`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitRefLogOpts {
    #[doc = "Exclude commits reachable from this ref, i.e. only list commits added on top of it.\n\n`None` omits GraphQL field `base`."]
    pub base: Option<crate::IdInput<super::GitRef>>,
    #[doc = "Maximum number of commits to return.\n\n`None` omits GraphQL field `limit` and preserves engine default `Int(10)`."]
    pub limit: Option<i64>,
    #[doc = "Only include commits touching these paths, relative to the root of the repository.\n\n`None` omits GraphQL field `paths`."]
    pub paths: Option<Vec<String>>,
}
impl GitRefLogOpts {
    #[doc = "Sets GraphQL argument `base` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_base(mut self, value: crate::IdInput<super::GitRef>) -> Self {
        self.base = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `limit` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `paths` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_paths(mut self, value: Vec<impl Into<String>>) -> Self {
        self.paths = Some(value.into_iter().map(Into::into).collect());
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `GitRef.push`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitRefPushOpts {
    #[doc = "Destination branch; a refs/ prefix is used verbatim. Defaults to this ref's branch name. Required for detached and non-branch refs.\n\n`None` omits GraphQL field `branch` and preserves engine default `String(\"\")`."]
    pub branch: Option<String>,
    #[doc = "Optional lease: a full lowercase object ID allows replacement only if the remote ref still has that value. Checked even for up-to-date pushes. Empty or omitted uses normal non-force rules, creating the ref if it does not exist.\n\n`None` omits GraphQL field `expectedRemoteSHA` and preserves engine default `String(\"\")`."]
    pub expected_remote_sha: Option<String>,
    #[doc = "Name of a registered remote to push to (see GitRepository.withRemote). Defaults to origin. The remote's push URLs, or its URL, become the destination; more than one push URL requires an explicit to instead.\n\n`None` omits GraphQL field `remote` and preserves engine default `String(\"\")`."]
    pub remote: Option<String>,
    #[doc = "Destination remote repository. Defaults to the origin remote's push routing, or the source's repository URL when none is registered. Required when the source has no remote URL.\n\n`None` omits GraphQL field `to`."]
    pub to: Option<crate::IdInput<super::GitRepository>>,
}
impl GitRefPushOpts {
    #[doc = "Sets GraphQL argument `branch` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_branch(mut self, value: impl Into<String>) -> Self {
        self.branch = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `expectedRemoteSHA` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_expected_remote_sha(mut self, value: impl Into<String>) -> Self {
        self.expected_remote_sha = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `remote` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_remote(mut self, value: impl Into<String>) -> Self {
        self.remote = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `to` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_to(mut self, value: crate::IdInput<super::GitRepository>) -> Self {
        self.to = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `GitRef.tree`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitRefTreeOpts {
    #[doc = "The depth of the tree to fetch.\n\n`None` omits GraphQL field `depth` and preserves engine default `Int(1)`."]
    pub depth: Option<i64>,
    #[doc = "Set to true to discard .git directory.\n\n`None` omits GraphQL field `discardGitDir` and preserves engine default `Boolean(false)`."]
    pub discard_git_dir: Option<bool>,
    #[doc = "Set to true to populate tag refs in the local checkout .git.\n\n`None` omits GraphQL field `includeTags` and preserves engine default `Boolean(false)`."]
    pub include_tags: Option<bool>,
}
impl GitRefTreeOpts {
    #[doc = "Sets GraphQL argument `depth` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_depth(mut self, value: i64) -> Self {
        self.depth = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `discardGitDir` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_discard_git_dir(mut self, value: bool) -> Self {
        self.discard_git_dir = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `includeTags` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_include_tags(mut self, value: bool) -> Self {
        self.include_tags = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `GitRef.withCommit`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitRefWithCommitOpts {
    #[doc = "Allow a commit whose tree matches its parent, including when the supplied edits are already present. Defaults to false.\n\n`None` omits GraphQL field `allowEmpty` and preserves engine default `Boolean(false)`."]
    pub allow_empty: Option<bool>,
    #[doc = "RFC3339 committer date. Defaults to date.\n\n`None` omits GraphQL field `committerDate`."]
    pub committer_date: Option<String>,
    #[doc = "Committer email. Defaults to authorEmail.\n\n`None` omits GraphQL field `committerEmail`."]
    pub committer_email: Option<String>,
    #[doc = "Committer name. Defaults to authorName.\n\n`None` omits GraphQL field `committerName`."]
    pub committer_name: Option<String>,
    #[doc = "Add a Signed-off-by trailer using the commit author's name and email.\n\n`None` omits GraphQL field `signoff` and preserves engine default `Boolean(false)`."]
    pub signoff: Option<bool>,
}
impl GitRefWithCommitOpts {
    #[doc = "Sets GraphQL argument `allowEmpty` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_allow_empty(mut self, value: bool) -> Self {
        self.allow_empty = Some(value);
        self
    }
    #[doc = "Sets GraphQL argument `committerDate` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_committer_date(mut self, value: impl Into<String>) -> Self {
        self.committer_date = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `committerEmail` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_committer_email(mut self, value: impl Into<String>) -> Self {
        self.committer_email = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `committerName` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_committer_name(mut self, value: impl Into<String>) -> Self {
        self.committer_name = Some(value.into());
        self
    }
    #[doc = "Sets GraphQL argument `signoff` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_signoff(mut self, value: bool) -> Self {
        self.signoff = Some(value);
        self
    }
}
impl crate::IntoID<crate::Id> for GitRef {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for GitRef {
    fn graphql_type() -> &'static str {
        "GitRef"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<GitRef> for crate::IdInput<GitRef> {
    fn from(value: GitRef) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<GitRef> for crate::IdInput<super::NodeClient> {
    fn from(value: GitRef) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl GitRef {
    #[doc = "Return this ref's repository with HEAD pinned to the selected commit.\n\nPreserves the original repository backend, connection information, and other refs. Does not modify a branch or checkout, or prune history.\n\nSelects GraphQL field `asRepository` on `GitRef`."]
    #[must_use]
    pub fn as_repository(&self) -> super::GitRepository {
        let query = self.selection.select("asRepository");
        super::GitRepository {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Creates a synthetic workspace from this git ref.\n\nSelects GraphQL field `asWorkspace` on `GitRef`."]
    #[must_use]
    pub fn as_workspace(&self) -> super::Workspace {
        let query = self.selection.select("asWorkspace");
        super::Workspace {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `asWorkspace` with a borrowed, reusable `GitRefAsWorkspaceOpts` value."]
    #[must_use]
    pub fn as_workspace_opts(&self, opts: &GitRefAsWorkspaceOpts) -> super::Workspace {
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
    #[doc = "The resolved commit id at this ref.\n\nSelects GraphQL field `commit` on `GitRef`.\n\n**Deprecated:** Use \"commitSHA\" instead."]
    #[deprecated(note = "Use \"commitSHA\" instead.")]
    pub async fn commit(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("commit");
        query.execute(&self.session).await
    }
    #[doc = "The resolved commit SHA at this ref.\n\nSelects GraphQL field `commitSHA` on `GitRef`."]
    pub async fn commit_sha(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("commitSHA");
        query.execute(&self.session).await
    }
    #[doc = "Find the best common ancestor between this ref and another ref.\n\nSelects GraphQL field `commonAncestor` on `GitRef`."]
    #[must_use]
    pub fn common_ancestor(
        &self,
        other: impl Into<crate::IdInput<super::GitRef>>,
    ) -> super::GitRef {
        let query = self.selection.select("commonAncestor");
        let query = query.arg_id_input("other", other.into());
        super::GitRef {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Return true when the other ref's commit equals this commit or is an ancestor of it.\n\nCompares commit history across branches, tags and detached refs. Incomplete or unavailable history is an error.\n\nSelects GraphQL field `contains` on `GitRef`."]
    pub async fn contains(
        &self,
        other: impl Into<crate::IdInput<super::GitRef>>,
    ) -> Result<bool, crate::QueryError> {
        let query = self.selection.select("contains");
        let query = query.arg_id_input("other", other.into());
        query.execute(&self.session).await
    }
    #[doc = "A unique identifier for this GitRef.\n\nSelects GraphQL field `id` on `GitRef`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "Commits reachable from this ref, newest first, starting with the commit this ref resolves to.\n\nSelects GraphQL field `log` on `GitRef`."]
    pub async fn log(&self) -> Result<Vec<super::GitCommit>, crate::QueryError> {
        let query = self.selection.select("log");
        let query = query.select("id");
        query
            .execute_reentry::<super::GitCommit, Vec<crate::Id>>(&self.session, "GitCommit")
            .await
    }
    #[doc = "Executes GraphQL operation `log` with a borrowed, reusable `GitRefLogOpts` value."]
    pub async fn log_opts(
        &self,
        opts: &GitRefLogOpts,
    ) -> Result<Vec<super::GitCommit>, crate::QueryError> {
        let query = self.selection.select("log");
        let query = if let Some(value) = &opts.base {
            query.arg_id_input("base", value.clone())
        } else {
            query
        };
        let query = if let Some(value) = &opts.limit {
            query.arg("limit", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.paths {
            query.arg("paths", value)
        } else {
            query
        };
        let query = query.select("id");
        query
            .execute_reentry::<super::GitCommit, Vec<crate::Id>>(&self.session, "GitCommit")
            .await
    }
    #[doc = "The resolved name of this ref.\n\nSelects GraphQL field `name` on `GitRef`."]
    pub async fn name(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("name");
        query.execute(&self.session).await
    }
    #[doc = "Push this ref's commit and history to a remote repository using the destination's credentials.\n\nThe source can come from a remote repository or an engine-side Git repository. To publish a workspace's commits, use Workspace.git.head.push. Pushing does not modify the calling client's checkout, and checkout hooks do not run.\n\nA missing remote ref is created. Without a lease, Git's normal non-force rules apply. Each invocation performs a push; loading the returned receipt does not push again.\n\nSelects GraphQL field `push` on `GitRef`."]
    #[must_use]
    pub fn push(&self) -> super::GitPushResult {
        let query = self.selection.select("push");
        super::GitPushResult {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `push` with a borrowed, reusable `GitRefPushOpts` value."]
    #[must_use]
    pub fn push_opts(&self, opts: &GitRefPushOpts) -> super::GitPushResult {
        let query = self.selection.select("push");
        let query = if let Some(value) = &opts.branch {
            query.arg("branch", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.expected_remote_sha {
            query.arg("expectedRemoteSHA", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.remote {
            query.arg("remote", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.to {
            query.arg_id_input("to", value.clone())
        } else {
            query
        };
        super::GitPushResult {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "The resolved ref name at this ref.\n\nSelects GraphQL field `ref` on `GitRef`.\n\n**Deprecated:** Use \"name\" instead."]
    #[deprecated(note = "Use \"name\" instead.")]
    pub async fn r#ref(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("ref");
        query.execute(&self.session).await
    }
    #[doc = "The commit this ref resolves to.\n\nSelects GraphQL field `targetCommit` on `GitRef`."]
    #[must_use]
    pub fn target_commit(&self) -> super::GitCommit {
        let query = self.selection.select("targetCommit");
        super::GitCommit {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "The filesystem tree at this ref.\n\nSelects GraphQL field `tree` on `GitRef`."]
    #[must_use]
    pub fn tree(&self) -> super::Directory {
        let query = self.selection.select("tree");
        super::Directory {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `tree` with a borrowed, reusable `GitRefTreeOpts` value."]
    #[must_use]
    pub fn tree_opts(&self, opts: &GitRefTreeOpts) -> super::Directory {
        let query = self.selection.select("tree");
        let query = if let Some(value) = &opts.depth {
            query.arg("depth", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.discard_git_dir {
            query.arg("discardGitDir", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.include_tags {
            query.arg("includeTags", value)
        } else {
            query
        };
        super::Directory {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Create a single-parent commit on this ref by applying a changeset's edits.\n\nThree-way merges the changeset against this ref's tree, using its before snapshot as the base. Preserves compatible parent edits and fails on conflicts. Does not modify the input repository or host checkout.\n\nIdentity and dates are explicit; neither client Git configuration nor the current clock is consulted.\n\nSelects GraphQL field `withCommit` on `GitRef`."]
    #[must_use]
    pub fn with_commit(
        &self,
        author_email: impl Into<String>,
        author_name: impl Into<String>,
        changes: impl Into<crate::IdInput<super::Changeset>>,
        date: impl Into<String>,
        message: impl Into<String>,
    ) -> super::GitRef {
        let query = self.selection.select("withCommit");
        let query = query.arg("authorEmail", author_email.into());
        let query = query.arg("authorName", author_name.into());
        let query = query.arg_id_input("changes", changes.into());
        let query = query.arg("date", date.into());
        let query = query.arg("message", message.into());
        super::GitRef {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `withCommit` with a borrowed, reusable `GitRefWithCommitOpts` value."]
    #[must_use]
    pub fn with_commit_opts(
        &self,
        author_email: impl Into<String>,
        author_name: impl Into<String>,
        changes: impl Into<crate::IdInput<super::Changeset>>,
        date: impl Into<String>,
        message: impl Into<String>,
        opts: &GitRefWithCommitOpts,
    ) -> super::GitRef {
        let query = self.selection.select("withCommit");
        let query = if let Some(value) = &opts.allow_empty {
            query.arg("allowEmpty", value)
        } else {
            query
        };
        let query = query.arg("authorEmail", author_email.into());
        let query = query.arg("authorName", author_name.into());
        let query = query.arg_id_input("changes", changes.into());
        let query = if let Some(value) = &opts.committer_date {
            query.arg("committerDate", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.committer_email {
            query.arg("committerEmail", value)
        } else {
            query
        };
        let query = if let Some(value) = &opts.committer_name {
            query.arg("committerName", value)
        } else {
            query
        };
        let query = query.arg("date", date.into());
        let query = query.arg("message", message.into());
        let query = if let Some(value) = &opts.signoff {
            query.arg("signoff", value)
        } else {
            query
        };
        super::GitRef {
            session: self.session.clone(),
            selection: query,
        }
    }
}
impl super::Node for GitRef {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
