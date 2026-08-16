# Design Document: Rust SDK Runtime Integrity

## Overview

Feature 10 hardens the running SDK process along five seams that Features 2–7
established but did not own end to end: what happens when the process is asked to
stop, which thread pays for filesystem work, what diagnostics may say, what the
compiler is allowed to stay silent about, and what the published packages contain.
Each seam already has shipped production code; this design records the decisions
that code embodies and the invariants a future editor must preserve.

The design deliberately adds no new subsystem. Cancellation rides the existing
adapter and dispatcher; scheduling safety rides the existing provisioning pipeline;
confidentiality rides the existing diagnostic types and source-policy guards;
liveness and packaging are workspace policy enforced by ordinary tests and Cargo
metadata. The one forward-looking element — engine-coordinated Termination_Grace —
is specified as the receiving contract the existing signal path already satisfies.

## Dependencies and Non-Goals

### Owning relationships

- Feature 2 owns the session lifecycle and close precedence that a cancelled serve
  still honours.
- Feature 3 owns provisioning's download, checksum, and cache semantics; Feature 10
  owns which thread executes each step and how the lock waits.
- Feature 5 owns the runtime container, engine adapter, and serve loop that host
  the Signal_Producer.
- Feature 6 owns the dispatcher, call envelope, and outcome publication model that
  cancellation feeds; its Property 22 (one publication winner) is the
  concurrency-side proof this design relies on.
- Feature 9 owns release assembly and the Build/Verify entry points that exercise
  packaging coherence.

### Construction rules

- No second cancellation mechanism: every stop path — supervisor signal today,
  engine Termination_Grace tomorrow — trips the same `ModuleCancellation` token.
- No blocking call on the async runtime thread inside provisioning; blocking work
  is named, moved to `spawn_blocking`, and kept small enough to reason about.
- No new dependencies for any of this: signals use Tokio's existing `signal`
  feature, locking uses the standard library, redaction is a hand-written `Debug`.

### Non-goals

- A general task-supervision or watchdog framework.
- Grace-period negotiation logic ahead of the engine contract existing.
- Log or trace infrastructure; the SDK stays `tracing`-free by policy.

## Components and Interfaces

### Signal producer (`dagger-sdk/src/module/adapter.rs`)

```rust
struct SignalProducer(Option<tokio::task::JoinHandle<()>>);

impl SignalProducer {
    fn install(cancellation: ModuleCancellation) -> Self;
}
```

`install` spawns one task that listens for SIGTERM and SIGINT (Unix) and trips the
shared token on the first signal observed. The serve loop constructs one
`ModuleCancellation` per serve, installs the producer before reading the first
call, and holds the guard for the serve's whole lifetime; `Drop` aborts the
listener so handlers never outlive the serve. The producer is deliberately
process-level: the engine runs one module process per invocation boundary, so a
process signal is a call-scoped cancellation in practice, and per-call tokens keep
sibling isolation intact if that ever changes.

### Cancellation token (`dagger-sdk/src/module/context.rs`)

`ModuleCancellation` wraps `Arc<CancellationState>` and exposes `cancel`,
`is_cancelled`, and an awaitable `cancelled()`. Clones share state; the dispatcher
and producer hold clones of the same per-serve token. The token carries no payload:
why the process is stopping is not the authored function's business, only that it
must stop.

### Dispatcher integration (`dagger-sdk/src/module/dispatch.rs`)

The dispatcher checks `is_cancelled` before starting user work, then races the
authored future against `cancellation.cancelled()` in one `tokio::select!`. A
cancellation win drops the authored future and publishes a structured cancelled
error (`ErrorOutcomeKind::Cancellation` → `PublishedOutcome::CancelledError`)
through the same single-assignment sink every other outcome uses. Feature 6's
design owns the full outcome model; the integrity constraint added here is that
the cancellation arm is not special: one terminal outcome, published once,
never rewritten.

### Provisioning scheduling (`dagger-sdk/src/provision.rs`)

Downloads stream through an async file handle. Extraction, cache publication, and
retention pruning are wrapped in `spawn_blocking` with join-error mapping that
distinguishes a panicked helper from an I/O failure. The cache lock is:

```rust
loop {
    match file.try_lock() {
        Ok(()) => break,
        Err(TryLockError::WouldBlock) => /* sleep briefly, re-check cancellation */,
        Err(TryLockError::Error(error)) => return Err(map(error)),
    }
}
```

The poll replaces a kernel-blocking `flock` whose `spawn_blocking` thread detached
on cancellation and could park a pool thread indefinitely. `WouldBlock` is the only
retry condition; every other error is surfaced typed. The sleep interval is short
enough for responsive cancellation and long enough not to spin.

### Redacting diagnostics (`dagger-sdk/src/launch.rs`, `preflight.rs`)

`CliLaunchProjection`'s Debug renders executable, `argument_count`,
`environment_keys` (key names only), and session allocation identity. The builder
type renders presence, not contents. `property_16_cli_launch_projection_complete_collision_free`
pins the projection as complete and collision-free, including the reserved
runner-token key: ambient environment cannot silently override the configured
token channel.

### Source-policy guards (`dagger-sdk/tests/source_policy.rs`)

Two guards carry this feature's confidentiality and identity policy: the
`tracing::` ban over library sources (the query-document TRACE leak cannot
return), and the planning-metadata ban over handwritten sources *and tests*
(specification features, task numbers, and planning phases stay out of the code;
`property_NN_*` test names remain the stable identity).

### Generator diagnostic totality (`dagger-codegen/src/module/metadata.rs`)

Candidate path segments bind through slice patterns; the Result-arity diagnostic
names the arity requirement and fires for `Result` generics with no type
arguments and for `std::io::Result<()>`-shaped returns, on exported functions and
interface methods alike. The pattern binding makes the eager-index panic class
unrepresentable rather than merely fixed.

### Packaging surface (`crates/*/Cargo.toml`)

`dagger-sdk` packages `/src/**`, `/examples/**` minus `/examples/*/app/**`,
README, and LICENSE. `dagger-sdk-macros` ships its LICENSE. The external consumer
fixture (`.dagger/modules/rust-client-dev/testdata/external-consumer/`) resolves
its lockfile through the Verify vendor layout, so a dependency the workspace no
longer ships fails Verify instead of lingering in a stale pin.

## Data Models and Invariants

- One `ModuleCancellation` per serve; clones share it; no global cancellation
  state.
- One `SignalProducer` per serve; its handle is aborted on drop.
- One terminal `CallOutcome` per call; `CancelledError` is a first-class published
  kind, not an absence of publication.
- Blocking work returns through `spawn_blocking` join results; a join error maps
  to a typed diagnostic, never a panic across the boundary.
- The lock poll's cancellation check happens between attempts; no attempt parks.

## Correctness Properties

### Property 1: Signals produce exactly one cooperative cancellation

*For any* serve during which SIGTERM or SIGINT arrives, the shared token SHALL
report cancelled, every in-flight call SHALL publish `CancelledError` or its
already-accepted outcome, no successful value SHALL be published after the token
trips, and dropping the producer SHALL uninstall the handlers.

**Validates: Requirements 1.1–1.9**

### Property 2: Lock acquisition is cancellable and non-parking

*For any* contention schedule over the cache lock, acquisition SHALL either
complete, surface a typed error, or stop at the next poll boundary when
cancelled; no schedule SHALL leave a detached thread holding or awaiting the
kernel lock.

**Validates: Requirements 3.5, 3.6**

### Property 3: Provisioning stages run on the declared executor

*For any* provisioning run, extraction, cache publication, and retention pruning
SHALL execute on the Blocking_Pool, downloads SHALL write through the async
handle, and no stage SHALL issue a blocking filesystem call from the runtime
thread.

**Validates: Requirements 3.1–3.4**

### Property 4: Rendered diagnostics carry identity, never material

*For any* launch projection and environment mapping, Debug output SHALL contain
key names, counts, and allocation identity only; generated values, tokens, and
query documents SHALL never appear, and the reserved runner-token key SHALL
resolve to the configured channel.

**Validates: Requirements 4.2–4.5**

### Property 5: Generator inputs fail with named diagnostics

*For any* exported function or interface method whose return type is a `Result`
with missing or `std::io::Result<()>`-shaped generics, generation SHALL produce
the Result-arity diagnostic at the authored coordinate and SHALL NOT panic.

**Validates: Requirements 5.3–5.5**

## Error Handling

| Condition | Behaviour |
| --- | --- |
| Signal arrives before first call | Serve observes cancellation and publishes no successful value |
| Cancellation during user future | Future dropped; structured cancelled error published once |
| Cancellation after sink accepted an outcome | Accepted outcome stands; no rewrite |
| Lock poll meets `WouldBlock` | Brief sleep, cancellation check, retry |
| Lock poll meets a real error | Typed provisioning error; no retry |
| Blocking helper panics | Join error mapped to a typed diagnostic |
| Generator meets malformed Result arity | Named diagnostic at the authored coordinate |

## Testing Strategy

- Dispatch tests drive cancellation through fake sinks across orderings: cancel
  before start, cancel mid-future, cancel racing publication, cancel after
  acceptance. Feature 6's loom-modelled publication races remain the
  concurrency-depth proof; these tests pin the cancellation arm's mapping.
- Provisioning tests exercise the try-lock poll under contention and cancellation
  and assert the executor of each stage.
- `launch_tests.rs` holds property_16 and the redaction assertions;
  `source_policy.rs` holds the `tracing::` and planning-metadata guards.
- `metadata.rs` unit tests cover the arity diagnostics, including the
  `std::io::Result<()>` and empty-generics shapes.
- Packaging is proven by offline `cargo package` verification and the Verify
  entry point's isolated consumer build (`ENGINE_INTEGRATION.md`).

## Iteration and Feedback Notes

- The audit found `ModuleCancellation` inert (nothing produced a cancellation) and
  `ResultElection` dead; the owner chose real cancellation over deletion, and the
  outcome model absorbed the election's single-winner guarantee without its state
  machine.
- The try-lock poll exists because the previous flock helper's thread detached on
  cancellation — correct-looking code whose failure mode was a parked pool thread
  under contention.
- Redaction preferred a complete hand-written projection over field-by-field
  masking so that adding a credential-bearing field without a redaction decision
  fails property_16's completeness check.
- SIGKILL cannot be observed, so the open Termination_Grace requirement is the
  engine's to enable; the SDK side is already the correct receiver.
