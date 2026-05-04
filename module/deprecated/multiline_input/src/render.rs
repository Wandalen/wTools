//! Screen rendering for multiline editor

use crate::
{
  buffer::TextBuffer,
  terminal::TerminalOps,
  error::Error,
};
use crossterm::style::{ Color, SetForegroundColor, ResetColor };

/// Rendering configuration
#[derive( Debug, Clone )]
pub struct RenderConfig
{
  /// Show line numbers
  pub show_line_numbers: bool,

  /// Show status line (line/col/chars)
  pub show_status: bool,

  /// Show character count
  pub show_char_count: bool,

  /// Enable colors
  pub color: bool,

  /// Prompt to display
  pub prompt: String,

  /// Placeholder text when empty
  pub placeholder: Option< String >,
}

impl Default for RenderConfig
{
  fn default() -> Self
  {
    Self
    {
      show_line_numbers: false,
      show_status: false,
      show_char_count: false,
      color: true,
      prompt: String::new(),
      placeholder: None,
    }
  }
}

/// Minimum terminal dimensions for rendering.
///
/// Terminal must be at least this size to render the editor UI.
const MIN_WIDTH: u16 = 20;   // Enough for prompt + minimal text
const MIN_HEIGHT: u16 = 3;   // Prompt + 1 line + status/cursor

/// Render editor state to terminal.
///
/// Returns `Error::TerminalTooSmall` if terminal dimensions are below minimum.
pub fn render< T >(
  terminal: &mut T,
  buffer: &TextBuffer,
  config: &RenderConfig,
) -> Result< (), Error >
where
  T: TerminalOps,
{
  let ( term_width, term_height ) = terminal.size()?;

  // Validate terminal size before rendering
  if term_width < MIN_WIDTH || term_height < MIN_HEIGHT
  {
    return Err( Error::TerminalTooSmall
    {
      width: term_width,
      height: term_height,
      min_width: MIN_WIDTH,
      min_height: MIN_HEIGHT,
    } );
  }

  // Move to start and clear screen
  terminal.move_cursor( 0, 0 )?;
  terminal.clear_screen()?;

  let mut row = 0;

  // Render prompt
  if !config.prompt.is_empty()
  {
    terminal.move_cursor( 0, row )?;
    if config.color
    {
      terminal.write_str( &format!( "{}", SetForegroundColor( Color::Cyan ) ) )?;
    }
    terminal.write_str( &config.prompt )?;
    if config.color
    {
      terminal.write_str( &format!( "{}", ResetColor ) )?;
    }
    row += 1;
  }

  // Render text lines
  let line_count = buffer.line_count();
  let line_number_width = if config.show_line_numbers
  {
    format!( "{}", line_count ).len() + 2
  }
  else
  {
    0
  };

  for ( idx, line ) in buffer.lines().iter().enumerate()
  {
    if row >= term_height - 1
    {
      break; // No room for more lines
    }

    terminal.move_cursor( 0, row )?;

    // Render line number
    if config.show_line_numbers
    {
      if config.color
      {
        terminal.write_str( &format!( "{}", SetForegroundColor( Color::DarkGrey ) ) )?;
      }
      let line_num = format!( "{:>width$} ", idx + 1, width = line_number_width - 1 );
      terminal.write_str( &line_num )?;
      if config.color
      {
        terminal.write_str( &format!( "{}", ResetColor ) )?;
      }
    }

    // Render line content
    if line.is_empty() && buffer.line_count() == 1
    {
      // Show placeholder for empty buffer
      if let Some( placeholder ) = &config.placeholder
      {
        if config.color
        {
          terminal.write_str( &format!( "{}", SetForegroundColor( Color::DarkGrey ) ) )?;
        }
        terminal.write_str( placeholder )?;
        if config.color
        {
          terminal.write_str( &format!( "{}", ResetColor ) )?;
        }
      }
    }
    else
    {
      terminal.write_str( line )?;
    }

    row += 1;
  }

  // Render status line
  if config.show_status && row < term_height
  {
    terminal.move_cursor( 0, row )?;
    terminal.clear_line()?;

    if config.color
    {
      terminal.write_str( &format!( "{}", SetForegroundColor( Color::DarkGrey ) ) )?;
    }

    let ( cursor_line, cursor_col ) = buffer.cursor_position();
    let status = if config.show_char_count
    {
      format!(
        "Line {}/{} Col {} | {} chars",
        cursor_line + 1,
        line_count,
        cursor_col + 1,
        buffer.char_count()
      )
    }
    else
    {
      format!(
        "Line {}/{} Col {}",
        cursor_line + 1,
        line_count,
        cursor_col + 1
      )
    };

    terminal.write_str( &status )?;

    if config.color
    {
      terminal.write_str( &format!( "{}", ResetColor ) )?;
    }
  }

  // Position cursor at edit location
  let ( cursor_line, cursor_col ) = buffer.cursor_position();
  let cursor_row = if config.prompt.is_empty()
  {
    cursor_line as u16
  }
  else
  {
    cursor_line as u16 + 1
  };

  let cursor_col_offset = if config.show_line_numbers
  {
    line_number_width as u16
  }
  else
  {
    0
  };

  // Calculate actual display column (accounting for wide chars)
  let current_line = buffer.current_line();
  let display_col = current_line
    .chars()
    .take( cursor_col )
    .map( |c| unicode_width::UnicodeWidthChar::width( c ).unwrap_or( 1 ) )
    .sum::< usize >() as u16;

  terminal.move_cursor( cursor_col_offset + display_col, cursor_row )?;
  terminal.flush()?;

  Ok( () )
}

/// Render final result (submitted text)
pub fn render_result< T >(
  terminal: &mut T,
  text: &str,
  config: &RenderConfig,
) -> std::io::Result< () >
where
  T: TerminalOps,
{
  terminal.move_cursor( 0, 0 )?;
  terminal.clear_screen()?;

  let mut row = 0;

  // Show prompt
  if !config.prompt.is_empty()
  {
    terminal.move_cursor( 0, row )?;
    if config.color
    {
      write!( terminal, "{}", SetForegroundColor( Color::Cyan ) )?;
    }
    terminal.write_str( &config.prompt )?;
    if config.color
    {
      write!( terminal, "{}", ResetColor )?;
    }
    row += 1;
  }

  // Show submitted text
  for line in text.lines()
  {
    terminal.move_cursor( 0, row )?;
    terminal.write_str( line )?;
    row += 1;
  }

  terminal.write_str( "\n" )?;
  terminal.flush()?;

  Ok( () )
}

/// Render cancellation message
pub fn render_cancelled< T >(
  terminal: &mut T,
  config: &RenderConfig,
) -> std::io::Result< () >
where
  T: TerminalOps,
{
  terminal.move_cursor( 0, 0 )?;
  terminal.clear_screen()?;

  if !config.prompt.is_empty()
  {
    if config.color
    {
      write!( terminal, "{}", SetForegroundColor( Color::Yellow ) )?;
    }
    terminal.write_str( "Cancelled\n" )?;
    if config.color
    {
      write!( terminal, "{}", ResetColor )?;
    }
  }

  terminal.flush()?;

  Ok( () )
}
