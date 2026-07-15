# Pitfall: Stream Merge Ordering

### Scope

- **Purpose**: Document the stdout/stderr ordering trap discovered in stream merging.
- **Responsibility**: Trap, failure mode, and mitigation for merging streams in declaration order instead of stderr-first order.
- **In Scope**: `merge_streams`'s stdout/stderr concatenation order.
- **Out of Scope**: Line filtering and width truncation — see pitfall 001 and `feature/001_output_processing.md`.

### Trap

Concatenating stdout and stderr in parameter-declaration order (stdout, then stderr) when merging streams, since that ordering looks natural and matches typical function-signature order.

### Failure

Error output (stderr) ends up appended after normal output (stdout) in the merged result, burying error text below potentially long stdout content instead of surfacing it first. The defect is invisible to tests that only assert both streams' content is *present* in the merged result — only a test asserting relative *order* (e.g., that the result starts with stderr content) detects it.

### Mitigation

When merging stdout and stderr, always place stderr before stdout. Any test exercising stream merging with both streams non-empty must assert order explicitly, not just content presence.

### Features

| File | Relationship |
|------|-------------|
| [`../feature/001_output_processing.md`](../feature/001_output_processing.md) | Stream selection stage this pitfall applies to |

### Sources

| File | Relationship |
|------|-------------|
| `src/output.rs` | `merge_streams` — stderr-first ordering enforcing this mitigation |

### Tests

| File | Relationship |
|------|-------------|
| [`../../tests/docs/pitfall/002_stream_merge_ordering.md`](../../tests/docs/pitfall/002_stream_merge_ordering.md) | Test specification verifying this mitigation holds |
| `tests/output.rs` | Stream ordering regression test (`bug_reproducer(BUG-006)`) |
