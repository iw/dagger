# Requirements Document: Rust SDK Runtime Integrity

## Introduction

Feature 10 captures the runtime-integrity contract delivered after Features 2–7
closed: the behaviours a running Rust SDK process must uphold between "the generated
API is correct" and "the process is a well-behaved citizen of its host". It was
specified retrospectively from the 2026-08 Rust SDK audit and its merged remediation
(pull request #72), which found the shipped runtime correct at the API layer but
carrying an inert cancellation scaffold, blocking filesystem work on the async
runtime thread, a query-document trace leak, blanket dead-code allows, and packaging
that shipped megabytes of example scaffolding.

Feature 10 owns five obligations: cooperative module cancellation, runtime-thread
scheduling safety, credential and query confidentiality in diagnostics, compiler-
audited liveness of production code, and packaging/lockfile coherence. Feature 2 owns
the session lifecycle these behaviours run inside; Feature 3 owns the provisioning
and transport pipeline whose scheduling this feature constrains; Feature 5 owns the
runtime container and engine adapter seam; Feature 6 owns the dispatcher whose
outcome model cancellation feeds; Feature 9 owns release assembly of the packaged
artifacts this feature keeps coherent.

The engine's own process supervision is an authority boundary, not something this
feature replaces: at the pinned target revision the engine's executor kills module
processes with SIGKILL only (`engine/engineutil/executor.go`, `newRunProcKiller` /
`newExecProcKiller`). The SDK therefore derives cancellation from the signals a
cooperating supervisor can send today (SIGTERM/SIGINT), and the engine-side
TERM-with-grace channel remains this feature's one open forward requirement.

If in doubt about a behaviour this document does not settle, the definitive Go SDK's
handling of the same concern is the reference evidence, filtered through Rust policy
for ownership, scheduling, and error idiom.

## Glossary

- **Blocking_Pool:** Tokio's dedicated blocking-thread pool, entered through
  `spawn_blocking`; the only place provisioning filesystem work may run.
- **Cache_Lock:** The advisory file lock guarding the shared CLI download cache,
  acquired through a cancellable try-lock poll using the standard library's file
  locking (stable since Rust 1.89).
- **Cancellation_Outcome:** The structured terminal result of a cancelled module
  call: `ErrorOutcomeKind::Cancellation` selected before publication and surfaced as
  `PublishedOutcome::CancelledError` through the single-assignment result sink.
- **Compiler_Audited_Liveness:** The discipline that production items kept alive only
  for tests carry `#[cfg_attr(not(test), expect(dead_code, reason = "..."))]` so the
  compiler fails the build when the justification stops being true, instead of a
  blanket `allow` that can hide genuinely dead code.
- **Module_Cancellation:** The shared cancellation token
  (`dagger-sdk/src/module/context.rs`) observed by the dispatcher and tripped by the
  Signal_Producer.
- **Redacting_Debug:** A hand-written `Debug` implementation that renders identity
  and shape (executable, argument count, environment key names, session allocation
  identity) while never rendering credential-bearing values.
- **Signal_Producer:** The process-level task (`dagger-sdk/src/module/adapter.rs`)
  that installs SIGTERM and SIGINT handlers for the duration of a module serve,
  trips Module_Cancellation when either arrives, and uninstalls on drop.
- **Termination_Grace:** The open engine-side contract in which the engine sends a
  catchable termination signal and a grace period before SIGKILL, expected to travel
  through `engineutil.ExecutionMetadata`.

## Target State

A Rust module process that receives SIGTERM or SIGINT stops its authored future,
publishes exactly one structured cancelled error to the engine, and exits cleanly; a
successful value is never published after cancellation is observed. CLI provisioning
never blocks the async runtime thread: downloads write through an async handle, and
extraction, cache publication, retention pruning, and lock acquisition run on the
Blocking_Pool or poll cooperatively. Diagnostics and traces never carry query
documents, secret values, or credential-bearing environment values. Every
production item that exists only for tests is compiler-audited. The published crates
package only what a consumer needs, and the external consumer fixture's lockfile
resolves against exactly the dependencies the workspace actually ships.

## Evidence From Current Code

- **Cancellation producer:** `crates/dagger-sdk/src/module/adapter.rs` installs
  `SignalProducer` around the serve loop; `ModuleCancellation::default()` is created
  per serve and handed to every call envelope.
- **Cancellation outcome:** `crates/dagger-sdk/src/module/dispatch.rs` selects one
  terminal `CallOutcome` before publication; a cancellation observed before the
  authored future completes drops that future and publishes
  `PublishedOutcome::CancelledError`. An early `is_cancelled` check refuses to start
  user work after cancellation.
- **Scheduling safety:** `crates/dagger-sdk/src/provision.rs` routes extraction,
  cache publication, and retention pruning through `spawn_blocking`; the cache lock
  is a cancellable poll over `std::fs::File::try_lock` distinguishing
  `TryLockError::WouldBlock` from `TryLockError::Error`. The fs4 dependency is gone.
- **Confidentiality:** `crates/dagger-sdk/src/launch.rs` implements Redacting_Debug
  for `CliLaunchProjection` (executable, argument count, environment key names
  only); `crates/dagger-sdk/tests/source_policy.rs` bans `tracing::` from library
  sources and bans planning-metadata comments from handwritten sources and tests;
  the query-document TRACE event was removed from `crates/dagger-sdk/src/query.rs`.
- **Liveness:** `crates/dagger-sdk/src/preflight.rs` and `launch.rs` carry reasoned
  `expect(dead_code)` attributes; the module code generator
  (`crates/dagger-codegen/src/module/metadata.rs`) binds candidate segments through
  slice patterns so its Result-arity diagnostic fires instead of an eager index
  panic, including for `std::io::Result<()>`-shaped returns and interface methods
  whose `Result` carries no type arguments.
- **Packaging:** `crates/dagger-sdk/Cargo.toml` packages
  `["/src/**", "/examples/**", "!/examples/*/app/**", "/README.md", "/LICENSE"]`;
  `crates/dagger-sdk-macros/` ships its LICENSE; the external consumer fixture
  lockfile at
  `.dagger/modules/rust-client-dev/testdata/external-consumer/Cargo.lock` is
  regenerated through the Verify vendor layout whenever workspace dependencies
  change.
- **Engine boundary:** `engine/engineutil/executor.go` documents both proc killers
  as SIGKILL-only at the pinned revision; no SDK-side handler can observe that kill.

## Runtime Integrity Contract Policy

### Cancellation Policy

Cancellation is cooperative and single-winner. The signal path is
SIGTERM/SIGINT → Signal_Producer → Module_Cancellation → dispatcher outcome
selection. Exactly one terminal outcome is published per call; cancellation cannot
rewrite an outcome the sink has already accepted, and a successful value is never
published after cancellation is observed. The producer's lifetime is scoped to the
serve: installation happens before the first call is read, and drop uninstalls the
handlers.

### Scheduling Policy

The async runtime thread executes no blocking filesystem or kernel-blocking lock
call. Work that must block runs on the Blocking_Pool; waits that must remain
cancellable poll cooperatively instead of parking a thread. A blocking helper whose
thread would outlive cancellation is a defect even when it eventually completes.

### Confidentiality Policy

Rendered diagnostics carry identity, never material: names of environment keys but
no values, counts of arguments but no argv, session allocation identity but no
token. Query documents never enter traces. The configured runner-token channel is
pinned by `property_16_cli_launch_projection_complete_collision_free` and reserved
keys cannot be silently overridden by ambient environment.

### Liveness and Packaging Policy

Dead-code suppression must be falsifiable: `expect` with a reason, scoped to
non-test builds, never a blanket `allow`. Published packages exclude example
application scaffolding; every packaged crate ships its license; the consumer
fixture lockfile is regenerated, not hand-edited.

## Requirements

### Requirement 1: Cooperative Module Cancellation

**User Story:** As an engine operator, I want a Rust module process to stop cleanly
when its supervisor asks, so that cancelled function calls release resources and
report a truthful terminal state instead of hanging or fabricating success.

#### Acceptance Criteria

1. WHEN a module serve begins, THE runtime SHALL install the Signal_Producer before
   reading the first call.
2. WHEN SIGTERM arrives during a serve, THE Signal_Producer SHALL trip
   Module_Cancellation.
3. WHEN SIGINT arrives during a serve, THE Signal_Producer SHALL trip
   Module_Cancellation.
4. WHEN cancellation is observed before the authored future completes, THE
   dispatcher SHALL drop that future and publish a structured cancelled error.
5. WHEN cancellation is observed before user work starts, THE dispatcher SHALL
   refuse to start the authored future.
6. WHEN a cancelled call publishes, THE published outcome SHALL be
   `PublishedOutcome::CancelledError`, never a successful value.
7. WHEN cancellation races result publication, THE result sink SHALL resolve the
   race through one deterministic terminal state.
8. WHEN the serve ends, THE Signal_Producer SHALL uninstall its handlers on drop.
9. THE cancellation error SHALL be structured and credential-safe; panic payloads
   and signal metadata are never rendered verbatim.

### Requirement 2: Engine-Coordinated Termination Grace (Open)

**User Story:** As an engine operator, I want the engine itself to grant a module a
catchable termination signal and a grace period before SIGKILL, so that cooperative
cancellation also covers engine-initiated aborts rather than only
supervisor-initiated ones.

#### Acceptance Criteria

1. THE SDK-side contract SHALL remain correct under today's SIGKILL-only engine:
   cooperative cancellation activates for any catchable signal and nothing depends
   on receiving one.
2. WHEN the engine grows a TERM-with-grace channel, THE integration SHALL carry the
   grace contract through `engineutil.ExecutionMetadata` rather than a hidden
   environment switch.
3. WHEN TERM-with-grace lands, THE existing Signal_Producer path SHALL serve as the
   receiving end without a second cancellation mechanism.
4. THE open status of this requirement SHALL be recorded honestly: no task in this
   specification claims the engine-side behaviour exists at the pinned revision.

### Requirement 3: Runtime-Thread Scheduling Safety

**User Story:** As an SDK user, I want provisioning and cache maintenance to never
stall the async runtime, so that connection setup stays responsive and cancellable
under contention.

#### Acceptance Criteria

1. WHEN a CLI archive is downloaded, THE download SHALL write through an async file
   handle on the runtime, not a blocking write on the runtime thread.
2. WHEN an archive is extracted, THE extraction SHALL run on the Blocking_Pool.
3. WHEN a CLI binary is published into the cache, THE publication SHALL run on the
   Blocking_Pool.
4. WHEN retention pruning runs, THE pruning SHALL run on the Blocking_Pool.
5. WHEN the Cache_Lock is acquired, THE acquisition SHALL be a cancellable try-lock
   poll distinguishing `WouldBlock` from real errors.
6. WHEN acquisition is cancelled, THE poll SHALL stop without leaving a detached
   thread parked on a kernel lock.
7. THE file locking implementation SHALL be the standard library's advisory locking;
   the fs4 dependency SHALL NOT return.

### Requirement 4: Credential and Query Confidentiality

**User Story:** As a security reviewer, I want diagnostics and traces to be safe to
share, so that a debug log or bug report never leaks tokens, secret values, or the
shape of proprietary pipelines.

#### Acceptance Criteria

1. THE library sources SHALL NOT emit `tracing::` events; the source-policy guard
   enforces the ban.
2. THE GraphQL query document SHALL NOT appear in any trace or diagnostic event.
3. WHEN `CliLaunchProjection` is rendered for Debug, THE output SHALL contain the
   executable, argument count, environment key names, and session allocation
   identity only.
4. THE configured runner-token channel SHALL remain the reserved key pinned by
   `property_16_cli_launch_projection_complete_collision_free`.
5. WHEN a diagnostic renders an environment mapping, THE rendering SHALL never
   include values.
6. THE handwritten sources and tests SHALL NOT embed planning metadata; the
   source-policy guard extends to test files.

### Requirement 5: Compiler-Audited Liveness and Diagnostic Totality

**User Story:** As a maintainer, I want the compiler to tell me when a "kept for
tests" justification stops being true and generator inputs to fail with named
diagnostics, so that dead code and panics cannot hide behind suppressions.

#### Acceptance Criteria

1. WHEN a production item exists only for tests, THE item SHALL carry
   `#[cfg_attr(not(test), expect(dead_code, reason = "..."))]` with a true reason.
2. THE workspace SHALL NOT reintroduce blanket `#[allow(dead_code)]` on production
   items.
3. WHEN the module code generator meets a `Result` return whose generics carry no
   type arguments, THE generator SHALL report its Result-arity diagnostic instead
   of panicking.
4. WHEN the generator meets `std::io::Result<()>`-shaped returns on exported
   functions or interface methods, THE generator SHALL report the same named
   diagnostic.
5. THE candidate-segment extraction SHALL bind through slice patterns so no eager
   index can precede its length check.

### Requirement 6: Packaging and Lockfile Coherence

**User Story:** As a downstream consumer, I want the packaged crates to contain
exactly what I need and the consumer fixture to prove it, so that installs stay
small and the fork's release story stays honest.

#### Acceptance Criteria

1. THE `dagger-sdk` package include set SHALL exclude example application
   scaffolding via `!/examples/*/app/**` while retaining example sources.
2. THE `dagger-sdk-macros` package SHALL ship its LICENSE.
3. WHEN workspace dependencies change, THE external consumer fixture lockfile SHALL
   be regenerated through the Verify vendor layout, never hand-edited.
4. THE crates SHALL remain upstream-publishable (`publish` not hard-locked to
   `false`) while nothing is published to crates.io from this fork.
5. THE changelog and release metadata SHALL reflect the repository-artifact release
   model rather than crates.io distribution.
6. THE dependencies retired with the legacy connector (derive_builder,
   platform-info, thiserror, which, graphql_client, fs4, eyre) SHALL NOT return
   without a design decision.

## Out of Scope

- Redesigning the dispatcher outcome model or the module ABI; Feature 6 owns them.
- The provisioning pipeline's transport behaviour and checksum verification;
  Feature 3 owns them, while this feature constrains their scheduling.
- Release assembly, artifact naming, and the Build/Verify entry points; Feature 9
  owns them.
- Engine-side implementation of Termination_Grace; Requirement 2 records the
  contract it must meet, not a commitment to implement the engine change here.

## Iteration and Feedback Notes

- This specification was written after the fact, from the audit and the merged
  remediation, at the operator's direction: the requirements and design of the
  post-Feature work deserved the same capture as Features 2–7.
- Real cancellation was an explicit owner decision ("provide real Module
  cancellation support in this effort") over merely deleting the inert scaffold;
  the deleted `ResultElection` and its dead election states are recorded in
  Feature 6's modernized design.
- The audit's runner-token finding was resolved against the audit: property_16
  pins the configured-token channel as intentional, so the remediation reserved the
  key rather than removing the channel.
- The engine TERM-with-grace follow-up was deliberately scoped out of the
  remediation to keep pull request #72 reviewable; it remains the one open
  requirement here.
