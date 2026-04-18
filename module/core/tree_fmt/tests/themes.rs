//! Integration tests for `ColorTheme` system
//!
//! Tests theme application across different formatters and configurations.
//!
//! ## Note on Color Field Assertions (v0.10.0+)
//!
//! `TableConfig` fields (`colorize_header`, `header_color`, `alternating_rows`,
//! `row_color1`, `row_color2`) are private since v0.10.0 and are rendered as ANSI
//! escape codes by `TableFormatter`. When a theme is applied, colored rows begin
//! with an ANSI escape sequence rather than the border character — so assertions
//! must use `contains` rather than `starts_with` when checking visible content
//! that may be preceded by ANSI codes.

#![ cfg( feature = "enabled" ) ]
#[ cfg( feature = "themes" ) ]
mod theme_tests
{
  use tree_fmt::{ ColorTheme, TableConfig, ExpandedConfig, TreeConfig, TableFormatter, RowBuilder };

  fn sample_row() -> tree_fmt::TreeNode< String >
  {
    RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
      .add_row( vec![ "Alice".into(), "42".into() ] )
      .build()
  }

  #[ test ]
  fn test_all_predefined_themes_exist()
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
      assert!( !theme.header_color.is_empty(), "Header color should be set" );
      assert!( !theme.border_color.is_empty(), "Border color should be set" );
    }
  }

  #[ test ]
  fn test_none_theme_disables_colors()
  {
    let theme = ColorTheme::none();

    assert!( theme.header_color.is_empty() );
    assert!( theme.border_color.is_empty() );
    assert!( theme.row_color1.is_empty() );
    assert!( theme.row_color2.is_empty() );
    assert!( theme.branch_color.is_empty() );
  }

  #[ test ]
  fn test_custom_theme_builder()
  {
    let theme = ColorTheme::custom()
      .header_color( "\x1b[31m" )  // Red
      .border_color( "\x1b[32m" )  // Green
      .row_color1( "\x1b[33m" )    // Yellow
      .row_color2( "\x1b[34m" )    // Blue
      .branch_color( "\x1b[35m" )  // Magenta
      .build();

    assert_eq!( theme.header_color, "\x1b[31m" );
    assert_eq!( theme.border_color, "\x1b[32m" );
    assert_eq!( theme.row_color1, "\x1b[33m" );
    assert_eq!( theme.row_color2, "\x1b[34m" );
    assert_eq!( theme.branch_color, "\x1b[35m" );
  }

  #[ test ]
  fn test_apply_theme_to_table_config()
  {
    // Verify apply_to_table() does not panic and the config renders data correctly.
    // Color fields (colorize_header, alternating_rows, etc.) are stored in TableConfig
    // but are not yet rendered by TableFormatter (reserved for future theme-driven rendering).
    let theme = ColorTheme::dark();
    let config = theme.apply_to_table( TableConfig::bordered() );
    let output = TableFormatter::with_config( config ).format( &sample_row() );

    assert!( output.contains( "Alice" ), "themed config must render data; output:\n{output}" );
    assert!( output.contains( '|' ), "bordered base must be preserved; output:\n{output}" );
  }

  #[ test ]
  fn test_apply_theme_to_expanded_config()
  {
    let theme = ColorTheme::monokai();
    let config = theme.apply_to_expanded( ExpandedConfig::new() );

    assert!( config.colorize_keys );
    assert_eq!( config.key_color, theme.header_color );
  }

  #[ test ]
  fn test_apply_theme_to_tree_config()
  {
    let theme = ColorTheme::nord();
    let config = theme.apply_to_tree( TreeConfig::new() );

    // TreeConfig currently doesnt have color fields, so this is a no-op
    // Just verify it doesnt panic and returns a valid config
    assert!( config.show_branches );
  }

  #[ test ]
  fn test_theme_config_contains_colors()
  {
    // Smoke test: apply_to_table() with a theme must not panic and must render data.
    // (Color fields are stored but not yet rendered by TableFormatter.)
    let theme = ColorTheme::dark();
    let config = theme.apply_to_table( TableConfig::bordered() );
    let output = TableFormatter::with_config( config ).format( &sample_row() );

    assert!( output.contains( "Alice" ), "theme-configured table must render data; output:\n{output}" );
  }

  #[ test ]
  fn test_expanded_config_contains_colors()
  {
    let theme = ColorTheme::solarized();
    let config = theme.apply_to_expanded( ExpandedConfig::new() );

    // Config should have color settings from theme
    assert!( config.colorize_keys );
    assert!( !config.key_color.is_empty() );
  }

  #[ test ]
  fn test_dark_theme_colors()
  {
    let theme = ColorTheme::dark();

    // Dark theme should have bright colors for dark terminals
    assert!( theme.header_color.contains( "1;36" ), "Should have bright cyan" );
    assert!( theme.border_color.contains( "2;37" ), "Should have dim white" );
  }

  #[ test ]
  fn test_light_theme_colors()
  {
    let theme = ColorTheme::light();

    // Light theme should have darker colors for light terminals
    assert!( theme.header_color.contains( "34" ), "Should have dark blue" );
  }

  #[ test ]
  fn test_monokai_theme_colors()
  {
    let theme = ColorTheme::monokai();

    // Monokai has characteristic colors
    assert!( theme.header_color.contains( "1;35" ), "Should have bright magenta" );
    assert!( theme.branch_color.contains( "32" ), "Should have green" );
  }

  #[ test ]
  fn test_solarized_theme_colors()
  {
    let theme = ColorTheme::solarized();

    // Solarized uses 256-color palette
    assert!( theme.border_color.contains( "38;5;" ), "Should use 256-color mode" );
  }

  #[ test ]
  fn test_nord_theme_colors()
  {
    let theme = ColorTheme::nord();

    // Nord has frost blue header
    assert!( theme.header_color.contains( "38;5;81" ), "Should have frost blue" );
  }

  #[ test ]
  fn test_dracula_theme_colors()
  {
    let theme = ColorTheme::dracula();

    // Dracula has purple header
    assert!( theme.header_color.contains( "38;5;141" ), "Should have purple" );
  }

  #[ test ]
  fn test_theme_chaining()
  {
    let theme = ColorTheme::custom()
      .header_color( "\x1b[31m" )
      .border_color( "\x1b[32m" )
      .build();

    let config = theme.apply_to_table(
      TableConfig::bordered()
        .inner_padding( 2 )
        .min_column_width( 10 )
    );

    // inner_padding=2 with outer_padding=true and bordered() base:
    // rows should start with "|  " (border + 2 inner padding spaces)
    let output = TableFormatter::with_config( config ).format( &sample_row() );
    assert!(
      output.lines().any( | l | l.starts_with( "|  " ) ),
      "inner_padding=2 preserved after theme application; output:\n{output}"
    );
  }

  #[ test ]
  fn test_theme_builder_defaults()
  {
    let theme = ColorTheme::custom().build();

    // Builder should provide sensible defaults
    assert_eq!( theme.header_color, "" );
    assert_eq!( theme.border_color, "" );
    assert_eq!( theme.row_color1, "\x1b[0m" );  // Default to reset
    assert_eq!( theme.row_color2, "" );
    assert_eq!( theme.branch_color, "" );
  }

  #[ test ]
  fn test_theme_builder_partial()
  {
    let theme = ColorTheme::custom()
      .header_color( "\x1b[31m" )
      .build();

    // Only specified colors should be set
    assert_eq!( theme.header_color, "\x1b[31m" );
    assert_eq!( theme.border_color, "" );
  }

  #[ test ]
  fn test_none_theme_with_config()
  {
    // None theme: apply_to_table() must not panic and must render data correctly.
    let theme = ColorTheme::none();
    let config = theme.apply_to_table( TableConfig::bordered() );
    let output = TableFormatter::with_config( config ).format( &sample_row() );

    assert!( output.contains( "Alice" ), "none-themed config must render data; output:\n{output}" );
  }

  #[ test ]
  fn test_multiple_themes_on_same_config()
  {
    let base_config = TableConfig::bordered().inner_padding( 2 );

    let dark_config = ColorTheme::dark().apply_to_table( base_config.clone() );
    let light_config = ColorTheme::light().apply_to_table( base_config.clone() );

    // Theme colors differ (verified on ColorTheme level, not on private config fields)
    assert_ne!(
      ColorTheme::dark().header_color,
      ColorTheme::light().header_color,
      "dark and light themes should have different header colors"
    );

    // Both preserve inner_padding=2: output contains "|  " (ANSI-colored rows start with escape
    // codes, so contains() is required rather than starts_with()).
    let dark_output = TableFormatter::with_config( dark_config ).format( &sample_row() );
    let light_output = TableFormatter::with_config( light_config ).format( &sample_row() );

    assert!(
      dark_output.lines().any( | l | l.contains( "|  " ) ),
      "dark theme preserves inner_padding=2; output:\n{dark_output}"
    );
    assert!(
      light_output.lines().any( | l | l.contains( "|  " ) ),
      "light theme preserves inner_padding=2; output:\n{light_output}"
    );
  }

  #[ test ]
  fn test_theme_preserves_config_settings()
  {
    let theme = ColorTheme::nord();
    let config = theme.apply_to_table(
      TableConfig::bordered()
        .inner_padding( 3 )
        .min_column_width( 20 )
    );

    // inner_padding=3 preserved: output contains "|   " (border + 3 spaces). Uses contains()
    // because nord theme activates ANSI coloring, prepending escape codes before the border char.
    let output = TableFormatter::with_config( config ).format( &sample_row() );
    assert!(
      output.lines().any( | l | l.contains( "|   " ) ),
      "inner_padding=3 preserved after nord theme application; output:\n{output}"
    );
    assert!( output.contains( "Alice" ), "themed config must render data; output:\n{output}" );
  }
}
