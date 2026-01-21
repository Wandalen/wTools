//! Error Path Tests
//!
//! ## Purpose
//!
//! Tests all error scenarios and failure modes for multiline_input editor.
//! Ensures graceful error handling and proper error propagation.
//!
//! ## Coverage
//!
//! - **NoTty Error**: stdin not a TTY (e.g., input redirection)
//! - **TerminalTooSmall Error**: Terminal dimensions below minimum (20x3)
//! - **I/O Errors**: Failures during terminal operations
//! - **Raw Mode Errors**: Failures enabling/disabling raw mode
//!
//! ## Test Strategy
//!
//! Uses MockTerminal to simulate error conditions:
//! - Configure MockTerminal with is_tty=false for NoTty tests
//! - Configure small dimensions for TerminalTooSmall tests
//! - Program MockTerminal to return I/O errors
//!
//! ## Anti-patterns
//!
//! - ❌ Relying on actual environment (e.g., redirecting stdin)
//! - ❌ Skipping tests based on environment
//! - ✅ Using MockTerminal for deterministic error injection

mod common;

use multiline_input::
{
  builder::Builder,
  error::Error,
};
use common::mock_terminal::MockTerminal;
use crossterm::event::{ KeyCode, KeyModifiers };

// ============================================================================
// NoTty Error Tests
// ============================================================================

/// Test NoTty error when stdin is not a TTY
///
/// ## Root Cause
/// Editor requires interactive terminal (TTY). When stdin is redirected
/// (e.g., `program < file`), is_tty() returns false.
///
/// ## Why Not Caught
/// No previous test verified behavior in non-TTY environment.
///
/// ## Fix Applied
/// MockTerminal allows configuring is_tty=false to simulate stdin redirect.
///
/// ## Prevention
/// All error paths must have explicit test coverage.
///
/// ## Pitfall
/// Forgetting to test non-interactive environments leads to confusing
/// failures when users pipe input.
#[ test ]
fn test_error_no_tty()
{
  // Given: Terminal that is NOT a TTY (simulates stdin redirect)
  let terminal = MockTerminal::new( false, ( 80, 24 ) );

  // When: Attempt to collect input
  let editor = Builder::new().build_with( terminal );
  let result = editor.collect();

  // Then: Should return NoTty error
  assert!( matches!( result, Err( Error::NoTty ) ), "Expected NoTty error, got {:?}", result );
}

/// Test NoTty error message is helpful
///
/// Verifies error can be displayed to user with clear message.
#[ test ]
fn test_error_no_tty_message()
{
  // Given: NoTty error
  let error = Error::NoTty;

  // When: Convert to string
  let message = format!( "{}", error );

  // Then: Should contain helpful description
  assert!( message.contains( "TTY" ) || message.contains( "terminal" ), "Error message should mention TTY: {}", message );
}

// ============================================================================
// TerminalTooSmall Error Tests
// ============================================================================

/// Test TerminalTooSmall error when width below minimum
///
/// ## Root Cause
/// Editor requires minimum 20 columns for prompt + text. Below this,
/// rendering would be illegible or cause panic.
///
/// ## Why Not Caught
/// No test verified minimum width enforcement.
///
/// ## Fix Applied
/// render() validates terminal size before rendering and returns
/// TerminalTooSmall with dimensions.
///
/// ## Prevention
/// Test boundary conditions (exact minimum, one below minimum).
///
/// ## Pitfall
/// Without size validation, rendering to tiny terminals causes garbled
/// output or crashes.
#[ test ]
fn test_error_terminal_too_narrow()
{
  // Given: Terminal width below minimum (19 < 20)
  let mut terminal = MockTerminal::new( true, ( 19, 24 ) );

  // Program one key to trigger render
  terminal.push_key( common::mock_terminal::key( KeyCode::Char( 'a' ), KeyModifiers::NONE ) );

  // When: Attempt to collect input
  let editor = Builder::new().build_with( terminal );
  let result = editor.collect();

  // Then: Should return TerminalTooSmall error
  match result
  {
    Err( Error::TerminalTooSmall { width, height, min_width, min_height } ) =>
    {
      assert_eq!( width, 19, "Should report actual width" );
      assert_eq!( height, 24, "Should report actual height" );
      assert_eq!( min_width, 20, "Should report required minimum width" );
      assert_eq!( min_height, 3, "Should report required minimum height" );
    }
    other => panic!( "Expected TerminalTooSmall error, got {:?}", other ),
  }
}

/// Test TerminalTooSmall error when height below minimum
///
/// Minimum height is 3 rows (prompt + text + status/cursor).
#[ test ]
fn test_error_terminal_too_short()
{
  // Given: Terminal height below minimum (2 < 3)
  let mut terminal = MockTerminal::new( true, ( 80, 2 ) );

  // Program one key to trigger render
  terminal.push_key( common::mock_terminal::key( KeyCode::Char( 'a' ), KeyModifiers::NONE ) );

  // When: Attempt to collect input
  let editor = Builder::new().build_with( terminal );
  let result = editor.collect();

  // Then: Should return TerminalTooSmall error
  match result
  {
    Err( Error::TerminalTooSmall { width, height, min_width, min_height } ) =>
    {
      assert_eq!( width, 80, "Should report actual width" );
      assert_eq!( height, 2, "Should report actual height" );
      assert_eq!( min_width, 20, "Should report required minimum width" );
      assert_eq!( min_height, 3, "Should report required minimum height" );
    }
    other => panic!( "Expected TerminalTooSmall error, got {:?}", other ),
  }
}

/// Test TerminalTooSmall error when both dimensions below minimum
///
/// Verifies error reported when terminal is too small in both dimensions.
#[ test ]
fn test_error_terminal_too_small_both()
{
  // Given: Terminal below minimum in both dimensions
  let mut terminal = MockTerminal::new( true, ( 10, 1 ) );

  // Program one key to trigger render
  terminal.push_key( common::mock_terminal::key( KeyCode::Char( 'a' ), KeyModifiers::NONE ) );

  // When: Attempt to collect input
  let editor = Builder::new().build_with( terminal );
  let result = editor.collect();

  // Then: Should return TerminalTooSmall error
  match result
  {
    Err( Error::TerminalTooSmall { width, height, .. } ) =>
    {
      assert_eq!( width, 10, "Should report actual width" );
      assert_eq!( height, 1, "Should report actual height" );
    }
    other => panic!( "Expected TerminalTooSmall error, got {:?}", other ),
  }
}

/// Test terminal at exact minimum size works (boundary condition)
///
/// Verifies that minimum size (20x3) is sufficient for basic operation.
#[ test ]
fn test_terminal_exactly_minimum_size()
{
  // Given: Terminal at exact minimum (20x3)
  let mut terminal = MockTerminal::new( true, ( 20, 3 ) );

  // Program workflow: type 'a' then submit
  terminal.push_key( common::mock_terminal::key( KeyCode::Char( 'a' ), KeyModifiers::NONE ) );
  terminal.push_key( common::mock_terminal::key( KeyCode::Enter, KeyModifiers::NONE ) );

  // When: Collect input
  let editor = Builder::new().build_with( terminal );
  let result = editor.collect();

  // Then: Should succeed (minimum size is sufficient)
  assert!( result.is_ok(), "Minimum size (20x3) should be sufficient, got {:?}", result );
  assert_eq!( result.unwrap(), Some( "a".to_string() ) );
}

/// Test TerminalTooSmall error message includes dimensions
///
/// Verifies error message is helpful for debugging.
#[ test ]
fn test_error_terminal_too_small_message()
{
  // Given: TerminalTooSmall error
  let error = Error::TerminalTooSmall
  {
    width: 10,
    height: 2,
    min_width: 20,
    min_height: 3,
  };

  // When: Convert to string
  let message = format!( "{}", error );

  // Then: Should contain dimensions
  assert!( message.contains( "10" ), "Should mention actual width: {}", message );
  assert!( message.contains( "2" ), "Should mention actual height: {}", message );
  assert!( message.contains( "20" ), "Should mention required width: {}", message );
  assert!( message.contains( "3" ), "Should mention required height: {}", message );
}

// ============================================================================
// Raw Mode Error Tests
// ============================================================================

/// Test raw mode enable failure on non-TTY
///
/// When enable_raw_mode() is called on non-TTY, should return NoTty error.
#[ test ]
fn test_error_raw_mode_enable_non_tty()
{
  // Given: Non-TTY terminal
  let terminal = MockTerminal::new( false, ( 80, 24 ) );

  // When: Attempt to collect (which enables raw mode)
  let editor = Builder::new().build_with( terminal );
  let result = editor.collect();

  // Then: Should fail with NoTty before any rendering
  assert!( matches!( result, Err( Error::NoTty ) ), "Expected NoTty error, got {:?}", result );
}

// ============================================================================
// Error Display Tests
// ============================================================================

/// Test all error variants can be displayed
///
/// Verifies Display trait implementation for all Error variants.
#[ test ]
fn test_error_display_all_variants()
{
  // NoTty error
  let no_tty = Error::NoTty;
  let msg = format!( "{}", no_tty );
  assert!( !msg.is_empty(), "NoTty error should have non-empty message" );

  // TerminalTooSmall error
  let too_small = Error::TerminalTooSmall { width: 10, height: 2, min_width: 20, min_height: 3 };
  let msg = format!( "{}", too_small );
  assert!( !msg.is_empty(), "TerminalTooSmall error should have non-empty message" );

  // Io error
  let io_error = Error::Io( std::io::Error::new( std::io::ErrorKind::BrokenPipe, "test" ) );
  let msg = format!( "{}", io_error );
  assert!( !msg.is_empty(), "Io error should have non-empty message" );

  // ValidationFailed error
  let validation_error = Error::ValidationFailed( "test validation".to_string() );
  let msg = format!( "{}", validation_error );
  assert!( msg.contains( "test validation" ), "ValidationFailed should include custom message" );
}

/// Test error source() implementation
///
/// Verifies std::error::Error::source() for Error::Io variant.
#[ test ]
fn test_error_source()
{
  use std::error::Error as StdError;

  // Given: I/O error wrapped in our Error
  let inner = std::io::Error::new( std::io::ErrorKind::BrokenPipe, "pipe broken" );
  let error = Error::Io( inner );

  // When: Get source
  let source = error.source();

  // Then: Should return inner I/O error
  assert!( source.is_some(), "Error::Io should have source" );
}
