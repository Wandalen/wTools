#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
#![ allow( dead_code ) ]

//!
//! String tools.
//!
#![ doc = include_str!( "../../../module/rust/wstring_tools/Readme.md" ) ]

///
/// String tools.
///
// /// # Sample
// /// ```
// /// use wstring_tools::*;
// ///
// /// fn main()
// /// {
// ///   /* delimeter exists */
// ///   let src = "abc def";
// ///   let iter = string::split()
// ///   .src( src )
// ///   .delimeter( " " )
// ///   .form();
// ///   let iterated = iter.map( | e | String::from( e ) ).collect::< Vec< _ > >();
// ///   assert_eq!( iterated, vec![ "abc", " ", "def" ] );
// ///
// ///   /* delimeter no exists */
// ///   let src = "abc def";
// ///   let iter = string::split()
// ///   .src( src )
// ///   .delimeter( "g" )
// ///   .form();
// ///   let iterated = iter.map( | e | String::from( e ) ).collect::< Vec< _ > >();
// ///   assert_eq!( iterated, vec![ "abc def" ] );
// /// }
// /// ```

pub mod string
{
  ///
  /// String split.
  ///
  pub mod split
  {
    include!( "./split.rs" );
  }
  pub use split::split;
}
