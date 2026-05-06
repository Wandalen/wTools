# Invariant: Blocking Network

### Scope

- **Purpose**: Define the synchronous I/O constraint that all network operations block the calling thread.
- **Responsibility**: Documents the blocking network guarantee and its implications.
- **In Scope**: HTTP download behavior, timeout configuration, feature gating.
- **Out of Scope**: Archive storage (see `001_in_memory_storage.md`), API contracts (see `../api/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | download and download_crates_io implementation |
| test | `tests/crates_tools_tests.rs` | Network download integration tests |
| test | `tests/corner_cases_comprehensive.rs` | Network error edge case tests |
| doc | `../feature/001_archive_inspection.md` | Feature relying on network behavior |

### Invariant Statement

All network operations in `crates_tools` are synchronous and blocking. `download` and `download_crates_io` block the calling thread until the download completes or fails. No async/await interface exists.

The HTTP client is configured with:
- Read timeout: 5 seconds
- Write timeout: 5 seconds

After the timeout window the operation returns an error rather than hanging indefinitely. No automatic retry occurs; each call is a single attempt.

The `network` feature must be enabled for download methods to compile. When `network` is disabled, only `read` and `decode` are available.

### Enforcement Mechanism

Enforced by dependency selection: `ureq` is a synchronous HTTP client with no async API surface. The feature flag `network` gates the download methods entirely — no compile path exists to call them asynchronously.

### Violation Consequences

- **Thread blocking**: Calling `download` or `download_crates_io` from an async executor blocks the executor thread. Callers in async contexts must dispatch the call to a dedicated blocking thread rather than calling it directly from async code.
- **No retry**: Network failures (transient DNS, TCP reset, crates.io rate limiting) are not retried. Callers that need retry semantics must implement the loop themselves.
- **5-second limit**: Downloads that exceed 5 seconds (large crates on slow connections) return a timeout error. No configuration exists to extend this.
