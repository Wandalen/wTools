# Algorithm: Single-Char Splitting

### Scope

- **Purpose**: Document the optimized split path for the common case of a single single-byte delimiter, which avoids the overhead of the general multi-delimiter machinery.
- **Responsibility**: Explains the single-char fast path approach, selection criteria, and throughput characteristics.
- **In Scope**: Delimiter width detection at configuration time, direct byte comparison loop, segment emission without automaton overhead.
- **Out of Scope**: Multi-delimiter search (`algorithm/001`); Boyer-Moore multi-char path (`algorithm/003`); public API (`api/001`).

### Design

When the split configuration contains exactly one delimiter that is a single byte, the iterator selects the single-char fast path instead of the general delimiter search machinery. Selection happens once at iterator construction time based on the delimiter set shape.

The fast path scans the source bytes with a direct comparison loop. Each byte is compared against the target byte. On a match, the current segment boundary is closed, the delimiter segment is emitted if delimiter preservation is enabled, and scanning resumes. No automaton is consulted and no pattern is compiled.

This path is substantially faster than the general path for short strings and densely-delimited inputs because it eliminates automaton transition overhead and minimises branch prediction pressure. The performance advantage is most pronounced when the delimiter is common in the input, producing many short segments.

The single-char path produces results identical to the general path for the same input. It is a transparent optimisation: the caller cannot distinguish it from any other code path.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/string/specialized.rs` | Single-char split specialization implementation |
| source | `src/string/split.rs` | Fast-path selection at iterator construction |
| doc | `docs/feature/001_string_splitting.md` | Split feature overview including algorithm selection |
| doc | `docs/algorithm/003_boyer_moore_splitting.md` | Multi-char single-delimiter counterpart |
