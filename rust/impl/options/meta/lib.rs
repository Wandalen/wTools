#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Mechanism to define map of options for a fuction and its defaults laconically.
//!

// extern crate wproc_macro;
// mod former;
mod options;

// ///
// /// Attribute macro to generate options adapter and its implementation for structure option.
// ///
//
// #[ allow( non_snake_case ) ]
// #[ proc_macro_attribute ]
// pub fn Options( attr : proc_macro::TokenStream, item : proc_macro::TokenStream ) -> proc_macro::TokenStream
// {
//   let result = options::options( attr, item );
//   match result
//   {
//     Ok( stream ) => stream.into(),
//     Err( err ) => err.to_compile_error().into(),
//   }
// }

///
/// Function-like macro to generate options adapter and its implementation for structure option.
///

#[ allow( non_snake_case ) ]
#[ proc_macro ]
pub fn Options( item : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let attr = proc_macro::TokenStream::new();
  let result = options::options( attr, item );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}
