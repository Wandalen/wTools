#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
#![ allow( dead_code ) ]

//!
//! String tools.
//!

///
/// String tools.
///

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
