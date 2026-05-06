# Feature Doc Entity

### Scope

- **Purpose**: Document every user-facing async conversion capability so developers understand what the crate provides and why.
- **Responsibility**: Define scope, design rationale, and cross-references for each implemented feature; serve as the navigational hub from feature to source, tests, API, and related docs.
- **In Scope**: Feature scope definitions, design decisions, and cross-reference tables linking source files, test files, and related docs for each capability.
- **Out of Scope**: API method contracts (see `api/`), invariant proofs (see `invariant/`), architectural pattern descriptions (see `pattern/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Infallible Async Conversion](001_infallible_async_conversion.md) | AsyncFrom/AsyncInto trait pair for infallible conversions | ✅ |
| 002 | [Fallible Async Conversion](002_fallible_async_conversion.md) | AsyncTryFrom/AsyncTryInto trait pair for fallible conversions | ✅ |
