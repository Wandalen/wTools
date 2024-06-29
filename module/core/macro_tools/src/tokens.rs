//!
//! Attributes analyzys and manipulation.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use core::fmt;

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

  impl core::fmt::Display for Tokens
  {
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      write!( f, "{}", self.inner.to_string() )
    }
  }

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
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  pub use super::super::tokens;
  // pub use super::protected as tokens;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    Tokens,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}

