//! Example demonstrating `ColorTheme` usage
//!
//! Shows how to apply color themes to different formatters.
//!
//! Run with:
//! ```bash
//! cargo run --example themes --features themes
//! ```

#[ cfg( feature = "themes" ) ]
use tree_fmt::{ ColorTheme, TableConfig, ExpandedConfig };

#[ cfg( not( feature = "themes" ) ) ]
fn main()
{
  println!( "This example requires the 'themes' feature." );
  println!( "Run with: cargo run --example themes --features themes" );
}

#[ cfg( feature = "themes" ) ]
fn main()
{
  println!( "=== ColorTheme Examples ===\n" );
  println!( "This example demonstrates the ColorTheme system for tree_fmt.\n" );

  // Example 1: Dark theme
  println!( "1. Dark Theme (optimized for dark terminals):" );
  let theme = ColorTheme::dark();
  println!( "   Header color: {} (bright cyan)", theme.header_color.escape_debug() );
  println!( "   Border color: {} (dim white)", theme.border_color.escape_debug() );
  println!( "   Row colors: {} / {}", theme.row_color1.escape_debug(), theme.row_color2.escape_debug() );
  println!();

  // Example 2: Light theme
  println!( "2. Light Theme (optimized for light terminals):" );
  let theme = ColorTheme::light();
  println!( "   Header color: {} (dark blue)", theme.header_color.escape_debug() );
  println!( "   Border color: {} (dark gray)", theme.border_color.escape_debug() );
  println!();

  // Example 3: Monokai theme
  println!( "3. Monokai Theme (popular code editor theme):" );
  let theme = ColorTheme::monokai();
  println!( "   Header color: {} (bright magenta)", theme.header_color.escape_debug() );
  println!( "   Branch color: {} (green)", theme.branch_color.escape_debug() );
  println!();

  // Example 4: Solarized theme
  println!( "4. Solarized Theme (low-contrast scientific palette):" );
  let theme = ColorTheme::solarized();
  println!( "   Header color: {} (yellow)", theme.header_color.escape_debug() );
  println!( "   Uses 256-color palette" );
  println!();

  // Example 5: Nord theme
  println!( "5. Nord Theme (arctic-inspired cool palette):" );
  let theme = ColorTheme::nord();
  println!( "   Header color: {} (frost blue)", theme.header_color.escape_debug() );
  println!( "   Branch color: {} (frost green)", theme.branch_color.escape_debug() );
  println!();

  // Example 6: Dracula theme
  println!( "6. Dracula Theme (dark with vibrant colors):" );
  let theme = ColorTheme::dracula();
  println!( "   Header color: {} (purple)", theme.header_color.escape_debug() );
  println!( "   Branch color: {} (pink)", theme.branch_color.escape_debug() );
  println!();

  // Example 7: None theme
  println!( "7. None Theme (no colors, plain text):" );
  let _theme = ColorTheme::none();
  println!( "   All colors disabled (empty strings)" );
  println!();

  // Example 8: Custom theme
  println!( "8. Custom Theme (user-defined colors):" );
  let theme = ColorTheme::custom()
    .header_color( "\x1b[1;31m" )        // Bold red
    .border_color( "\x1b[36m" )          // Cyan
    .row_color1( "\x1b[0m" )             // Default
    .row_color2( "\x1b[48;5;234m" )      // Dark gray background
    .branch_color( "\x1b[32m" )          // Green
    .build();

  println!( "   Header: {}", theme.header_color.escape_debug() );
  println!( "   Border: {}", theme.border_color.escape_debug() );
  println!( "   Rows: {} / {}", theme.row_color1.escape_debug(), theme.row_color2.escape_debug() );
  println!();

  // Example 9: Applying themes to TableConfig
  println!( "9. Applying Dark Theme to TableConfig:" );
  let theme = ColorTheme::dark();
  let config = theme.apply_to_table( TableConfig::bordered() );
  println!( "   colorize_header: {}", config.colorize_header );
  println!( "   alternating_rows: {}", config.alternating_rows );
  println!( "   header_color: {}", config.header_color.escape_debug() );
  println!();

  // Example 10: Applying themes to ExpandedConfig
  println!( "10. Applying Monokai Theme to ExpandedConfig:" );
  let theme = ColorTheme::monokai();
  let config = theme.apply_to_expanded( ExpandedConfig::new() );
  println!( "   colorize_keys: {}", config.colorize_keys );
  println!( "   key_color: {}", config.key_color.escape_debug() );
  println!();

  // Example 11: Theme with additional config
  println!( "11. Theme Combined with Custom Configuration:" );
  let theme = ColorTheme::nord();
  let config = theme.apply_to_table(
    TableConfig::bordered()
      .inner_padding( 2 )
      .min_column_width( 15 )
  );
  println!( "   Inner padding: {}", config.inner_padding );
  println!( "   Min column width: {}", config.min_column_width );
  println!( "   Colors applied: colorize_header={}", config.colorize_header );
  println!();

  // Example 12: Color comparison
  println!( "12. Theme Color Comparison:" );
  let themes = vec![
    ( "Dark", ColorTheme::dark() ),
    ( "Light", ColorTheme::light() ),
    ( "Monokai", ColorTheme::monokai() ),
    ( "Solarized", ColorTheme::solarized() ),
    ( "Nord", ColorTheme::nord() ),
    ( "Dracula", ColorTheme::dracula() ),
  ];

  for ( name, theme ) in themes
  {
    println!( "   {}: {}", name, theme.header_color.escape_debug() );
  }
  println!();

  println!( "=== Usage Tips ===" );
  println!( "- Use ColorTheme::dark() for dark terminal backgrounds" );
  println!( "- Use ColorTheme::light() for light terminal backgrounds" );
  println!( "- Themes work with TableFormatter and ExpandedFormatter" );
  println!( "- ColorTheme::none() disables all colors for plain text output" );
  println!( "- Custom themes can be built with ColorTheme::custom()" );
  println!( "- Themes preserve existing configuration (padding, borders, etc)" );
  println!( "- All themes use ANSI escape codes (widely supported)" );
  println!( "- Try different themes to match your terminal and preference" );
}
