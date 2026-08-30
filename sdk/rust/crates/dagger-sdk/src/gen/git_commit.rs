//! Generated bindings owned by the GraphQL `GitCommit` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
#[doc = "An immutable git commit."]
#[derive(Clone)]
pub struct GitCommit {
    pub(crate) session: crate::lifecycle::SessionHandle,
    pub(crate) selection: crate::query::Selection,
}
#[doc = "Owned optional arguments for GraphQL operation `GitCommit.ancestorReleaseTag`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitCommitAncestorReleaseTagOpts {
    #[doc = "Include pre-release tags when choosing the latest tag.\n\n`None` omits GraphQL field `includePreRelease` and preserves engine default `Boolean(false)`."]
    pub include_pre_release: Option<bool>,
}
impl GitCommitAncestorReleaseTagOpts {
    #[doc = "Sets GraphQL argument `includePreRelease` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_include_pre_release(mut self, value: bool) -> Self {
        self.include_pre_release = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `GitCommit.releaseTag`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitCommitReleaseTagOpts {
    #[doc = "Include pre-release tags when choosing the latest tag.\n\n`None` omits GraphQL field `includePreRelease` and preserves engine default `Boolean(false)`."]
    pub include_pre_release: Option<bool>,
}
impl GitCommitReleaseTagOpts {
    #[doc = "Sets GraphQL argument `includePreRelease` to a concrete value instead of omitting it."]
    #[must_use]
    pub fn with_include_pre_release(mut self, value: bool) -> Self {
        self.include_pre_release = Some(value);
        self
    }
}
#[doc = "Owned optional arguments for GraphQL operation `GitCommit.tree`; reuse does not mutate caller state."]
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct GitCommitTreeOpts {
    #[doc = "The depth of the tree to fetch.\n\n`None` omits GraphQL field `depth` and preserves engine default `Int(1)`."]
    pub depth: Option<i64>,
    #[doc = "Set to true to discard .git directory.\n\n`None` omits GraphQL field `discardGitDir` and preserves engine default `Boolean(false)`."]
    pub discard_git_dir: Option<bool>,
    #[doc = "Set to true to populate tag refs in the local checkout .git.\n\n`None` omits GraphQL field `includeTags` and preserves engine default `Boolean(false)`."]
    pub include_tags: Option<bool>,
}
impl GitCommitTreeOpts {
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
impl crate::IntoID<crate::Id> for GitCommit {
    fn into_id(
        self,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send>,
    > {
        Box::pin(async move { self.id().await })
    }
}
impl crate::loadable::private::Sealed for GitCommit {
    fn graphql_type() -> &'static str {
        "GitCommit"
    }
    fn from_query(
        session: crate::lifecycle::SessionHandle,
        selection: crate::query::Selection,
    ) -> Self {
        Self { session, selection }
    }
}
impl From<GitCommit> for crate::IdInput<GitCommit> {
    fn from(value: GitCommit) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl From<GitCommit> for crate::IdInput<super::NodeClient> {
    fn from(value: GitCommit) -> Self {
        crate::IdInput::lazy(value)
    }
}
impl GitCommit {
    #[doc = "The latest semver release tag reachable from this commit.\n\nSelects GraphQL field `ancestorReleaseTag` on `GitCommit`."]
    pub async fn ancestor_release_tag(&self) -> Result<Option<super::GitRef>, crate::QueryError> {
        let query = self.selection.select("ancestorReleaseTag");
        let query = query.select("id");
        query
            .execute_reentry::<super::GitRef, Option<crate::Id>>(&self.session, "GitRef")
            .await
    }
    #[doc = "Executes GraphQL operation `ancestorReleaseTag` with a borrowed, reusable `GitCommitAncestorReleaseTagOpts` value."]
    pub async fn ancestor_release_tag_opts(
        &self,
        opts: &GitCommitAncestorReleaseTagOpts,
    ) -> Result<Option<super::GitRef>, crate::QueryError> {
        let query = self.selection.select("ancestorReleaseTag");
        let query = if let Some(value) = &opts.include_pre_release {
            query.arg("includePreRelease", value)
        } else {
            query
        };
        let query = query.select("id");
        query
            .execute_reentry::<super::GitRef, Option<crate::Id>>(&self.session, "GitRef")
            .await
    }
    #[doc = "Git author email.\n\nSelects GraphQL field `authorEmail` on `GitCommit`."]
    pub async fn author_email(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("authorEmail");
        query.execute(&self.session).await
    }
    #[doc = "Git author name.\n\nSelects GraphQL field `authorName` on `GitCommit`."]
    pub async fn author_name(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("authorName");
        query.execute(&self.session).await
    }
    #[doc = "Git author date, in RFC3339 format.\n\nSelects GraphQL field `authoredDate` on `GitCommit`."]
    pub async fn authored_date(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("authoredDate");
        query.execute(&self.session).await
    }
    #[doc = "Git committer date, in RFC3339 format.\n\nSelects GraphQL field `committedDate` on `GitCommit`."]
    pub async fn committed_date(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("committedDate");
        query.execute(&self.session).await
    }
    #[doc = "Git committer email.\n\nSelects GraphQL field `committerEmail` on `GitCommit`."]
    pub async fn committer_email(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("committerEmail");
        query.execute(&self.session).await
    }
    #[doc = "Git committer name.\n\nSelects GraphQL field `committerName` on `GitCommit`."]
    pub async fn committer_name(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("committerName");
        query.execute(&self.session).await
    }
    #[doc = "A unique identifier for this GitCommit.\n\nSelects GraphQL field `id` on `GitCommit`."]
    pub async fn id(&self) -> Result<crate::Id, crate::QueryError> {
        let query = self.selection.select("id");
        query.execute(&self.session).await
    }
    #[doc = "Full commit message.\n\nSelects GraphQL field `message` on `GitCommit`."]
    pub async fn message(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("message");
        query.execute(&self.session).await
    }
    #[doc = "Commit message body, excluding the headline.\n\nSelects GraphQL field `messageBody` on `GitCommit`."]
    pub async fn message_body(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("messageBody");
        query.execute(&self.session).await
    }
    #[doc = "First line of the commit message.\n\nSelects GraphQL field `messageHeadline` on `GitCommit`."]
    pub async fn message_headline(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("messageHeadline");
        query.execute(&self.session).await
    }
    #[doc = "Parent commit SHAs.\n\nSelects GraphQL field `parentShas` on `GitCommit`."]
    pub async fn parent_shas(&self) -> Result<Vec<String>, crate::QueryError> {
        let query = self.selection.select("parentShas");
        query.execute(&self.session).await
    }
    #[doc = "The latest semver release tag that points directly at this commit.\n\nSelects GraphQL field `releaseTag` on `GitCommit`."]
    pub async fn release_tag(&self) -> Result<Option<super::GitRef>, crate::QueryError> {
        let query = self.selection.select("releaseTag");
        let query = query.select("id");
        query
            .execute_reentry::<super::GitRef, Option<crate::Id>>(&self.session, "GitRef")
            .await
    }
    #[doc = "Executes GraphQL operation `releaseTag` with a borrowed, reusable `GitCommitReleaseTagOpts` value."]
    pub async fn release_tag_opts(
        &self,
        opts: &GitCommitReleaseTagOpts,
    ) -> Result<Option<super::GitRef>, crate::QueryError> {
        let query = self.selection.select("releaseTag");
        let query = if let Some(value) = &opts.include_pre_release {
            query.arg("includePreRelease", value)
        } else {
            query
        };
        let query = query.select("id");
        query
            .execute_reentry::<super::GitRef, Option<crate::Id>>(&self.session, "GitRef")
            .await
    }
    #[doc = "The full commit SHA.\n\nSelects GraphQL field `sha` on `GitCommit`."]
    pub async fn sha(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("sha");
        query.execute(&self.session).await
    }
    #[doc = "The abbreviated commit SHA.\n\nSelects GraphQL field `shortSha` on `GitCommit`."]
    pub async fn short_sha(&self) -> Result<String, crate::QueryError> {
        let query = self.selection.select("shortSha");
        query.execute(&self.session).await
    }
    #[doc = "The filesystem tree at this commit.\n\nSelects GraphQL field `tree` on `GitCommit`."]
    #[must_use]
    pub fn tree(&self) -> super::Directory {
        let query = self.selection.select("tree");
        super::Directory {
            session: self.session.clone(),
            selection: query,
        }
    }
    #[doc = "Executes GraphQL operation `tree` with a borrowed, reusable `GitCommitTreeOpts` value."]
    #[must_use]
    pub fn tree_opts(&self, opts: &GitCommitTreeOpts) -> super::Directory {
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
impl super::Node for GitCommit {
    fn id(
        &self,
    ) -> impl core::future::Future<Output = Result<crate::Id, crate::QueryError>> + Send {
        let query = self.selection.select("id");
        let session = self.session.clone();
        async move { query.execute(&session).await }
    }
}
