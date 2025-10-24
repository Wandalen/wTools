//! Tests for table style presets and new configuration options (v0.3.0)
//!
//! ## What This Tests
//!
//! Tests the 9 table style preset constructors and configuration builder methods
//! introduced in v0.3.0 comprehensive parametrization refactoring.
//!
//! ## Style Presets Tested
//!
//! 1. **Plain** - CLI tool output (space-separated, dash separator) - DEFAULT
//! 2. **Minimal** - Maximum simplicity (no separator)
//! 3. **Bordered** - Traditional pipe borders
//! 4. **Markdown** - GitHub-flavored Markdown tables
//! 5. **Grid** - Full ASCII grid with intersections
//! 6. **Unicode Box** - Unicode box-drawing characters
//! 7. **CSV** - Comma-separated values
//! 8. **TSV** - Tab-separated values
//! 9. **Compact** - Minimal spacing for density
//!
//! ## Key Insights Captured
//!
//! 1. **Preset Correctness**: Each preset configures BorderVariant, HeaderSeparatorVariant, and ColumnSeparator correctly
//! 2. **Builder Pattern**: Fluent API allows combining presets with custom options
//! 3. **Enum Defaults**: BorderVariant, HeaderSeparatorVariant, ColumnSeparator have sensible defaults
//!
//! ## Design Rationale
//!
//! **Why 9 presets?** Different output contexts have different requirements:
//! - CLI tools need clean, scannable output (Plain)
//! - Documentation needs Markdown compatibility
//! - Data export needs CSV/TSV
//! - Reports need visual clarity (Grid, Unicode Box)
//!
//! These tests ensure each preset produces the intended style and that builder
//! methods properly override preset defaults.
//!
//! Split from tests/table_styles.rs (509 lines) in v0.4.0 compliance cleanup.

#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use tree_fmt::
{
  RowBuilder, TableFormatter, TableConfig,
  BorderVariant, HeaderSeparatorVariant, ColumnSeparator,
};

// Helper function to create sample data
fn sample_data() -> tree_fmt::TreeNode< String >
{
  RowBuilder::new( vec![ "NAME".into(), "AGE".into(), "CITY".into() ] )
    .add_row( vec![ "Alice".into(), "30".into(), "NYC".into() ] )
    .add_row( vec![ "Bob".into(), "25".into(), "LA".into() ] )
    .build()
}

// =============================================================================
// Configuration Enum Tests
// =============================================================================

#[ test ]
fn test_border_style_enum_defaults()
{
  assert_eq!( BorderVariant::default(), BorderVariant::Ascii );
}

#[ test ]
fn test_header_separator_style_enum_defaults()
{
  assert_eq!( HeaderSeparatorVariant::default(), HeaderSeparatorVariant::AsciiGrid );
}

#[ test ]
fn test_column_separator_enum_defaults()
{
  assert_eq!( ColumnSeparator::default(), ColumnSeparator::Character( '|' ) );
}

// =============================================================================
// Style Preset Constructor Tests
// =============================================================================

#[ test ]
fn test_plain_style_config()
{
  let config = TableConfig::plain();

  assert_eq!( config.border_variant, BorderVariant::None );
  assert_eq!( config.header_separator_variant, HeaderSeparatorVariant::Dash );
  assert!( matches!( config.column_separator, ColumnSeparator::Spaces( 2 ) ) );
  assert_eq!( config.outer_padding, true );
  assert_eq!( config.inner_padding, 0 );
}

#[ test ]
fn test_minimal_style_config()
{
  let config = TableConfig::minimal();

  assert_eq!( config.border_variant, BorderVariant::None );
  assert_eq!( config.header_separator_variant, HeaderSeparatorVariant::None );
  assert!( matches!( config.column_separator, ColumnSeparator::Spaces( 2 ) ) );
}

#[ test ]
fn test_bordered_style_config()
{
  let config = TableConfig::bordered();

  // Bordered style should use ASCII borders and grid separator
  assert_eq!( config.border_variant, BorderVariant::Ascii );
  assert_eq!( config.header_separator_variant, HeaderSeparatorVariant::AsciiGrid );
  assert!( matches!( config.column_separator, ColumnSeparator::Character( '|' ) ) );
  assert_eq!( config.inner_padding, 1 );
}

#[ test ]
fn test_markdown_style_config()
{
  let config = TableConfig::markdown();

  assert_eq!( config.border_variant, BorderVariant::Markdown );
  assert_eq!( config.header_separator_variant, HeaderSeparatorVariant::Markdown );
  assert!( matches!( config.column_separator, ColumnSeparator::Character( '|' ) ) );
}

#[ test ]
fn test_grid_style_config()
{
  let config = TableConfig::grid();

  assert_eq!( config.border_variant, BorderVariant::AsciiGrid );
  assert_eq!( config.header_separator_variant, HeaderSeparatorVariant::AsciiGrid );
}

#[ test ]
fn test_unicode_box_style_config()
{
  let config = TableConfig::unicode_box();

  assert_eq!( config.border_variant, BorderVariant::Unicode );
  assert_eq!( config.header_separator_variant, HeaderSeparatorVariant::Unicode );
  assert!( matches!( config.column_separator, ColumnSeparator::Character( '│' ) ) );
}

#[ test ]
fn test_csv_style_config()
{
  let config = TableConfig::csv();

  assert_eq!( config.border_variant, BorderVariant::None );
  assert_eq!( config.header_separator_variant, HeaderSeparatorVariant::None );
  assert!( matches!( config.column_separator, ColumnSeparator::Character( ',' ) ) );
  assert_eq!( config.outer_padding, false );
  assert_eq!( config.inner_padding, 0 );
}

#[ test ]
fn test_tsv_style_config()
{
  let config = TableConfig::tsv();

  assert_eq!( config.border_variant, BorderVariant::None );
  assert_eq!( config.header_separator_variant, HeaderSeparatorVariant::None );
  assert!( matches!( config.column_separator, ColumnSeparator::Character( '\t' ) ) );
}

#[ test ]
fn test_compact_style_config()
{
  let config = TableConfig::compact();

  assert_eq!( config.border_variant, BorderVariant::None );
  assert_eq!( config.header_separator_variant, HeaderSeparatorVariant::None );
  assert!( matches!( config.column_separator, ColumnSeparator::Spaces( 1 ) ) );
  assert_eq!( config.outer_padding, false );
}

// =============================================================================
// Builder Method Tests
// =============================================================================

#[ test ]
fn test_table_config_builder_border_style()
{
  let config = TableConfig::new().border_variant( BorderVariant::None );
  assert_eq!( config.border_variant, BorderVariant::None );
}

#[ test ]
fn test_table_config_builder_header_separator()
{
  let config = TableConfig::new().header_separator_variant( HeaderSeparatorVariant::Dash );
  assert_eq!( config.header_separator_variant, HeaderSeparatorVariant::Dash );
}

#[ test ]
fn test_table_config_builder_column_separator()
{
  let config = TableConfig::new().column_separator( ColumnSeparator::Spaces( 4 ) );
  assert!( matches!( config.column_separator, ColumnSeparator::Spaces( 4 ) ) );
}

#[ test ]
fn test_table_config_builder_padding()
{
  let config = TableConfig::new()
    .outer_padding( false )
    .inner_padding( 2 );

  assert_eq!( config.outer_padding, false );
  assert_eq!( config.inner_padding, 2 );
}

#[ test ]
fn test_table_config_builder_colors()
{
  let config = TableConfig::new()
    .colorize_header( true )
    .header_color( "\x1b[36m".to_string() )
    .alternating_rows( true )
    .row_colors( "\x1b[0m".to_string(), "\x1b[48;5;236m".to_string() );

  assert_eq!( config.colorize_header, true );
  assert_eq!( config.header_color, "\x1b[36m" );
  assert_eq!( config.alternating_rows, true );
  assert_eq!( config.row_color1, "\x1b[0m" );
  assert_eq!( config.row_color2, "\x1b[48;5;236m" );
}

#[ test ]
fn test_table_config_builder_width_constraints()
{
  let config = TableConfig::new()
    .min_column_width( 10 )
    .max_column_width( Some( 50 ) )
    .truncation_marker( "…".to_string() );

  assert_eq!( config.min_column_width, 10 );
  assert_eq!( config.max_column_width, Some( 50 ) );
  assert_eq!( config.truncation_marker, "…" );
}

#[ test ]
fn test_table_config_builder_chaining()
{
  let config = TableConfig::plain()
    .colorize_header( true )
    .header_color( "\x1b[1;36m".to_string() )
    .min_column_width( 5 );

  // Should preserve plain() settings plus new ones
  assert_eq!( config.border_variant, BorderVariant::None );
  assert_eq!( config.colorize_header, true );
  assert_eq!( config.min_column_width, 5 );
}

