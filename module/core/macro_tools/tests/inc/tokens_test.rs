
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
