//! Mock terminal implementation for testing
//!
//! ## Purpose
//!
//! Provides MockTerminal test double for non-fragile integration testing.
//! Enables programmatic control of terminal state and key events without
//! requiring actual terminal I/O.
//!
//! ## Design
//!
//! - Implements TerminalOps trait with configurable behavior
//! - Allows pre-programming key event sequences
//! - Captures output for verification
//! - Fully deterministic, no environment dependencies

use multiline_input::
{
  error::Error,
  terminal::TerminalOps,
};
use crossterm::event::{ KeyEvent, KeyCode, KeyModifiers, KeyEventKind };
use std::{ io, time::Duration };
use std::collections::VecDeque;

/// Mock terminal for testing
///
/// Provides programmable terminal behavior for integration tests.
/// All state is explicit and controlled by test code.
pub struct MockTerminal
{
  /// Is this a TTY?
  is_tty: bool,

  /// Terminal size (width, height)
  size: ( u16, u16 ),

  /// Raw mode enabled?
  raw_mode_enabled: bool,

  /// Cursor hidden?
  cursor_hidden: bool,

  /// Pre-programmed key events
  key_events: VecDeque< KeyEvent >,

  /// Captured output
  output: String,
}

impl MockTerminal
{
  /// Create new mock terminal with explicit configuration
  ///
  /// # Parameters
  ///
  /// - `is_tty`: Whether terminal is a TTY
  /// - `size`: Terminal dimensions (width, height)
  ///
  /// # Example
  ///
  /// ```
  /// let terminal = MockTerminal::new( true, ( 80, 24 ) );
  /// ```
  pub fn new( is_tty: bool, size: ( u16, u16 ) ) -> Self
  {
    Self
    {
      is_tty,
      size,
      raw_mode_enabled: false,
      cursor_hidden: false,
      key_events: VecDeque::new(),
      output: String::new(),
    }
  }

  /// Program a key event to be returned by read_key()
  ///
  /// Events are returned in FIFO order.
  ///
  /// # Example
  ///
  /// ```
  /// terminal.push_key( KeyEvent::new( KeyCode::Char( 'h' ), KeyModifiers::NONE ) );
  /// terminal.push_key( KeyEvent::new( KeyCode::Enter, KeyModifiers::NONE ) );
  /// ```
  pub fn push_key( &mut self, event: KeyEvent )
  {
    self.key_events.push_back( event );
  }

  /// Get captured output
  ///
  /// Returns all text written to the terminal via write().
  #[ allow( dead_code ) ]
  pub fn output( &self ) -> &str
  {
    &self.output
  }

  /// Check if raw mode is enabled
  #[ allow( dead_code ) ]
  pub fn is_raw_mode_enabled( &self ) -> bool
  {
    self.raw_mode_enabled
  }

  /// Check if cursor is hidden
  #[ allow( dead_code ) ]
  pub fn is_cursor_hidden( &self ) -> bool
  {
    self.cursor_hidden
  }
}

impl TerminalOps for MockTerminal
{
  fn is_tty( &self ) -> bool
  {
    self.is_tty
  }

  fn size( &self ) -> io::Result< ( u16, u16 ) >
  {
    Ok( self.size )
  }

  fn enable_raw_mode( &mut self ) -> Result< (), Error >
  {
    if !self.is_tty
    {
      return Err( Error::NoTty );
    }

    self.raw_mode_enabled = true;
    Ok( () )
  }

  fn disable_raw_mode( &mut self ) -> Result< (), Error >
  {
    self.raw_mode_enabled = false;
    Ok( () )
  }

  fn clear_screen( &mut self ) -> io::Result< () >
  {
    // Mock: Just mark in output
    self.output.push_str( "[CLEAR_SCREEN]" );
    Ok( () )
  }

  fn clear_line( &mut self ) -> io::Result< () >
  {
    // Mock: Just mark in output
    self.output.push_str( "[CLEAR_LINE]" );
    Ok( () )
  }

  fn move_cursor( &mut self, col: u16, row: u16 ) -> io::Result< () >
  {
    // Mock: Just mark in output
    self.output.push_str( &format!( "[CURSOR({},{})]", col, row ) );
    Ok( () )
  }

  fn hide_cursor( &mut self ) -> io::Result< () >
  {
    self.cursor_hidden = true;
    self.output.push_str( "[HIDE_CURSOR]" );
    Ok( () )
  }

  fn show_cursor( &mut self ) -> io::Result< () >
  {
    self.cursor_hidden = false;
    self.output.push_str( "[SHOW_CURSOR]" );
    Ok( () )
  }

  fn write_str( &mut self, text: &str ) -> io::Result< () >
  {
    self.output.push_str( text );
    Ok( () )
  }

  fn read_key( &mut self, timeout: Option< Duration > ) -> io::Result< KeyEvent >
  {
    if let Some( event ) = self.key_events.pop_front()
    {
      Ok( event )
    }
    else if timeout.is_some()
    {
      Err( io::Error::new( io::ErrorKind::TimedOut, "No key event programmed" ) )
    }
    else
    {
      Err( io::Error::new( io::ErrorKind::UnexpectedEof, "No more key events programmed" ) )
    }
  }
}

impl io::Write for MockTerminal
{
  fn write( &mut self, buf: &[ u8 ] ) -> io::Result< usize >
  {
    let text = String::from_utf8_lossy( buf );
    self.output.push_str( &text );
    Ok( buf.len() )
  }

  fn flush( &mut self ) -> io::Result< () >
  {
    Ok( () )
  }
}

/// Helper to create key events more concisely
pub fn key( code: KeyCode, modifiers: KeyModifiers ) -> KeyEvent
{
  KeyEvent
  {
    code,
    modifiers,
    kind: KeyEventKind::Press,
    state: crossterm::event::KeyEventState::empty(),
  }
}
