# Rust SDK engine integration

This document is the maintainer contract for building, exercising, and auditing the
built-in Rust SDK. It complements [ARCHITECTURE.md](ARCHITECTURE.md): that document
describes the public client, while this one describes the private engine-packaged
compiler, adapter, runtime image, and exact-target evidence workflow.

## Runtime build audit

The Rust build was reviewed against the definitive Go SDK runtime and representative
module-backed SDKs before the exact-target matrix was closed. The comparison is about
observable build hygiene, not copying another language's implementation shape.

| Contract | Reviewed source | Rust decision |
| --- | --- | --- |
| Current and legacy generation are distinct | `core/sdk/go_sdk.go`, `sdk/python/runtime/main.go`, and `sdk/typescript/runtime/main.go` | A missing introspection file selects checked committed generation; a present file selects private legacy generation. Neither mode silently falls through to the other. |
| Dependency selection is lock-aware | Go's build path, Python's locked `uv sync`, and the TypeScript package-manager paths | Runtime verification requires `Cargo.lock`, checks it with `cargo metadata --locked`, and builds with `cargo build --locked`. The generated protocol binary also declares every crate it names directly. |
| Mutable inputs arrive late | Go's scoped module mount and Python's late source mount | Schema, project, and request inputs are mounted after the immutable tool and policy layers. Engine content and toolchain caches therefore do not depend on test-only or unrelated SDK source. |
| Build caches are not runtime state | Go removes module/build cache mounts; Java promotes one JAR into a fresh JRE image | Cargo registry, Git, and target caches exist only on the builder. The final image starts from a fresh digest-pinned distroless base and receives only the stripped binary and canonical provenance. |
| Credentials cannot become artifacts | Go brackets the build with SSH socket attachment/removal | Rust control JSON, provenance, cache keys, and generated files contain no credentials. Unsafe Cargo output is replaced by a bounded typed diagnostic, and no build socket, secret, Cargo home, or cache is copied into the runtime image. Ambient credential forwarding is deliberately not inferred from another SDK. |
| Runtime shape is explicit | Go and Java set a fixed entrypoint and workdir | Rust clears inherited default arguments, installs one fixed entrypoint, and uses `/scratch`, matching the engine's `core.RuntimeWorkdirPath`. |
| Platform and executable are engine-owned | Go selects its fixed runtime output; Java promotes the resolved application JAR | Rust selects only the engine platform's reviewed target triple and the manifest-owned `dagger-module` binary. Callers cannot supply a command, target directory, executable, or workdir. |

`toolchains/rust-sdk-dev/internal/enginefree` keeps the source-level audit executable.
If one of the reviewed SDKs changes its build contract, that test directs maintainers
back to this table so the Rust decision is reconsidered rather than drifting silently.

## Four-stage development workflow

Run these commands from the repository root. Ordinary development should use one case;
the complete matrix is reserved for feature-end or release evidence.

```console
./bin/dagger api call -m toolchains/rust-sdk-dev engine-unit
./bin/dagger api call -m toolchains/rust-sdk-dev engine-content manifest-digest
./bin/dagger api call -m toolchains/rust-sdk-dev engine-content engine-integration --cases operations
./bin/dagger api call -m toolchains/rust-sdk-dev engine-content engine-evidence
```

- `engine-unit` runs Rust engine-tool, completeness-boundary, Go adapter, and focused
  engine/source graph tests without constructing an engine.
- `engine-content` builds one target-bound OCI content object. Its manifest and
  descriptor digests are evidence coordinates, not a promise that another runner can
  recover the object's bytes.
- `engine-integration` accepts only the documented closed case names. Multiple cases
  fan out from the same retained content object inside one Dagger graph.
- `engine-evidence` requires every positive and negative case to pass before it emits
  an observation. A failure, skip, unknown case, wrong digest, or incomplete set is an
  atomic rejection.

The closed case inventory is:

| Case | Boundary proved |
| --- | --- |
| `resolution` | Canonical built-in selection, idempotent installation, and pre-fallback shorthand rejection |
| `init-empty` | New Cargo package, lockfile, toolchain, starter source, and checked generation |
| `init-existing` | Semantic Cargo adoption and byte-preservation of caller-owned source |
| `init-no-generate` | Initialization without accidental generated publication |
| `operations` | Library, module, client, and entrypoint hook outputs from the engine-visible schema |
| `runtime-checked` | Checked-generation runtime registration plus overlapping scalar calls |
| `runtime-legacy` | Private legacy regeneration, registration, invocation, and unchanged host source |
| `negative-generated-lock-toolchain` | Missing generation, stale lockfile, and incompatible toolchain rejection |
| `negative-path-ownership` | Lexical escape, symlink escape, and unknown generated-file ownership rejection |
| `negative-redaction` | Credential-bearing immutable-dependency rejection without secret rendering |

The checked target is declared in `sdk/rust/completeness/target.json`. The packaged
dependency descriptor may select the canonical crates.io release or a credential-free
fork at a full immutable Git revision. Refresh the descriptor, target, schema snapshot,
runtime policy, and generated bindings together; a mixed target must fail before Cargo
runs.

For an unpublished development dependency, add
`--engine-repository <credential-free-fork-url>` before `engine-content`. The builder
resolves that repository to one full immutable revision and records it in the packaged
descriptor. Omit the option only when the canonical registry dependency is actually
published; a mutable branch name is never evidence.

## Integration fixture preflight

Do not use an engine run to discover fixture semantics. Before invoking
`engine-integration`, write down and review the complete transition for the selected
case:

1. List the initial workspace/module files and both configuration formats.
2. Record every command's mutation, workspace cwd, module-source root, output root,
   generation mode, and runtime-load precondition.
3. Ground each transition in the corresponding core or established SDK fixture. In
   particular, inspect checked-versus-legacy generation and generator scoping.
4. Confirm that schema consumers receive an already-loadable module. A checked module
   cannot supply its runtime schema while simultaneously bootstrapping the committed
   bindings needed to load that runtime; use a separate stable schema fixture.
5. Assert exact commands, config anchors, paths, and forbidden sequencing in
   `toolchains/rust-sdk-dev/internal/enginefree`.
6. Inspect the Dagger graph for broad generator discovery, repeated content/engine
   construction, unrelated SDK source, and unbounded fan-out.
7. Run the engine-free audit before the one focused case:

   ```console
   cd toolchains/rust-sdk-dev
   go test ./internal/enginefree -count=1
   ```

Only a fixture that passes this review proceeds to an engine-backed run. A failed
focused case returns to the preflight model first; repeatedly changing the fixture and
using a multi-minute engine build as the next assertion is not the development loop.

## Local triage

Start with `engine-unit`. For an integration failure, rerun only its named case and
inspect the stable Rust diagnostic code and coordinate. Repair generated ownership with
`dagger generate`; do not delete caller-authored Cargo, source, VCS, or workspace files.
Missing or stale locks are repaired by generation, never by an unlocked runtime build.

The private protocol probe has one registration branch and one scalar invocation. It
proves the nested-session boundary only; it is not a public module authoring API. The
standalone client renderer likewise proves the engine hook without claiming complete
client content.

Before evidence or release review, inspect the runtime image for exactly the installed
binary and provenance additions, confirm that Cargo homes/caches/source are absent, run
the repository Rust security workflow, regenerate scoped outputs twice, and require the
second render to be byte-clean. Evidence may close only the capability-local domains
declared in `completeness/engine-integration-mappings.json`; remaining sibling content
stays visible as a blocker.

The development module's generated Go bindings have their own smaller generation
boundary. Preview and apply only that module source rather than invoking every workspace
generator:

```console
./bin/dagger api call -M -j module-source \
  --ref-string toolchains/rust-sdk-dev --require-kind LOCAL_SOURCE \
  generated-context-changeset modified-paths
./bin/dagger api call -M module-source \
  --ref-string toolchains/rust-sdk-dev --require-kind LOCAL_SOURCE \
  generated-context-changeset export --path .
```

The preview is expected to name only `toolchains/rust-sdk-dev/dagger.gen.go` and the
affected files beneath `toolchains/rust-sdk-dev/internal/dagger`. The unscoped
`dagger generate` command belongs to repository-wide generation and is not part of a
Rust engine-integration checkpoint.
