# Feature: Workspace Root Resolution

### Scope

**Purpose**: Provide reliable workspace root detection for Rust projects running across diverse execution contexts (cargo development, CI, installed binaries, Docker).
**Responsibility**: Resolve the workspace root path via a prioritized multi-strategy fallback chain and expose workspace-relative path construction and standard directory accessors.
**In Scope**: Workspace root detection via multi-strategy fallback, named single-strategy constructors, path construction relative to the root, standard directory accessors, and path normalization.
**Out of Scope**: File content reading, configuration parsing, secret loading, build system integration, git operations beyond workspace root detection.

### Design

The workspace handle is a lightweight, cloneable value wrapping one normalized absolute path. Resolution proceeds through six ordered strategies; the first to succeed wins. See `docs/pattern/001_workspace_resolution_fallback.md` for the full chain and the rationale for the priority ordering.

Path normalization strips trailing `/.` components automatically on construction. This prevents path comparison failures when cargo workspace metadata returns a path ending with `/.`, a regression that was triggered in dependent projects and fixed in task 022.

Standard directory accessors implement convention over configuration: the five named accessors (`config`, `data`, `logs`, `docs`, `tests`) return deterministic sub-paths relative to the workspace root without requiring any configuration file. Projects can rely on this convention to discover structure without explicit wiring.

Named single-strategy constructors expose each resolution strategy individually. Callers who know their execution context can bypass the full fallback chain by choosing the constructor that matches their context.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Source | `src/lib.rs` | `Workspace` struct, all creation methods, path methods, normalization |
| Test | `tests/workspace_tests.rs` | Core workspace creation, resolution, and path operations |
| Test | `tests/path_operations_comprehensive_tests.rs` | Path manipulation, normalization, and canonicalization |
| Test | `tests/path_normalization_tests.rs` | Workspace root path normalization (trailing components) |
| Test | `tests/cargo_integration_tests.rs` | Integration with cargo workspace and metadata |
| Test | `tests/cross_platform_compatibility_tests.rs` | Platform-specific path handling |
| Task | `task/completed/001_cargo_integration.md` | Cargo auto-detection implementation |
| Task | `task/completed/022_fix_workspace_root_path_normalization.md` | Path normalization bug fix |
| Task | `task/completed/023_extend_workspace_resolution_for_installed_applications.md` | Extended fallback chain |
| Doc | `docs/pattern/001_workspace_resolution_fallback.md` | Fallback chain strategy and rationale |
| Doc | `docs/api/001_workspace.md` | Full public API surface |
