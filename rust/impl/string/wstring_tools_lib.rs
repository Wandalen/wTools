#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! String tools.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

// ///
// /// String tools.
// ///
// /// ### Sample
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

/// String tools.
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
