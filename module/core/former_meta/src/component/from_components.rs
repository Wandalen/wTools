#[ allow( clippy::wildcard_imports ) ]
use super::*;
// Use re-exports from macro_tools
use macro_tools::
{
  attr, diag, item_struct, Result,
  proc_macro2::TokenStream,
};


///
/// Generates an implementation of the `From< T >` trait for a custom struct, enabling
/// type-based conversion from `T` to the struct. This function parses the given
/// `TokenStream` representing a struct, and produces code that allows for its
/// fields to be initialized from an instance of type `T`, assuming `T` can be
/// converted into each of the struct's field types.
///
/// # Example of generated code for a tuple struct
///
/// ```ignore
/// impl< T > From< T > for TargetTuple
/// where
///   T : Clone,
///   T : Into< i32 >,
///   T : Into< String >,
/// {
///   #[ inline( always ) ]
///   fn from( src : T ) -> Self
///   {
///     let field_0 = Into::< i32 >::into( src.clone() );
///     let field_1 = Into::< String >::into( src.clone() );
///     Self( field_0, field_1 ) // Uses tuple construction
///   }
/// }
/// ```
///