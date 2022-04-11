#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Collection of general purpose meta tools.
//!
#![ doc = include_str!( "../../../module/rust/meta_tools/Readme.md" ) ]

///
/// Collection of general purpose meta tools.
///
// /// # Sample
// /// ```
// /// use meta_tools::*;
// ///
// /// fn main()
// /// {
// ///   let meta_map = hmap! { 3 => 13 };
// ///   let mut std_map = std::collections::HashMap::new();
// ///   std_map.insert( 3, 13 );
// ///   assert_eq!( meta_map, std_map );
// /// }
// /// ```

pub mod meta
{
  // pub use maplit::*;
  pub use literally::*;
}

pub use meta::*;
