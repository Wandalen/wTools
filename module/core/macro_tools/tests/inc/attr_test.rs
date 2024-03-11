
use super::*;

//

#[ test ]
fn basic()
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
