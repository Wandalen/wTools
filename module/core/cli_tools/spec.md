# Technical Specification: `cli_tools`

## Section 1: Overview

### 1.1. Purpose

CLI application building blocks for command-line tools. Provides utilities specifically designed for CLI applications, distinct from general-purpose string manipulation.

### 1.2. Architectural Distinction

**cli_tools vs strs_tools:**

- **`strs_tools`**: General-purpose string/ANSI manipulation (any application)
  - Pure text transformation utilities
  - No assumptions about use case (terminals, logs, files, any display)
  - Reusable across web apps, embedded systems, servers, CLI tools

- **`cli_tools`**: CLI-application-specific helpers (command-line tools only)
  - Policy decisions specific to CLI conventions
  - Stream ordering (stderr before stdout)
  - Output formats specific to command execution
  - CLI-focused configuration and APIs

**Policy Decisions in cli_tools:**
- Stream merging: stderr appears before stdout (CLI convention)
- Output filtering: head/tail semantics match Unix tools
- Builder pattern: optimized for CLI configuration chains

### 1.3. Migration History

**Evolution:**
1. **v0.30.x**: Original implementation in `unilang::output` (449 lines)
2. **v0.31.0-0.43.0**: Migrated to `strs_tools::cli_output` (eliminated duplication)
3. **v0.44.0+**: Moved to dedicated `cli_tools` crate (proper architectural separation)

**Rationale for Separation:**
- Architectural boundary: CLI-specific ≠ general-purpose string utilities
- Single Responsibility: strs_tools focuses on text manipulation, cli_tools on CLI helpers
- Future growth: Room for additional CLI utilities (progress bars, tables, prompts)

---

## Section 2: Module Specifications

### 2.1. Module: `cli_output`

#### Purpose

Unified CLI output processing utilities for command-line applications. Provides head/tail line filtering, width truncation, and stream merging with ANSI preservation.

**Key Features:**
- Head/tail line filtering (show first N and/or last M lines)
- ANSI-aware width truncation (preserves color codes during truncation)
- Stream merging (combine stdout/stderr with configurable ordering)
- Builder pattern API for clean configuration
- Zero data loss - tracks omitted lines and truncation status

#### Core Data Structures & API

**Configuration:**

*   **`struct OutputConfig`**: Configuration for output processing with builder pattern
    *   `head: Option<usize>` - Show only first N lines
    *   `tail: Option<usize>` - Show only last N lines
    *   `width: Option<usize>` - Maximum visible characters per line (ANSI-aware)
    *   `width_suffix: String` - Suffix when width-truncated (default: "→")
    *   `stream_filter: StreamFilter` - Which streams to process
    *   `unicode_aware: bool` - Use grapheme-based width (requires `ansi_unicode` feature)

*   **Builder Methods:**
    *   `OutputConfig::new() -> Self` - Create with defaults
    *   `.with_head(count: usize) -> Self` - Set head line limit
    *   `.with_tail(count: usize) -> Self` - Set tail line limit
    *   `.with_width(width: usize) -> Self` - Set width limit
    *   `.with_width_suffix(suffix: impl Into<String>) -> Self` - Customize truncation suffix
    *   `.with_stream_filter(filter: StreamFilter) -> Self` - Set stream selection
    *   `.with_unicode_aware(enabled: bool) -> Self` - Enable grapheme-based width

*   **Helper Methods:**
    *   `.is_default() -> bool` - Check if config is default (no processing)
    *   `.has_processing() -> bool` - Check if any processing is configured

**Stream Selection:**

*   **`enum StreamFilter`**: Controls which streams are processed
    *   `Both` - Process both stdout and stderr (default, stderr appears before stdout)
    *   `Stdout` - Process only stdout
    *   `Stderr` - Process only stderr

**Result Type:**

*   **`struct ProcessedOutput`**: Result of output processing
    *   `content: String` - Processed output text
    *   `lines_omitted: usize` - Number of lines omitted by head/tail filtering
    *   `width_truncated: bool` - Whether any line was truncated by width limit

**Processing Functions:**

*   **`process_output(stdout: &str, stderr: &str, config: &OutputConfig) -> ProcessedOutput`**
    - Main entry point for CLI output processing
    - Applies stream selection → line filtering → width truncation in sequence
    - Preserves ANSI escape codes throughout processing
    - Returns processed output with metadata about omissions

*   **`merge_streams(stdout: &str, stderr: &str, filter: &StreamFilter) -> String`**
    - Merge stdout and stderr streams based on filter
    - stderr appears before stdout when both selected (CLI convention)
    - Returns empty string if both inputs empty

#### Processing Pipeline

Output processing follows this sequence:

1. **Stream Selection**: Merge stdout/stderr based on `stream_filter`
2. **Line Filtering**: Apply head/tail limits via `strs_tools::string::lines::{head, tail, head_and_tail}`
3. **Width Truncation**: Truncate each line if `width` configured
   - Only truncates if line exceeds max_width (uses `strs_tools::ansi::truncate_lines()`)
   - Preserves ANSI codes
   - Adds `width_suffix` indicator when truncated

#### ANSI Handling

*   Width measurement excludes ANSI escape sequences
*   Truncation preserves ANSI formatting
*   Reset codes automatically added after truncation for safety
*   Lines that fit exactly are NOT truncated (respects visible width boundary)

#### Unicode Support

*   **Char-based (default)**: Fast, works for ASCII/Latin text
*   **Grapheme-based (`unicode_aware: true`)**: Accurate for CJK, emoji, combining marks
    - Requires `ansi_unicode` feature
    - Delegates to `strs_tools::ansi::truncate_lines_unicode()`

#### Dependencies on strs_tools

This module uses the following general-purpose functions from `strs_tools`:

- **`strs_tools::ansi::truncate_lines()`** - Multi-line ANSI truncation with boundary detection
- **`strs_tools::ansi::truncate_lines_unicode()`** - Unicode-aware version
- **`strs_tools::string::lines::head()`** - Extract first N lines
- **`strs_tools::string::lines::tail()`** - Extract last N lines
- **`strs_tools::string::lines::head_and_tail()`** - Extract first N and last M lines

These functions were extracted from cli_output to make them available as general-purpose
utilities for any application requiring ANSI-aware text processing.

#### Feature Dependencies

*   **`cli_output`**: Main feature flag (requires `enabled`, `std`, `string_split`)
*   **`ansi_unicode`**: Optional, enables grapheme-based Unicode support (passes through to strs_tools)

#### Usage Example

```rust
use cli_tools::cli_output::*;

// Show first 5 and last 5 lines, max 80 chars width
let config = OutputConfig::default()
  .with_head(5)
  .with_tail(5)
  .with_width(80);

let result = process_output(stdout_str, stderr_str, &config);

println!("{}", result.content);
println!("Lines omitted: {}", result.lines_omitted);
println!("Width truncated: {}", result.width_truncated);
```

#### Migration from unilang::output

| Old (unilang 0.31.x) | New (cli_tools) |
|---------------------|------------------|
| `TruncationConfig { head: Some(10), .. }` | `OutputConfig::default().with_head(10)` |
| `apply_truncation(stdout, stderr, &config)` | `process_output(stdout, stderr, &config)` |
| `TruncatedOutput` | `ProcessedOutput` |
| `OutputFilter::Both` | `StreamFilter::Both` |
| `truncate_head(text, 10)` | `strs_tools::string::lines::head(text, 10)` |
| `truncate_tail(text, 10)` | `strs_tools::string::lines::tail(text, 10)` |
| `truncate_width(text, 80)` | `strs_tools::ansi::truncate_if_needed(text, 80, &opts)` |

**Key Improvements:**
- Builder pattern replaces struct initialization
- Configurable suffix (vs hardcoded arrow)
- Proper width boundary detection (doesn't truncate text that fits exactly)
- Two-tier Unicode support (char vs grapheme)

---

## Section 3: Architecture Compliance

### 3.1. Architectural Principles

This crate adheres to clean separation of concerns:

1. **Delegates to strs_tools**: Uses general-purpose functions for text transformation
2. **CLI-Specific Policy**: Implements CLI conventions (stream ordering, output formats)
3. **Builder Pattern**: Provides ergonomic configuration for CLI tools
4. **Zero Dependencies**: Only depends on strs_tools (which is general-purpose)

### 3.2. Design Decisions

**Why stderr before stdout?**
CLI convention - errors should be visible immediately without scrolling past normal output.

**Why builder pattern?**
CLI tools often need to configure multiple options. Builder pattern enables:
```rust
let config = OutputConfig::default()
  .with_head(10)
  .with_tail(5)
  .with_width(80)
  .with_stream_filter(StreamFilter::Both);
```

Instead of verbose struct initialization:
```rust
let config = OutputConfig {
  head: Some(10),
  tail: Some(5),
  width: Some(80),
  stream_filter: StreamFilter::Both,
  ..Default::default()
};
```

**Why track metadata (lines_omitted, width_truncated)?**
CLI tools often need to inform users about data omissions for transparency.

---

## Section 4: Future Roadmap

Potential additions to cli_tools (as separate modules):

- **Progress Bars**: CLI progress indication
- **Tables**: Formatted table output with ANSI support
- **Prompts**: Interactive CLI prompts
- **Colors**: CLI color scheme management
- **Spinners**: Loading indicators

These would follow the same principle: CLI-specific helpers that delegate to
general-purpose utilities in strs_tools where applicable.

---

## Section 5: Testing

**Test Coverage:**
- 31 integration tests (cli_output module)
- 4 doc tests
- Total: 35 tests

**Test Categories:**
- OutputConfig behavior
- Stream filtering (stdout/stderr selection)
- Head/tail line filtering
- Width truncation with ANSI preservation
- Combined operations (head+tail+width)
- Unicode handling

**Test Location:** `tests/cli_output.rs`

---

## Section 6: Performance Characteristics

| Operation | Complexity | Notes |
|-----------|------------|-------|
| `process_output()` | O(n) | Single pass through text |
| Stream merging | O(n) | Simple concatenation |
| Line filtering | O(n) | Uses strs_tools::string::lines |
| Width truncation | O(n × m) | n = text length, m = avg line count |

All operations minimize allocations and use efficient string handling.

---

## Appendix A: Version History

See `changelog.md` for detailed version history.

**Current Version:** 0.1.0 (Initial release)
