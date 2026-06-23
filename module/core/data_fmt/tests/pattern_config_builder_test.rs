//! Config builder pattern spec tests (PT-1..PT-3)
//!
//! Covers fluent setter chaining, default value preservation,
//! and by-value config consumption by formatter constructors.

#![ cfg( feature = "enabled" ) ]

use data_fmt::{
  RowBuilder, TableFormatter, ExpandedFormatter, TreeFormatter,
  TableConfig, ExpandedConfig, TreeConfig, ColumnSeparator, Format,
};

/// PT-1: config setter returns Self for chaining
///
/// Fluent setters on `TableConfig`, `ExpandedConfig`, and `TreeConfig`
/// return `Self`, allowing multi-setter chains.
// test_kind: spec_case(PT-1)
#[ test ]
fn pattern_004_pt_01_config_setter_returns_self_for_chaining()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ "val".into() ] )
    .build_view();

  // TableConfig: chain two setters and verify the result is usable
  let tc = TableConfig::default()
    .with_column_separator( ColumnSeparator::Character( '|' ) )
    .with_min_column_width( 5 );
  let tf = TableFormatter::with_config( tc );
  let out = Format::format( &tf, &view );
  assert!( out.is_ok(), "TableConfig chaining: formatter works after multi-setter chain" );

  // ExpandedConfig: chain two setters
  let ec = ExpandedConfig::new()
    .with_colorize_keys( true )
    .with_show_record_numbers( false );
  let ef = ExpandedFormatter::with_config( ec );
  assert!( Format::format( &ef, &view ).is_ok(), "ExpandedConfig chaining works" );

  // TreeConfig: chain two setters
  let trc = TreeConfig::default()
    .with_indent_size( 8 )
    .with_show_root( false );
  let _tree_fmt = TreeFormatter::with_config( trc );
  // Compiles and constructs — proof that chaining returns Self
}

/// PT-2: default values apply to unchained fields
///
/// Chaining one setter on a preset preserves all other preset defaults.
// test_kind: spec_case(PT-2)
#[ test ]
fn pattern_004_pt_02_default_values_apply_to_unchained_fields()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .add_row( vec![ "Alice".into(), "100".into() ] )
    .build_view();

  // bordered() preset output
  let preset_fmt = TableFormatter::with_config( TableConfig::bordered() );
  let preset_out = Format::format( &preset_fmt, &view ).unwrap();

  // bordered() with only column_separator overridden
  let modified_cfg = TableConfig::bordered()
    .with_column_separator( ColumnSeparator::String( " | ".into() ) );
  let modified_fmt = TableFormatter::with_config( modified_cfg );
  let modified_out = Format::format( &modified_fmt, &view ).unwrap();

  // The modified output uses the new separator
  assert!( modified_out.contains( " | " ), "new separator is present in output" );

  // The preset's border structure (top/bottom borders) is preserved
  // bordered() has top/bottom borders — both outputs should have them
  let preset_has_border = preset_out.starts_with( '+' ) || preset_out.starts_with( '┌' );
  let modified_has_border = modified_out.starts_with( '+' ) || modified_out.starts_with( '┌' );
  assert_eq!(
    preset_has_border, modified_has_border,
    "border structure preserved: only separator changed",
  );
}

/// PT-3: config passed by value to formatter constructor
///
/// `with_config()` consumes the config struct by value (not borrowed).
/// Works for `TableFormatter`, `ExpandedFormatter`, and `TreeFormatter`.
// test_kind: spec_case(PT-3)
#[ test ]
fn pattern_004_pt_03_config_passed_by_value()
{
  let view = RowBuilder::new( vec![ "X".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  // TableFormatter::with_config consumes TableConfig by value
  let tc = TableConfig::plain();
  let tf = TableFormatter::with_config( tc );
  // tc is moved — cannot use after this point
  assert!( Format::format( &tf, &view ).is_ok(), "TableFormatter from consumed config" );

  // ExpandedFormatter::with_config consumes ExpandedConfig by value
  let ec = ExpandedConfig::new();
  let ef = ExpandedFormatter::with_config( ec );
  assert!( Format::format( &ef, &view ).is_ok(), "ExpandedFormatter from consumed config" );

  // TreeFormatter::with_config consumes TreeConfig by value
  let trc = TreeConfig::default();
  let _tree_fmt = TreeFormatter::with_config( trc );
  // TreeFormatter doesn't impl Format, but constructed successfully
}
