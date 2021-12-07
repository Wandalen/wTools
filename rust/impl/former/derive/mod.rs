#![ warn( missing_docs ) ]
// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

//!
//! Former - a variation of builder pattern. Implementation of its derive macro. Should not be used independently, instead use module::former which relies on the module.
//!
//! Not intended to be used without runtime. This module and runtime is aggregate in module::former is [here](https://github.com/Wandalen/wTools/tree/master/module/rust/former).
//!

extern crate wproc_macro;
mod former;

/// Derive macro to generate former for a structure. Former is variation of Builder Pattern.

#[proc_macro_derive( Former )]
// #[proc_macro_error::proc_macro_error]
pub fn former( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = former::former( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}
