# Feature: Text Indentation

### Scope

- **Purpose**: Add consistent indentation to every line of a multi-line string using configurable prefix and postfix strings.
- **Responsibility**: Documents the indentation capability and links to its source, tests, and API contract.
- **In Scope**: Per-line prefix insertion, optional postfix appending, handling of empty lines.
- **Out of Scope**: String splitting (`feature/001`); API operation signatures (`api/002`).

### Design

The indentation operation takes a source multi-line string, a prefix string to prepend to every line, and a postfix string to append to every line. It iterates over lines and constructs the indented result.

Empty lines receive the prefix and postfix treatment identically to non-empty lines, ensuring consistent visual block structure regardless of content.

The operation is allocation-based: it constructs a new owned string. No borrowing variant is provided because the result necessarily differs from the source.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/string/indentation.rs` | Indentation implementation |
| test | `tests/inc/indentation_test.rs` | Indentation correctness tests |
| doc | `docs/api/002_string_utilities_api.md` | Indentation operation contract |
| doc | `docs/invariant/004_no_std_alloc_contract.md` | No-std compatibility guarantee for core operations |
