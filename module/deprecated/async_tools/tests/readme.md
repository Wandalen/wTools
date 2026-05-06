# Tests

### Scope

Validates the async_tools facade crate — that all re-exported traits from the dependency crates are accessible under the async_tools namespace.

#### Responsibility Table

| File | Responsibility |
|------|----------------|
| `tests.rs` | Test harness entry point delegating to async_from test suite |
| `manual/readme.md` | Manual testing plan (not applicable for this facade crate) |

### Domain Map

| Domain | Test Location | What It Tests |
|--------|---------------|---------------|
| Facade re-export | `tests.rs` via `async_from/tests/inc/mod.rs` | All async conversion traits accessible via async_tools |

### Manual Testing

Not applicable — facade crate with no independent logic. All substantive tests live in `async_from/tests/`.
