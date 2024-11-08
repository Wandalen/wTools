//!
//! Structures and functions for handling `syn::punctuated::Punctuated` collections.
//!
//! This module provides functionality to manipulate and ensure correct punctuation in `syn::punctuated::Punctuated` collections, commonly used in procedural macros to represent sequences of elements separated by punctuation marks, such as commas.
//!

/// Define a private namespace for all its items.
mod private
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
pub use own::*;

#[ allow( unused_imports ) ]
/// Own namespace of the module.
pub mod own
{
  use super::*;

  #[ doc( inline ) ]
  pub use orphan::*;
  #[ doc( inline ) ]
  pub use private::
  {
    ensure_trailing_comma,
  };
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  pub use super::super::punctuated;
  // pub use super::own as punctuated;

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
  use super::*;
}
