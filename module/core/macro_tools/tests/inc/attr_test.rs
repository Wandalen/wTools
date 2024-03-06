
use super::*;

//

#[ test ]
fn basic()
{

  let attr : syn::Attribute = syn::parse_quote!( #[ former( default = 31 ) ] );
  let ( key, val, _meta ) = attr::eq_pair( &attr ).unwrap();
  assert_eq!( key, "default" );
  assert_eq!( val, syn::Lit::Int( syn::LitInt::new( "31", proc_macro2::Span::call_site() ) ) );

}
