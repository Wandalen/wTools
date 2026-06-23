# API: String Utilities API

### Scope

- **Purpose**: Define the public operations and return types for text indentation, string isolation, number parsing, command parsing, and ANSI utility functions.
- **Responsibility**: Contracts the observable behaviour callers depend on for all string utility features outside the split and parser integration APIs.
- **In Scope**: Indentation, left/right isolation, number parsing, command parsing, ANSI detection, parsing, stripping, visual length (char count), visual width (display columns), and truncation operations. Between isolation is a planned extension not yet implemented.
- **Out of Scope**: Split API (`api/001`); parser integration API (`api/003`); algorithm internals (`algorithm/`).

### Operations

**Indentation** accepts a source string, a prefix, and a postfix. It returns an owned string where every line of the source is wrapped by the prefix and postfix.

**Isolation** provides two operations — left and right — each accepting a source string and a delimiter pattern. Each returns a 3-tuple of string slices: the content before, the delimiter (if found), and the content after. Both operations borrow from the source. Between isolation (using two different delimiter patterns) is a planned extension not yet implemented.

**Number parsing** accepts a string slice and a target numeric type. It returns the parsed numeric value or a typed error. The `string_parse_number` feature must be enabled; otherwise the standard library path is used.

**Command parsing** accepts a raw command string and returns a structured value holding the command name and an ordered list of argument strings.

**ANSI detection** accepts a string and returns a boolean indicating whether any ANSI escape sequences are present.

**ANSI parsing** accepts a string and yields tokens, alternating between ANSI escape sequences and visible text spans.

**ANSI stripping** accepts a string and returns an owned string with all ANSI sequences removed.

**Visual length** accepts a string and returns the number of visible characters (Unicode codepoints) it contains after stripping ANSI escape sequences. This is a char count, not a display-column count. The grapheme-aware variant counts grapheme clusters instead of codepoints.

**Visual width** accepts a string and returns the number of terminal display columns it occupies after stripping ANSI escape sequences. Wide characters (CJK, emoji) contribute 2 columns; combining marks contribute 0. The grapheme-aware variant processes grapheme clusters rather than individual codepoints. Requires the `ansi` feature; the grapheme-aware variant additionally requires `ansi_unicode`.

**ANSI truncation** accepts a string and a target column width and returns the string trimmed to that width without splitting multi-byte characters or orphaning escape sequences.

### Sources

- [src/string/indentation.rs](../../src/string/indentation.rs) — Indentation implementation
- [src/string/isolate.rs](../../src/string/isolate.rs) — Isolation implementation
- [src/string/number.rs](../../src/string/number.rs) — Number parsing wrapper
- [src/string/parse_request/mod.rs](../../src/string/parse_request/mod.rs) — Command parsing implementation
- [src/ansi/detect.rs](../../src/ansi/detect.rs) — ANSI sequence detection
- [src/ansi/parse.rs](../../src/ansi/parse.rs) — ANSI token parsing
- [src/ansi/strip.rs](../../src/ansi/strip.rs) — ANSI sequence stripping
- [src/ansi/visual.rs](../../src/ansi/visual.rs) — Visual-width calculation
- [src/ansi/truncate.rs](../../src/ansi/truncate.rs) — Visual-width-aware truncation

### Features

- [002_text_indentation.md](../feature/002_text_indentation.md) — Indentation feature design
- [003_string_isolation.md](../feature/003_string_isolation.md) — Isolation feature design
- [004_number_parsing.md](../feature/004_number_parsing.md) — Number parsing feature design
- [005_command_parsing.md](../feature/005_command_parsing.md) — Command parsing feature design
- [006_ansi_utilities.md](../feature/006_ansi_utilities.md) — ANSI utilities feature design
