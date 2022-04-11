#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
// #![ allow( dead_code ) ]

//!
//! String tools.
//!
#![ doc = include_str!( "../../../module/rust/wstring_tools/Readme.md" ) ]

// ///
// /// String tools.
// ///
// /// # Sample
// /// ```
// /// use wstring_tools as wtools;
// /// use wstring_tools::prelude::*;
// ///
// /// fn main()
// /// {
// ///   /* delimeter exists */
// ///   let src = "abc def";
// ///   let iter = wtools::string::split()
// ///   .src( src )
// ///   .delimeter( " " )
// ///   .perform();
// ///   let iterated = iter.map( | e | String::from( e ) ).collect::< Vec< _ > >();
// ///   assert_eq!( iterated, vec![ "abc", " ", "def" ] );
// ///
// ///   /* delimeter no exists */
// ///   let src = "abc def";
// ///   let iter = wtools::string::split()
// ///   .src( src )
// ///   .delimeter( "g" )
// ///   .perform();
// ///   let iterated = iter.map( | e | String::from( e ) ).collect::< Vec< _ > >();
// ///   assert_eq!( iterated, vec![ "abc def" ] );
// /// }
// /// ```

pub mod string
{
  include!( "./lib.rs" );
}

/// Exposed namespace of the module.
pub mod exposed
{
}
pub use exposed::*;

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  pub use super::string::prelude::*;
}
