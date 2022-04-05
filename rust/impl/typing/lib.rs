#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Collection of general purpose tools for type checking.
//!

///
/// Collection of general purpose tools for type checking.
///
/// # Sample
/// ```
/// use typing_tools::*;
///
/// fn main()
/// {
///   let src = Box::new( true );
///   assert_eq!( implements!( src => Copy ), false );
///   assert_eq!( implements!( src => Clone ), true );
/// }
/// ```

pub mod typing
{
  pub use inspect_type::*;
  pub use is_slice::*;
  pub use implements::*;
}

pub use typing::*;
