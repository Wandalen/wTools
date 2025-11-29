//! ANSI escape sequence handling utilities
//!
//! Provides two tiers of ANSI support:
//! - **Tier 1** (feature `ansi`): Zero-dependency, char-based, lightweight
//! - **Tier 2** (feature `ansi_unicode`): Full Unicode grapheme support
//!
//! ## Feature Comparison
//!
//! | Feature | Tier 1 (`ansi`) | Tier 2 (`ansi_unicode`) |
//! |---------|----------------|------------------------|
//! | Dependencies | Zero | unicode-segmentation |
//! | ASCII text | ✅ Perfect | ✅ Perfect |
//! | CJK characters | ⚠️ Approximate | ✅ Accurate |
//! | Emoji | ⚠️ Approximate | ✅ Accurate |
//! | Combining marks | ❌ Broken | ✅ Accurate |
//! | Performance | Fastest | Fast (~5% slower) |
//!
//! ## When to Use Each Tier
//!
//! **Use Tier 1** (`ansi`) when:
//! - You only handle ASCII/Latin text
//! - Binary size is critical (embedded, WASM)
//! - You can't afford external dependencies
//!
//! **Use Tier 2** (`ansi_unicode`) when:
//! - You handle internationalized text (CJK, emoji)
//! - Accurate truncation is critical
//! - Standard CLI application (dependencies acceptable)

// Internal modules (not exposed directly)
mod parse;
mod visual;
mod strip;
mod detect;
mod truncate;

/// Own namespace of the module.
#[ doc( inline ) ]
pub use own::*;

/// Own namespace of the module.
pub mod own
{
  // Re-export core types
  pub use super::Segment;
  pub use super::truncate::TruncateOptions;

  // Re-export Tier 1 functions
  pub use super::parse::parse_segments;
  pub use super::visual::{ visual_len, pad_to_width };
  pub use super::strip::strip;
  pub use super::detect::{ has_ansi, has_unclosed_formatting };
  pub use super::truncate::truncate;

  // ANSI truncation with boundary detection
  pub use super::truncate::truncate_if_needed;
  pub use super::truncate::truncate_lines;

  // Re-export Tier 2 functions (unicode)
  #[ cfg( feature = "ansi_unicode" ) ]
  pub use super::visual::visual_len_unicode;
  #[ cfg( feature = "ansi_unicode" ) ]
  pub use super::truncate::truncate_unicode;
  #[ cfg( feature = "ansi_unicode" ) ]
  pub use super::truncate::truncate_if_needed_unicode;
  #[ cfg( feature = "ansi_unicode" ) ]
  pub use super::truncate::truncate_lines_unicode;
}

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;

  // Types that should be visible when using `exposed::*`
  pub use super::Segment;
  pub use super::truncate::TruncateOptions;
}

/// Prelude namespace - most commonly used items.
pub mod prelude
{
  pub use super::parse::parse_segments;
  pub use super::visual::visual_len;
  pub use super::strip::strip;
}

/// ANSI escape sequence segment.
///
/// Represents either an ANSI escape code (invisible) or visible text content.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( feature = "ansi" ) ]
/// # {
/// use strs_tools::ansi::{ parse_segments, Segment };
///
/// let text = "\x1b[31mred\x1b[0m";
/// let segments = parse_segments( text );
///
/// match &segments[ 0 ]
/// {
///   Segment::Ansi( code ) => println!( "ANSI: {}", code.escape_debug() ),
///   Segment::Text( text ) => println!( "Text: {}", text ),
/// }
/// # }
/// ```
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum Segment< 'a >
{
  /// ANSI escape code (invisible in terminal output).
  Ansi( &'a str ),
  /// Visible text content.
  Text( &'a str ),
}
