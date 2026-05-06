# Pattern: Progressive API Disclosure

### Scope

- **Purpose**: Reduces cognitive load for common use cases by exposing complexity proportional to caller need.
- **Responsibility**: Documents the three-tier public API design and its layering rationale.
- **In Scope**: API tier definitions, layering rationale, tradeoffs of thin wrapper functions.
- **Out of Scope**: Builder implementation internals (→ source comments), validation semantics (→ source).

### Problem

A builder-only API forces all callers to learn the full configuration surface even when their use case is simple text collection with no options. This creates an unnecessary complexity barrier for the majority of callers.

### Solution

Layer the public API into three tiers of increasing complexity, each building on the tier below:

1. **Zero-config tier** — single function call; no imports or configuration required; covers the common case
2. **Validated tier** — adds a validation callback; covers most non-trivial cases with minimal added surface
3. **Full builder tier** — exposes all configuration parameters for complete customization

Each tier is a thin wrapper over the builder; no configuration logic is duplicated.

### Applicability

- A capability serves callers ranging from simple to power-user
- Builder complexity would deter adoption for common cases
- The configuration space is well-understood and stable

### Consequences

- Common case requires one function call with no setup
- Each tier is independently documentable and testable
- Builder remains the single source of configuration logic — no duplication of business logic
- Wrapper functions introduce minimal code repetition (two wrapper functions), accepted as a justified tradeoff for usability
- Three public entry points require three test surface paths for complete API coverage

### Cross-References

| Type   | File                           | Responsibility                                                   |
|--------|--------------------------------|------------------------------------------------------------------|
| source | `src/lib.rs`                   | Zero-config and validated tier (`collect`, `collect_validated`)  |
| source | `src/builder.rs`               | Full builder tier and shared configuration logic                 |
| test   | `tests/api_surface_test.rs`    | API surface coverage across all three public entry points        |
| test   | `tests/builder_config_test.rs` | Builder configuration options and all validation scenarios       |
| doc    | `docs/feature/001_multiline_input.md` | Feature that applies this pattern                         |

### Sources

| File                        | Notes                                                                                   |
|-----------------------------|-----------------------------------------------------------------------------------------|
| [../architecture.md](../architecture.md) | Combined source covering four patterns; concepts 1, 2, 4 extracted to 001, 002, 004 |
