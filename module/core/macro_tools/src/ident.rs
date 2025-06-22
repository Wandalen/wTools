//!
//! Utilities for manipulating identifiers, including keyword handling.
//!

/// Define a private namespace for all its items.
mod private
{
  #[ allow( clippy::wildcard_imports ) ]
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

    let is_keyword = RUST_KEYWORDS.contains( &cased_name_str );

    if is_keyword
    {
      return Ok( syn::Ident::new_raw( cased_name_str, span ) );
    }

    match syn::parse_str::< syn::Ident >( cased_name_str )
    {
      Ok( ident ) => Ok( ident ),
      Err( _ ) =>
      {
        if source_had_raw_prefix
        {
          return Ok( syn::Ident::new_raw( cased_name_str, span ) );
        }
        Err( syn::Error::new( span, format!( "Invalid identifier string: '{}'", cased_name_str ) ) )
      }
    }
  }

  /// Creates a new `syn::Ident` from an existing one, converting it to the specified case.
  ///
  /// This function is a convenient wrapper around `new_ident_from_cased_str`.
  /// It handles extracting the string representation and span from the original `Ident`,
  /// and converting the case.
  pub fn cased_ident_from_ident( original: &syn::Ident, case: convert_case::Case ) -> Result< syn::Ident >
  {
    let original_str = original.to_string();
    let had_raw_prefix = original_str.starts_with("r#");
    let core_str = if had_raw_prefix { &original_str[2..] } else { &original_str };

    if kw::is(core_str) && !had_raw_prefix {
        return Ok(syn::Ident::new_raw(core_str, original.span()));
    }

    let cased_str = core_str.to_case(case);
    new_ident_from_cased_str(&cased_str, original.span(), had_raw_prefix)
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
  #[ doc( inline ) ]
  pub use private::cased_ident_from_ident;
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
