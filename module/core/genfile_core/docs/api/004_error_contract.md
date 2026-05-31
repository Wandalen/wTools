# API: Error Contract

### Scope

- **Purpose**: Defines the typed error surface callers match against to handle genfile_core failures.
- **Responsibility**: Documents all public error variants and guidance for distinguishing user-fixable from system errors.
- **In Scope**: All public error variants; caller-facing matching guidance.
- **Out of Scope**: Error message formatting for CLI display (→ genfile crate), internal error propagation.

### Design

All genfile_core failure modes are exposed through a single typed error enum. The four variants are: render failure (template engine error — check template syntax), missing parameters (mandatory parameters unfilled before generation — user-fixable), filesystem I/O (OS-level error — system error), and invalid template (malformed template syntax — user-fixable). Callers match on variants to distinguish errors that the user can correct (missing parameters, invalid template) from errors requiring system investigation (render failure, filesystem I/O).

### Features

| File | Relationship |
|------|--------------|
| [feature/016_typed_errors.md](../feature/016_typed_errors.md) | Typed error variants documented here |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/006_error_message_quality.md](../invariant/006_error_message_quality.md) | Quality constraint governing error messages |

### Sources

| File | Relationship |
|------|--------------|
| `src/error.rs` | Typed error enum definition |
