# Feature: ANSI Utilities

### Scope

- **Purpose**: Detect, parse, strip, and truncate ANSI escape sequences in terminal strings, enabling correct visual-width calculations and clean text extraction.
- **Responsibility**: Documents the ANSI processing capability and links to its source, tests, and API contract.
- **In Scope**: ANSI escape sequence detection, sequence parsing, sequence stripping, visual-width-aware truncation, visual length calculation (char count), visual width calculation (display columns).
- **Out of Scope**: String splitting (`feature/001`); command parsing (`feature/005`); API operation signatures (`api/002`).

### Design

ANSI processing is organized into five focused operations, each implemented in its own source module.

Detection identifies whether a string contains any ANSI escape sequences, providing a fast predicate without full parsing.

Parsing walks the string and yields each ANSI sequence and each plain-text span as discrete tokens, allowing callers to process escape codes and visible content separately.

Stripping removes all ANSI escape sequences and returns only the visible text content as an owned string.

Visual-length calculation counts the number of visible characters (Unicode codepoints) a string contains after stripping ANSI escape sequences. This is a char count, not a display-column count. The grapheme-aware variant uses unicode-segmentation for accurate grapheme cluster counting, making it a two-tier operation: ANSI awareness first, then Unicode grapheme boundaries.

Visual-width calculation measures the number of terminal display columns a string occupies after stripping ANSI escape sequences. Unlike visual length (char count), visual width accounts for wide characters (CJK, emoji) that occupy two display columns and zero-width combining marks. The grapheme-aware variant processes grapheme clusters rather than individual codepoints.

Truncation cuts a string to a target visual width without splitting multi-byte characters or orphaning escape sequences, ensuring the truncated result renders correctly in a terminal.

### Sources

- [src/ansi/detect.rs](../../src/ansi/detect.rs) — ANSI sequence presence detection
- [src/ansi/parse.rs](../../src/ansi/parse.rs) — Token-by-token sequence and span parsing
- [src/ansi/strip.rs](../../src/ansi/strip.rs) — Escape sequence removal
- [src/ansi/visual.rs](../../src/ansi/visual.rs) — Visual-width calculation with Unicode grapheme support
- [src/ansi/truncate.rs](../../src/ansi/truncate.rs) — Visual-width-aware string truncation

### Tests

- [tests/ansi_truncate_tests.rs](../../tests/ansi_truncate_tests.rs) — ANSI truncation correctness and edge case tests

### APIs

- [002_string_utilities_api.md](../api/002_string_utilities_api.md) — ANSI utility operation contract
