#[ allow( unused_imports ) ]
use super::*;
use the_module::ansi::{ has_ansi, has_unclosed_formatting, is_reset_code, is_sgr_code };

// ==================== has_ansi tests ====================

#[ test ]
fn has_ansi_empty()
{
  assert!( !has_ansi( "" ) );
}

#[ test ]
fn has_ansi_plain_text()
{
  assert!( !has_ansi( "hello world" ) );
  assert!( !has_ansi( "no colors here" ) );
}

#[ test ]
fn has_ansi_with_ansi()
{
  assert!( has_ansi( "\x1b[31mred\x1b[0m" ) );
  assert!( has_ansi( "\x1b[0m" ) );
  assert!( has_ansi( "prefix \x1b[32mgreen" ) );
}

#[ test ]
fn has_ansi_lone_escape()
{
  // Lone ESC without '[' is not a valid ANSI CSI sequence
  assert!( !has_ansi( "before\x1bafter" ) );
}

#[ test ]
fn has_ansi_rgb_color()
{
  assert!( has_ansi( "\x1b[38;2;255;128;0morange\x1b[0m" ) );
}

// ==================== has_unclosed_formatting tests ====================

#[ test ]
fn unclosed_empty()
{
  assert!( !has_unclosed_formatting( "" ) );
}

#[ test ]
fn unclosed_plain_text()
{
  assert!( !has_unclosed_formatting( "hello world" ) );
}

#[ test ]
fn unclosed_properly_closed()
{
  assert!( !has_unclosed_formatting( "\x1b[31mred\x1b[0m" ) );
  assert!( !has_unclosed_formatting( "\x1b[1;31mbold\x1b[0m" ) );
}

#[ test ]
fn unclosed_not_closed()
{
  assert!( has_unclosed_formatting( "\x1b[31mred" ) );
  assert!( has_unclosed_formatting( "\x1b[1m" ) );
}

#[ test ]
fn unclosed_reset_only()
{
  // Reset without prior formatting is fine
  assert!( !has_unclosed_formatting( "\x1b[0m" ) );
  assert!( !has_unclosed_formatting( "\x1b[m" ) );
}

#[ test ]
fn unclosed_multiple_sequences()
{
  // Closed in middle, unclosed at end
  assert!( has_unclosed_formatting( "\x1b[31mred\x1b[0m\x1b[32mgreen" ) );

  // All closed
  assert!( !has_unclosed_formatting( "\x1b[31mred\x1b[0m\x1b[32mgreen\x1b[0m" ) );
}

#[ test ]
fn unclosed_reset_abbreviation()
{
  // \x1b[m is equivalent to \x1b[0m
  assert!( !has_unclosed_formatting( "\x1b[31mred\x1b[m" ) );
}

// ==================== helper function tests ====================

#[ test ]
fn is_reset_code_test()
{
  assert!( is_reset_code( "\x1b[0m" ) );
  assert!( is_reset_code( "\x1b[m" ) );
  assert!( !is_reset_code( "\x1b[31m" ) );
  assert!( !is_reset_code( "\x1b[1m" ) );
}

#[ test ]
fn is_sgr_code_test()
{
  assert!( is_sgr_code( "\x1b[0m" ) );
  assert!( is_sgr_code( "\x1b[31m" ) );
  assert!( is_sgr_code( "\x1b[1;31;44m" ) );
  assert!( is_sgr_code( "\x1b[38;2;255;128;0m" ) );
  // Non-SGR CSI sequences
  assert!( !is_sgr_code( "\x1b[2J" ) ); // Clear screen
  assert!( !is_sgr_code( "\x1b[H" ) );  // Cursor home
}
