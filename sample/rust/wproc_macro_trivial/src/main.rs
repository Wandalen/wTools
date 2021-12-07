pub use wproc_macro::*;
use quote::quote;

fn main()
{

  let code = quote!( core::option::Option< i8, i16, i32, i64 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = type_parameters( &tree_type, 0..=2 );
  println!( "{:?}", got );

}
