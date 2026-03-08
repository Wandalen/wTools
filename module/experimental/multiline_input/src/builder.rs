//! Builder pattern for configuring multiline editor

use crate::
{
  editor::Editor,
  render::RenderConfig,
  terminal::{ RealTerminal, TerminalOps },
};

/// Type alias for validation function
pub type ValidatorFn = Box< dyn Fn( &str ) -> Result< (), String > >;

/// Builder for configuring Editor
pub struct Builder
{
  /// Prompt message
  pub prompt: String,

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

  /// Placeholder text when empty
  pub placeholder: Option< String >,

  /// Show line numbers
  pub show_line_numbers: bool,

  /// Show status line
  pub show_status: bool,

  /// Show character count
  pub show_char_count: bool,

  /// Enable colors
  pub color: bool,
}

impl Builder
{
  /// Create new builder with default settings
  pub fn new() -> Self
  {
    Self
    {
      prompt: String::new(),
      allow_empty: true,
      min_length: None,
      max_length: None,
      validator: None,
      initial_text: None,
      placeholder: None,
      show_line_numbers: false,
      show_status: false,
      show_char_count: false,
      color: true,
    }
  }

  /// Set prompt message
  pub fn prompt( mut self, prompt: impl Into< String > ) -> Self
  {
    self.prompt = prompt.into();
    self
  }

  /// Set whether to allow empty input
  pub fn allow_empty( mut self, allow: bool ) -> Self
  {
    self.allow_empty = allow;
    self
  }

  /// Set minimum text length
  pub fn min_length( mut self, min: usize ) -> Self
  {
    self.min_length = Some( min );
    self
  }

  /// Set maximum text length
  pub fn max_length( mut self, max: usize ) -> Self
  {
    self.max_length = Some( max );
    self
  }

  /// Set custom validation function
  pub fn validator< F >( mut self, validator: F ) -> Self
  where
    F: Fn( &str ) -> Result< (), String > + 'static,
  {
    self.validator = Some( Box::new( validator ) );
    self
  }

  /// Set initial text content
  pub fn initial_text( mut self, text: impl Into< String > ) -> Self
  {
    self.initial_text = Some( text.into() );
    self
  }

  /// Set placeholder text
  pub fn placeholder( mut self, text: impl Into< String > ) -> Self
  {
    self.placeholder = Some( text.into() );
    self
  }

  /// Set whether to show line numbers
  pub fn show_line_numbers( mut self, show: bool ) -> Self
  {
    self.show_line_numbers = show;
    self
  }

  /// Set whether to show status line
  pub fn show_status( mut self, show: bool ) -> Self
  {
    self.show_status = show;
    self
  }

  /// Set whether to show character count
  pub fn show_char_count( mut self, show: bool ) -> Self
  {
    self.show_char_count = show;
    self
  }

  /// Set whether to enable colors
  pub fn color( mut self, enable: bool ) -> Self
  {
    self.color = enable;
    self
  }

  /// Build editor with configured settings using default real terminal
  pub fn build( self ) -> Editor< RealTerminal >
  {
    let terminal = RealTerminal::new();
    self.build_with( terminal )
  }

  /// Build editor with custom terminal implementation
  ///
  /// Enables dependency injection for testing with MockTerminal.
  pub fn build_with< T >( self, terminal: T ) -> Editor< T >
  where
    T: TerminalOps,
  {
    let render_config = RenderConfig
    {
      show_line_numbers: self.show_line_numbers,
      show_status: self.show_status,
      show_char_count: self.show_char_count,
      color: self.color,
      prompt: self.prompt,
      placeholder: self.placeholder,
    };

    Editor
    {
      allow_empty: self.allow_empty,
      min_length: self.min_length,
      max_length: self.max_length,
      validator: self.validator,
      initial_text: self.initial_text,
      render_config,
      terminal,
    }
  }
}

impl Default for Builder
{
  fn default() -> Self
  {
    Self::new()
  }
}
