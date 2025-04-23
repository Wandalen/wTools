//!
//! Utilities for manipulating identifiers, including keyword handling.
//!

/// Define a private namespace for all its items.
mod private
{
  #[ allow( clippy::wildcard_imports ) ]
  use crate::*; // Use crate's prelude/exposed items
  use proc_macro2::Ident;
  // use syn::spanned::Spanned; // Needed for span

  /// Creates a new identifier, adding the `r#` prefix if the input identifier's
  /// string representation is a Rust keyword.
  ///
  /// Preserves the span of the original identifier.
  /// Requires the `kw` feature.
  ///
  /// # Example
  /// ```rust
  /// use macro_tools::{ syn, format_ident, ident };
  ///
  /// let ident_normal = format_ident!( "my_var" );
  /// let ident_keyword = format_ident!( "fn" );
  ///
  /// let got_normal = ident::ident_maybe_raw( &ident_normal );
  /// let got_keyword = ident::ident_maybe_raw( &ident_keyword );
  ///
  /// assert_eq!( got_normal.to_string(), "my_var" );
  /// assert_eq!( got_keyword.to_string(), "r#fn" );
  /// ```
  #[must_use]
  pub fn ident_maybe_raw( ident : &syn::Ident ) -> Ident
  {
    let name = ident.to_string();
    if kw::is( &name )
    {
      // Use r# prefix if the name is a keyword
      format_ident!( "r#{}", name, span = ident.span() )
    }
    else
    {
      // Otherwise, use the name directly (cloned)
      ident.clone()
    }
  }
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
  #[ doc( inline ) ]
  pub use private::ident_maybe_raw; // Export the renamed function
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  pub use super::super::ident; // Use the new module name

  #[ doc( inline ) ]
  pub use prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
}
