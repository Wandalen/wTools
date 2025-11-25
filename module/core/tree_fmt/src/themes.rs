//! Color themes for visual formatters
//!
//! ## Purpose
//!
//! Provides predefined color schemes for terminal output formatters,
//! eliminating manual ANSI code management and ensuring consistent styling.
//!
//! ## Available Themes
//!
//! - **Dark** - High contrast for dark terminals
//! - **Light** - Optimized for light terminals
//! - **Monokai** - Popular code editor theme
//! - **Solarized** - Low-contrast scientific palette
//! - **Nord** - Arctic-inspired cool palette
//! - **Dracula** - Dark theme with vibrant colors
//!
//! ## Usage
//!
//! ```
//! # #[cfg(feature = "themes")]
//! # {
//! use tree_fmt::{ ColorTheme, TableConfig };
//!
//! // Apply theme to table config
//! let theme = ColorTheme::dark();
//! let config = theme.apply_to_table( TableConfig::bordered() );
//! # }
//! ```
//!
//! ## ANSI Color Reference
//!
//! Common ANSI escape codes used:
//! - Reset: `\x1b[0m`
//! - Bold: `\x1b[1m`
//! - Dim: `\x1b[2m`
//! - Colors: `\x1b[3Xm` (30-37 for foreground)
//! - Bright: `\x1b[9Xm` (90-97 for bright foreground)
//! - 256-color: `\x1b[38;5;Nm` (N = 0-255)
//! - Background: `\x1b[4Xm` or `\x1b[48;5;Nm`

use crate::config::{ TableConfig, ExpandedConfig, TreeConfig };

/// Color theme with ANSI escape codes
///
/// Provides consistent color schemes across visual formatters.
#[ derive( Debug, Clone ) ]
pub struct ColorTheme
{
  /// Header color (table headers, expanded keys when colored)
  pub header_color : String,
  /// Border/separator color
  pub border_color : String,
  /// First row color (for alternating rows)
  pub row_color1 : String,
  /// Second row color (for alternating rows)
  pub row_color2 : String,
  /// Tree branch symbol color
  pub branch_color : String,
  /// Reset sequence
  pub reset : String,
}

impl ColorTheme
{
  /// Dark theme - High contrast for dark terminals
  ///
  /// Bright cyan headers, dim borders, dark gray alternating rows.
  pub fn dark() -> Self
  {
    Self
    {
      header_color : "\x1b[1;36m".to_string(),      // Bright cyan bold
      border_color : "\x1b[2;37m".to_string(),      // Dim white
      row_color1 : "\x1b[0m".to_string(),           // Default
      row_color2 : "\x1b[48;5;235m".to_string(),    // Dark gray background
      branch_color : "\x1b[36m".to_string(),        // Cyan
      reset : "\x1b[0m".to_string(),
    }
  }

  /// Light theme - Optimized for light terminals
  ///
  /// Dark blue headers, dark gray borders, light gray alternating rows.
  pub fn light() -> Self
  {
    Self
    {
      header_color : "\x1b[34m".to_string(),        // Dark blue
      border_color : "\x1b[90m".to_string(),        // Bright black (dark gray)
      row_color1 : "\x1b[0m".to_string(),           // Default (black on white)
      row_color2 : "\x1b[48;5;255m".to_string(),    // Light gray background
      branch_color : "\x1b[34m".to_string(),        // Dark blue
      reset : "\x1b[0m".to_string(),
    }
  }

  /// Monokai theme - Popular code editor theme
  ///
  /// Bright magenta headers, dark gray borders, dark background.
  pub fn monokai() -> Self
  {
    Self
    {
      header_color : "\x1b[1;35m".to_string(),      // Bright magenta bold
      border_color : "\x1b[38;5;239m".to_string(),  // Dark gray
      row_color1 : "\x1b[38;5;231m".to_string(),    // White text
      row_color2 : "\x1b[48;5;236m".to_string(),    // Dark gray background
      branch_color : "\x1b[32m".to_string(),        // Green
      reset : "\x1b[0m".to_string(),
    }
  }

  /// Solarized theme - Low-contrast scientific palette
  ///
  /// Yellow headers, base01 borders, base03/base02 alternating rows.
  pub fn solarized() -> Self
  {
    Self
    {
      header_color : "\x1b[33m".to_string(),        // Yellow
      border_color : "\x1b[38;5;240m".to_string(),  // Base01
      row_color1 : "\x1b[38;5;234m".to_string(),    // Base03
      row_color2 : "\x1b[48;5;235m".to_string(),    // Base02 background
      branch_color : "\x1b[36m".to_string(),        // Cyan
      reset : "\x1b[0m".to_string(),
    }
  }

  /// Nord theme - Arctic-inspired cool palette
  ///
  /// Frost blue headers, polar night borders, cool backgrounds.
  pub fn nord() -> Self
  {
    Self
    {
      header_color : "\x1b[38;5;81m".to_string(),   // Frost blue
      border_color : "\x1b[38;5;236m".to_string(),  // Polar night
      row_color1 : "\x1b[0m".to_string(),           // Default
      row_color2 : "\x1b[48;5;236m".to_string(),    // Polar night background
      branch_color : "\x1b[38;5;150m".to_string(),  // Frost green
      reset : "\x1b[0m".to_string(),
    }
  }

  /// Dracula theme - Dark theme with vibrant colors
  ///
  /// Purple headers, comment gray borders, selection background.
  pub fn dracula() -> Self
  {
    Self
    {
      header_color : "\x1b[38;5;141m".to_string(),  // Purple
      border_color : "\x1b[38;5;61m".to_string(),   // Comment gray
      row_color1 : "\x1b[38;5;231m".to_string(),    // Foreground white
      row_color2 : "\x1b[48;5;236m".to_string(),    // Selection background
      branch_color : "\x1b[38;5;212m".to_string(),  // Pink
      reset : "\x1b[0m".to_string(),
    }
  }

  /// No colors - Disable all coloring
  pub fn none() -> Self
  {
    Self
    {
      header_color : String::new(),
      border_color : String::new(),
      row_color1 : String::new(),
      row_color2 : String::new(),
      branch_color : String::new(),
      reset : String::new(),
    }
  }

  /// Custom theme builder
  pub fn custom() -> ColorThemeBuilder
  {
    ColorThemeBuilder::default()
  }

  /// Apply theme to `TableConfig`
  ///
  /// Configures header colors and alternating row colors.
  pub fn apply_to_table( &self, config : TableConfig ) -> TableConfig
  {
    config
      .colorize_header( !self.header_color.is_empty() )
      .header_color( self.header_color.clone() )
      .alternating_rows( !self.row_color2.is_empty() )
      .row_colors( self.row_color1.clone(), self.row_color2.clone() )
  }

  /// Apply theme to `ExpandedConfig`
  ///
  /// Configures key colors.
  pub fn apply_to_expanded( &self, config : ExpandedConfig ) -> ExpandedConfig
  {
    config
      .colorize_keys( !self.header_color.is_empty() )
      .key_color( self.header_color.clone() )
  }

  /// Apply theme to `TreeConfig`
  ///
  /// Currently a no-op as `TreeConfig` doesn't have color fields yet.
  /// Future: Could add branch color formatter parameters.
  #[ allow( clippy::unnecessary_wraps ) ]
  pub fn apply_to_tree( &self, config : TreeConfig ) -> TreeConfig
  {
    // TreeConfig doesn't currently support colors
    // This is a no-op for now, included for API consistency
    config
  }
}

/// Builder for custom color themes
#[ derive( Debug, Clone, Default ) ]
pub struct ColorThemeBuilder
{
  header_color : Option< String >,
  border_color : Option< String >,
  row_color1 : Option< String >,
  row_color2 : Option< String >,
  branch_color : Option< String >,
}

impl ColorThemeBuilder
{
  /// Set header color
  #[ must_use ]
  pub fn header_color( mut self, color : impl Into< String > ) -> Self
  {
    self.header_color = Some( color.into() );
    self
  }

  /// Set border color
  #[ must_use ]
  pub fn border_color( mut self, color : impl Into< String > ) -> Self
  {
    self.border_color = Some( color.into() );
    self
  }

  /// Set first row color
  #[ must_use ]
  pub fn row_color1( mut self, color : impl Into< String > ) -> Self
  {
    self.row_color1 = Some( color.into() );
    self
  }

  /// Set second row color (for alternating)
  #[ must_use ]
  pub fn row_color2( mut self, color : impl Into< String > ) -> Self
  {
    self.row_color2 = Some( color.into() );
    self
  }

  /// Set branch color
  #[ must_use ]
  pub fn branch_color( mut self, color : impl Into< String > ) -> Self
  {
    self.branch_color = Some( color.into() );
    self
  }

  /// Build the color theme
  #[ must_use ]
  pub fn build( self ) -> ColorTheme
  {
    ColorTheme
    {
      header_color : self.header_color.unwrap_or_default(),
      border_color : self.border_color.unwrap_or_default(),
      row_color1 : self.row_color1.unwrap_or_else( || "\x1b[0m".to_string() ),
      row_color2 : self.row_color2.unwrap_or_default(),
      branch_color : self.branch_color.unwrap_or_default(),
      reset : "\x1b[0m".to_string(),
    }
  }
}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn test_dark_theme()
  {
    let theme = ColorTheme::dark();
    assert!( !theme.header_color.is_empty() );
    assert!( !theme.border_color.is_empty() );
    assert_eq!( theme.reset, "\x1b[0m" );
  }

  #[ test ]
  fn test_none_theme()
  {
    let theme = ColorTheme::none();
    assert!( theme.header_color.is_empty() );
    assert!( theme.border_color.is_empty() );
    assert!( theme.reset.is_empty() );
  }

  #[ test ]
  fn test_custom_theme()
  {
    let theme = ColorTheme::custom()
      .header_color( "\x1b[31m" )
      .border_color( "\x1b[32m" )
      .build();

    assert_eq!( theme.header_color, "\x1b[31m" );
    assert_eq!( theme.border_color, "\x1b[32m" );
  }

  #[ test ]
  fn test_all_predefined_themes()
  {
    let themes = vec![
      ColorTheme::dark(),
      ColorTheme::light(),
      ColorTheme::monokai(),
      ColorTheme::solarized(),
      ColorTheme::nord(),
      ColorTheme::dracula(),
    ];

    for theme in themes
    {
      assert!( !theme.header_color.is_empty() );
      assert!( !theme.border_color.is_empty() );
      assert_eq!( theme.reset, "\x1b[0m" );
    }
  }

  #[ test ]
  fn test_apply_to_table()
  {
    let theme = ColorTheme::dark();
    let config = theme.apply_to_table( TableConfig::bordered() );

    assert!( config.colorize_header );
    assert_eq!( config.header_color, theme.header_color );
    assert!( config.alternating_rows );
  }

  #[ test ]
  fn test_apply_to_expanded()
  {
    let theme = ColorTheme::dark();
    let config = theme.apply_to_expanded( ExpandedConfig::new() );

    assert!( config.colorize_keys );
    assert_eq!( config.key_color, theme.header_color );
  }
}
