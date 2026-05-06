//! Extended corner case tests for `format_tools`
//!
//! Tests additional edge cases not covered in existing test suite:
//! - Zero-width characters
//! - Control characters
//! - Mixed RTL/LTR text
//! - Emoji sequences
//! - Single character strings
//! - Very large strings (>10KB)

use format_tools::{ WithDebug, WithDisplay, to_string_with_fallback };
use std::{ fmt, borrow::Cow };

#[ test ]
fn single_character_display()
{
  struct SingleChar;
  impl fmt::Display for SingleChar
  {
    fn fmt( &self, f: &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "x" )
    }
  }
  impl fmt::Debug for SingleChar
  {
    fn fmt( &self, f: &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "debug" )
    }
  }

  let src = SingleChar;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  assert_eq!( got.len(), 1 );
  assert_eq!( got, "x" );
}

#[ test ]
fn zero_width_characters()
{
  struct ZeroWidthDisplay;
  impl fmt::Display for ZeroWidthDisplay
  {
    fn fmt( &self, f: &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      // Zero-width joiner (U+200D), zero-width non-joiner (U+200C)
      write!( f, "a\u{200D}b\u{200C}c" )
    }
  }
  impl fmt::Debug for ZeroWidthDisplay
  {
    fn fmt( &self, f: &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "debug" )
    }
  }

  let src = ZeroWidthDisplay;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  assert!( got.contains( '\u{200D}' ) );
  assert!( got.contains( '\u{200C}' ) );
  assert_eq!( got, "a\u{200D}b\u{200C}c" );
}

#[ test ]
fn control_characters()
{
  struct ControlCharsDisplay;
  impl fmt::Display for ControlCharsDisplay
  {
    fn fmt( &self, f: &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      // Bell (0x07), Backspace (0x08), Form feed (0x0C)
      write!( f, "a\x07b\x08c\x0C" )
    }
  }
  impl fmt::Debug for ControlCharsDisplay
  {
    fn fmt( &self, f: &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "debug" )
    }
  }

  let src = ControlCharsDisplay;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  assert!( got.contains( '\x07' ) );
  assert!( got.contains( '\x08' ) );
  assert!( got.contains( '\x0C' ) );
}

#[ test ]
fn mixed_rtl_ltr_text()
{
  struct MixedDirectionDisplay;
  impl fmt::Display for MixedDirectionDisplay
  {
    fn fmt( &self, f: &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      // English (LTR) + Arabic (RTL) + Hebrew (RTL)
      write!( f, "Hello مرحبا שלום World" )
    }
  }
  impl fmt::Debug for MixedDirectionDisplay
  {
    fn fmt( &self, f: &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "debug" )
    }
  }

  let src = MixedDirectionDisplay;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  assert!( got.contains( "Hello" ) );
  assert!( got.contains( "مرحبا" ) );
  assert!( got.contains( "שלום" ) );
  assert!( got.contains( "World" ) );
}

#[ test ]
fn emoji_sequences()
{
  struct EmojiSeqDisplay;
  impl fmt::Display for EmojiSeqDisplay
  {
    fn fmt( &self, f: &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      // Family emoji (man + woman + child = family using ZWJ)
      // Skin tone modifier
      write!( f, "👨‍👩‍👧 👍🏽 🏴󠁧󠁢󠁥󠁮󠁧󠁿" )
    }
  }
  impl fmt::Debug for EmojiSeqDisplay
  {
    fn fmt( &self, f: &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "debug" )
    }
  }

  let src = EmojiSeqDisplay;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  // Just verify it doesn't panic and contains the sequences
  assert!( got.contains( "👨" ) );
  assert!( got.contains( "👍" ) );
  assert!( !got.is_empty() );
}

#[ test ]
fn very_large_string_10kb()
{
  struct VeryLargeDisplay;
  impl fmt::Display for VeryLargeDisplay
  {
    fn fmt( &self, f: &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "{}", "x".repeat( 10240 ) ) // 10KB
    }
  }
  impl fmt::Debug for VeryLargeDisplay
  {
    fn fmt( &self, f: &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "debug" )
    }
  }

  let src = VeryLargeDisplay;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  assert_eq!( got.len(), 10240 );
  assert!( got.chars().all( | c | c == 'x' ) );
}

#[ test ]
fn null_byte_in_string()
{
  struct NullByteDisplay;
  impl fmt::Display for NullByteDisplay
  {
    fn fmt( &self, f: &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "before\0after" )
    }
  }
  impl fmt::Debug for NullByteDisplay
  {
    fn fmt( &self, f: &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "debug" )
    }
  }

  let src = NullByteDisplay;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  assert!( got.contains( '\0' ) );
  assert!( got.contains( "before" ) );
  assert!( got.contains( "after" ) );
}
