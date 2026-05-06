#![ cfg( feature = "enabled" ) ]
//! Key event handling tests
//!
//! ## Domain
//!
//! Tests for key event parsing and action mapping:
//! - Submit actions (ENTER, CTRL+D)
//! - Cancel actions (ESC, CTRL+C)
//! - Text editing (character insertion, backspace)
//! - Cursor movement (arrows, home/end)
//! - Special key combinations (CTRL+ENTER for newline)
//!
//! ## Organization
//!
//! Tests migrated from `src/keys.rs` module tests.

use multiline_input::
{
  buffer::TextBuffer,
  keys::{ handle_key, KeyAction },
};
use crossterm::event::{ KeyCode, KeyModifiers, KeyEvent };

fn key( code: KeyCode, modifiers: KeyModifiers ) -> KeyEvent
{
  KeyEvent::new( code, modifiers )
}

#[ test ]
fn test_submit_on_enter()
{
  let mut buffer = TextBuffer::new();
  let action = handle_key( key( KeyCode::Enter, KeyModifiers::NONE ), &mut buffer );
  assert_eq!( action, KeyAction::Submit );
}

#[ test ]
fn test_newline_on_ctrl_enter()
{
  let mut buffer = TextBuffer::new();
  buffer.insert_char( 'h' );
  buffer.insert_char( 'i' );

  let action = handle_key( key( KeyCode::Enter, KeyModifiers::CONTROL ), &mut buffer );
  assert_eq!( action, KeyAction::Continue );
  assert_eq!( buffer.line_count(), 2 );
  assert_eq!( buffer.text(), "hi\n" );
}

#[ test ]
fn test_newline_on_shift_enter()
{
  let mut buffer = TextBuffer::new();
  buffer.insert_char( 'h' );
  buffer.insert_char( 'e' );
  buffer.insert_char( 'l' );
  buffer.insert_char( 'l' );
  buffer.insert_char( 'o' );

  let action = handle_key( key( KeyCode::Enter, KeyModifiers::SHIFT ), &mut buffer );
  assert_eq!( action, KeyAction::Continue );
  assert_eq!( buffer.line_count(), 2 );
  assert_eq!( buffer.text(), "hello\n" );
}

#[ test ]
fn test_newline_on_ctrl_shift_enter()
{
  let mut buffer = TextBuffer::new();
  buffer.insert_char( 't' );
  buffer.insert_char( 'e' );
  buffer.insert_char( 's' );
  buffer.insert_char( 't' );

  // Both CTRL and SHIFT pressed together
  let mods = KeyModifiers::CONTROL | KeyModifiers::SHIFT;
  let action = handle_key( key( KeyCode::Enter, mods ), &mut buffer );
  assert_eq!( action, KeyAction::Continue );
  assert_eq!( buffer.line_count(), 2 );
  assert_eq!( buffer.text(), "test\n" );
}

#[ test ]
fn test_cancel_on_esc()
{
  let mut buffer = TextBuffer::new();
  let action = handle_key( key( KeyCode::Esc, KeyModifiers::NONE ), &mut buffer );
  assert_eq!( action, KeyAction::Cancel );
}

#[ test ]
fn test_cancel_on_ctrl_c()
{
  let mut buffer = TextBuffer::new();
  let action = handle_key(
    key( KeyCode::Char( 'c' ), KeyModifiers::CONTROL ),
    &mut buffer
  );
  assert_eq!( action, KeyAction::Cancel );
}

#[ test ]
fn test_char_insertion()
{
  let mut buffer = TextBuffer::new();
  handle_key( key( KeyCode::Char( 'a' ), KeyModifiers::NONE ), &mut buffer );
  handle_key( key( KeyCode::Char( 'b' ), KeyModifiers::NONE ), &mut buffer );
  assert_eq!( buffer.text(), "ab" );
}

#[ test ]
fn test_backspace()
{
  let mut buffer = TextBuffer::new();
  buffer.insert_char( 'a' );
  buffer.insert_char( 'b' );

  handle_key( key( KeyCode::Backspace, KeyModifiers::NONE ), &mut buffer );
  assert_eq!( buffer.text(), "a" );
}

#[ test ]
fn test_cursor_movement()
{
  let mut buffer = TextBuffer::new();
  buffer.insert_char( 'h' );
  buffer.insert_char( 'i' );

  handle_key( key( KeyCode::Left, KeyModifiers::NONE ), &mut buffer );
  assert_eq!( buffer.cursor_position(), ( 0, 1 ) );

  handle_key( key( KeyCode::Right, KeyModifiers::NONE ), &mut buffer );
  assert_eq!( buffer.cursor_position(), ( 0, 2 ) );
}
