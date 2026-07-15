# Pitfall Test: Stream Merge Ordering

### Scope

- **Purpose**: Verify the mitigation documented in `docs/pitfall/002_stream_merge_ordering.md` holds — the stream ordering trap does not recur.
- **Responsibility**: Test spec proving `merge_streams` places stderr before stdout, not parameter-declaration order.
- **In Scope**: PF-1..PF-2 — direct two-stream ordering (the historical failure scenario) and ordering preserved under compound head+width processing.
- **Out of Scope**: Line filtering and width truncation mechanics themselves — see `tests/docs/feature/001_output_processing.md`.

### PF-1: stderr precedes stdout in the merged result — the historical failure scenario

- **Given:** non-empty stdout and non-empty stderr content, `StreamFilter::Both`
- **When:** streams are merged
- **Then:** the result begins with stderr content, followed by stdout — not parameter-declaration order
- **Note:** Regression guard for BUG-006 — stream merging previously placed stdout before stderr, burying error text.

### PF-2: stderr-first ordering holds under compound head+width processing

- **Given:** non-empty stdout and stderr content, combined with active head-limit and width-limit configuration
- **When:** `process_output` applies stream selection, then head filtering, then width truncation
- **Then:** the processed output still begins with stderr content — the ordering guarantee survives composition with other pipeline stages, not just the isolated two-stream case
- **Note:** Order assertion, not just content-presence assertion — the specific class of test the pitfall's Mitigation requires.

### Pitfalls

| File | Relationship |
|------|-------------|
| [`../../../docs/pitfall/002_stream_merge_ordering.md`](../../../docs/pitfall/002_stream_merge_ordering.md) | Authoritative trap/failure/mitigation for this spec |

### Sources

| File | Relationship |
|------|-------------|
| `../../../src/output.rs` | `merge_streams` — stderr-first ordering under test |

### Tests

| File | Relationship |
|------|-------------|
| `../../../tests/output.rs` | PF-1: `merge_streams_ordering` (`bug_reproducer(BUG-006)`); PF-2: `combined_streams_head_width` |
