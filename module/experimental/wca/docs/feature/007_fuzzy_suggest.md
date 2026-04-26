# Feature: Fuzzy Suggest

When a user types an unrecognized command name, the error message includes a suggestion for the closest matching registered command.

### Scope

- **Purpose**: Improves user experience by reducing friction from command name typos.
- **Responsibility**: Documents the feature gate, matching algorithm, and suggestion integration.
- **In Scope**: on_unknown_suggest feature flag, similarity matching, error message enrichment.
- **Out of Scope**: Normal command verification (see api/003), error taxonomy (see api/001).

### Design

This feature is gated behind the on_unknown_suggest feature flag in Cargo.toml, which activates the textdistance dependency. When disabled, unknown command errors show a generic message directing users to the dot-list command.

When enabled, the verifier computes string similarity between the unknown command name and all registered command names using the JaroWinkler algorithm. If the best match exceeds a threshold, the error message suggests the closest command.

The suggestion is appended to the existing validation error, preserving the error type structure. Callers see an enriched error message without any change to error handling logic.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/verifier/verifier.rs` | Conditional suggestion via cfg(feature) |
| config | `Cargo.toml` | on_unknown_suggest feature flag and textdistance dep |
| test | `examples/wca_suggest.rs` | Demonstration of suggest behavior |
| doc | [api/003_verifier.md](../api/003_verifier.md) | Verifier API where suggestion is injected |
