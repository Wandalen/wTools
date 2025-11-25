# Extract String Formatting and ANSI Utilities from wplan

**Date**: 2025-11-20
**Completed**: 2025-11-22
**Priority**: HIGH
**Category**: API Enhancement - Code Extraction
**Status**: ✅ COMPLETE (ANSI utilities only)
**Source**: wplan_client/src/cli/formatting.rs, wplan_client/src/cli/output_config.rs

## Implementation Status

**Decision**: REJECT byte formatting (doesn't fit strs_tools scope - number formatting, not string manipulation)
**Decision**: ACCEPT ANSI utilities (perfect fit for string manipulation)

### Completed Work:
- ✅ **strs_tools::ansi module** - Full implementation with 68 tests
  - `parse_segments()` - Parse text into ANSI/text segments
  - `visual_len()` / `visual_len_unicode()` - Visible character counting
  - `strip()` - Remove ANSI codes
  - `has_ansi()` / `has_unclosed_formatting()` - Detection utilities
  - `truncate()` / `truncate_unicode()` - ANSI-aware truncation
  - `pad_to_width()` - ANSI-aware padding
  - `Segment` enum + `TruncateOptions` struct
- ✅ **tree_fmt migration** - Now uses strs_tools::ansi (~67 lines deleted)
- ✅ **Two-tier architecture** - Tier 1 (char-based) + Tier 2 (Unicode grapheme-aware)
- ✅ **Spec updated** - Section 2.6 added to spec.md

### Remaining (Separate Tasks):
- [wplan_client/004](../../../../../willbe/module/wplan_client/task/004_adopt_formatting_from_strs_tools.md) - Replace local formatting with strs_tools

---

---

## Executive Summary

Extract general-purpose string formatting and ANSI handling utilities from the wplan ecosystem to `strs_tools`, making them available to all wTools projects. These utilities handle byte size formatting, ANSI escape sequence parsing/preservation, and Unicode-aware string operations - all broadly useful for CLI applications.

---

## Problem Statement

### Current Location and Duplication

The wplan codebase contains several general-purpose string utilities that would benefit all wTools projects:

**wplan_client/src/cli/formatting.rs**:
- Lines 41-67: `format_bytes()` - Formats byte counts as human-readable strings ("1.5 KiB", "3.2 MiB")
- Functionality: Converts raw byte counts to human-friendly sizes with appropriate units

**wplan_client/src/cli/output_config.rs**:
- Lines 203-266: `truncate_line_ansi_aware()` - Truncates strings while preserving ANSI escape sequences
- Lines 271-318: `parse_ansi_segments()` - Parses text into ANSI and plain text segments
- Lines 220-266: Unicode grapheme cluster width calculation
- Functionality: ANSI-aware truncation with proper Unicode handling for terminal output

### Why Extract to strs_tools

1. **General Utility**: These are domain-agnostic string operations useful for any CLI tool
2. **ANSI Complexity**: Proper ANSI handling is non-trivial and should be centralized
3. **Unicode Correctness**: Unicode grapheme cluster handling is critical for internationalization
4. **Code Reuse**: Multiple wTools projects will benefit (willbe, unitore, benchkit, etc.)
5. **Testing**: Centralizes test coverage for complex string operations
6. **Consistency**: Provides standard formatting across wTools ecosystem

---

## Detailed Functionality Analysis

### 1. Byte Size Formatting

**Current Location**: `wplan_client/src/cli/formatting.rs:41-67`

```rust
pub fn format_bytes( bytes : usize ) -> String
{
  const UNITS : &[ &str ] = &[ "B", "KiB", "MiB", "GiB", "TiB" ];
  let mut size = bytes as f64;
  let mut unit_idx = 0;

  while size >= 1024.0 && unit_idx < UNITS.len() - 1
  {
    size /= 1024.0;
    unit_idx += 1;
  }

  if size >= 100.0
  {
    format!( "{:.0} {}", size, UNITS[ unit_idx ] )
  }
  else if size >= 10.0
  {
    format!( "{:.1} {}", size, UNITS[ unit_idx ] )
  }
  else
  {
    format!( "{:.2} {}", size, UNITS[ unit_idx ] )
  }
}
```

**Use Cases**:
- Displaying file sizes
- Memory usage reports
- Network transfer sizes
- Log file sizes

**Enhancement Opportunities**:
- Support for SI units (KB, MB) vs binary units (KiB, MiB)
- Configurable precision
- Parse formatted sizes back to bytes

### 2. ANSI-Aware String Truncation

**Current Location**: `wplan_client/src/cli/output_config.rs:203-266`

**Functionality**:
- Truncates strings to visible character limit
- Preserves ANSI escape sequences (colors, styles)
- Accounts for Unicode grapheme clusters
- Adds "..." indicator when truncated

**Algorithm**:
1. Parse input into ANSI sequences and visible text
2. Count visible characters using Unicode grapheme clusters
3. Stop at width limit, preserving complete grapheme clusters
4. Keep all ANSI sequences encountered before truncation point
5. Add truncation indicator

**Why Complex**:
- ANSI sequences are invisible but present in string
- Unicode grapheme clusters can span multiple bytes
- Some characters are zero-width or double-width
- Must preserve ANSI state for proper rendering

### 3. ANSI Segment Parsing

**Current Location**: `wplan_client/src/cli/output_config.rs:271-318`

**Functionality**:
- Splits text into ANSI escape sequences and plain text segments
- Identifies sequence boundaries correctly
- Handles all ANSI escape types (CSI, OSC, etc.)

**Use Cases**:
- ANSI stripping for log files
- Calculating visible text width
- ANSI sequence transformation
- Terminal output analysis

### 4. Unicode Grapheme Width Calculation

**Current Location**: `wplan_client/src/cli/output_config.rs:220-266`

**Functionality**:
- Calculates visible width of text in terminal
- Handles Unicode grapheme clusters (multi-codepoint characters)
- Accounts for double-width characters (CJK, emoji)
- Accounts for zero-width characters (combining marks)

**Dependencies**:
- `unicode-segmentation` for grapheme cluster iteration
- `unicode-width` for character width calculation

---

## Proposed API Design

### Target Location

```
strs_tools/src/format/
  mod.rs           # Module exports
  bytes.rs         # Byte size formatting
  ansi.rs          # ANSI handling utilities
  truncate.rs      # ANSI-aware truncation
  width.rs         # Unicode width calculation
```

### API Structure

```rust
//! String formatting utilities for strs_tools
//!
//! Provides general-purpose string formatting for CLI applications:
//! - Byte size formatting (human-readable file/memory sizes)
//! - ANSI escape sequence handling (parsing, preservation, stripping)
//! - Unicode-aware text width calculation
//! - ANSI-aware string truncation

// ============================================================================
// bytes.rs - Byte Size Formatting
// ============================================================================

/// Format byte count as human-readable string.
///
/// # Arguments
///
/// * `bytes` - Number of bytes to format
/// * `binary` - Use binary units (KiB, MiB) vs SI units (KB, MB)
///
/// # Returns
///
/// Formatted string like "1.5 KiB" or "3.2 MB"
///
/// # Example
///
/// ```rust
/// use strs_tools::format::format_bytes;
///
/// assert_eq!( format_bytes( 1536, true ), "1.5 KiB" );
/// assert_eq!( format_bytes( 1500, false ), "1.5 KB" );
/// assert_eq!( format_bytes( 1_048_576, true ), "1.0 MiB" );
/// ```
pub fn format_bytes( bytes : usize, binary : bool ) -> String;

/// Parse human-readable size string to bytes.
///
/// # Example
///
/// ```rust
/// use strs_tools::format::parse_bytes;
///
/// assert_eq!( parse_bytes( "1.5 KiB" )?, 1536 );
/// assert_eq!( parse_bytes( "3.2 MB" )?, 3_200_000 );
/// ```
pub fn parse_bytes( s : &str ) -> Result< usize, ParseBytesError >;

// ============================================================================
// ansi.rs - ANSI Handling
// ============================================================================

/// Represents a segment of text (ANSI sequence or plain text).
#[ derive( Debug, Clone, PartialEq ) ]
pub enum TextSegment< 'a >
{
  /// ANSI escape sequence (invisible).
  Ansi( &'a str ),
  /// Plain visible text.
  Text( &'a str ),
}

/// Parse text into ANSI and plain text segments.
///
/// # Example
///
/// ```rust
/// use strs_tools::format::{ parse_ansi_segments, TextSegment };
///
/// let text = "\x1b[31mRed\x1b[0m text";
/// let segments = parse_ansi_segments( text );
///
/// assert_eq!( segments[ 0 ], TextSegment::Ansi( "\x1b[31m" ) );
/// assert_eq!( segments[ 1 ], TextSegment::Text( "Red" ) );
/// assert_eq!( segments[ 2 ], TextSegment::Ansi( "\x1b[0m" ) );
/// assert_eq!( segments[ 3 ], TextSegment::Text( " text" ) );
/// ```
pub fn parse_ansi_segments( text : &str ) -> Vec< TextSegment >;

/// Strip all ANSI escape sequences from text.
///
/// # Example
///
/// ```rust
/// use strs_tools::format::strip_ansi;
///
/// let colored = "\x1b[31mRed\x1b[0m text";
/// assert_eq!( strip_ansi( colored ), "Red text" );
/// ```
pub fn strip_ansi( text : &str ) -> String;

/// Check if text contains ANSI escape sequences.
pub fn has_ansi( text : &str ) -> bool;

// ============================================================================
// width.rs - Unicode Width Calculation
// ============================================================================

/// Calculate visible width of text in terminal (Unicode-aware).
///
/// Accounts for:
/// - Double-width characters (CJK, emoji)
/// - Zero-width characters (combining marks)
/// - Grapheme clusters (multi-codepoint characters)
/// - ANSI escape sequences (invisible)
///
/// # Example
///
/// ```rust
/// use strs_tools::format::visible_width;
///
/// assert_eq!( visible_width( "hello" ), 5 );
/// assert_eq!( visible_width( "日本語" ), 6 );      // CJK = double width
/// assert_eq!( visible_width( "e\u{0301}" ), 1 );  // e + combining acute = 1 grapheme
/// assert_eq!( visible_width( "\x1b[31mRed\x1b[0m" ), 3 );  // ANSI invisible
/// ```
pub fn visible_width( text : &str ) -> usize;

/// Calculate visible width ignoring ANSI sequences.
pub fn visible_width_no_ansi( text : &str ) -> usize;

// ============================================================================
// truncate.rs - ANSI-Aware Truncation
// ============================================================================

/// Configuration for text truncation.
#[ derive( Debug, Clone ) ]
pub struct TruncateOptions
{
  /// Maximum visible characters.
  pub max_width : usize,
  /// Truncation indicator (default: "...").
  pub indicator : String,
  /// Preserve ANSI sequences.
  pub preserve_ansi : bool,
  /// Use Unicode grapheme clusters.
  pub grapheme_aware : bool,
}

impl Default for TruncateOptions
{
  fn default() -> Self
  {
    Self
    {
      max_width : 80,
      indicator : "...".to_string(),
      preserve_ansi : true,
      grapheme_aware : true,
    }
  }
}

/// Truncate text to maximum visible width, preserving ANSI and Unicode.
///
/// # Example
///
/// ```rust
/// use strs_tools::format::{ truncate, TruncateOptions };
///
/// let text = "\x1b[31mVery long red text here\x1b[0m";
/// let opts = TruncateOptions { max_width : 10, ..Default::default() };
/// let result = truncate( text, &opts );
///
/// // Result: "\x1b[31mVery long...\x1b[0m"
/// // - First 10 visible chars kept
/// // - ANSI sequences preserved
/// // - Truncation indicator added
/// ```
pub fn truncate( text : &str, options : &TruncateOptions ) -> String;

/// Truncate to width (convenience wrapper).
pub fn truncate_to_width( text : &str, max_width : usize ) -> String
{
  truncate( text, &TruncateOptions { max_width, ..Default::default() } )
}
```

---

## Implementation Phases

### Phase 1: Byte Size Formatting (2 hours)

**Tasks**:
1. Create `strs_tools/src/format/bytes.rs`
2. Implement `format_bytes()` with binary/SI unit support
3. Implement `parse_bytes()` for reverse operation
4. Add comprehensive tests (units, edge cases, precision)
5. Benchmark performance

**Acceptance Criteria**:
- [ ] `format_bytes()` handles all unit ranges (B to TiB)
- [ ] Binary (1024) and SI (1000) modes both work
- [ ] Precision adapts to magnitude (3 significant figures)
- [ ] `parse_bytes()` correctly reverses `format_bytes()`
- [ ] Edge cases handled (0 bytes, huge values, invalid input)
- [ ] Tests cover all units and precisions
- [ ] Documentation includes usage examples

### Phase 2: ANSI Parsing and Stripping (3 hours)

**Tasks**:
1. Create `strs_tools/src/format/ansi.rs`
2. Implement `TextSegment` enum
3. Implement `parse_ansi_segments()` with proper ANSI parsing
4. Implement `strip_ansi()` for ANSI removal
5. Implement `has_ansi()` for detection
6. Add tests for all ANSI sequence types (CSI, OSC, etc.)
7. Benchmark parsing performance

**Acceptance Criteria**:
- [ ] Correctly identifies all ANSI escape sequence types
- [ ] Handles nested/malformed ANSI gracefully
- [ ] `strip_ansi()` removes all ANSI without affecting visible text
- [ ] `has_ansi()` detects ANSI presence accurately
- [ ] Tests cover SGR, cursor movement, colors, styles
- [ ] Documentation explains ANSI sequence types

### Phase 3: Unicode Width Calculation (2 hours)

**Tasks**:
1. Create `strs_tools/src/format/width.rs`
2. Implement `visible_width()` with grapheme cluster support
3. Implement `visible_width_no_ansi()` variant
4. Add tests for Unicode edge cases
5. Verify against unicode-width crate behavior

**Acceptance Criteria**:
- [ ] Correctly calculates width for ASCII, CJK, emoji
- [ ] Handles combining marks (zero-width)
- [ ] Handles grapheme clusters properly
- [ ] Ignores ANSI sequences in width calculation
- [ ] Tests cover edge cases (zero-width joiners, RTL marks)
- [ ] Performance acceptable for long strings

### Phase 4: ANSI-Aware Truncation (4 hours)

**Tasks**:
1. Create `strs_tools/src/format/truncate.rs`
2. Implement `TruncateOptions` configuration
3. Implement `truncate()` combining ANSI + Unicode + width
4. Implement convenience wrapper `truncate_to_width()`
5. Add comprehensive integration tests
6. Test edge cases (truncate at ANSI, at grapheme boundary)
7. Performance testing on large strings

**Acceptance Criteria**:
- [ ] Truncates to exact visible width
- [ ] Preserves all ANSI sequences before truncation point
- [ ] Respects grapheme cluster boundaries
- [ ] Adds truncation indicator correctly
- [ ] Configurable options work as expected
- [ ] Edge cases handled (empty string, width=0, huge width)
- [ ] Tests verify ANSI state preservation

### Phase 5: Integration and Migration (2 hours)

**Tasks**:
1. Update `strs_tools/src/lib.rs` to export new modules
2. Update `strs_tools/Cargo.toml` dependencies
3. Migrate wplan_client to use new API
4. Delete old implementations from wplan_client
5. Verify all wplan tests pass
6. Update documentation

**Acceptance Criteria**:
- [ ] All modules exported from `strs_tools::format`
- [ ] Dependencies added (unicode-segmentation, unicode-width)
- [ ] wplan_client uses `strs_tools::format::*`
- [ ] Old code deleted from wplan_client
- [ ] All wplan tests pass with new imports
- [ ] Documentation updated with examples

---

## Dependencies

```toml
# strs_tools/Cargo.toml
[dependencies]
unicode-segmentation = { workspace = true }
unicode-width = { workspace = true }
```

Both dependencies are already in workspace, no new additions needed.

---

## Expected Impact

| Metric | Before | After |
|--------|--------|-------|
| wplan_client formatting.rs LOC | ~150 | ~20 (imports only) |
| wplan_client output_config.rs LOC | ~300 | 0 (deleted) |
| Code duplication | Isolated | Shared across wTools |
| Test coverage | Per-project | Centralized |
| ANSI handling bugs | Per-project | Fixed once |
| Reusability | None | All wTools projects |

---

## Testing Strategy

### Unit Tests

**Byte Formatting**:
- All unit ranges (B, KiB, MiB, GiB, TiB)
- Edge cases (0, 1, 1023, 1024, large values)
- Precision logic (100+, 10-100, <10)
- Binary vs SI modes
- Parse round-trip

**ANSI Parsing**:
- All ANSI sequence types (SGR, cursor, colors)
- Nested ANSI
- Malformed ANSI
- Empty strings
- Plain text (no ANSI)

**Unicode Width**:
- ASCII text
- CJK characters (double-width)
- Emoji (double-width)
- Combining marks (zero-width)
- Grapheme clusters
- ANSI + Unicode mixed

**Truncation**:
- Exact width truncation
- ANSI preservation
- Grapheme boundary respect
- Truncation indicator placement
- Edge cases (width 0, huge width, empty)

### Integration Tests

- End-to-end truncation with real terminal output
- wplan_client migration tests
- Performance benchmarks

---

## Performance Considerations

**Byte Formatting**:
- O(1) complexity
- No allocations except final String
- Target: <100ns per format

**ANSI Parsing**:
- O(n) complexity (single pass)
- Minimal allocations (reuse vectors)
- Target: <1µs per 1KB text

**Unicode Width**:
- O(n) complexity (grapheme iteration)
- Uses unicode-width lookup tables
- Target: <5µs per 1KB text

**Truncation**:
- O(n) complexity (single pass)
- Combined ANSI parsing + width calculation
- Target: <10µs per 1KB text

---

## Documentation Requirements

Each module must include:
1. Module-level documentation with overview
2. Function documentation with examples
3. Usage patterns for common scenarios
4. Edge case handling explanations
5. Performance characteristics

---

## Acceptance Criteria

- [ ] All 4 modules implemented (bytes, ansi, width, truncate)
- [ ] Comprehensive test coverage (>90% for each module)
- [ ] All edge cases tested and handled
- [ ] Documentation complete with examples
- [ ] wplan_client successfully migrated
- [ ] Old wplan implementations deleted
- [ ] Performance targets met
- [ ] `cargo test -p strs_tools` passes
- [ ] `cargo test -p wplan_client` passes
- [ ] `cargo bench -p strs_tools` shows acceptable performance

---

## References

**Source Files**:
- `/home/user1/pro/lib/willbe/module/wplan_client/src/cli/formatting.rs:41-67` (format_bytes)
- `/home/user1/pro/lib/willbe/module/wplan_client/src/cli/output_config.rs:203-318` (ANSI/Unicode)

**Related Tasks**:
- unilang Task 089: Extract output truncation to unilang (head/tail/width)
- This task extracts lower-level primitives used by Task 089

**Dependencies**:
- unicode-segmentation (workspace)
- unicode-width (workspace)

---

## Estimated Effort

- Phase 1: 2 hours (byte formatting)
- Phase 2: 3 hours (ANSI parsing)
- Phase 3: 2 hours (Unicode width)
- Phase 4: 4 hours (truncation)
- Phase 5: 2 hours (migration)

**Total**: 13 hours

---

## Priority Justification

**HIGH Priority** because:
1. **Broad Applicability**: Every CLI tool needs string formatting
2. **Correctness**: ANSI handling is complex and error-prone when duplicated
3. **Immediate Value**: wplan migration eliminates ~300 LOC immediately
4. **Foundation**: Required for unilang Task 089 (output truncation)
5. **Quality**: Centralizes testing for critical functionality
