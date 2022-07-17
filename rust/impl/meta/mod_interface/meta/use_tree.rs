/// Internal namespace.
pub( crate ) mod private
{
  use proc_macro_tools::prelude::*;

  #[ derive( Debug, PartialEq, Eq, Clone ) ]
  pub struct UseTree
  {
    pub leading_colon : Option< syn::token::Colon2 >,
    pub tree : syn::UseTree,
  }

  impl UseTree
  {

    /// Is adding prefix to the tree path required?
    pub fn to_add_prefix( &self ) -> bool
    {
      use syn::UseTree::*;
      if self.leading_colon.is_some()
      {
        return false;
      }
      match &self.tree
      {
        Path( e ) => e.ident.to_string() != "super" && e.ident.to_string() != "crate",
        Rename( e ) => e.ident.to_string() != "super" && e.ident.to_string() != "crate",
        _ => true,
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

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;

  pub use super::private::
  {
    UseTree,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
