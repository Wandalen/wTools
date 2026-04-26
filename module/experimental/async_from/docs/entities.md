# Doc Entities

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `api/` | Documents each public trait's operations, error handling, and compatibility guarantees | [api/readme.md](api/readme.md) | 4 |
| `feature/` | Defines feature scope, design rationale, and cross-references for each implemented feature | [feature/readme.md](feature/readme.md) | 2 |
| `invariant/` | Document each invariant with a precise statement, enforcement mechanism, and consequences of violation | [invariant/readme.md](invariant/readme.md) | 2 |
| `pattern/` | Defines the problem, solution, applicability, and consequences of each pattern applied in this crate's design | [pattern/readme.md](pattern/readme.md) | 1 |

## Master Doc Instances Table

| Entity | ID | Name | File |
|--------|-----|------|------|
| api | 001 | AsyncFrom | [api/001_async_from.md](api/001_async_from.md) |
| api | 002 | AsyncInto | [api/002_async_into.md](api/002_async_into.md) |
| api | 003 | AsyncTryFrom | [api/003_async_try_from.md](api/003_async_try_from.md) |
| api | 004 | AsyncTryInto | [api/004_async_try_into.md](api/004_async_try_into.md) |
| feature | 001 | Infallible Async Conversion | [feature/001_infallible_async_conversion.md](feature/001_infallible_async_conversion.md) |
| feature | 002 | Fallible Async Conversion | [feature/002_fallible_async_conversion.md](feature/002_fallible_async_conversion.md) |
| invariant | 001 | Send Bounds on Async Conversions | [invariant/001_send_bounds.md](invariant/001_send_bounds.md) |
| invariant | 002 | Blanket Impl Non-Conflict | [invariant/002_blanket_impl_chain.md](invariant/002_blanket_impl_chain.md) |
| pattern | 001 | Std Mirror Pattern | [pattern/001_std_mirror_pattern.md](pattern/001_std_mirror_pattern.md) |
