# Feature Doc Entity

## Scope

- **In Scope**: Feature doc instances describing behavioral contracts for `optimize_split!` and `optimize_match!` macros.
- **Out of Scope**: API call syntax (see `api/`); optimization thresholds (see `invariant/`).
- **Boundary**: Feature instances describe what each macro does; API instances describe how to call them.
- **Status**: Active.

### Overview Table

| # | File | Status | Responsibility |
|---|------|--------|----------------|
| 1 | `001_compile_time_split.md` | ✅ Implemented | Compile-time string splitting via `optimize_split!` |
| 2 | `002_compile_time_match.md` | ✅ Implemented | Compile-time string matching via `optimize_match!` |
