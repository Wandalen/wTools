//! Multi-Format Data Visualization Library
//!
//! 10 formatters, 33 variants, zero core dependencies.
//! Build your data structure once, then output as table, tree, expanded,
//! JSON, HTML, SQL, YAML, TOML, logfmt, or text.
//!
//! # Features
//!
//! - **`RowBuilder`**: Constructs tabular data (headers + rows)
//! - **`TreeBuilder`**: Constructs trees from flat data with path-based insertion
//! - **10 Formatters**: Table (9 styles), Tree (3), Expanded (2), JSON, HTML (4),
//!   SQL (4), YAML, TOML, Logfmt, Text (6)
//! - **String Output**: All formatters return `String`, no direct console output

#![ allow( clippy::missing_inline_in_public_items ) ]
#![ allow( clippy::must_use_candidate ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::format_push_string ) ]
#![ cfg_attr( not( feature = "enabled" ), allow( unused ) ) ]
//!
//! The library supports 10 output formats across 33 variants:
//!
//! - **Table**: 9 styles (plain, markdown, csv, bordered, grid, unicode, minimal, tsv, compact)
//! - **Expanded**: Vertical record display (`PostgreSQL` `\x` mode, property list)
//! - **Tree**: Hierarchical display (hierarchical, aligned, aggregated)
//! - **JSON/YAML/TOML**: Data serialization formats
//! - **HTML**: 4 themes (minimal, bootstrap, tailwind, custom)
//! - **SQL**: 4 dialects (ANSI, `PostgreSQL`, `MySQL`, `SQLite`)
//! - **Logfmt**: Structured log output
//! - **Text**: 6 styles (bullets, numbered, sections, key-value, compact, CLI help)
//!
//! # Examples
//!
//! ## Same data in three visual formats
//!
//! ```
//! # #[ cfg( feature = "enabled" ) ]
//! # {
//! use data_fmt::{ RowBuilder, TableFormatter, ExpandedFormatter, TreeFormatter };
//!
//! // Create tabular data
//! let tree = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
//!   .add_row( vec![ "Alice".into(), "30".into() ] )
//!   .add_row( vec![ "Bob".into(), "25".into() ] )
//!   .build();
//!
//! // Table format
//! let table_fmt = TableFormatter::new();
//! let output = table_fmt.format( &tree );
//!
//! // Expanded format
//! let expanded_fmt = ExpandedFormatter::new();
//! let output = expanded_fmt.format( &tree );
//!
//! // Tree format (table-shaped tree)
//! let data_fmt = TreeFormatter::default();
//! let output = data_fmt.format( &tree, Clone::clone );
//! # }
//! ```
//!
//! ## Expanded format with colored keys
//!
//! ```
//! # #[ cfg( feature = "enabled" ) ]
//! # {
//! use data_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };
//!
//! let tree = RowBuilder::new( vec![ "Name".into(), "Score".into() ] )
//!   .add_row( vec![ "Alice".into(), "95".into() ] )
//!   .build();
//!
//! // Gray keys for terminal output (PostgreSQL style)
//! let formatter = ExpandedFormatter::with_config(
//!   ExpandedConfig::new().colorize_keys( true )
//! );
//! let output = formatter.format( &tree );
//! # }
//! ```
//!
//! ## Property list style (colon separator)
//!
//! ```
//! # #[ cfg( feature = "enabled" ) ]
//! # {
//! use data_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };
//!
//! let tree = RowBuilder::new( vec![ "Command".into(), "Status".into() ] )
//!   .add_row( vec![ "build".into(), "success".into() ] )
//!   .build();
//!
//! // Property list style: no record headers, colon separator
//! let formatter = ExpandedFormatter::with_config( ExpandedConfig::property_style() );
//! let output = formatter.format( &tree );
//! // Output:
//! // Command: build
//! // Status:  success
//! # }
//! ```
//!
//! ## Building and formatting a tree
//!
//! ```
//! # #[ cfg( feature = "enabled" ) ]
//! # {
//! use data_fmt::{ TreeBuilder, TreeFormatter };
//!
//! let tree = TreeBuilder::new( "root" )
//!   .insert( &[ "src", "main.rs" ], 150 )
//!   .insert( &[ "src", "lib.rs" ], 300 )
//!   .build();
//!
//! let formatter = TreeFormatter::new();
//! let output = formatter.format( &tree, | lines | format!( "{} lines", lines ) );
//! println!( "{}", output );
//! # }
//! ```
//!
//! ## Building from items
//!
//! ```
//! # #[ cfg( feature = "enabled" ) ]
//! # {
//! use data_fmt::TreeBuilder;
//! use std::path::PathBuf;
//!
//! let files = vec![
//!   ( PathBuf::from( "src/main.rs" ), 100 ),
//!   ( PathBuf::from( "tests/test.rs" ), 50 ),
//! ];
//!
//! let tree = TreeBuilder::from_items( &files, | ( path, _size ) : &( PathBuf, i32 ) | {
//!   path.components().map( | c | c.as_os_str().to_string_lossy().to_string() ).collect()
//! }, | ( path, size ) : &( PathBuf, i32 ) | ( path.clone(), *size ) );
//! # }
//! ```
//!
//! ## Formatting a table
//!
//! ```
//! # #[ cfg( feature = "enabled" ) ]
//! # {
//! use data_fmt::{ RowBuilder, TableFormatter };
//!
//! let tree = RowBuilder::new( vec![ "File".into(), "Lines".into() ] )
//!   .add_row( vec![ "main.rs".into(), "100".into() ] )
//!   .add_row( vec![ "lib.rs".into(), "200".into() ] )
//!   .build();
//!
//! let formatter = TableFormatter::new();
//! let output = formatter.format( &tree );
//! println!( "{}", output );
//! # }
//! ```

// Module declarations
#[ cfg( feature = "enabled" ) ] mod data;
#[ cfg( feature = "enabled" ) ] mod config;
#[ cfg( feature = "enabled" ) ] mod ansi_str;
#[ cfg( feature = "enabled" ) ] mod builder;
#[ cfg( feature = "enabled" ) ] mod table_tree;
#[ cfg( feature = "enabled" ) ] mod wrap;
#[ cfg( feature = "enabled" ) ] pub mod conversions;
#[ cfg( feature = "enabled" ) ] pub mod formatters;

#[ cfg( feature = "themes" ) ]
pub mod themes;

// Public re-exports - Core data types
#[ cfg( feature = "enabled" ) ]
pub use data::{
  TreeNode, ColumnData,
  TableView, TableMetadata, DataType, TableShapedView
};
#[ cfg( feature = "enabled" ) ]
pub use color_tools::DecoratedText;
#[ cfg( feature = "enabled" ) ]
pub use config::{
  TreeConfig, TableConfig, ExpandedConfig, PaddingSide, TreeSymbols,
  BorderVariant, HeaderSeparatorVariant, ColumnSeparator, ColumnFlex, FoldStyle,
};
#[ cfg( feature = "enabled" ) ]
pub use ansi_str::{ visual_len, pad_to_width, truncate_cell };
#[ cfg( feature = "enabled" ) ]
pub use wrap::{ WrapConfig, WrapFormatter, BreakStrategy, Overflow };
#[ cfg( feature = "enabled" ) ]
pub use builder::TreeBuilder;
#[ cfg( feature = "enabled" ) ]
pub use table_tree::RowBuilder;

// Format trait
#[ cfg( feature = "enabled" ) ]
pub use formatters::{ Format, FormatError };

// Conditional formatter exports (feature-gated)
#[ cfg( any(
  feature = "table_plain",
  feature = "table_minimal",
  feature = "table_bordered",
  feature = "table_markdown",
  feature = "table_grid",
  feature = "table_unicode",
  feature = "table_csv",
  feature = "table_tsv",
  feature = "table_compact"
) ) ]
pub use formatters::TableFormatter;

#[ cfg( any(
  feature = "expanded_postgres",
  feature = "expanded_property"
) ) ]
pub use formatters::ExpandedFormatter;

#[ cfg( any(
  feature = "tree_hierarchical",
  feature = "tree_aligned",
  feature = "tree_aggregated"
) ) ]
pub use formatters::TreeFormatter;

#[ cfg( feature = "format_logfmt" ) ]
pub use formatters::LogfmtFormatter;

#[ cfg( any(
  feature = "html_minimal",
  feature = "html_bootstrap",
  feature = "html_tailwind",
  feature = "html_custom"
) ) ]
pub use formatters::{ HtmlFormatter, HtmlVariant };

#[ cfg( any(
  feature = "sql_ansi",
  feature = "sql_postgres",
  feature = "sql_mysql",
  feature = "sql_sqlite"
) ) ]
pub use formatters::{ SqlFormatter, SqlVariant };

#[ cfg( feature = "format_json" ) ]
pub use formatters::JsonFormatter;

#[ cfg( feature = "format_yaml" ) ]
pub use formatters::YamlFormatter;

#[ cfg( feature = "format_toml" ) ]
pub use formatters::TomlFormatter;

#[ cfg( feature = "format_text" ) ]
pub use formatters::{ TextFormatter, TextVariant };

// Color themes (feature-gated)
#[ cfg( feature = "themes" ) ]
pub use themes::{ ColorTheme, ColorThemeBuilder };

// Backward compatibility trait
#[ allow( deprecated ) ]
#[ cfg( feature = "enabled" ) ]
pub use formatters::TableShapedFormatter;
