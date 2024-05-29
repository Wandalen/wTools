//!
//! Advanced syntax elements.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use interval_adapter::BoundExt;

  /// Check is the rightmost item of path refering a type is specified type.
  ///
  /// Good to verify `core::option::Option< i32 >` is optional.
  /// Good to verify `alloc::vec::Vec< i32 >` is vector.
  ///
  /// ### Basic use-case.
  /// ```rust
  /// use macro_tools::exposed::*;
  ///
  /// let code = qt!( core::option::Option< i32 > );
  /// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  /// let got = typ::type_rightmost( &tree_type );
  /// assert_eq!( got, Some( "Option".to_string() ) );
  /// ```

  pub fn type_rightmost( ty : &syn::Type ) -> Option< String >
  {
    if let syn::Type::Path( path ) = ty
    {
      let last = &path.path.segments.last();
      if last.is_none()
      {
        return None;
      }
      return Some( last.unwrap().ident.to_string() );
    }
    None
  }

  /// Return the specified number of parameters of the type.
  ///
  /// Good to getting `i32` from `core::option::Option< i32 >` or `alloc::vec::Vec< i32 >`
  ///
  /// ### Basic use-case.
  /// ```
  /// use macro_tools::{ typ, qt };
  ///
  /// let code = qt!( core::option::Option< i8, i16, i32, i64 > );
  /// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  /// let got = typ::type_parameters( &tree_type, 0..=2 );
  /// got.iter().for_each( | e | println!( "{}", qt!( #e ) ) );
  /// // < i8
  /// // < i16
  /// // < i32
  /// ```

  pub fn type_parameters( ty : &syn::Type, range : impl NonIterableInterval ) -> Vec< &syn::Type >
  {
    if let syn::Type::Path( syn::TypePath{ path : syn::Path { ref segments, .. }, .. } ) = ty
    {
      let last = &segments.last();
      if last.is_none()
      {
        return vec![ ty ]
      }
      let args = &last.unwrap().arguments;
      if let syn::PathArguments::AngleBracketed( ref args2 ) = args
      {
        let args3 = &args2.args;
        let left = range.left().into_left_closed();
        let mut right = range.right().into_right_closed();
        let len = args3.len();
        if right == isize::MAX
        {
          right = len as isize;
        }
        // dbg!( left );
        // dbg!( right );
        // dbg!( len );
        let selected : Vec< &syn::Type > = args3
        .iter()
        .skip_while( | e | !matches!( e, syn::GenericArgument::Type( _ ) ) )
        .skip( usize::try_from( left.max( 0 ) ).unwrap() )
        .take( usize::try_from( ( right - left + 1 ).min( len as isize - left ).max( 0 ) ).unwrap() )
        .map( | e | if let syn::GenericArgument::Type( ty ) = e { ty } else { unreachable!( "Expects Type" ) } )
        .collect();
        return selected;
      }
    }
    vec![ ty ]
  }

//   /// Extract generics from a type.
//   pub fn all_type_parameters( type_example : &syn::Type )
//   ->
//   Option< syn::punctuated::Punctuated< syn::GenericArgument, syn::token::Comma > >
//   {
//     if let syn::Type::Path( type_path ) = type_example
//     {
//       let segments = &type_path.path.segments;
//       let last_segment = segments.last()?;
//
//       if let syn::PathArguments::AngleBracketed( generics ) = &last_segment.arguments
//       {
//         return Some( generics.args.clone() );
//       }
//     }
//     None
//   }


  /// Checks if a given [`syn::Type`] is an `Option` type.
  ///
  /// This function examines a type to determine if it represents an `Option`.
  /// It is useful for scenarios where type-specific behavior needs to be conditional
  /// on whether the type is optional or not.
  ///
  /// # Example
  ///
  /// ```rust
  /// let type_string = "Option< i32 >";
  /// let parsed_type : syn::Type = syn::parse_str( type_string ).expect( "Type should parse correctly" );
  /// assert!( macro_tools::typ::is_optional( &parsed_type ) );
  /// ```
  ///

  pub fn is_optional( ty : &syn::Type ) -> bool
  {
    typ::type_rightmost( ty ) == Some( "Option".to_string() )
  }

  /// Extracts the first generic parameter from a given `syn::Type` if any exists.
  ///
  /// This function is designed to analyze a type and retrieve its first generic parameter.
  /// It is particularly useful when working with complex types in macro expansions and needs
  /// to extract specific type information for further processing.
  ///
///
  /// # Example
  /// ```rust
  /// let type_string = "Result< Option< i32 >, Error >";
  /// let parsed_type : syn::Type = syn::parse_str( type_string ).expect( "Type should parse correctly" );
  /// let first_param = macro_tools::typ::parameter_first( &parsed_type ).expect( "Should have at least one parameter" );
  /// // Option< i32 >
  /// ```

  pub fn parameter_first( ty : &syn::Type ) -> Result< &syn::Type >
  {
    typ::type_parameters( ty, 0 ..= 0 )
    .first()
    .copied()
    .ok_or_else( || syn_err!( ty, "Expects at least one parameter here:\n  {}", qt!{ #ty } ) )
  }

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    type_rightmost,
    type_parameters,
    // all_type_parameters,
    is_optional,
    parameter_first,
  };
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::protected as typ;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}

