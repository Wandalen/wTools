#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Collection of general purpose tools for type checking.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

///
/// Collection of general purpose tools for type checking.
///
// /// # Sample
// /// ```
// /// use typing_tools::*;
// ///
// /// fn main()
// /// {
// ///   let src = Box::new( true );
// ///   assert_eq!( implements!( src => Copy ), false );
// ///   assert_eq!( implements!( src => Clone ), true );
// /// }
// /// ```

pub mod typing
{
  pub use inspect_type::*;
  pub use is_slice::*;
  pub use implements::*;
}

pub use typing::*;
