//! qqq : write proper description
#[ cfg( not( feature = "enabled" ) ) ]
fn main(){}

#[ cfg( feature = "enabled" ) ]
fn main()
{
  use macro_tools::{ typ, qt };

  let code = qt!( core::option::Option< i8, i16, i32, i64 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = typ::type_parameters( &tree_type, 0..=2 );
  got.iter().for_each( | e | println!( "{}", qt!( #e ) ) );
  /* print :
    i8
    i16
    i32
  */
}