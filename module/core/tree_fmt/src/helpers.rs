//! Helper utilities for ANSI-aware string handling
//!
//! ## Why ANSI-Aware Functions Are Needed
//!
//! ### The Problem: Color Codes Break Alignment
//!
//! ANSI escape sequences (color codes) affect byte length but not visual length, breaking
//! table and tree alignment when columns contain colored text.
//!
//! **Example Problem**:
//! ```text
//! // String: "\x1b[36mHello\x1b[0m"
//! byte_len: 17 bytes (escape codes included)
//! visual_len: 5 chars (only "Hello" is visible)
//! ```
//!
//! If we use standard `str.len()` for column width calculation, colored cells get
//! way too much padding:
//!
//! ```text
//! // WITHOUT visual_len (BROKEN):
//! Name              | Value
//! ------------------+------------------
//! [cyan]Hello[reset]                  | World  // Too much space after "Hello"!
//!
//! // WITH visual_len (CORRECT):
//! Name  | Value
//! ------+-------
//! Hello | World  // Proper alignment despite color codes
//! ```
//!
//! ### ANSI Escape Sequence Format
//!
//! Color codes follow the pattern: `\x1b[...m`
//! - Start: `\x1b[` (ESC + left bracket)
//! - Parameters: Numbers and semicolons (e.g., `31` for red, `1;36` for bold cyan)
//! - End: `m`
//!
//! Common examples:
//! - Red text: `\x1b[31m`
//! - Reset: `\x1b[0m`
//! - Bold cyan: `\x1b[1;36m`
//!
//! ### Solution: `visual_len()` and `pad_to_width()`
//!
//! **`visual_len()`**: Counts only visible characters by skipping escape sequences
//! **`pad_to_width()`**: Pads based on visual length, preserving ANSI codes
//!
//! ### Edge Cases Handled
//!
//! 1. **Nested escape sequences**: (uncommon but supported)
//!    - Example: `\x1b[31m\x1b[1mBold Red\x1b[0m`
//! 2. **Malformed sequences**: Counted as visible characters (graceful degradation)
//!    - Example: `\x1b[31Hello` (missing 'm' terminator)
//! 3. **Empty strings**: `visual_len("") == 0` (no special handling needed)
//! 4. **No ANSI codes**: Fast path, same as `str.chars().count()`
//!
//! ### Historical Context
//!
//! Added in v0.1.0 during initial linter improvements. Before this, colored header
//! rows in tables were completely misaligned, making tools like `cargo list` output
//! unreadable.
//!
//! ### Implementation Note
//!
//! These utilities are re-exported from `strs_tools::ansi` for centralized ANSI
//! handling across the wTools ecosystem. The implementations provide:
//! - Tier 1: Zero-dependency char-based counting (used here)
//! - Tier 2: Unicode grapheme-aware counting (available via `strs_tools::ansi_unicode`)
//!
//! ### Testing
//!
//! See integration tests for real-world scenarios with colored output. All formatters
//! use these functions throughout for consistent ANSI-aware behavior.

// Re-export ANSI utilities from strs_tools for backward compatibility
pub use strs_tools::ansi::visual_len;
pub use strs_tools::ansi::pad_to_width;
