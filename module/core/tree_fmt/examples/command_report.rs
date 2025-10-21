//! Command execution report formatting example
//!
//! Demonstrates property list style for clean command/process status output.
//! Shows the specialized `property_style()` configuration with automatic gray key coloring.
//!
//! NOTE: `ExpandedFormatter` also implements the `Format` trait for format-agnostic code.
//! See `unified_formats.rs` example for usage with the unified interface.

use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };

fn main()
{
  // Command execution data
  let report = RowBuilder::new( vec![
    "Command".into(),
    "Working Directory".into(),
    "Status".into(),
    "Started At".into(),
    "Completed At".into(),
    "Duration".into(),
    "Exit Code".into(),
  ])
  .add_row( vec![
    "sleep 10 && echo hello1".into(),
    "/".into(),
    "Completed".into(),
    "2025-10-24 22:16:26".into(),
    "2025-10-24 22:16:36".into(),
    "10 seconds".into(),
    "0".into(),
  ])
  .build();

  // Format as property list (colon separator, aligned values, gray keys)
  // property_style() automatically enables gray key coloring for better readability
  let formatter = ExpandedFormatter::with_config( ExpandedConfig::property_style() );
  let output = formatter.format( &report );

  println!( "{output}" );
}
