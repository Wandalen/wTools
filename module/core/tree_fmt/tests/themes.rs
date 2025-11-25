//! Integration tests for `ColorTheme` system
//!
//! Tests theme application across different formatters and configurations.

#[ cfg( feature = "themes" ) ]
mod theme_tests
{
  use tree_fmt::{ ColorTheme, TableConfig, ExpandedConfig, TreeConfig };

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
      assert_eq!( theme.reset, "\x1b[0m", "Reset should be standard ANSI reset" );
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
    assert!( theme.reset.is_empty() );
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
    assert_eq!( theme.reset, "\x1b[0m" );
  }

  #[ test ]
  fn test_apply_theme_to_table_config()
  {
    let theme = ColorTheme::dark();
    let config = theme.apply_to_table( TableConfig::bordered() );

    assert!( config.colorize_header );
    assert_eq!( config.header_color, theme.header_color );
    assert!( config.alternating_rows );
    assert_eq!( config.row_color1, theme.row_color1 );
    assert_eq!( config.row_color2, theme.row_color2 );
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
    let theme = ColorTheme::dark();
    let config = theme.apply_to_table( TableConfig::bordered() );

    // Config should have color settings from theme
    assert!( config.colorize_header );
    assert!( !config.header_color.is_empty() );
    assert!( config.alternating_rows );
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

    // Theme settings should be applied
    assert_eq!( config.header_color, "\x1b[31m" );
    // Original config settings should be preserved
    assert_eq!( config.inner_padding, 2 );
    assert_eq!( config.min_column_width, 10 );
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
    assert_eq!( theme.reset, "\x1b[0m" );
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
    assert_eq!( theme.reset, "\x1b[0m" );
  }

  #[ test ]
  fn test_none_theme_with_config()
  {
    let theme = ColorTheme::none();
    let config = theme.apply_to_table( TableConfig::bordered() );

    // None theme should not set colors
    assert!( !config.colorize_header );
    assert!( !config.alternating_rows );
  }

  #[ test ]
  fn test_multiple_themes_on_same_config()
  {
    let base_config = TableConfig::bordered().inner_padding( 2 );

    let dark_config = ColorTheme::dark().apply_to_table( base_config.clone() );
    let light_config = ColorTheme::light().apply_to_table( base_config.clone() );

    // Different themes should produce different colors
    assert_ne!( dark_config.header_color, light_config.header_color );

    // But preserve original config
    assert_eq!( dark_config.inner_padding, 2 );
    assert_eq!( light_config.inner_padding, 2 );
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

    // Theme colors should be applied
    assert!( config.colorize_header );
    assert!( !config.header_color.is_empty() );

    // Original settings should be preserved
    assert_eq!( config.inner_padding, 3 );
    assert_eq!( config.min_column_width, 20 );
  }
}
