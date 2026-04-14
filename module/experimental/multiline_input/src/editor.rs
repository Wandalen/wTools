//! Main editor logic for multiline input collection

use crate::
{
  buffer::TextBuffer,
  builder::ValidatorFn,
  error::Error,
  keys::{ handle_key, KeyAction },
  render::{ self, RenderConfig },
  terminal::{ RealTerminal, TerminalOps },
};

/// Multiline text editor
///
/// Generic over terminal implementation to enable dependency injection and testing.
pub struct Editor< T = RealTerminal >
where
  T: TerminalOps,
{
  /// Allow empty input
  pub allow_empty: bool,

  /// Minimum text length
  pub min_length: Option< usize >,

  /// Maximum text length
  pub max_length: Option< usize >,

  /// Custom validation function
  pub validator: Option< ValidatorFn >,

  /// Initial text content
  pub initial_text: Option< String >,

  /// Rendering configuration
  pub render_config: RenderConfig,

  /// Terminal implementation
  pub terminal: T,
}

impl< T > Editor< T >
where
  T: TerminalOps,
{
  /// Collect multiline input from terminal
  ///
  /// Returns:
  /// - `Ok(Some(String))` - User submitted text
  /// - `Ok(None)` - User cancelled (ESC or CTRL+C)
  /// - `Err(Error)` - Terminal error or validation error
  pub fn collect( mut self ) -> Result< Option< String >, Error >
  {
    // Enter raw mode
    self.terminal.enable_raw_mode()?;

    // Hide cursor
    self.terminal.hide_cursor()?;

    // Initialize text buffer
    let mut buffer = if let Some( initial ) = &self.initial_text
    {
      TextBuffer::with_text( initial )
    }
    else
    {
      TextBuffer::new()
    };

    // Main edit loop
    let result = loop
    {
      // Render current state
      render::render( &mut self.terminal, &buffer, &self.render_config )?;
      self.terminal.show_cursor()?;

      // Read next key event
      let key = self.terminal.read_key( None )?;

      // Handle key event
      match handle_key( key, &mut buffer )
      {
        KeyAction::Submit =>
        {
          let text = buffer.text();

          // Validate before submitting
          match self.validate( &text )
          {
            Ok( () ) =>
            {
              break Some( text );
            }
            Err( err ) =>
            {
              // Show validation error and continue editing
              // For now, just ignore and continue
              // In production, would show error message
              let _ = err;
              continue;
            }
          }
        }

        KeyAction::Cancel =>
        {
          break None;
        }

        KeyAction::Continue =>
        {
          // Continue editing
          continue;
        }
      }
    };

    // Cleanup and render final state
    self.terminal.show_cursor()?;

    match &result
    {
      Some( text ) =>
      {
        render::render_result( &mut self.terminal, text, &self.render_config )?;
      }
      None =>
      {
        render::render_cancelled( &mut self.terminal, &self.render_config )?;
      }
    }

    // Disable raw mode before returning
    self.terminal.disable_raw_mode()?;

    Ok( result )
  }

  /// Validate input text
  pub fn validate( &self, text: &str ) -> Result< (), String >
  {
    // Check empty
    if !self.allow_empty && text.is_empty()
    {
      return Err( "Input cannot be empty".to_string() );
    }

    // Check min length
    if let Some( min ) = self.min_length
    {
      if text.chars().count() < min
      {
        return Err( format!( "Input too short (min {} chars)", min ) );
      }
    }

    // Check max length
    if let Some( max ) = self.max_length
    {
      if text.chars().count() > max
      {
        return Err( format!( "Input too long (max {} chars)", max ) );
      }
    }

    // Custom validation
    if let Some( validator ) = &self.validator
    {
      validator( text )?;
    }

    Ok( () )
  }
}
