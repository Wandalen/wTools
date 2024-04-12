//!
//! Advanced syntax elements.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use super::super::*;
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

