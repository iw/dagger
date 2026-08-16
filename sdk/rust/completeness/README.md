# Pinned harness sources

This tree vendors commit-pinned snapshots of external check harnesses that the
Rust SDK's contribution guide names as sources of truth, so their contract can be
read without network access:

- `sources/sdk-sdk/<commit>/` — the upstream `sdk-sdk` harness's Dang end-to-end
  check module (`mod-test-e2e` and its `echo` fixture), at the pinned commit in
  the directory name.

Nothing in the workspace consumes these files at build or test time; they are
reference material for maintainers porting behaviour. The snapshot is not
self-contained (its `mod-test` dependency is not vendored) and is refreshed by
re-vendoring at a newer commit and renaming the directory — there is no
automated freshness check, so treat the pinned commit as the statement of which
harness revision was last consulted.
