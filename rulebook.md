# rulebook

Local, repository-specific overrides. Per CLAUDE.md § Rulebook Discovery : Local-First Mandate, read this before applying any global CLAUDE.md or `$GENAI` rulebook convention — this file takes precedence.

### Exceptions

- **Exception to `longrun.rulebook.md § Long-Run Execution : Breadth Selection`**: Full-workspace test execution as an *unattended or agent-driven* action is forbidden in this repo. This workspace has 48+ member crates (`module/core/*`, `module/experimental/*`); a bare `cargo test`/`cargo nextest run` with no scope, or `make ctest*`/`wtest*` with no `crate=`, has repeatedly exhausted local disk space via `target/` growth.

  This does **not** relax what Final Verification means: `verb/test` still runs the complete suite, unscoped — per `verb.rulebook.md § Verb Protocol : test` (VE009, Unified Test Interface) and its Forbidden Pattern VE018 ("test_only at a gate"), narrowing scope to satisfy a gate faster remains forbidden. What changes is *who* may trigger that full sweep and *when*, not what it tests.

  - **Mechanism**: `verb/test` (`/home/user1/pro/lib/yrd_core/wtools/dev/verb/test`) reads a confirmation from `/dev/tty` before running the full suite, with a 20s timeout. A non-interactive Bash call — including an agent's — has no controlling terminal, so `/dev/tty` fails to open and the script refuses (exit 1) in milliseconds, before any `cargo` invocation. There is no environment-variable or flag bypass.
  - **Agent behavior at a Final Verification gate**: attempt `verb/test` once, directly via Bash, exactly as CLAUDE.md's Direct Test Execution Mandate and `longrun.rulebook.md` already require. If it exits non-zero because confirmation was unavailable, STOP — do not fall back to `will .test level::3`, a bare `cargo nextest run --all-features`, or any other workspace-wide command; those are the same expensive operation by a different name and defeat the guard. Report to the user that Final Verification requires them to run `verb/test` themselves, in an interactive terminal, and wait for that to happen before treating the gate as cleared.
  - **Ordinary verification is unaffected**: scoped/filtered/per-crate runs (`verb/test_only <filter>` once it exists, `make ctest* crate=<name>`, `cargo nextest run -p <crate>`) remain the correct default for in-progress work and are never blocked.
  - **`make ctest1..5` / `wtest1..5`** (`/home/user1/pro/lib/yrd_core/wtools/dev/Makefile`) enforce this mechanically: `crate=<name>` is mandatory; omitting it is a hard `error:` exit rather than a silent full-workspace run.
  - **`test_all_crates.sh`** (`/home/user1/pro/lib/yrd_core/wtools/dev/test_all_crates.sh`) carries the same `/dev/tty` confirmation gate as `verb/test`, since its entire purpose is a full-workspace sweep.
  - **CI is unaffected**: `.github/workflows/workspace_push.yml` invokes `will .test <crate-path>/ dry:0 ...` once per crate via its own dynamic matrix — it never calls `verb/test` and runs on ephemeral runners, so none of the above applies there.
