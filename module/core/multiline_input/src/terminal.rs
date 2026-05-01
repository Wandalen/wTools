//! Terminal abstraction and raw mode management
//!
//! ## Architecture
//!
//! This module provides terminal operations through a trait-based abstraction:
//! - **TerminalOps trait** - Defines all terminal operations
//! - **RealTerminal** - Production implementation using crossterm
//! - **MockTerminal** - Test double for testing (tests/ directory)
//!
//! This design enables dependency injection and comprehensive testing.

use crossterm::
{
  terminal::{ self, ClearType },
  cursor,
  execute,
  event::{ self, Event, KeyEvent },
};
use std::io::{ self, Write, IsTerminal };
use std::time::Duration;
use crate::error::Error;

/// Terminal operations abstraction
///
/// Defines all terminal operations needed for multiline input editing.
/// Enables dependency injection and testing with mock implementations.
///
/// Note: Implementations must also implement `std::io::Write` for write/flush operations.
pub trait TerminalOps: Write
{
  /// Check if running in a TTY
  fn is_tty( &self ) -> bool;

  /// Get terminal size (width, height)
  fn size( &self ) -> io::Result< ( u16, u16 ) >;

  /// Enter raw terminal mode
  fn enable_raw_mode( &mut self ) -> Result< (), Error >;

  /// Exit raw terminal mode
  fn disable_raw_mode( &mut self ) -> Result< (), Error >;

  /// Clear entire screen
  fn clear_screen( &mut self ) -> io::Result< () >;

  /// Clear current line
  fn clear_line( &mut self ) -> io::Result< () >;

  /// Move cursor to position (column, row) - 0-indexed
  fn move_cursor( &mut self, col: u16, row: u16 ) -> io::Result< () >;

  /// Hide cursor
  fn hide_cursor( &mut self ) -> io::Result< () >;

  /// Show cursor
  fn show_cursor( &mut self ) -> io::Result< () >;

  /// Write string to terminal (convenience wrapper for write_all)
  fn write_str( &mut self, text: &str ) -> io::Result< () >;

  /// Read next key event (blocks until key pressed or timeout)
  fn read_key( &mut self, timeout: Option< Duration > ) -> io::Result< KeyEvent >;
}

/// Check if running in a TTY
pub fn is_tty() -> bool
{
  io::stdin().is_terminal()
}

/// Get terminal size (width, height)
pub fn size() -> io::Result< ( u16, u16 ) >
{
  terminal::size()
}

/// RAII guard for raw terminal mode
///
/// Automatically restores terminal state when dropped
pub struct RawModeGuard
{
  was_enabled: bool,
}

impl RawModeGuard
{
  /// Enter raw mode
  pub fn enable() -> Result< Self, Error >
  {
    if !is_tty()
    {
      return Err( Error::NoTty );
    }

    terminal::enable_raw_mode()?;

    Ok( Self { was_enabled: true } )
  }

  /// Check if raw mode is currently enabled
  pub fn is_enabled( &self ) -> bool
  {
    self.was_enabled
  }
}

impl Drop for RawModeGuard
{
  fn drop( &mut self )
  {
    if self.was_enabled
    {
      let _ = terminal::disable_raw_mode();
    }
  }
}

/// Production terminal implementation using crossterm
pub struct RealTerminal
{
  stdout: io::Stdout,
  raw_mode_enabled: bool,
}

impl RealTerminal
{
  /// Create new real terminal
  pub fn new() -> Self
  {
    Self
    {
      stdout: io::stdout(),
      raw_mode_enabled: false,
    }
  }

}

impl TerminalOps for RealTerminal
{
  fn is_tty( &self ) -> bool
  {
    is_tty()
  }

  fn size( &self ) -> io::Result< ( u16, u16 ) >
  {
    size()
  }

  fn enable_raw_mode( &mut self ) -> Result< (), Error >
  {
    if !self.is_tty()
    {
      return Err( Error::NoTty );
    }

    terminal::enable_raw_mode()?;
    self.raw_mode_enabled = true;
    Ok( () )
  }

  fn disable_raw_mode( &mut self ) -> Result< (), Error >
  {
    if self.raw_mode_enabled
    {
      terminal::disable_raw_mode()?;
      self.raw_mode_enabled = false;
    }
    Ok( () )
  }

  fn clear_screen( &mut self ) -> io::Result< () >
  {
    execute!( self.stdout, terminal::Clear( ClearType::All ) )?;
    Ok( () )
  }

  fn clear_line( &mut self ) -> io::Result< () >
  {
    execute!( self.stdout, terminal::Clear( ClearType::CurrentLine ) )?;
    Ok( () )
  }

  fn move_cursor( &mut self, col: u16, row: u16 ) -> io::Result< () >
  {
    execute!( self.stdout, cursor::MoveTo( col, row ) )?;
    Ok( () )
  }

  fn hide_cursor( &mut self ) -> io::Result< () >
  {
    execute!( self.stdout, cursor::Hide )?;
    Ok( () )
  }

  fn show_cursor( &mut self ) -> io::Result< () >
  {
    execute!( self.stdout, cursor::Show )?;
    Ok( () )
  }

  fn write_str( &mut self, text: &str ) -> io::Result< () >
  {
    self.stdout.write_all( text.as_bytes() )?;
    Ok( () )
  }

  fn read_key( &mut self, timeout: Option< Duration > ) -> io::Result< KeyEvent >
  {
    loop
    {
      let has_event = if let Some( timeout ) = timeout
      {
        event::poll( timeout )?
      }
      else
      {
        // Block indefinitely
        event::poll( Duration::from_secs( 86400 ) )?
      };

      if has_event
      {
        if let Event::Key( key_event ) = event::read()?
        {
          // Only handle key press events, ignore release
          if key_event.kind == event::KeyEventKind::Press
          {
            return Ok( key_event );
          }
        }
      }
      else if timeout.is_some()
      {
        // Timeout occurred
        return Err( io::Error::new( io::ErrorKind::TimedOut, "No key event" ) );
      }
    }
  }
}

impl Drop for RealTerminal
{
  fn drop( &mut self )
  {
    // Ensure raw mode is disabled on drop
    let _ = self.disable_raw_mode();
  }
}

impl Default for RealTerminal
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl io::Write for RealTerminal
{
  fn write( &mut self, buf: &[ u8 ] ) -> io::Result< usize >
  {
    self.stdout.write( buf )
  }

  fn flush( &mut self ) -> io::Result< () >
  {
    self.stdout.flush()
  }
}
