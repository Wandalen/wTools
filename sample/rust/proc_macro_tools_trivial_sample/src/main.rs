
fn main()
{
  #[ cfg( feature = "use_std" ) ]
  {
    use proc_macro_tools::*;

    let code = qt!( core::option::Option< i8, i16, i32, i64 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = type_parameters( &tree_type, 0..=2 );
    got.iter().for_each( | e | println!( "{}", qt!( #e ) ) );
    /* print :
      i8
      i16
      i32
    */
  }
}
