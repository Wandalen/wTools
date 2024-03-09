//!
//! Attributes analyzys and manipulation.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use super::super::*;
  use std::fmt;

  /// `Tokens` is a wrapper around `proc_macro2::TokenStream`.
  /// It is designed to facilitate the parsing and manipulation of token streams
  /// within procedural macros.
  ///
  /// # Examples
  ///
  /// Creating a new `Tokens` instance from a token stream :
  ///
  /// ```rust
  /// use macro_tools::exposed::*;
  ///
  /// let ts : proc_macro2::TokenStream = qt! { let x = 10; };
  /// let tokens = tokens::Tokens::new( ts );
  /// ```
  #[ derive( Default ) ]
  pub struct Tokens
  {
    /// `proc_macro2::TokenStream`
    pub inner : proc_macro2::TokenStream,
  }

  impl Tokens
  {
    /// Constructor from `proc_macro2::TokenStream`.
    pub fn new( inner : proc_macro2::TokenStream ) -> Self
    {
      Tokens { inner }
    }
  }

  impl syn::parse::Parse for Tokens
  {
    fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
    {
      let inner : proc_macro2::TokenStream = input.parse()?;
      Ok( Tokens::new( inner ) )
    }
  }

  impl quote::ToTokens for Tokens
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      self.inner.to_tokens( tokens );
    }
  }

  impl fmt::Debug for Tokens
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "{}", self.inner.to_string() )
    }
  }

  impl std::fmt::Display for Tokens
  {
    fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      write!( f, "{}", self.inner.to_string() )
    }
  }

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
  /// use macro_tools::exposed::*;
  /// let got : tokens::Equation = syn::parse_quote!( default = 31 );
  /// tree_print!( got );
  /// assert_eq!( code_to_str!( got ), "default = 31".to_string() );
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

  // impl std::fmt::Display for Equation
  // {
  //   fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
  //   {
  //     write!( f, "{}", self.left.to_string() );
  //     write!( f, "{}", self.op.to_string() );
  //     write!( f, "{}", self.right.to_string() )
  //   }
  // }

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
  pub use super::protected as tokens;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    Tokens,
    Equation,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}

