# API: Wrapper Types API

### Scope

- **Purpose**: Provide data-free strategy marker types that direct the compile-time fallback dispatch to the correct formatting approach.
- **Responsibility**: Documents the wrapper type API — the available markers, their roles, and the reference adapter types.
- **In Scope**: Strategy markers (display, debug, reference) and reference adapter types used in fallback chains.
- **Out of Scope**: The fallback chain mechanism itself (→ api/001), optional value wrapping internals.

### Abstract

Wrapper types are data-free marker types used as strategy arguments to the fallback conversion macro and interface. Each marker designates a formatting approach: format using the display interface, format using the debug interface, or format via a reference adapter. The markers carry no data and optimize away at compile time.

Reference adapters extend the chain mechanism to borrowed values: they wrap a reference to a value together with its strategy configuration, enabling the fallback mechanism to operate on references without extra allocation.

### Operations

**Display marker**: Directs formatting to use the display interface. Used as the primary or fallback strategy when the value implements the display interface.

**Debug marker**: Directs formatting to use the debug interface. Commonly used as a fallback when display is unavailable.

**Reference marker**: Directs formatting through the reference adapter path. Used when the value is borrowed and must be formatted without cloning.

**Reference adapters**: Internal wrappers used by the dispatch mechanism. One adapter per chain position (primary, first fallback, second fallback) implements formatted output for that level. Callers do not construct these directly — the conversion macro produces them automatically.

### Error Handling

No runtime errors. Incorrect marker-to-value combinations fail at compilation with a type error.

### Compatibility Guarantees

The three primary markers (display, debug, reference) are stable. The reference adapter types are public but their internal construction is handled by the macro. The adapter interface is stable.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/format/wrapper.rs` | Wrapper type module root |
| source | `src/format/wrapper/aref.rs` | Reference wrapper implementations |
| source | `src/format/wrapper/maybe_as.rs` | Optional value wrappers |
| source | `src/format/to_string/aref.rs` | ToString reference adapters |
| test | `tests/inc/to_string_with_fallback_test.rs` | Wrapper types exercised in fallback tests |
| doc | `docs/api/001_fallback_conversion_api.md` | Fallback conversion API that uses these markers |
| doc | `docs/pattern/001_fallback_chain.md` | Pattern explaining wrapper type role |
