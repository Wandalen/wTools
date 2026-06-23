#[ allow( unused_imports ) ]
use super::*;
use the_module::ansi::{ truncate, TruncateOptions };

// ==================== TruncateOptions tests ====================

#[ test ]
fn options_new()
{
  let opts = TruncateOptions::new( 10 );
  assert_eq!( opts.max_width, 10 );
  assert!( opts.suffix.is_none() );
  assert!( !opts.append_reset );
}

#[ test ]
#[ should_panic( expected = "max_width must be greater than 0" ) ]
fn options_panic_on_zero()
{
  TruncateOptions::new( 0 );
}

#[ test ]
fn options_builder()
{
  let opts = TruncateOptions::new( 10 )
    .with_suffix( "..." )
    .with_reset( true );

  assert_eq!( opts.max_width, 10 );
  assert_eq!( opts.suffix, Some( "...".to_string() ) );
  assert!( opts.append_reset );
}

#[ test ]
fn options_default()
{
  let opts = TruncateOptions::default();
  assert_eq!( opts.max_width, 80 );
  assert!( opts.suffix.is_none() );
  assert!( !opts.append_reset );
}

// ==================== truncate tests ====================

#[ test ]
fn truncate_no_change_when_fits()
{
  let opts = TruncateOptions::new( 10 );
  assert_eq!( truncate( "hello", &opts ), "hello" );
}

#[ test ]
fn truncate_plain_text()
{
  let opts = TruncateOptions::new( 5 );
  assert_eq!( truncate( "hello world", &opts ), "hello" );
}

#[ test ]
fn truncate_with_suffix()
{
  let opts = TruncateOptions::new( 8 ).with_suffix( "..." );
  assert_eq!( truncate( "hello world", &opts ), "hello..." );
}

#[ test ]
fn truncate_with_ellipsis()
{
  let opts = TruncateOptions::new( 6 ).with_suffix( "…" );
  assert_eq!( truncate( "hello world", &opts ), "hello…" );
}

#[ test ]
fn truncate_preserves_ansi()
{
  let opts = TruncateOptions::new( 3 );
  assert_eq!( truncate( "\x1b[31mhello\x1b[0m", &opts ), "\x1b[31mhel" );
}

#[ test ]
fn truncate_with_reset()
{
  let opts = TruncateOptions::new( 3 ).with_reset( true );
  assert_eq!( truncate( "\x1b[31mhello\x1b[0m", &opts ), "\x1b[31mhel\x1b[0m" );
}

#[ test ]
fn truncate_ansi_only_fits()
{
  let opts = TruncateOptions::new( 10 );
  // ANSI codes don't count toward width
  assert_eq!( truncate( "\x1b[31m\x1b[0m", &opts ), "\x1b[31m\x1b[0m" );
}

#[ test ]
fn truncate_multiple_ansi_segments()
{
  let opts = TruncateOptions::new( 5 );
  let input = "\x1b[31mre\x1b[32md green\x1b[0m";
  // "re" (2) + "d gr" (4) = 6 > 5, so truncate
  // Should get: \x1b[31mre\x1b[32md g (5 visible chars)
  assert_eq!( truncate( input, &opts ), "\x1b[31mre\x1b[32md g" );
}

#[ test ]
fn truncate_empty()
{
  let opts = TruncateOptions::new( 5 );
  assert_eq!( truncate( "", &opts ), "" );
}

#[ test ]
fn truncate_suffix_too_long()
{
  // Suffix longer than max_width - should truncate without suffix
  let opts = TruncateOptions::new( 2 ).with_suffix( "..." );
  assert_eq!( truncate( "hello", &opts ), "he" );
}

#[ test ]
fn truncate_unicode_char_based()
{
  let opts = TruncateOptions::new( 2 );
  // Char-based: 日本語 = 3 chars, truncate to 2
  assert_eq!( truncate( "日本語", &opts ), "日本" );
}

// ==================== truncate_unicode tests ====================

#[ cfg( feature = "ansi_unicode" ) ]
#[ test ]
fn truncate_grapheme_emoji()
{
  use the_module::ansi::truncate_unicode;
  let opts = TruncateOptions::new( 2 );
  // 👋🏽 is 1 grapheme, so "👋🏽a" = 2 graphemes
  assert_eq!( truncate_unicode( "👋🏽ab", &opts ), "👋🏽a" );
}

#[ cfg( feature = "ansi_unicode" ) ]
#[ test ]
fn truncate_grapheme_with_ansi()
{
  use the_module::ansi::truncate_unicode;
  let opts = TruncateOptions::new( 2 ).with_reset( true );
  assert_eq!(
    truncate_unicode( "\x1b[33m日本語\x1b[0m", &opts ),
    "\x1b[33m日本\x1b[0m"
  );
}

#[ cfg( feature = "ansi_unicode" ) ]
#[ test ]
fn truncate_grapheme_combining()
{
  use the_module::ansi::truncate_unicode;
  let opts = TruncateOptions::new( 2 );
  // "e\u{0301}" (e + combining acute) is 1 grapheme
  // "e\u{0301}ab" = 3 graphemes, truncate to 2
  assert_eq!( truncate_unicode( "e\u{0301}ab", &opts ), "e\u{0301}a" );
}
