#[ allow( unused_imports ) ]
use super::*;
use the_module::ansi::{ visual_len, pad_to_width };

// ==================== visual_len tests ====================

#[ test ]
fn visual_len_empty()
{
  assert_eq!( visual_len( "" ), 0 );
}

#[ test ]
fn visual_len_plain_text()
{
  assert_eq!( visual_len( "hello" ), 5 );
  assert_eq!( visual_len( "hello world" ), 11 );
}

#[ test ]
fn visual_len_ansi_only()
{
  assert_eq!( visual_len( "\x1b[31m" ), 0 );
  assert_eq!( visual_len( "\x1b[0m" ), 0 );
  assert_eq!( visual_len( "\x1b[1;31;44m" ), 0 );
}

#[ test ]
fn visual_len_ansi_with_text()
{
  assert_eq!( visual_len( "\x1b[31mred\x1b[0m" ), 3 );
  assert_eq!( visual_len( "\x1b[1;31mbold red\x1b[0m" ), 8 );
}

#[ test ]
fn visual_len_multiple_ansi()
{
  assert_eq!( visual_len( "\x1b[1m\x1b[31mtest\x1b[0m" ), 4 );
}

#[ test ]
fn visual_len_unicode_codepoints()
{
  // Tier 1: char-based - counts codepoints
  assert_eq!( visual_len( "日本語" ), 3 );
  // Emoji may be multiple codepoints in char-based counting
  assert_eq!( visual_len( "🎉" ), 1 );
}

// ==================== pad_to_width tests ====================

#[ test ]
fn pad_left_align()
{
  assert_eq!( pad_to_width( "hi", 5, false ), "hi   " );
  assert_eq!( pad_to_width( "test", 10, false ), "test      " );
}

#[ test ]
fn pad_right_align()
{
  assert_eq!( pad_to_width( "hi", 5, true ), "   hi" );
  assert_eq!( pad_to_width( "test", 10, true ), "      test" );
}

#[ test ]
fn pad_no_change_when_equal_or_larger()
{
  assert_eq!( pad_to_width( "hello", 5, false ), "hello" );
  assert_eq!( pad_to_width( "hello world", 5, false ), "hello world" );
}

#[ test ]
fn pad_with_ansi()
{
  let result = pad_to_width( "\x1b[31mhi\x1b[0m", 5, false );
  assert_eq!( result, "\x1b[31mhi\x1b[0m   " );

  let result = pad_to_width( "\x1b[31mhi\x1b[0m", 5, true );
  assert_eq!( result, "   \x1b[31mhi\x1b[0m" );
}

#[ test ]
fn pad_empty_string()
{
  assert_eq!( pad_to_width( "", 3, false ), "   " );
  assert_eq!( pad_to_width( "", 3, true ), "   " );
}

// ==================== visual_len_unicode tests ====================

#[ cfg( feature = "ansi_unicode" ) ]
#[ test ]
fn grapheme_emoji_with_modifier()
{
  use the_module::ansi::visual_len_unicode;
  // Emoji with skin tone modifier = 1 grapheme
  assert_eq!( visual_len_unicode( "👋🏽" ), 1 );
}

#[ cfg( feature = "ansi_unicode" ) ]
#[ test ]
fn grapheme_flag_emoji()
{
  use the_module::ansi::visual_len_unicode;
  // Flag emoji (2 regional indicators = 1 grapheme)
  assert_eq!( visual_len_unicode( "🇺🇸" ), 1 );
}

#[ cfg( feature = "ansi_unicode" ) ]
#[ test ]
fn grapheme_combining_marks()
{
  use the_module::ansi::visual_len_unicode;
  // e + combining acute accent = 1 grapheme
  assert_eq!( visual_len_unicode( "e\u{0301}" ), 1 );
}

#[ cfg( feature = "ansi_unicode" ) ]
#[ test ]
fn grapheme_cjk()
{
  use the_module::ansi::visual_len_unicode;
  assert_eq!( visual_len_unicode( "日本語" ), 3 );
}

#[ cfg( feature = "ansi_unicode" ) ]
#[ test ]
fn grapheme_with_ansi()
{
  use the_module::ansi::visual_len_unicode;
  assert_eq!( visual_len_unicode( "\x1b[33m日本語\x1b[0m" ), 3 );
  assert_eq!( visual_len_unicode( "\x1b[31m👋🏽\x1b[0m" ), 1 );
}
