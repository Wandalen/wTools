
use super::*;

//

#[ test ]
fn parse()
{

  let attr : syn::Attribute = syn::parse_quote!( #[ default( 31 ) ] );
  tree_print!( attr );

  let attr : syn::Attribute = syn::parse_quote!( #[ default[ 31 ] ] );
  tree_print!( attr );

  let attr : syn::Attribute = syn::parse_quote!( #[ former( default = 31 ) ] );
  // tree_print!( attr );
  let got = equation( &attr ).unwrap();
  a_id!( code_to_str!( got ), "default = 31".to_string() );
  a_id!( got.left, syn::parse_quote!( default ) );
  a_id!( got.op, syn::token::Eq::default() );
  a_id!( code_to_str!( got.right ), "31".to_string() );

}

#[ test ]
fn is_standard_standard()
{
  // Test a selection of attributes known to be standard
  assert!( is_standard( "cfg" ), "Expected 'cfg' to be a standard attribute." );
  assert!( is_standard( "derive" ), "Expected 'derive' to be a standard attribute." );
  assert!( is_standard( "inline" ), "Expected 'inline' to be a standard attribute." );
  assert!( is_standard( "test" ), "Expected 'test' to be a standard attribute." );
  assert!( is_standard( "doc" ), "Expected 'doc' to be a standard attribute." );
}

#[ test ]
fn is_standard_non_standard()
{
  // Test some made-up attributes that should not be standard
  assert!( !is_standard( "custom_attr" ), "Expected 'custom_attr' to not be a standard attribute." );
  assert!( !is_standard( "my_attribute" ), "Expected 'my_attribute' to not be a standard attribute." );
  assert!( !is_standard( "special_feature" ), "Expected 'special_feature' to not be a standard attribute." );
}

#[ test ]
fn is_standard_edge_cases()
{
  // Test edge cases like empty strings or unusual input
  assert!( !is_standard( "" ), "Expected empty string to not be a standard attribute." );
  assert!( !is_standard( " " ), "Expected a single space to not be a standard attribute." );
  assert!( !is_standard( "cfg_attr_extra" ), "Expected 'cfg_attr_extra' to not be a standard attribute." );
}
