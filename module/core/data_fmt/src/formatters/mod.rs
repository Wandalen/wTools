//! Formatting implementations for different display modes

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

/// Build a `Vec<HashMap<String, String>>` from `TableView` by zipping column names
/// with row cell text — shared by json, yaml, and toml formatters.
#[ cfg( any( feature = "format_json", feature = "format_yaml", feature = "format_toml" ) ) ]
fn table_view_to_row_maps( data : &crate::TableView ) -> Vec< std::collections::HashMap< String, String > >
{
  let column_names = &data.metadata.column_names;
  data.rows
    .iter()
    .map( | row |
    {
      column_names
        .iter()
        .zip( row.iter() )
        .map( | ( name, value ) | ( name.clone(), value.text.clone() ) )
        .collect()
    })
    .collect()
}

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

