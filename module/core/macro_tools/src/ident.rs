//!
//! Utilities for manipulating identifiers, including keyword handling.
//!

/// Define a private namespace for all its items.
mod private 
{

  use crate :: *; // Use crate's prelude/exposed items
  use convert_case ::Casing;
  use proc_macro2 ::Ident;
  // use syn ::spanned ::Spanned; // Needed for span

  /// Ensures keyword safety by applying raw identifier escaping when needed to prevent compilation errors.
  ///
  /// Preserves the span of the original identifier.
  /// Requires the `kw` feature.
  ///
  /// # Example
  /// ```rust
  /// use macro_tools :: { syn, format_ident, ident };
  ///
  /// let ident_normal = format_ident!( "my_var" );
  /// let ident_keyword = format_ident!( "fn" );
  ///
  /// let got_normal = ident ::ident_maybe_raw( &ident_normal );
  /// let got_keyword = ident ::ident_maybe_raw( &ident_keyword );
  ///
  /// assert_eq!( got_normal.to_string(), "my_var" );
  /// assert_eq!( got_keyword.to_string(), "r#fn" );
  /// ```
  #[ must_use ]
  pub fn ident_maybe_raw(ident: &syn ::Ident) -> Ident
  {
  let name = ident.to_string();
  if kw ::is(&name)
  {
   // Use r# prefix if the name is a keyword
   format_ident!("r#{}", name, span = ident.span())
 } else {
   // Otherwise, use the name directly (cloned)
   ident.clone()
 }
}

  /// Transforms identifier casing while preserving keyword safety to support code generation scenarios
  /// that require consistent naming conventions.
  ///
  /// # Arguments
  ///
  /// * `original` - The original `syn ::Ident` to convert.
  /// * `case` - The target `convert_case ::Case` to convert the identifier to.
  ///
  /// # Returns
  ///
  /// Maintains span information and raw identifier semantics to ensure generated code correctness.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use macro_tools :: { syn, format_ident };
  /// use convert_case ::Case;
  ///
  /// let ident_normal = format_ident!( "my_variable" );
  /// let ident_keyword = format_ident!( "r#fn" );
  ///
  /// // Convert to PascalCase
  /// let got_pascal = macro_tools ::ident ::cased_ident_from_ident( &ident_normal, Case ::Pascal );
  /// assert_eq!( got_pascal.to_string(), "MyVariable" );
  ///
  /// // Convert a raw identifier to SnakeCase
  /// let got_snake_raw = macro_tools ::ident ::cased_ident_from_ident( &ident_keyword, Case ::Snake );
  /// assert_eq!( got_snake_raw.to_string(), "r#fn" );
  ///
  /// // Convert a raw identifier: the raw prefix is preserved in the output.
  /// let ident_raw_struct = format_ident!( "r#struct" );
  /// let got_pascal_raw = macro_tools ::ident ::cased_ident_from_ident( &ident_raw_struct, Case ::Pascal );
  /// assert_eq!( got_pascal_raw.to_string(), "r#Struct" ); // Raw prefix is always preserved.
  /// ```
  #[ must_use ]
  pub fn cased_ident_from_ident(original: &syn ::Ident, case: convert_case ::Case) -> syn ::Ident
  {
  let original_str = original.to_string();
  let had_raw_prefix = original_str.starts_with("r#");
  let core_str = if had_raw_prefix { &original_str[2..] } else { &original_str };

  let cased_str = core_str.to_case(case);

  // If the cased form is not a valid identifier (e.g. kebab produces "my-var"),
  // fall back to the original identifier unchanged.
  let is_valid_ident =
  {
    let mut chars = cased_str.chars();
    match chars.next()
    {
      Some( c ) if c == '_' || c.is_alphabetic() => chars.all( | c | c == '_' || c.is_alphanumeric() ),
      _ => false,
    }
  };
  if !is_valid_ident
  {
    return original.clone();
  }

  if kw ::is( core_str ) || kw ::is( &cased_str )
  {
    syn ::Ident ::new_raw( &cased_str, original.span() )
  }
  else
  {
    syn ::Ident ::new( &cased_str, original.span() )
  }
}

  /// Creates an identifier from a string, validating identifier syntax and handling keyword escaping.
  ///
  /// If `raw` is `true` or the string is a Rust keyword, produces a raw identifier (`r#name`).
  ///
  /// # Errors
  ///
  /// Returns `Err` if the string is empty or contains characters invalid for a Rust identifier.
  pub fn new_ident_from_cased_str
  (
    s : &str,
    span : proc_macro2 ::Span,
    raw : bool,
  ) -> crate ::Result< syn ::Ident >
  {
    if s.is_empty()
    {
      return Err( syn ::Error ::new( span, "identifier string must not be empty" ) );
    }
    let valid =
    {
      let mut chars = s.chars();
      match chars.next()
      {
        Some( c ) if c == '_' || c.is_alphabetic() => chars.all( | c | c == '_' || c.is_alphanumeric() ),
        _ => false,
      }
    };
    if !valid
    {
      return Err( syn ::Error ::new( span, format!( "invalid identifier: {s:?}" ) ) );
    }
    if raw || kw ::is( s )
    {
      Ok( syn ::Ident ::new_raw( s, span ) )
    }
    else
    {
      Ok( syn ::Ident ::new( s, span ) )
    }
  }
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own :: *;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own 
{

  use super :: *;
  #[ doc( inline ) ]
  pub use orphan :: *;
  #[ doc( inline ) ]
  pub use private ::ident_maybe_raw;
  #[ doc( inline ) ]
  pub use private ::cased_ident_from_ident;
  #[ doc( inline ) ]
  pub use private ::new_ident_from_cased_str;
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan 
{

  use super :: *;
  #[ doc( inline ) ]
  pub use exposed :: *;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed 
{

  use super :: *;
  pub use super ::super ::ident; // Use the new module name

  #[ doc( inline ) ]
  pub use prelude :: *;
}

/// Prelude to use essentials: `use my_module ::prelude :: *`.
#[ allow( unused_imports ) ]
pub mod prelude 
{

  use super :: *;
}
