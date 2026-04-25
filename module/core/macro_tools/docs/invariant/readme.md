# Invariant Doc Entity

### Scope
- **Purpose**: Document correctness properties that must hold regardless of how macro_tools is used.
- **Responsibility**: List all design invariants that callers and contributors must preserve.
- **In Scope**: Properties with defined enforcement mechanisms and violation consequences.
- **Out of Scope**: Behavioral features → feature/; implementation constraints visible only in src/.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Unified Versioning](001_unified_versioning.md) | All consumers share one version of syn, quote, and proc-macro2 | ✅ |
| 002 | [Span-Aware Errors](002_span_aware_errors.md) | All proc-macro errors carry source span information | ✅ |
