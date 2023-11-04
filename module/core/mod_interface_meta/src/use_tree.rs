/// Internal namespace.
pub( crate ) mod private
{
  use macro_tools::prelude::*;
  use macro_tools::Result;

  #[ derive( Debug, PartialEq, Eq, Clone ) ]
  pub struct UseTree
  {
    pub leading_colon : Option< syn::token::Colon2 >,
    pub tree : syn::UseTree,
  }

  impl UseTree
  {

    /// Is adding prefix to the tree path required?
    /// Add `super::private::` to path unless it starts from `::` or `super` or `crate`.
    pub fn to_add_prefix( &self ) -> bool
    {
      use syn::UseTree::*;

      // println!( "to_add_prefix : {:?}", self );
      // println!( "to_add_prefix : self.leading_colon : {:?}", self.leading_colon );

      if self.leading_colon.is_some()
      {
        return false;
      }
      match &self.tree
      {
        Path( e ) => e.ident != "super" && e.ident != "crate",
        Rename( e ) => e.ident != "super" && e.ident != "crate",
        _ => true,
      }
    }

    /// Get pure path, cutting off `as module2` from `use module1 as module2`.
    pub fn pure_path( &self ) -> syn::punctuated::Punctuated< syn::Ident, Token![::] >
    {
      use syn::UseTree::*;

      // let leading_colon = None;
      match &self.tree
      {
        Name( e ) =>
        {
          let mut path = syn::punctuated::Punctuated::< syn::Ident, Token![::] >::new();
          path.push( e.ident.clone() );
          path
        },
        // Path( e ) => e.ident != "super" && e.ident != "crate",
        // Rename( e ) => e.ident != "super" && e.ident != "crate",
        _ => unimplemented!(),
      }
    }

    /// Adjusted path.
    /// Add `super::private::` to path unless it starts from `::` or `super` or `crate`.
    pub fn adjsuted_path( &self ) -> syn::punctuated::Punctuated< syn::Ident, Token![::] >
    {
      // use syn::UseTree::*;
      let pure_path = self.pure_path();
      if self.to_add_prefix()
      {
        parse_qt!{ super::private::#pure_path }
      }
      else
      {
        pure_path
      }
    }

  }

  impl syn::parse::Parse for UseTree
  {
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      Ok( Self
      {
        leading_colon : input.parse()?,
        tree : input.parse()?,
      })
    }
  }

  impl quote::ToTokens for UseTree
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      self.leading_colon.to_tokens( tokens );
      self.tree.to_tokens( tokens );
    }
  }

}

#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

/// Parented namespace of the module.
pub mod orphan
{
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;

  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    UseTree,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
