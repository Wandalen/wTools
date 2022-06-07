#[ allow( unused_imports ) ]
use proc_macro_tools::*;
use quote::quote;

fn main()
{
  let code = quote!( core::option::Option< i8, i16, i32, i64 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  #[ cfg( feature = "use_std" ) ]
  {
    let got = type_parameters( &tree_type, 0..=2 );
    got.iter().for_each( | e | println!( "{}", quote!( #e ) ) );
    // < i8
    // < i16
    // < i32
  }
  #[ cfg( not( feature = "use_std" ) ) ]
  println!( "{:?}", tree_type );
}
