//! Formatter parameter types and configuration structures
//!
//! Previously a single `config.rs` (~1050 lines, three distinct domains).
//! Now split into focused sub-modules, each owning one config type family:
//! - [`tree_config`] — `TreeConfig`, `TreeSymbols`
//! - [`table_enums`] — style enums used by `TableConfig`
//! - [`table_caption`] — `TableCaption` and caption constants
//! - [`table_config`] — `TableConfig`, `PaddingSide`
//! - [`expanded_config`] — `ExpandedConfig`

mod tree_config;
pub use tree_config::{ TreeConfig, TreeSymbols };

mod table_enums;
pub use table_enums::{ BorderVariant, HeaderSeparatorVariant, ColumnSeparator, ColumnFlex, FoldStyle };

mod table_caption;
pub use table_caption::{ CAPTION_FIELD_SEP, CAPTION_RULE_CHAR, CAPTION_LEAD_WIDTH, TableCaption };

mod table_config;
pub use table_config::{ TableConfig, PaddingSide };

mod expanded_config;
pub use expanded_config::ExpandedConfig;
