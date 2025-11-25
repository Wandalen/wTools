# Extract Output Truncation Utilities to unilang

**Date**: 2025-11-19
**Completed**: 2025-11-22
**Priority**: HIGH - Reduces Code Duplication
**Category**: API Enhancement
**Status**: Completed
**Source**: wplan_client/src/cli/output_config.rs

**⚠️ CRITICAL**: This task is INCOMPLETE without follow-up adoption. Task will be CANCELED if adoption not implemented.

**Follow-up Adoption Required:**
- [wplan_client/001](../../../../../willbe/module/wplan_client/task/001_adopt_output_truncation_from_unilang.md) - Replace local truncation with unilang

---

## Executive Summary

Extract the output truncation utilities from `wplan_client` to `unilang` as reusable CLI output formatting tools. These utilities handle head/tail truncation with ANSI sequence preservation and Unicode-aware width limiting, which are broadly useful for any CLI application built with unilang.

---

## Problem Statement

### Current Location

**File**: `wplan_client/src/cli/output_config.rs`
**Functionality**:
- Head truncation (first N lines)
- Tail truncation (last N lines)
- Width truncation (max visible characters per line)
- ANSI escape sequence preservation
- Unicode grapheme cluster awareness

### Why Extract

1. **General Utility**: Output truncation is useful for any CLI application, not just wplan
2. **Already Defined in unilang.commands.yaml**: Parameters `head`, `tail`, `width`, `output` are defined in wplan's command schema
3. **Complexity**: ANSI-aware and Unicode-aware truncation is non-trivial, worth centralizing
4. **Consistency**: Provides standard output formatting across unilang-based CLIs

---

## Proposed Solution

### Target Location

```
unilang/src/output/
  mod.rs           # Module exports
  truncation.rs    # Core truncation logic
```

### API Design

```rust
//! Output truncation utilities for CLI applications.
//!
//! Provides ANSI-aware and Unicode-aware truncation for:
//! - Head truncation (first N lines)
//! - Tail truncation (last N lines)
//! - Width truncation (max visible characters)

use unicode_segmentation::UnicodeSegmentation;

/// Configuration for output truncation.
#[derive( Debug, Clone, Default )]
pub struct TruncationConfig
{
  /// Show only first N lines (0 = show all).
  pub head : usize,
  /// Show only last N lines (0 = show all).
  pub tail : usize,
  /// Maximum visible characters per line (0 = no limit).
  pub width : usize,
}

impl TruncationConfig
{
  /// Create with head truncation.
  pub fn with_head( lines : usize ) -> Self
  {
    Self { head : lines, ..Default::default() }
  }

  /// Create with tail truncation.
  pub fn with_tail( lines : usize ) -> Self
  {
    Self { tail : lines, ..Default::default() }
  }

  /// Create with width truncation.
  pub fn with_width( chars : usize ) -> Self
  {
    Self { width : chars, ..Default::default() }
  }

  /// Apply truncation to output string.
  pub fn apply( &self, output : &str ) -> String
  {
    let mut result = output.to_string();

    // Apply head truncation first
    if self.head > 0
    {
      result = truncate_head( &result, self.head );
    }

    // Apply tail truncation
    if self.tail > 0
    {
      result = truncate_tail( &result, self.tail );
    }

    // Apply width truncation per line
    if self.width > 0
    {
      result = truncate_width( &result, self.width );
    }

    result
  }
}

/// Truncate to first N lines.
pub fn truncate_head( text : &str, lines : usize ) -> String
{
  text.lines()
    .take( lines )
    .collect::< Vec< _ > >()
    .join( "\n" )
}

/// Truncate to last N lines.
pub fn truncate_tail( text : &str, lines : usize ) -> String
{
  let all_lines : Vec< _ > = text.lines().collect();
  let start = all_lines.len().saturating_sub( lines );
  all_lines[ start.. ]
    .join( "\n" )
}

/// Truncate each line to max visible width (ANSI and Unicode aware).
pub fn truncate_width( text : &str, max_width : usize ) -> String
{
  text.lines()
    .map( | line | truncate_line_width( line, max_width ) )
    .collect::< Vec< _ > >()
    .join( "\n" )
}

/// Truncate a single line preserving ANSI sequences.
fn truncate_line_width( line : &str, max_width : usize ) -> String
{
  let mut result = String::new();
  let mut visible_width = 0;
  let mut in_escape = false;

  for grapheme in line.graphemes( true )
  {
    if grapheme == "\x1b"
    {
      in_escape = true;
      result.push_str( grapheme );
    }
    else if in_escape
    {
      result.push_str( grapheme );
      if grapheme.chars().all( | c | c.is_ascii_alphabetic() )
      {
        in_escape = false;
      }
    }
    else
    {
      let width = unicode_width::UnicodeWidthStr::width( grapheme );
      if visible_width + width > max_width
      {
        result.push_str( "..." );
        break;
      }
      visible_width += width;
      result.push_str( grapheme );
    }
  }

  result
}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn head_truncation()
  {
    let text = "line1\nline2\nline3\nline4\nline5";
    assert_eq!( truncate_head( text, 3 ), "line1\nline2\nline3" );
  }

  #[ test ]
  fn tail_truncation()
  {
    let text = "line1\nline2\nline3\nline4\nline5";
    assert_eq!( truncate_tail( text, 2 ), "line4\nline5" );
  }

  #[ test ]
  fn width_truncation_preserves_ansi()
  {
    let text = "\x1b[31mRed text\x1b[0m";
    let result = truncate_line_width( text, 5 );
    assert!( result.contains( "\x1b[31m" ) );
  }

  #[ test ]
  fn config_applies_all()
  {
    let config = TruncationConfig
    {
      head : 5,
      tail : 3,
      width : 10,
    };

    let text = "line1\nline2\nline3\nline4\nline5\nline6\nline7";
    let result = config.apply( text );

    // Head 5, then tail 3
    assert_eq!( result.lines().count(), 3 );
  }
}
```

### Re-export from unilang

```rust
// In unilang/src/lib.rs
pub mod output;
pub use output::truncation::{ TruncationConfig, truncate_head, truncate_tail, truncate_width };
```

---

## Implementation Phases

### Phase 1: Extract to unilang (2 hours)

1. Create `unilang/src/output/mod.rs`
2. Create `unilang/src/output/truncation.rs`
3. Implement `TruncationConfig` and truncation functions
4. Add tests for all functionality
5. Re-export from lib.rs

### Phase 2: Update wplan_client (1 hour)

1. Update `wplan_client/Cargo.toml` to depend on unilang
2. Replace local implementation with `use unilang::output::truncation::*`
3. Delete `wplan_client/src/cli/output_config.rs`
4. Verify all tests pass

### Phase 3: Documentation (30 minutes)

1. Add module-level documentation
2. Add examples to function docs
3. Update unilang README if needed

---

## Dependencies

```toml
# unilang/Cargo.toml
[dependencies]
unicode-segmentation = { workspace = true }
unicode-width = { workspace = true }
```

---

## Expected Impact

| Metric | Before | After |
|--------|--------|-------|
| wplan_client output_config.rs LOC | ~150 | 0 (deleted) |
| Code duplication | Isolated | Shared |
| ANSI handling bugs | Per-project | Fixed once |
| Test coverage | Per-project | Centralized |

---

## Acceptance Criteria

- [x] `unilang/src/output/truncation.rs` implements all truncation functions
- [x] TruncationConfig provides config struct with has_truncation/is_default methods
- [x] ANSI escape sequences preserved during truncation
- [x] Unicode grapheme clusters handled correctly
- [ ] All existing wplan_client tests pass with new import (pending adoption task)
- [x] `cargo test -p unilang` passes (792 tests pass)
- [ ] `cargo test -p wplan_client` passes (pending adoption task)

---

## Verification

```bash
cargo test -p unilang -- output
cargo test -p wplan_client
```

---

## Notes

- The `head`, `tail`, `width`, `output` parameters are already defined in `wplan/unilang.commands.yaml`
- This extraction makes these parameters usable by any unilang-based CLI
- Consider adding `OutputFilter` enum for `stdout`/`stderr`/`both` selection

---

## Priority

**HIGH** - Already defined in command schema, reduces duplication, broadly useful.

## Estimated Effort

3-4 hours total.

---

## RESOLUTION - 2025-11-22

**Status**: ✅ COMPLETED

### Implementation Summary

Task 089 was implemented using Test-Driven Development (TDD) approach.

#### Files Created

1. **`src/output/mod.rs`** - Module structure using mod_interface pattern
   ```rust
   pub mod truncation;
   mod private {}
   mod_interface::mod_interface!
   {
     exposed use { truncation::* };
   }
   ```

2. **`src/output/truncation.rs`** (~200 lines) - Core implementation:
   - `TruncationConfig` struct with `head`, `tail`, `width`, `output_filter` fields
   - `OutputFilter` enum: `Both`, `Stdout`, `Stderr`
   - `TruncatedOutput` result struct with `content`, `lines_omitted`, `width_truncated`
   - `apply_truncation()` - main entry point combining all operations
   - `truncate_head()` - first N lines
   - `truncate_tail()` - last N lines
   - `truncate_width()` - ANSI-aware, Unicode-aware width limiting

3. **`tests/output_truncation.rs`** (33 tests) - Comprehensive test coverage:
   - TruncationConfig default/detection tests
   - OutputFilter stream selection tests
   - Head/tail truncation tests
   - Width truncation with ANSI preservation
   - Unicode grapheme cluster handling
   - Combined operations tests

#### Key Design Decisions

1. **ANSI-Aware Truncation**: Two-pass algorithm:
   - First pass: count visible characters (excluding ANSI codes)
   - Second pass: truncate to (max_width - 1) + arrow indicator (→)
   - Preserves escape sequences, adds reset code if truncating mid-formatting

2. **Unicode Grapheme Clusters**: Uses `unicode-segmentation` crate for proper character counting (handles emojis, combining characters, ZWJ sequences)

3. **Stream Selection**: `OutputFilter` enum allows selecting stdout, stderr, or both (merged with proper newline handling)

4. **Result Tracking**: `TruncatedOutput` tracks what was truncated for display purposes

#### Dependencies Added

```toml
# Cargo.toml
unicode-segmentation = { workspace = true }
```

#### Wiring

```rust
// lib.rs
layer output;
```

### Validation Results

- **33 tests pass** in `tests/output_truncation.rs`
- **792 total tests pass** across unilang crate
- **Clippy clean** - no warnings
- **Doc tests pass**

### Deviation from Original Design

| Aspect | Original Proposal | Actual Implementation |
|--------|-------------------|----------------------|
| Config fields | `head: usize` (0 = all) | `head: Option<usize>` (None = all) |
| Builder API | Fluent `.with_head()` | Struct with `has_truncation()` / `is_default()` |
| Truncation indicator | `"..."` | `"→"` (arrow) |
| Output filter | Not proposed | Added `OutputFilter` enum |
| Result type | `String` | `TruncatedOutput` struct |

### Pending

- **Adoption task**: `wplan_client` needs to replace local implementation with `use unilang::output::truncation::*`

---

**Resolution Date**: 2025-11-22
**Test Coverage**: 33 new tests
**Files Created**: 3 (mod.rs, truncation.rs, output_truncation.rs)
