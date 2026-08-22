//! Formatter parameter types and configuration structures
//!
//! Previously a single `config.rs` (~1050 lines, three distinct domains).
//! Now split into focused sub-modules, each owning one config type family:
//! - [`tree_config`] — `TreeConfig`, `TreeSymbols`
//! - [`table_enums`] — style enums used by `TableConfig`
//! - [`table_heading`] — `Heading` and heading constants
//! - [`table_config`] — `TableConfig`, `PaddingSide`
//! - [`expanded_config`] — `ExpandedConfig`

mod tree_config;
pub use tree_config::{ TreeConfig, TreeSymbols };

mod table_enums;
pub use table_enums::{ BorderVariant, HeaderSeparatorVariant, ColumnSeparator, ColumnFlex, FoldStyle };

mod table_heading;
pub use table_heading::{ HEADING_FIELD_SEP, HEADING_RULE_CHAR, HEADING_LEAD_WIDTH, Heading };
pub( crate ) use table_heading::render_rule_if_present;
// Only `sql`/`toml_fmt`/`yaml`/`html` formatters call this — mirror their exact mod-level
// cfg gates (formatters/mod.rs) so `enabled`-only builds (e.g. the heading_basic
// example) don't see it as dead code (Fix FT-9).
#[ cfg( any(
  feature = "sql_ansi",
  feature = "sql_postgres",
  feature = "sql_mysql",
  feature = "sql_sqlite",
  feature = "format_toml",
  feature = "format_yaml",
  feature = "html_minimal",
  feature = "html_bootstrap",
  feature = "html_tailwind",
  feature = "html_custom"
) ) ]
pub( crate ) use table_heading::render_commented_rule_if_present;

mod table_config;
pub use table_config::{ TableConfig, PaddingSide };

mod expanded_config;
pub use expanded_config::ExpandedConfig;
