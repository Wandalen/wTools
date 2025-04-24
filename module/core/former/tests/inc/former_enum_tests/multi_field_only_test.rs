// File: module/core/former/tests/inc/former_enum_tests/multi_field_only_test.rs
use super::*;

#[ test ]
fn enum_variant_constructors()
{
  // Test the Simple variant - Expects direct constructor due to #[scalar]
  let got_simple = EnumWithMultiField::simple( "test simple" );
  let exp_simple = EnumWithMultiField::Simple( "test simple".to_string() );
  assert_eq!( got_simple, exp_simple );

  // Test the MultiTuple variant - Expects former builder due to #[scalar] and multi-fields
  let got_multi = EnumWithMultiField::multi_tuple()
    ._0( 42 )
    ._1( "hello" )
    ._2( true )
    .form();
  let exp_multi = EnumWithMultiField::MultiTuple( 42, "hello".to_string(), true );
  assert_eq!( got_multi, exp_multi );

  // Test the Empty variant - Expects direct constructor (default for unit)
  let got_empty = EnumWithMultiField::empty();
  let exp_empty = EnumWithMultiField::Empty;
  assert_eq!( got_empty, exp_empty );

  // Test the Struct variant - Expects subformer due to #[subform_scalar]
  let got_struct = EnumWithMultiField::r#struct() // Use raw identifier for method name
    .data1( -1 )
    .data2( false )
    .form();
  let exp_struct = EnumWithMultiField::Struct( InnerData { data1: -1, data2: false } );
  assert_eq!( got_struct, exp_struct );

  // Test the ImplicitSubform variant - Expects subformer (default for single Former field)
  let got_implicit = EnumWithMultiField::implicit_subform()
    .info( "implicit data".to_string() )
    .form();
  let exp_implicit = EnumWithMultiField::ImplicitSubform( OtherInnerData { info: "implicit data".to_string() } );
  assert_eq!( got_implicit, exp_implicit );

}