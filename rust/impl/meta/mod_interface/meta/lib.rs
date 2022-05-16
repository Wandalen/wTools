#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

//!
//! Protocol of modularity unifying interface of a module.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

mod impls;

///
/// Protocol of modularity unifying interface of a module.
///

#[ proc_macro ]
pub fn mod_interface( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = impls::impls( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

/*

mod_interface!
{

  pub mod file1;
  pub mod file2;

  private mod micro_private;
  protected mod micro_protected;
  orphan mod micro_orphan;
  exposed mod micro_exposed;
  prelude mod micro_prelude;

  use prelude_file::*;

}

      private      <      protected      <      orphan      <      exposed      <      prelude
      itself               itself             its parent       its inter-module    its inter-module
      private              public               public             public              public

micro-module < meso-module < macro-module < inter-module

slim module - ?
complete module - ?

*/

