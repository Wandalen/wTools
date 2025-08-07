//!
//! Utilities for manipulating identifiers, including keyword handling.
//!

/// Define a private namespace for all its items.
mod private {

  use crate::*; // Use crate's prelude/exposed items
  use convert_case::Casing;
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
  pub fn ident_maybe_raw(ident: &syn::Ident) -> Ident {
    let name = ident.to_string();
    if kw::is(&name) {
      // Use r# prefix if the name is a keyword
      format_ident!("r#{}", name, span = ident.span())
    } else {
      // Otherwise, use the name directly (cloned)
      ident.clone()
    }
  }

  /// Creates a new `syn::Ident` from an existing one, converting it to the specified case.
  ///
  /// This function handles raw identifier prefixes (`r#`) correctly and ensures that
  /// the newly created identifier is also a raw identifier if its cased version is a
  /// Rust keyword.
  ///
  /// # Arguments
  ///
  /// * `original` - The original `syn::Ident` to convert.
  /// * `case` - The target `convert_case::Case` to convert the identifier to.
  ///
  /// # Returns
  ///
  /// Returns a new `syn::Ident` in the specified case, preserving the span of the original
  /// identifier and handling raw identifiers (`r#`) appropriately.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use macro_tools::{ syn, format_ident };
  /// use convert_case::Case;
  ///
  /// let ident_normal = format_ident!( "my_variable" );
  /// let ident_keyword = format_ident!( "r#fn" );
  ///
  /// // Convert to PascalCase
  /// let got_pascal = macro_tools::ident::cased_ident_from_ident( &ident_normal, Case::Pascal );
  /// assert_eq!( got_pascal.to_string(), "MyVariable" );
  ///
  /// // Convert a raw identifier to SnakeCase
  /// let got_snake_raw = macro_tools::ident::cased_ident_from_ident( &ident_keyword, Case::Snake );
  /// assert_eq!( got_snake_raw.to_string(), "r#fn" );
  ///
  /// // Convert a normal identifier that becomes a keyword in the new case
  /// let ident_struct = format_ident!( "struct" );
  /// let got_pascal_keyword = macro_tools::ident::cased_ident_from_ident( &ident_struct, Case::Pascal );
  /// assert_eq!( got_pascal_keyword.to_string(), "Struct" ); // qqq: "Struct" is not a keyword, so `r#` is not added.
  /// ```
  #[must_use]
  pub fn cased_ident_from_ident(original: &syn::Ident, case: convert_case::Case) -> syn::Ident {
    let original_str = original.to_string();
    let had_raw_prefix = original_str.starts_with("r#");
    let core_str = if had_raw_prefix { &original_str[2..] } else { &original_str };

    let cased_str = core_str.to_case(case);

    if kw::is(&cased_str) {
      syn::Ident::new_raw(&cased_str, original.span())
    } else {
      syn::Ident::new(&cased_str, original.span())
    }
  }
}

#[doc(inline)]
#[allow(unused_imports)]
pub use own::*;

/// Own namespace of the module.
#[allow(unused_imports)]
pub mod own {

  use super::*;
  #[doc(inline)]
  pub use orphan::*;
  #[doc(inline)]
  pub use private::ident_maybe_raw;
  #[doc(inline)]
  pub use private::cased_ident_from_ident;
}

/// Orphan namespace of the module.
#[allow(unused_imports)]
pub mod orphan {

  use super::*;
  #[doc(inline)]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[allow(unused_imports)]
pub mod exposed {

  use super::*;
  pub use super::super::ident; // Use the new module name

  #[doc(inline)]
  pub use prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[allow(unused_imports)]
pub mod prelude {

  use super::*;
}
