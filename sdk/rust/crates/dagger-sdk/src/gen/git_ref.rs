//! Generated bindings owned by the GraphQL `GitRef` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
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
