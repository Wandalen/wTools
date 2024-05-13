
use super::*;

//

#[ test ]
fn tokens()
{

  let got : the_module::Tokens = syn::parse_quote!( a = b );
  // tree_print!( got );
  a_id!( got.to_string(), "a = b".to_string() );

  let got : the_module::Tokens = syn::parse_quote!( #[ former( default = 31 ) ] );
  // tree_print!( got );
  a_id!( got.to_string(), "# [former (default = 31)]".to_string() );

}

//

#[ test ]
fn equation()
{

  let got : the_module::Equation = syn::parse_quote!( default = 31 );
  tree_print!( got );
  a_id!( code_to_str!( got ), "default = 31".to_string() );

  a_id!( got.left, syn::parse_quote!( default ) );
  a_id!( got.op, syn::token::Eq::default() );
  a_id!( code_to_str!( got.right ), "31".to_string() );

}
