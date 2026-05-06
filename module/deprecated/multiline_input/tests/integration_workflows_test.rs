//! Integration workflow tests
//!
//! ## Domain
//!
//! End-to-end workflow tests using MockTerminal for deterministic testing:
//! - Submit workflow (single line)
//! - Cancel workflow (ESC)
//! - Multiline editing workflow
//! - Validation workflow
//!
//! ## Design
//!
//! Tests use MockTerminal to avoid environment dependencies.
//! All terminal state and key events are explicitly programmed.

mod common;

use multiline_input::Builder;
use common::mock_terminal::{ MockTerminal, key };
use crossterm::event::{ KeyCode, KeyModifiers };

#[ test ]
fn test_submit_single_line_workflow()
{
  // ARRANGE: Create mock terminal with explicit state
  let mut terminal = MockTerminal::new(
    true,              // is_tty = true (explicit)
    ( 80, 24 )         // size = 80x24 (explicit)
  );

  // Program key sequence: h → e → l → l → o → ENTER
  terminal.push_key( key( KeyCode::Char( 'h' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'e' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'l' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'l' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'o' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Enter, KeyModifiers::NONE ) );

  let editor = Builder::new()
    .prompt( "Input:" )
    .build_with( terminal );

  // ACT: Collect input
  let result = editor.collect();

  // ASSERT: Verify result
  assert!( result.is_ok() );
  let text = result.unwrap();
  assert_eq!( text, Some( "hello".to_string() ) );
}

#[ test ]
fn test_cancel_on_esc_workflow()
{
  // ARRANGE: Create mock terminal
  let mut terminal = MockTerminal::new( true, ( 80, 24 ) );

  // Program key sequence: h → e → ESC
  terminal.push_key( key( KeyCode::Char( 'h' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'e' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Esc, KeyModifiers::NONE ) );

  let editor = Builder::new()
    .prompt( "Input:" )
    .build_with( terminal );

  // ACT: Collect input
  let result = editor.collect();

  // ASSERT: Verify cancellation
  assert!( result.is_ok() );
  let text = result.unwrap();
  assert_eq!( text, None );
}

#[ test ]
fn test_multiline_editing_workflow()
{
  // ARRANGE: Create mock terminal
  let mut terminal = MockTerminal::new( true, ( 80, 24 ) );

  // Program key sequence: h → i → CTRL+ENTER → b → y → ENTER
  terminal.push_key( key( KeyCode::Char( 'h' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'i' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Enter, KeyModifiers::CONTROL ) );
  terminal.push_key( key( KeyCode::Char( 'b' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'y' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Enter, KeyModifiers::NONE ) );

  let editor = Builder::new()
    .prompt( "Input:" )
    .build_with( terminal );

  // ACT: Collect input
  let result = editor.collect();

  // ASSERT: Verify multiline result
  assert!( result.is_ok() );
  let text = result.unwrap();
  assert_eq!( text, Some( "hi\nby".to_string() ) );
}

#[ test ]
fn test_validation_empty_workflow()
{
  // ARRANGE: Create mock terminal
  let mut terminal = MockTerminal::new( true, ( 80, 24 ) );

  // Program key sequence: ENTER (submit empty) → h → i → ENTER (submit valid)
  terminal.push_key( key( KeyCode::Enter, KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'h' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'i' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Enter, KeyModifiers::NONE ) );

  let editor = Builder::new()
    .prompt( "Input:" )
    .allow_empty( false )  // Reject empty input
    .build_with( terminal );

  // ACT: Collect input
  let result = editor.collect();

  // ASSERT: Verify validation rejected empty, then accepted "hi"
  assert!( result.is_ok() );
  let text = result.unwrap();
  assert_eq!( text, Some( "hi".to_string() ) );
}

#[ test ]
fn test_shift_enter_inserts_newline_workflow()
{
  // ARRANGE: Create mock terminal
  let mut terminal = MockTerminal::new( true, ( 80, 24 ) );

  // Program key sequence: h → i → SHIFT+ENTER → b → y → ENTER
  terminal.push_key( key( KeyCode::Char( 'h' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'i' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Enter, KeyModifiers::SHIFT ) );
  terminal.push_key( key( KeyCode::Char( 'b' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'y' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Enter, KeyModifiers::NONE ) );

  let editor = Builder::new()
    .prompt( "Input:" )
    .build_with( terminal );

  // ACT: Collect input
  let result = editor.collect();

  // ASSERT: Verify SHIFT+ENTER inserted newline
  assert!( result.is_ok() );
  let text = result.unwrap();
  assert_eq!( text, Some( "hi\nby".to_string() ) );
}
