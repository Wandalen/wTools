#![ cfg( feature = "enabled" ) ]
//! Buffer operations tests
//!
//! ## Domain
//!
//! Tests for TextBuffer operations including:
//! - Text insertion and deletion
//! - Cursor movement and positioning
//! - Multiline editing
//! - Unicode character handling
//!
//! ## Organization
//!
//! Tests migrated from `src/buffer.rs` module tests.
//! Organized by functional domain rather than source file location.

use multiline_input::buffer::TextBuffer;

#[ test ]
fn test_new_buffer()
{
  let buffer = TextBuffer::new();
  assert_eq!( buffer.text(), "" );
  assert_eq!( buffer.line_count(), 1 );
  assert_eq!( buffer.cursor_position(), ( 0, 0 ) );
}

#[ test ]
fn test_insert_char()
{
  let mut buffer = TextBuffer::new();
  buffer.insert_char( 'h' );
  buffer.insert_char( 'i' );
  assert_eq!( buffer.text(), "hi" );
  assert_eq!( buffer.cursor_position(), ( 0, 2 ) );
}

#[ test ]
fn test_insert_newline()
{
  let mut buffer = TextBuffer::new();
  buffer.insert_char( 'h' );
  buffer.insert_char( 'i' );
  buffer.insert_newline();
  buffer.insert_char( 'b' );
  buffer.insert_char( 'y' );
  assert_eq!( buffer.text(), "hi\nby" );
  assert_eq!( buffer.line_count(), 2 );
}

#[ test ]
fn test_delete_char_before()
{
  let mut buffer = TextBuffer::new();
  buffer.insert_char( 'h' );
  buffer.insert_char( 'i' );
  buffer.delete_char_before();
  assert_eq!( buffer.text(), "h" );
  assert_eq!( buffer.cursor_position(), ( 0, 1 ) );
}

#[ test ]
fn test_delete_newline()
{
  let mut buffer = TextBuffer::with_text( "hi\nbye" );
  buffer.cursor_line = 1;
  buffer.cursor_col = 0;
  buffer.delete_char_before();
  assert_eq!( buffer.text(), "hibye" );
  assert_eq!( buffer.line_count(), 1 );
}

#[ test ]
fn test_cursor_movement()
{
  let mut buffer = TextBuffer::with_text( "hello\nworld" );

  // Move right
  buffer.move_right();
  assert_eq!( buffer.cursor_position(), ( 0, 1 ) );

  // Move down
  buffer.move_down();
  assert_eq!( buffer.cursor_position(), ( 1, 1 ) );

  // Move left
  buffer.move_left();
  assert_eq!( buffer.cursor_position(), ( 1, 0 ) );

  // Move up
  buffer.move_up();
  assert_eq!( buffer.cursor_position(), ( 0, 0 ) );
}

#[ test ]
fn test_unicode_handling()
{
  let mut buffer = TextBuffer::new();
  buffer.insert_char( '你' );
  buffer.insert_char( '好' );
  assert_eq!( buffer.text(), "你好" );
  assert_eq!( buffer.cursor_position(), ( 0, 2 ) );

  buffer.delete_char_before();
  assert_eq!( buffer.text(), "你" );
}
