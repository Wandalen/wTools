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
  #[ must_use ]
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

  /// Creates a `syn::Ident` from a string that is already in the target case.
  /// Handles Rust keywords and original raw identifier status.
  /// If `cased_name_str` is a keyword, or if `source_had_raw_prefix` is true,
  /// `syn::Ident::new_raw` is used. Otherwise, `syn::Ident::new` is used.
  ///
  /// Returns an error if `cased_name_str` is empty or an invalid identifier.
  pub fn new_ident_from_cased_str
  (
    cased_name_str: &str,
    span: proc_macro2::Span,
    source_had_raw_prefix: bool
  ) -> Result<syn::Ident> // Use local Result<T> alias
  {
    if cased_name_str.is_empty() {
      return Err(syn::Error::new(span, "Cannot create identifier from empty string"));
    }

    // Comprehensive list of Rust 2021 keywords that are problematic as idents.
    // Based on https://doc.rust-lang.org/reference/keywords.html
    const RUST_KEYWORDS: &[&str] = &[
      // Strict keywords
      "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
      "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
      "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true",
      "type", "unsafe", "use", "where", "while",
      // Reserved keywords
      "abstract", "async", "await", "become", "box", "do", "final", "macro", "override",
      "priv", "try", "typeof", "unsized", "virtual", "yield",
      // Weak keywords
      "dyn", "union",
    ];

    let is_keyword = RUST_KEYWORDS.contains(&cased_name_str);

    if source_had_raw_prefix || is_keyword {
      // Validate if the string is permissible for new_raw, even if it's a keyword.
      // For example, "123" is not a keyword but also not valid for new_raw("123", span).
      // A simple validation is to check if it would parse if it *weren't* a keyword.
      // This is tricky because `syn::parse_str` would fail for actual keywords.
      // Let's rely on `syn::Ident::new_raw` to do its job, but catch obvious non-ident chars.
      if cased_name_str.chars().any(|c| !c.is_alphanumeric() && c != '_') {
         if !( cased_name_str.starts_with('_') && cased_name_str.chars().skip(1).all(|c| c.is_alphanumeric() || c == '_') ) && cased_name_str != "_" {
            return Err(syn::Error::new(span, format!("Invalid characters in identifier string for raw creation: {}", cased_name_str)));
         }
      }
      Ok(syn::Ident::new_raw(cased_name_str, span))
    } else {
      // Not a keyword and source was not raw. Try to create a normal identifier.
      // syn::Ident::new would panic on keywords, but we've established it's not a keyword.
      // It will also panic on other invalid idents like "123" or "with space".
      // To provide a Result, we attempt to parse it.
      match syn::parse_str::<syn::Ident>(cased_name_str) {
        Ok(ident) => Ok(ident),
        Err(_e) => {
          // Construct a new error, because the error from parse_str might not have the right span or context.
          Err(syn::Error::new(span, format!("Invalid identifier string: '{}'", cased_name_str)))
        }
      }
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
  pub use private::ident_maybe_raw;
  #[ doc( inline ) ]
  pub use private::new_ident_from_cased_str;
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
