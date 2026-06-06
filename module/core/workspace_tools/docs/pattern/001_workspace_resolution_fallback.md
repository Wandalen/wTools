# Pattern: Workspace Resolution Fallback Chain

### Scope

- **Purpose**: Enable a library or tool to locate a workspace root reliably across all execution environments without requiring explicit configuration in each context.
- **Responsibility**: Define an ordered sequence of resolution strategies where each strategy targets one execution context; the first to succeed provides the workspace root.
- **In Scope**: Strategy ordering, per-strategy trigger conditions, named-constructor exposure of individual strategies, path deduplication via canonicalization.
- **Out of Scope**: Application-level path routing, git operations, build system integration, multi-workspace topologies.

### Problem

Rust tools and libraries are invoked from many execution environments — a developer running `cargo run` inside a workspace, a CI pipeline executing a binary from a checkout directory, an installed CLI tool invoked from an unrelated directory, or an application running inside a Docker container with a mapped volume. Each environment places the project files at a different path relative to the current working directory. A tool that hard-codes a resolution strategy (e.g., "look for Cargo.toml in ancestors of cwd") fails in environments where that assumption does not hold.

### Solution

Define a prioritized list of resolution strategies. Attempt each in order; return the first result that succeeds. Each strategy targets one class of execution environment:

| Priority | Strategy | Trigger Condition | Target Context |
|----------|----------|-------------------|----------------|
| 1 | Cargo workspace detection | `cargo_metadata` finds a workspace root | Cargo-managed development builds |
| 2 | `WORKSPACE_PATH` env var | Variable is set and non-empty | CI/CD pipelines, explicit override |
| 3 | Git root | `.git` directory + `Cargo.toml` found walking up from cwd | Git checkouts without cargo context |
| 4 | `$PRO` env var | Variable is set | Developer machines with project root convention |
| 5 | `$HOME` directory | Home directory is available | Installed tools with per-user config |
| 6 | Current working directory | Always available | Last-resort fallback |

Individual strategies are exposed as named single-strategy constructors for callers that know their context and want to skip the chain.

All resolved paths are normalized: trailing `/.` components are stripped, and deduplication uses canonicalized paths to avoid treating the same physical location as two distinct results.

### Applicability

Apply this pattern when:
- The library or tool must work in development, CI, and installed-binary contexts without per-environment configuration.
- The caller cannot always know the workspace root at compile time.
- Different fallback contexts have clearly distinct trigger conditions that do not overlap in normal use.

Do not apply when:
- The workspace root is always known and can be provided explicitly.
- Only a single execution context is supported.
- Fallback to `$HOME` or `cwd` would produce surprising results in the deployment environment.

### Consequences

**Benefits:**
- Resolves reliably across all supported contexts without requiring per-deployment configuration.
- Lower-priority strategies (`$PRO`, `$HOME`, cwd) provide graceful degradation instead of hard failures in unconfigured environments.
- Named constructors preserve testability and allow callers to opt out of the full chain.

**Trade-offs:**
- Priority ordering is fixed; callers cannot reorder strategies without using named constructors explicitly.
- The last-resort cwd fallback may silently resolve to an unexpected directory if no higher-priority strategy succeeds.
- Additional resolution strategies require a priority decision that affects behavior in all environments.

### Sources

| File | Relationship |
|------|-------------|
| [src/lib.rs](../../src/lib.rs) | `resolve()`, `resolve_with_extended_fallbacks()`, all `from_*()` methods |

### Tests

| File | Relationship |
|------|-------------|
| [tests/workspace_tests.rs](../../tests/workspace_tests.rs) | Resolution strategy correctness |
| [tests/task_022_installed_app_resolution.rs](../../tests/task_022_installed_app_resolution.rs) | Installed-app resolution (strategies 4–6) |
| [tests/test_fallback_integration.rs](../../tests/test_fallback_integration.rs) | Fallback chain integration |

### Tasks

| File | Relationship |
|------|-------------|
| [task/completed/023_extend_workspace_resolution_for_installed_applications.md](../../task/completed/023_extend_workspace_resolution_for_installed_applications.md) | Added strategies 4–6 |

### Features

| File | Relationship |
|------|-------------|
| [feature/001_workspace_root_resolution.md](../feature/001_workspace_root_resolution.md) | Feature scope and usage |

### APIs

| File | Relationship |
|------|-------------|
| [api/001_workspace.md](../api/001_workspace.md) | Named constructor signatures |
