# Wrong Publish Set in graphs_tools Case (Won't Fix — Legacy Crate)

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** 📥 (Backlog)

## Goal

Document the known algorithmic bug in `willbe`'s publish graph that causes an incorrectly large publish set when running `willbe publish graphs_tools`. The bug produces ~20 packages in the publish plan where only ~4 are actually required, and is rooted in three interacting defects across `tool/graph.rs`, `entity/staleness.rs`, and `entity/package.rs`.

**⚠️ This task is knowledge-preservation only — `willbe` is a legacy crate and this bug WILL NOT be fixed.** The task exists to record the root cause for future workspace maintainers and to prevent costly re-investigation if the symptom surfaces again.

## In Scope

- This task file — as the authoritative root-cause record for the wrong publish set bug
- No code changes whatsoever

## Out of Scope

- **Fixing this bug** — `willbe` is a legacy crate kept only because no replacement exists yet. Any proper fix requires substantial refactoring of `tool/graph.rs`, `entity/staleness.rs`, and `entity/package.rs` with no long-term payoff. Work on `willbe` is limited to critical blocking defects only.
- Refactoring `tool/graph.rs`, `entity/staleness.rs`, or `entity/package.rs`
- Writing tests for the broken behavior
- Upgrading or improving the publish algorithm

## Requirements

- N/A — no code changes; this is knowledge-preservation only

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Confirm accuracy** — check that the defect locations in "Root Cause Analysis" still match the current source (line numbers drift as code changes)
2. **Update line references** if needed — adjust file:line citations only; do not change logic
3. **Walk validation checklist** — every item answers YES before closing

## Root Cause Analysis

Three interacting defects cause `willbe publish graphs_tools` to emit ~20 packages
instead of the ~4 actually required.

### Defect 1 — `graph::subgraph()` follows wrong DFS direction (`tool/graph.rs:212`)

`subgraph()` performs DFS from `graphs_tools` following **outgoing edges**.
Graph edges are built as `A → B` meaning "A depends on B", so following outgoing
edges from `graphs_tools` traverses all its **transitive dependencies** (~20 nodes),
not the packages that transitively depend on `graphs_tools`.

- **Expected:** include `graphs_tools` and packages that depend on it (reverse/incoming direction)
- **Actual:** include `graphs_tools` and all packages it depends on (forward/outgoing direction)

### Defect 2 — Cascade `continue 'main` bypasses `publish_need()` (`tool/graph.rs:323-330`)

```rust
'main: while let Some( n ) = dfs.next( &graph )
{
  for neighbor in graph.neighbors_directed( n, Outgoing )
  {
    if nodes.contains( &neighbor )
    {
      nodes.insert( n );
      continue 'main;  // ← SKIPS publish_need() entirely
    }
  }
  // Only reached when NO dependency is in nodes
  if publish_need( package, temp_path.clone(), workspace.target_directory() ).unwrap()
  {
    nodes.insert( n );
    packages_with_changes.insert( graph[ n ].clone() );
  }
}
```

When any dependency of package `n` is already in `nodes`, `n` is inserted
unconditionally — `publish_need()` is never called. This cascade causes packages
with no local changes (and whose current published version satisfies all dependents)
to be added to the publish set.

### Defect 3 — `publish_need()` uses the wrong oracle (`entity/package.rs:251`)

`publish_need()` asks: **"Is the local version of this package already on crates.io?"**

For `interval_adapter` with local version `0.29.0` (only `0.28.x` on crates.io), it
returns `true` ("yes, 0.29.0 needs publishing") even when no dependent actually
requires `0.29.0`. The correct question is: **"Does crates.io have a version
satisfying the dependency requirements of all dependents in the chain?"**

Because only `0.29.0` is checked against crates.io (returning not-found → needs
publish), the function misidentifies packages as requiring publication.

### Secondary — `detect_stale_dependencies()` BeingPublished over-trigger (`entity/staleness.rs:238`)

`detect_stale_dependencies()` marks **all** dependents of any being-published
package as stale regardless of version compatibility. If `former` is being
published, every package requiring `former` (even via a requirement satisfied by
the current crates.io version) gets marked stale. This secondary defect amplifies
the set computed by the main cascade.

### Summary

| # | Location | Defect | Effect |
|---|----------|--------|--------|
| 1 | `tool/graph.rs:212` | `subgraph()` DFS follows outgoing (deps) instead of incoming (dependents) | ~20 nodes instead of ~4 in subgraph |
| 2 | `tool/graph.rs:323` | `continue 'main` skips `publish_need()` for any package whose dependency is in nodes | Blindly cascades all transitive deps into publish set |
| 3 | `entity/package.rs:251` | `publish_need()` checks whether local version is on crates.io, not whether required version is available | Misidentifies unchanged packages as needing publication |
| 4 | `entity/staleness.rs:238` | `BeingPublished` marks all dependents stale regardless of version match | Amplifies publish set via staleness cascade |

## Acceptance Criteria

- This file accurately documents all four defect locations with file and approximate line references
- The "Won't Fix — Legacy Crate" framing is prominent in the title and Goal
- "Fixing this bug" is explicitly listed as the first Out of Scope item
- No source code files are modified

## Validation

### Checklist

Desired answer for every question is YES.

- [ ] V1 — Root cause documented: all four defects described with `file:line` references?
- [ ] V2 — Legacy disclaimer present: "Won't Fix — Legacy Crate" in title and Goal section?
- [ ] V3 — Out of scope explicit: "Fixing this bug" is the first Out of Scope bullet?
- [ ] V4 — No code changes: `git diff --stat` shows zero source file modifications?
- [ ] V5 — Line references current: cited line numbers match current source (checked after any code drift)?

## Outcomes

*(To be filled when task is acknowledged and closed.)*
