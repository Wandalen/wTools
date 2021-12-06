#![ warn( missing_docs ) ]
// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

//!
//! Former - a variation of builder pattern. Implementation of its derive macro. Should not be used independently, instead use module::former which relies on the module.
//!

// extern crate proc_macro;
// extern crate proc_macro_error;

/* xxx */
// #[macro_use]
extern crate wproc_macro;
// mod wproc_macro;
mod former;

// use tools_proc_macro as tpm;

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
