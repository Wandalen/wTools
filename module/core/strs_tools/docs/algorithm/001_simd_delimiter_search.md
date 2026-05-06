# Algorithm: SIMD Delimiter Search

### Scope

- **Purpose**: Document the hardware-accelerated delimiter search algorithm that locates byte positions using vectorized byte matching.
- **Responsibility**: Explains the SIMD delimiter search approach, selection criteria, and performance characteristics for maintainers and performance-conscious contributors.
- **In Scope**: Vectorized byte scanning, multi-delimiter dispatch via Aho-Corasick, memchr single-byte fast path, pattern compilation and caching.
- **Out of Scope**: Public API surface (`api/001`); SIMD feature flag and fallback guarantee (`invariant/003`, `feature/007`); single-char and Boyer-Moore paths (`algorithm/002`, `algorithm/003`).

### Design

The SIMD delimiter search path activates when the `simd` feature is enabled and the split configuration contains one or more delimiter patterns. The algorithm selects the appropriate sub-strategy based on delimiter count and character width.

For a single single-byte delimiter the implementation delegates to a byte-search library that uses hardware SIMD instructions to scan for the target byte at the width of the native SIMD register. On platforms with wide vector units this scans multiple bytes per cycle.

For multiple delimiters the implementation compiles the delimiter set into an Aho-Corasick automaton at the first invocation and caches the compiled automaton for subsequent searches over the same pattern set. The automaton transitions are stored in a compact representation that fits in cache lines, allowing the search to maintain high throughput even on inputs with many potential match positions.

The algorithm produces byte offset positions for each match. The split iterator consumes these offsets to emit segments without re-scanning the string.

### Sources

- `../../architecture.md` — SIMD Optimization section; dependency list and performance range migrated to this instance.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/simd.rs` | SIMD feature gating and library import |
| source | `src/string/split/simd.rs` | Vectorized search integration with the split iterator |
| doc | `docs/feature/007_simd_acceleration.md` | SIMD feature boundary and activation |
| doc | `docs/invariant/003_simd_fallback_contract.md` | Result equivalence guarantee between SIMD and scalar |
