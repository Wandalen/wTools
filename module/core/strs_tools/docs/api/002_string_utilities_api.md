# API: String Utilities API

### Scope

- **Purpose**: Define the public operations and return types for text indentation, string isolation, number parsing, command parsing, and ANSI utility functions.
- **Responsibility**: Contracts the observable behaviour callers depend on for all string utility features outside the split and parser integration APIs.
- **In Scope**: Indentation, left/right/between isolation, number parsing, command parsing, ANSI detection, parsing, stripping, visual length, and truncation operations.
- **Out of Scope**: Split API (`api/001`); parser integration API (`api/003`); algorithm internals (`algorithm/`).

### Operations

**Indentation** accepts a source string, a prefix, and a postfix. It returns an owned string where every line of the source is wrapped by the prefix and postfix.

**Isolation** provides three operations — left, right, and between — each accepting a source string and one or two delimiter patterns. Each returns an optional string slice: present when the delimiter was found, absent otherwise. All three operations borrow from the source, returning slices rather than owned strings.

**Number parsing** accepts a string slice and a target numeric type. It returns the parsed numeric value or a typed error. The `string_parse_number` feature must be enabled; otherwise the standard library path is used.

**Command parsing** accepts a raw command string and returns a structured value holding the command name and an ordered list of argument strings.

**ANSI detection** accepts a string and returns a boolean indicating whether any ANSI escape sequences are present.

**ANSI parsing** accepts a string and yields tokens, alternating between ANSI escape sequences and visible text spans.

**ANSI stripping** accepts a string and returns an owned string with all ANSI sequences removed.

**Visual length** accepts a string and returns the number of terminal display columns it occupies, accounting for multi-byte characters.

**ANSI truncation** accepts a string and a target column width and returns the string trimmed to that width without splitting multi-byte characters or orphaning escape sequences.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/string/indentation.rs` | Indentation implementation |
| source | `src/string/isolate.rs` | Isolation implementation |
| source | `src/string/number.rs` | Number parsing wrapper |
| source | `src/string/parse_request.rs` | Command parsing implementation |
| source | `src/ansi/detect.rs` | ANSI sequence detection |
| source | `src/ansi/parse.rs` | ANSI token parsing |
| source | `src/ansi/strip.rs` | ANSI sequence stripping |
| source | `src/ansi/visual.rs` | Visual-width calculation |
| source | `src/ansi/truncate.rs` | Visual-width-aware truncation |
| doc | `docs/feature/002_text_indentation.md` | Indentation feature design |
| doc | `docs/feature/003_string_isolation.md` | Isolation feature design |
| doc | `docs/feature/004_number_parsing.md` | Number parsing feature design |
| doc | `docs/feature/005_command_parsing.md` | Command parsing feature design |
| doc | `docs/feature/006_ansi_utilities.md` | ANSI utilities feature design |
