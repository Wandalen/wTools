#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Collection of general purpose meta tools.
//!
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

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
  pub use either::*;
}

pub use meta::*;

// zzz : use
//
// macro_rules! for_each_tuple_ {
//     ( $m:ident !! ) => (
//         $m! { }
//     );
//     ( $m:ident !! $h:ident, $($t:ident,)* ) => (
//         $m! { $h $($t)* }
//         for_each_tuple_! { $m !! $($t,)* }
//     );
// }
// macro_rules! for_each_tuple {
//     ($m:ident) => {
//         for_each_tuple_! { $m !! A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, }
//     };
// }