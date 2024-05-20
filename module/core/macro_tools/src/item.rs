//! This module provides various utilities and namespaces for working with `syn::Item`, specifically focusing on
//! ensuring syntactical correctness and managing different visibility levels within the code. It includes functions
//! to manipulate the structure of items, handle different kinds of fields, and provide a structured approach to
//! organizing the codebase into different access levels.

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  /// Ensures the last field in a struct has a trailing comma.
  ///
  /// This function checks and modifies the fields of a given struct, `input`, ensuring that the last field, whether in
  /// named or unnamed structs, ends with a trailing comma. This adjustment is commonly needed in macro-generated
  /// code to maintain consistency and syntactical correctness across different struct types, including unit structs
  /// which are unaffected as they do not contain fields.
  ///
  /// # Arguments
  ///
  /// * `input` - A reference to the struct (`syn::ItemStruct`) whose fields are to be checked and modified.
  ///
  /// # Returns
  ///
  /// Returns a modified clone of the input struct (`syn::ItemStruct`) where the last field in named or unnamed
  /// structs has a trailing comma. Unit structs remain unchanged as they do not contain fields.
  ///
  /// # Examples
  ///
  /// ```
  /// use macro_tools::
  /// {
  ///   syn::{ parse_quote, ItemStruct },
  ///   quote::quote,
  /// };
  ///
  /// // Create a struct using `parse_quote!` macro
  /// let input_struct : ItemStruct = parse_quote!
  /// {
  ///   struct Example
  ///   {
  ///     field1 : i32,
  ///     field2 : String
  ///   }
  /// };
  ///
  /// // Apply `ensure_comma` to ensure the last field has a trailing comma
  /// let modified_struct = macro_tools::item::ensure_comma( &input_struct );
  ///
  /// // Now `modified_struct` will have a trailing comma after `field2`
  /// assert_eq!( quote!( #modified_struct ).to_string(), quote!
  /// {
  ///   struct Example
  ///   {
  ///     field1 : i32,
  ///     field2 : String,
  ///   }
  /// }.to_string() );
  /// ```

  pub fn ensure_comma( input : &syn::ItemStruct ) -> syn::ItemStruct
  {
    let mut new_input = input.clone(); // Clone the input to modify it

    match &mut new_input.fields
    {
      // Handle named fields
      syn::Fields::Named( syn::FieldsNamed { named, .. } ) =>
      {
        punctuated::ensure_trailing_comma( named )
      },
      // Handle unnamed fields (tuples)
      syn::Fields::Unnamed( syn::FieldsUnnamed { unnamed, .. } ) =>
      {
        punctuated::ensure_trailing_comma( unnamed )
      },
      // Do nothing for unit structs
      syn::Fields::Unit => {}
    }

    new_input
  }

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

// qqq : zzz : make sure documentation look good. generate, review and fix every file
/// This module provides various utilities and namespaces for working with `syn::Item`, specifically focusing on
/// ensuring syntactical correctness and managing different visibility levels within the code. It includes functions
/// to manipulate the structure of items, handle different kinds of fields, and provide a structured approach to
/// organizing the codebase into different access levels.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    ensure_comma,
  };
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };
}

/// Exposed namespace of the module.
pub mod exposed
{
  // pub use super::protected as item;
  pub use super::protected as item;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    prelude::*,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
