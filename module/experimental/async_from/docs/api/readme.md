# API Doc Entity

### Scope

- **Purpose**: Document the public contract of `async_from` so consumers know what traits are available and under what conditions they apply.
- **Responsibility**: Define each public trait's operations, error handling, and compatibility guarantees as the authoritative reference for downstream crates.
- **In Scope**: AsyncFrom, AsyncInto, AsyncTryFrom, AsyncTryInto traits; their methods, bounds, and blanket impl behaviour.
- **Out of Scope**: Feature scope rationale (see `feature/`), internal async_trait boxing, and test fixture details.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [AsyncFrom](001_async_from.md) | Infallible async conversion from source type to implementing type | ✅ |
| 002 | [AsyncInto](002_async_into.md) | Infallible async conversion from implementing type to target type (blanket) | ✅ |
| 003 | [AsyncTryFrom](003_async_try_from.md) | Fallible async conversion from source type to implementing type | ✅ |
| 004 | [AsyncTryInto](004_async_try_into.md) | Fallible async conversion from implementing type to target type (blanket) | ✅ |
