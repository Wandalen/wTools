# Delete stale module/core/config_hierarchy stub to resolve workspace build conflict

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** 🎯 (Available)

## Goal

Delete the stale `module/core/config_hierarchy` v0.4.0 core stub and remove its workspace exclude entry so the structural conflict (workspace-inherited deps in an excluded crate) is eliminated and no directory under `module/core/` is permanently unbuildable. (Motivated: root_cause investigation confirmed the stub uses 13 `{workspace = true}` dep entries but is explicitly excluded from the workspace, making every `cargo build` or `cargo test` invocation from within it fail with "failed to find a workspace root"; Observable: `module/core/config_hierarchy/` deleted, `"module/core/config_hierarchy"` exclude entry removed from root `Cargo.toml`; Scoped: one directory deletion and one `Cargo.toml` edit; Testable: `ls module/core/config_hierarchy/` → "No such file or directory" and `cargo nextest run -p config_hierarchy --all-features` from workspace root exits 0.)

The core stub (`module/core/config_hierarchy`, v0.4.0) is a stale diverged copy of `module/experimental/config_hierarchy` (v0.5.0, workspace member). Five source files differ and four test files are missing from the stub. Both crates have the same package name, so they cannot coexist as workspace members. The active implementation is exclusively in experimental; the core stub provides no unique value and cannot be built or tested standalone.

The root `Cargo.toml` contains the pattern `"module/core/*"` as a members glob, with `"module/core/config_hierarchy"` listed as an explicit exclude override. After deleting the directory, the exclude entry becomes a dangling reference and should be removed to keep the workspace manifest clean.

## In Scope

- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/config_hierarchy/` — entire directory; delete completely
- `/home/user1/pro/lib/wip_core/wtools/dev/Cargo.toml` § exclude list — remove the comment + `"module/core/config_hierarchy"` line (lines 14–15)

## Out of Scope

- Documentation updates (already completed by doc_tsk)
- Changes to `module/experimental/config_hierarchy/` — the active crate is completely untouched
- Changes to other workspace members or workspace `[dependencies]`
- Any source code changes to the experimental crate or its tests

## Requirements

- All work must strictly adhere to all applicable rulebooks
  (discover via `kbase .rulebooks`)
- Verify no workspace crate `path`-depends on `module/core/config_hierarchy` before deleting
- Confirm `module/experimental/config_hierarchy` (workspace member, v0.5.0) continues to pass all 126 tests after deletion and `Cargo.toml` edit

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Verify no workspace dependents** — `grep -rn "module/core/config_hierarchy" /home/user1/pro/lib/wip_core/wtools/dev/Cargo.toml` → should show exactly the exclude entry and the comment, zero `path =` lines pointing to the core stub.
2. **Delete the core stub** — `rm -rf /home/user1/pro/lib/wip_core/wtools/dev/module/core/config_hierarchy/`
3. **Edit root Cargo.toml** — Remove both lines (comment + exclude entry):
   ```
     # Exclude module/core stub that still has Cargo.toml (active crate is in module/experimental)
     "module/core/config_hierarchy",
   ```
4. **Verify workspace integrity** — `cd /home/user1/pro/lib/wip_core/wtools/dev && cargo nextest run -p config_hierarchy --all-features` → 126 tests pass, 0 failures.
5. **Validate** — Run full verification per `w3 .test level::3` on `module/experimental/config_hierarchy/` → 126 nextest + 16 doc + 0 clippy.
6. **Walk Validation Checklist** — Check every item. Every answer must be YES.
7. **Update task status** — Mark 🎯 → ✅ in `.github/task/readme.md` Tasks Index; move file to `task/completed/`; write Outcomes.

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|----------------|-------------------|-------------------|
| `ls module/core/config_hierarchy/` after deletion | filesystem | Fails: "No such file or directory" |
| `grep "module/core/config_hierarchy" Cargo.toml` | root Cargo.toml | 0 matches after edit |
| `cargo nextest run -p config_hierarchy --all-features` from workspace root | experimental crate | 126 tests pass, 0 failures |
| `cargo metadata --no-deps 2>&1 | grep config_hierarchy` | workspace graph | Shows only `module/experimental/config_hierarchy` path |
| `cargo check -p config_hierarchy --all-features` | experimental crate compilation | Exits 0 with 0 warnings |

## Acceptance Criteria

- `module/core/config_hierarchy/` no longer exists on disk
- `grep -c "module/core/config_hierarchy" /home/user1/pro/lib/wip_core/wtools/dev/Cargo.toml` returns 0
- `cargo nextest run -p config_hierarchy --all-features` from workspace root passes 126 tests
- `w3 .test level::3` on `module/experimental/config_hierarchy/` exits 0 (126 nextest + 16 doc + 0 clippy)

## Validation

### Checklist

Desired answer for every question is YES.

**Deletion confirmation**
- [ ] Does `ls /home/user1/pro/lib/wip_core/wtools/dev/module/core/config_hierarchy/` fail with "No such file or directory"?
- [ ] Does `grep -c "module/core/config_hierarchy" /home/user1/pro/lib/wip_core/wtools/dev/Cargo.toml` return 0?

**Workspace integrity**
- [ ] Does `cargo nextest run -p config_hierarchy --all-features` pass 126/126 tests?
- [ ] Does `cargo check -p config_hierarchy --all-features` exit 0?

**Out of Scope confirmation**
- [ ] Is `module/experimental/config_hierarchy/` unchanged (Cargo.toml, src/, tests/ all unmodified)?
- [ ] Is no other crate's Cargo.toml changed?

### Measurements

**M1 — Workspace exclude references**
Command: `grep -c "module/core/config_hierarchy" /home/user1/pro/lib/wip_core/wtools/dev/Cargo.toml`
Before: 1 (the exclude entry). Expected: 0. Deviation: stale reference remains.

**M2 — Experimental crate test count unchanged**
Command: `cargo nextest run -p config_hierarchy --all-features 2>&1 | grep "tests run"`
Before: 126 tests run. Expected: 126 tests run. Deviation: any count change or failure.

### Invariants

- [ ] I1 — workspace nextest: `cargo nextest run -p config_hierarchy --all-features` → 126 passed, 0 failed
- [ ] I2 — workspace check: `cargo check -p config_hierarchy --all-features` → exit 0

### Anti-faking checks

**AF1 — Directory truly absent**
Check: `test -d /home/user1/pro/lib/wip_core/wtools/dev/module/core/config_hierarchy && echo EXISTS || echo ABSENT`
Expected: ABSENT. Why: confirms the directory was deleted, not just emptied or hidden.

**AF2 — Only experimental path in workspace deps**
Check: `grep "path.*config_hierarchy" /home/user1/pro/lib/wip_core/wtools/dev/Cargo.toml`
Expected: one line: `path = "module/experimental/config_hierarchy"`. Why: ensures only the active crate is referenced; no residual core stub path.

## Outcomes

[Empty — populated upon task completion]
