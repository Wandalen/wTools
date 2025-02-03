//!
//! Attributes analyzys and manipulation.
//!

/// Define a private namespace for all its items.
mod private
{
  #[ allow( clippy::wildcard_imports ) ]
  use crate::*;

  /// Represents an equation parsed from a procedural macro input.
  ///
  /// This struct models an equation consisting of a left-hand side, an operator,
  /// and a right-hand side. The `Equation` is typically constructed during the
  /// parsing process of macro input, where the `left` and `op` fields are expected
  /// to be syntactically represented by `syn::Path` and `syn::BinOp` respectively,
  /// indicating the variable and operation involved. The `right` field is a
  /// `proc_macro2::TokenStream`, which can represent more complex expressions
  /// including, but not limited to, literals, function calls, or further operations.
  ///
  /// # Fields
  /// - `left`: The left-hand side of the equation, represented as a path.
  ///   This could be a variable or a more complex path in the code being
  ///   processed by the macro.
  ///
  /// - `op`: The binary operator used in the equation, such as addition,
  ///   subtraction, multiplication, etc.
  ///
  /// - `right`: The right-hand side of the equation. Given the potential
  ///   complexity of expressions on this side, it is represented as a
  ///   `proc_macro2::TokenStream` to accommodate any valid Rust expression.
  ///
  /// # Examples
  ///
  /// Parsing an equation from macro input:
  ///
  /// ```rust
  /// use macro_tools::equation;
  /// let got : equation::Equation = syn::parse_quote!( default = 31 );
  /// macro_tools::tree_print!( got );
  /// assert_eq!( macro_tools::code_to_str!( got ), "default = 31".to_string() );
  /// ```
  #[ derive( Debug ) ]
  pub struct Equation
  {
    /// The LHS of the equation, represented by a syntactic path.
    pub left : syn::Path,
    // /// The binary operator (e.g., +, -, *, /) of the equation.
    // pub op : syn::BinOp,
    /// Equality token.
    pub op : syn::Token![ = ],
    /// The RHS of the equation, capable of holding complex expressions.
    pub right : proc_macro2::TokenStream,
  }

  impl syn::parse::Parse for Equation
  {
    fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
    {
      let left : syn::Path = input.parse()?;
      let op : syn::Token![ = ] = input.parse()?;
      let right : proc_macro2::TokenStream = input.parse()?;
      Ok( Equation { left, op, right } )
    }
  }

  impl quote::ToTokens for Equation
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      self.left.to_tokens( tokens );
      self.op.to_tokens( tokens );
      self.right.to_tokens( tokens );
    }
  }

  // impl core::fmt::Display for Equation
  // {
  //   fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  //   {
  //     write!( f, "{}", self.left.to_string() );
  //     write!( f, "{}", self.op.to_string() );
  //     write!( f, "{}", self.right.to_string() )
  //   }
  // }

  ///
  /// For attribute like `#[former( default = 31 ) ]` return key `default` and value `31`,
  /// as well as `syn::Meta` as the last element of result tuple.
  ///
  /// ### Basic use-case.
  ///
  /// ```rust
  /// use macro_tools::equation;
  /// let attr : syn::Attribute = syn::parse_quote!( #[ former( default = 31 ) ] );
  /// // tree_print!( attr );
  /// let got = equation::from_meta( &attr ).unwrap();
  /// assert_eq!( macro_tools::code_to_str!( got ), "default = 31".to_string() );
  /// ```
  /// # Errors
  /// qqq: doc
  pub fn from_meta( attr : &syn::Attribute ) -> Result< Equation >
  {
    let meta = &attr.meta;
    match meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        let eq : Equation = syn::parse2( meta_list.tokens.clone() )?;
        Ok( eq )
      }
      _ => Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected syn::Meta::List( meta_list )" ) ),
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
  pub use private::
  {
    from_meta,
  };
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
  pub use super::super::equation;

  #[ doc( inline ) ]
  pub use prelude::*;
  #[ doc( inline ) ]
  pub use private::
  {
    Equation,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
