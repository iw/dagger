# Implementation Plan

Tasks 1–6 are recorded complete: they were delivered by the 2026-08 audit
remediation (pull request #72) and verified by the engine-free gate on two hosts
plus the engine-backed Build/Verify entry points from `sdk/rust/docs/engine-integration.md`.
Task 7 is the one open follow-up.

- [x] 1. Implement cooperative module cancellation
  - [x] 1.1 Add the process-level signal producer
    - Install SIGTERM/SIGINT handlers scoped to the serve in
      `module/adapter.rs`; trip the shared `ModuleCancellation` on the first
      signal; abort the listener on drop.
    - Construct one token per serve and hand clones to every call envelope.
    - _Requirements: 1.1-1.3, 1.8_
  - [x] 1.2 Wire cancellation into outcome publication
    - Check `is_cancelled` before starting user work; race the authored future
      against `cancelled()`; on a cancellation win, drop the future and publish a
      structured cancelled error as `ErrorOutcomeKind::Cancellation` →
      `PublishedOutcome::CancelledError`.
    - Delete the inert `ResultElection` scaffold; keep single-winner publication
      as a property of outcome selection, not a state machine.
    - _Requirements: 1.4-1.7, 1.9_
  - [x] 1.3 Cover cancellation orderings in dispatch tests
    - Exercise cancel-before-start, cancel-mid-future, cancel racing
      publication, and cancel-after-acceptance through fake sinks; assert no
      successful value follows an observed cancellation.
    - _Requirements: 1.4-1.7_

- [x] 2. Move provisioning off the runtime thread
  - [x] 2.1 Route blocking stages through the blocking pool
    - Wrap extraction, cache publication, and retention pruning in
      `spawn_blocking` with typed join-error mapping; stream downloads through an
      async file handle.
    - _Requirements: 3.1-3.4_
  - [x] 2.2 Replace the parked flock with a cancellable try-lock poll
    - Poll `std::fs::File::try_lock`, retrying only on
      `TryLockError::WouldBlock` with a cancellation check between attempts;
      surface `TryLockError::Error` typed; remove the fs4 dependency in favour
      of standard-library locking (Rust 1.89+).
    - _Requirements: 3.5-3.7_

- [x] 3. Enforce credential and query confidentiality
  - [x] 3.1 Remove the query-document trace and ban the channel
    - Delete the TRACE event carrying the GraphQL document from `query.rs`; add
      the `tracing::` ban to `tests/source_policy.rs`.
    - _Requirements: 4.1, 4.2_
  - [x] 3.2 Redact the launch projection
    - Hand-write `Debug` for `CliLaunchProjection` rendering executable,
      argument count, environment key names, and session allocation identity
      only; render builder presence, not contents; keep the reserved
      runner-token key pinned by
      `property_16_cli_launch_projection_complete_collision_free`.
    - _Requirements: 4.3-4.5_
  - [x] 3.3 Extend identity guards to tests
    - Extend the planning-metadata ban to test files; strip specification
      feature/property comments from tests; rename true duplicate property
      numbers without a full renumber.
    - _Requirements: 4.6_

- [x] 4. Make liveness compiler-audited and generator diagnostics total
  - [x] 4.1 Replace blanket dead-code allows
    - Carry `#[cfg_attr(not(test), expect(dead_code, reason = "..."))]` on every
      production item kept only for tests, in `preflight.rs`, `launch.rs`, and
      `module/codec.rs`.
    - _Requirements: 5.1, 5.2_
  - [x] 4.2 Fix the eager-index panic class in module codegen
    - Bind candidate segments through slice patterns in `module/metadata.rs`;
      report the Result-arity diagnostic for empty `Result` generics and
      `std::io::Result<()>`-shaped returns, for exported functions and
      interface methods; cover both shapes in unit tests.
    - _Requirements: 5.3-5.5_

- [x] 5. Restore packaging and lockfile coherence
  - [x] 5.1 Slim the published packages
    - Exclude example application scaffolding with `!/examples/*/app/**` while
      keeping example sources; ship the macros crate LICENSE; keep
      `publish` upstream-publishable while the fork releases repository
      artifacts only.
    - _Requirements: 6.1, 6.2, 6.4, 6.5_
  - [x] 5.2 Regenerate the external consumer lockfile
    - Rebuild
      `.dagger/modules/rust-client-dev/testdata/external-consumer/Cargo.lock`
      through the Verify vendor layout after the dependency prune (247 → 217
      packages) so `--locked` verification resolves against what the workspace
      actually ships.
    - _Requirements: 6.3, 6.6_

- [x] 6. Checkpoint: runtime-integrity gate is green
  - Run from `sdk/rust`: `cargo fmt --all --check`; locked workspace check,
    test, clippy (warning-denied), rustdoc (warning-denied), and
    `cargo deny check`; then the engine-backed
    `dagger -m .dagger/modules/rust-client-dev api call build
    --platform=linux/amd64 verify` per `sdk/rust/docs/engine-integration.md`.
  - Require the dispatch cancellation orderings, provisioning executor
    assertions, property_16, the source-policy guards, the arity diagnostics,
    and the isolated consumer build to pass.

- [ ] 7. Engine-coordinated termination grace (open follow-up)
  - Carry a TERM-with-grace contract through `engineutil.ExecutionMetadata` so
    the engine sends a catchable termination signal and a grace period before
    SIGKILL; the existing Signal_Producer is the receiving end.
  - Blocked on engine-side work: at the pinned revision both proc killers in
    `engine/engineutil/executor.go` are SIGKILL-only. Do not simulate the
    behaviour or claim it from SDK-side tests.
  - _Requirements: 2.1-2.4_

## Notes

- Property identity lives in stable `property_NN_*` test names; production and
  test comments do not cite this specification, its feature number, or its
  tasks.
- The remediation landed as one reviewed pull request rather than per-task
  checkpoints; the engine-free gate ran on two hosts and the engine-backed
  Build/Verify ran against the pinned runner before merge.
- Task 7 is the only open work. When it lands, extend the dispatch cancellation
  tests with an engine-delivered TERM case rather than adding a second signal
  path.
