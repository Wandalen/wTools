//! Tree and Table Formatter Module
//!
//! Provides abstract, reusable formatters for hierarchical data display.
//!
//! # Features
//!
//! - **Generic `TreeNode`**: Works with any data type
//! - **`TreeBuilder`**: Constructs trees from flat data with path-based insertion
//! - **`TreeFormatter`**: Renders trees with configurable symbols and display options
//! - **`TableFormatter`**: Renders tabular data with borders and alignment
//! - **String Output**: All formatters return `String`, no direct console output

#![ allow( clippy::missing_inline_in_public_items ) ]
#![ allow( clippy::must_use_candidate ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::format_push_string ) ]
//!
//! The library supports three interchangeable display formats:
//!
//! - **Table**: Horizontal tabular display (standard row-column layout)
//! - **Expanded**: Vertical record display (`PostgreSQL` `\x` mode, key-value pairs)
//! - **Tree**: Hierarchical tree display (outline with box-drawing characters)
//!
//! # Examples
//!
//! ## Same data in all three formats
//!
//! ```
//! use tree_fmt::{ RowBuilder, TableFormatter, ExpandedFormatter, TreeFormatter };
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
//! let tree_fmt = TreeFormatter::default();
//! let output = tree_fmt.format( &tree, Clone::clone );
//! ```
//!
//! ## Expanded format with colored keys
//!
//! ```
//! use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };
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
//! ```
//!
//! ## Property list style (colon separator)
//!
//! ```
//! use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };
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
//! ```
//!
//! ## Building and formatting a tree
//!
//! ```
//! use tree_fmt::{ TreeBuilder, TreeFormatter };
//!
//! let tree = TreeBuilder::new( "root" )
//!   .insert( &[ "src", "main.rs" ], 150 )
//!   .insert( &[ "src", "lib.rs" ], 300 )
//!   .build();
//!
//! let formatter = TreeFormatter::new();
//! let output = formatter.format( &tree, | lines | format!( "{} lines", lines ) );
//! println!( "{}", output );
//! ```
//!
//! ## Building from items
//!
//! ```
//! use tree_fmt::TreeBuilder;
//! use std::path::PathBuf;
//!
//! let files = vec![
//!   ( PathBuf::from( "src/main.rs" ), 100 ),
//!   ( PathBuf::from( "tests/test.rs" ), 50 ),
//! ];
//!
//! let tree = TreeBuilder::from_items( &files, | ( path, _size ) | {
//!   path.components().map( | c | c.as_os_str().to_string_lossy().to_string() ).collect()
//! }, | ( path, size ) | ( path.clone(), *size ) );
//! ```
//!
//! ## Formatting a table
//!
//! ```
//! use tree_fmt::{ RowBuilder, TableFormatter };
//!
//! let tree = RowBuilder::new( vec![ "File".into(), "Lines".into() ] )
//!   .add_row( vec![ "main.rs".into(), "100".into() ] )
//!   .add_row( vec![ "lib.rs".into(), "200".into() ] )
//!   .build();
//!
//! let formatter = TableFormatter::new();
//! let output = formatter.format( &tree );
//! println!( "{}", output );
//! ```

// Module declarations
mod data;
mod config;
mod helpers;
mod builder;
mod table_tree;
pub mod conversions;
pub mod formatters;

#[ cfg( feature = "themes" ) ]
pub mod themes;

// Public re-exports - Core data types (always available)
pub use data::{
  TreeNode, ColumnData,
  TableView, TableMetadata, DataType, TableShapedView
};
pub use config::{
  TreeConfig, TableConfig, ExpandedConfig, PaddingSide, TreeSymbols,
  BorderVariant, HeaderSeparatorVariant, ColumnSeparator,
};
pub use helpers::{ visual_len, pad_to_width };
pub use builder::TreeBuilder;
pub use table_tree::RowBuilder;

// Format trait (always available)
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

// Backward compatibility trait (always available for now)
pub use formatters::TableShapedFormatter;
