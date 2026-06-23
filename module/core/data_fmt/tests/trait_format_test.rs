//! Format trait spec tests (TR-1..TR-6)
//!
//! Covers `Format::format` dispatch, `FormatError` variants, trait object usage,
//! and empty-table handling.

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, Format, FormatError, TableFormatter, TableConfig };

/// TR-1: format method returns Ok on valid input
// test_kind: spec_case(TR-1)
#[ test ]
fn trait_001_tr_01_format_method_returns_ok_on_valid_input()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::plain() );
  let result = Format::format( &formatter, &view );

  assert!( result.is_ok(), "format should return Ok on valid input" );
  let output = result.unwrap();
  assert!( output.contains( "Name" ), "output should contain header 'Name'" );
  assert!( output.contains( "Age" ), "output should contain header 'Age'" );
  assert!( output.contains( "Alice" ), "output should contain cell 'Alice'" );
  assert!( output.contains( "30" ), "output should contain cell '30'" );
}

/// TR-2: `FormatError::InvalidData` carries message
// test_kind: spec_case(TR-2)
#[ test ]
fn trait_001_tr_02_format_error_invalid_data_carries_message()
{
  let err = FormatError::InvalidData( "missing columns".into() );
  let display = format!( "{err}" );

  assert!(
    display.contains( "Invalid data: missing columns" ),
    "Display should contain 'Invalid data: missing columns', got: {display}",
  );

  // Pattern match to verify variant
  assert!(
    matches!( err, FormatError::InvalidData( _ ) ),
    "error should match InvalidData variant",
  );
}

/// TR-3: `FormatError::Serialization` cfg-gated construction
// test_kind: spec_case(TR-3)
#[ cfg( feature = "serde_support" ) ]
#[ test ]
fn trait_001_tr_03_format_error_serialization_cfg_gated()
{
  let err = FormatError::Serialization( "unexpected token".into() );
  let display = format!( "{err}" );

  assert!(
    display.contains( "Serialization error: unexpected token" ),
    "Display should contain 'Serialization error: unexpected token', got: {display}",
  );

  // Verify it's distinct from other variants
  assert!( !matches!( err, FormatError::InvalidData( _ ) ) );
  assert!( !matches!( err, FormatError::UnsupportedOperation( _ ) ) );
}

/// TR-4: format dispatches through trait object
// test_kind: spec_case(TR-4)
#[ test ]
fn trait_001_tr_04_format_dispatches_through_trait_object()
{
  let view = RowBuilder::new( vec![ "x".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::plain() );

  // Direct call
  let direct = Format::format( &formatter, &view ).unwrap();

  // Trait object call
  let dyn_fmt : &dyn Format = &formatter;
  let via_trait_obj = dyn_fmt.format( &view ).unwrap();

  assert_eq!( direct, via_trait_obj, "trait object dispatch should produce identical output" );
}

/// TR-5: `FormatError::UnsupportedOperation` carries message
// test_kind: spec_case(TR-5)
#[ test ]
fn trait_001_tr_05_format_error_unsupported_operation_carries_message()
{
  let err = FormatError::UnsupportedOperation( "pivot not supported".into() );
  let display = format!( "{err}" );

  assert!(
    display.contains( "Unsupported operation: pivot not supported" ),
    "Display should contain 'Unsupported operation: pivot not supported', got: {display}",
  );

  assert!(
    matches!( err, FormatError::UnsupportedOperation( _ ) ),
    "error should match UnsupportedOperation variant",
  );
}

/// TR-6: format on empty table returns Ok
// test_kind: spec_case(TR-6)
#[ test ]
fn trait_001_tr_06_format_on_empty_table_returns_ok()
{
  let view = RowBuilder::new( vec![ "col_a".into(), "col_b".into() ] )
    .build_view();

  let formatter = TableFormatter::new();
  let result = Format::format( &formatter, &view );

  assert!( result.is_ok(), "empty table should not produce an error" );
  let output = result.unwrap();
  assert!( output.contains( "col_a" ), "output should contain header 'col_a'" );
}
