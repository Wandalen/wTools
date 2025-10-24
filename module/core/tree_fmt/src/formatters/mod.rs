//! Formatting implementations for different display modes

use crate::TreeNode;

// Core format trait (always available)
mod format_trait;
pub use format_trait::{ Format, FormatError };

// Visual formatters (feature-gated)
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
mod table;
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
pub use table::TableFormatter;

#[ cfg( any(
  feature = "expanded_postgres",
  feature = "expanded_property"
) ) ]
mod expanded;
#[ cfg( any(
  feature = "expanded_postgres",
  feature = "expanded_property"
) ) ]
pub use expanded::ExpandedFormatter;

#[ cfg( any(
  feature = "tree_hierarchical",
  feature = "tree_aligned",
  feature = "tree_aggregated"
) ) ]
mod tree;
#[ cfg( any(
  feature = "tree_hierarchical",
  feature = "tree_aligned",
  feature = "tree_aggregated"
) ) ]
pub use tree::TreeFormatter;

#[ cfg( feature = "format_logfmt" ) ]
mod logfmt;
#[ cfg( feature = "format_logfmt" ) ]
pub use logfmt::LogfmtFormatter;

#[ cfg( any(
  feature = "html_minimal",
  feature = "html_bootstrap",
  feature = "html_tailwind",
  feature = "html_custom"
) ) ]
mod html;
#[ cfg( any(
  feature = "html_minimal",
  feature = "html_bootstrap",
  feature = "html_tailwind",
  feature = "html_custom"
) ) ]
pub use html::{ HtmlFormatter, HtmlVariant };

#[ cfg( any(
  feature = "sql_ansi",
  feature = "sql_postgres",
  feature = "sql_mysql",
  feature = "sql_sqlite"
) ) ]
mod sql;
#[ cfg( any(
  feature = "sql_ansi",
  feature = "sql_postgres",
  feature = "sql_mysql",
  feature = "sql_sqlite"
) ) ]
pub use sql::{ SqlFormatter, SqlVariant };

// Data serialization formatters (feature-gated)
#[ cfg( feature = "format_json" ) ]
mod json;
#[ cfg( feature = "format_json" ) ]
pub use json::JsonFormatter;

#[ cfg( feature = "format_yaml" ) ]
mod yaml;
#[ cfg( feature = "format_yaml" ) ]
pub use yaml::YamlFormatter;

#[ cfg( feature = "format_toml" ) ]
mod toml_fmt;
#[ cfg( feature = "format_toml" ) ]
pub use toml_fmt::TomlFormatter;

#[ cfg( feature = "format_text" ) ]
mod text;
#[ cfg( feature = "format_text" ) ]
pub use text::{ TextFormatter, TextVariant };

/// Common trait for formatters that work with table-shaped trees
///
/// This trait provides a unified interface for formatters that operate on
/// table-shaped `TreeNode<String>` structures (those constructed via `RowBuilder`
/// or formatted via `TableView` trait).
///
/// # Examples
///
/// ```
/// use tree_fmt::{ RowBuilder, formatters::TableShapedFormatter, TableFormatter, ExpandedFormatter };
///
/// let tree = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
///   .add_row( vec![ "Alice".into(), "30".into() ] )
///   .build();
///
/// // Use formatters through the trait
/// let table : &dyn TableShapedFormatter = &TableFormatter::new();
/// let expanded : &dyn TableShapedFormatter = &ExpandedFormatter::new();
///
/// let table_output = table.format( &tree );
/// let expanded_output = expanded.format( &tree );
/// ```
pub trait TableShapedFormatter
{
  /// Format a table-shaped tree as a string
  ///
  /// # Arguments
  ///
  /// * `tree` - A table-shaped `TreeNode<String>` to format
  ///
  /// # Returns
  ///
  /// Formatted string representation
  fn format( &self, tree : &TreeNode< String > ) -> String;
}
