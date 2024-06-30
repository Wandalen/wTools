//!
//! Structures and functions for handling `syn::punctuated::Punctuated` collections.
//!
//! This module provides functionality to manipulate and ensure correct punctuation in `syn::punctuated::Punctuated` collections, commonly used in procedural macros to represent sequences of elements separated by punctuation marks, such as commas.
//!

/// Internal namespace.
pub( crate ) mod private
{

  /// Ensures that a `syn::punctuated::Punctuated` collection ends with a comma if it contains elements.
  pub fn ensure_trailing_comma< T : Clone >
  ( punctuated : &mut syn::punctuated::Punctuated< T, syn::token::Comma > )
  {
    if !punctuated.empty_or_trailing()
    {
      punctuated.push_punct( syn::token::Comma::default() );
    }
  }

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

#[ allow( unused_imports ) ]
pub mod protected
{
  //!
  //! Structures and functions for handling `syn::punctuated::Punctuated` collections.
  //!
  //! This module provides functionality to manipulate and ensure correct punctuation in `syn::punctuated::Punctuated` collections, commonly used in procedural macros to represent sequences of elements separated by punctuation marks, such as commas.
  //!

  #[ doc( inline ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    ensure_trailing_comma,
  };
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  pub use super::super::punctuated;
  // pub use super::protected as punctuated;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    prelude::*,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
}
