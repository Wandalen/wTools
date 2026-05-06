# Pattern: Layered Configuration

### Scope

- **Purpose**: Document the override precedence model that unifies programmatic and CLI configuration into a single coherent parameter set.
- **Responsibility**: Describes the three-layer stack — programmatic defaults, environment variables, CLI flags — and how each layer overrides the one below.
- **In Scope**: Override semantics for all configuration parameters; layer resolution order; merge rules for collection parameters versus scalar parameters.
- **Out of Scope**: Specific parameter definitions (→ `feature/005`); CLI flag names and types (→ `api/004`); builder field names (→ `api/001`).

### Context

`program_tools` exposes the same configuration surface through two channels: the builder API for programmatic use in tests and build scripts, and CLI flags for interactive and shell-script use. Both channels must converge on identical runtime behavior for identical parameter values, and a principled model is needed to resolve conflicts when both channels supply a value for the same parameter.

### Problem

Without an explicit override model, a value set in code and a flag supplied on the command line both apply to the same parameter with no defined winner. This produces unpredictable behavior when channels are combined, and makes the system difficult to reason about from either channel independently.

### Solution

Define three layers with explicit top-down precedence:

1. **CLI flags** — highest priority; values passed on the command line override all lower layers
2. **Environment variables** — middle priority; values from the process environment override programmatic defaults
3. **Programmatic defaults** — lowest priority; values set through the builder API serve as the baseline

At resolution time, the runner walks each parameter from layer 1 down, applying the first non-absent value it finds. The result is a single resolved parameter set used for execution.

**Scalar semantics**: For a scalar parameter, the highest-priority non-absent value wins. Lower layers are ignored once a winning value is found.

**Collection semantics**: For collection parameters (environment variable additions, feature flags), values from all layers are merged additively. Each layer contributes its entries; no layer replaces another's entries.

### Consequences

**Benefits**: The CLI is always "in charge" when invoked explicitly; programmatic code does not need to unset environment variables to get predictable defaults; environment variables allow CI configuration without modifying code; both channels are first-class citizens.

**Tradeoffs**: Debugging unexpected parameter values requires checking all three layers; collection merge semantics differ from scalar override semantics and must be documented per-parameter; the pattern requires the CLI parser to be aware of programmatic defaults.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `docs/feature/005_configuration_surface.md` | Complete parameter reference with per-parameter layer annotations |
| doc | `docs/api/001_builder_api.md` | Builder API anchoring the programmatic defaults layer |
| doc | `docs/api/004_cli_interface.md` | CLI flags anchoring the top layer of the stack |
