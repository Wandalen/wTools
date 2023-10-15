
// #[ cfg( not( feature = "no_std" ) ) ]
mod former_impl;

///
/// Derive macro to generate former for a structure. Former is variation of Builder Pattern.
///

// #[ cfg( not( feature = "no_std" ) ) ]
#[ proc_macro_derive( Former, attributes( perform, default, setter, alias, doc ) ) ]
pub fn former( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = former_impl::former( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}
