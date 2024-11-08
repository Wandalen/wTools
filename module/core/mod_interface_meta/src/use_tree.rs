/// Define a private namespace for all its items.
mod private
{
  use macro_tools::prelude::*;
  // use macro_tools::syn::Result;
  // use macro_tools::err;

  #[ derive( Debug, PartialEq, Eq, Clone ) ]
  pub struct UseTree
  {
    pub leading_colon : Option< syn::token::PathSep >,
    pub tree : syn::UseTree,
    pub rename : Option< syn::Ident >,
    pub glob : bool,
    pub group : bool,
  }

  // pub struct SimplePath
  // {
  // }

  impl UseTree
  {

    /// Is adding prefix to the tree path required?
    /// Add `super::private::` to path unless it starts from `::` or `super` or `crate`.
    pub fn private_prefix_is_needed( &self ) -> bool
    {
      use syn::UseTree::*;

      // println!( "private_prefix_is_needed : {:?}", self );
      // println!( "private_prefix_is_needed : self.leading_colon : {:?}", self.leading_colon );

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
    pub fn pure_path( &self ) -> syn::Result< syn::punctuated::Punctuated< syn::Ident, Token![::] > >
    {
      use syn::UseTree::*;

      // let leading_colon = None;
      let mut path = syn::punctuated::Punctuated::< syn::Ident, Token![::] >::new();
      let use_tree = &mut &self.tree;

      loop
      {
        match &use_tree
        {
          Name( e ) =>
          {
            path.push( e.ident.clone() );
            break;
          },
          Path( e ) =>
          {
            path.push( e.ident.clone() );
            *use_tree = e.tree.as_ref();
          },
          Rename( e ) =>
          {
            path.push( e.ident.clone() );
            break;
          },
          Glob( _e ) =>
          {
            // return Err( syn_err!( "Complex glob uses like `use module1::*` are not supported." ) );
            break;
          },
          Group( _e ) =>
          {
            return Err( syn_err!( "Complex group uses like `use module1::{ module2, module3 }` are not supported." ) );
          },
        };
      }

      Ok( path )
    }

    /// Pure path without super.
    /// Get pure path, cutting off `as module2` from `use module1 as module2`.
    /// Strip first `super::` in `super::some::module`
    pub fn pure_without_super_path( &self ) -> syn::Result< syn::punctuated::Punctuated< syn::Ident, Token![::] > >
    {
      let path = self.pure_path()?;
      if path.len() < 1
      {
        return Ok( path );
      }
      if path[ 0 ].to_string() == "super"
      {
        // let mut path2 = syn::punctuated::Punctuated::< syn::Ident, Token![::] >::new();
        let path2 : syn::punctuated::Punctuated< syn::Ident, Token![::] > = path.into_iter().skip(1).collect();
        return Ok( path2 );
      }
      Ok( path )
    }

    /// Prefix path with __all__ if it's appropriate.
    pub fn prefixed_with_all( &self ) -> Self
    {

      // use syn::UseTree::*;
      if self.private_prefix_is_needed()
      {
        let mut clone = self.clone();
        let tree = parse_qt!{ __all__::#self };
        clone.tree = tree;
        clone
      }
      else
      {
        self.clone()
      }

    }

    /// Prefix path with `super::` if it's appropriate to avoid "re-export of crate public `child`" problem.
    pub fn prefixed_with_super_maybe( &self ) -> Self
    {

      // use syn::UseTree::*;
      if self.private_prefix_is_needed()
      {
        let mut clone = self.clone();
        let tree = parse_qt!{ super::#self };
        clone.tree = tree;
        clone
      }
      else
      {
        self.clone()
      }

    }

  }

  impl syn::parse::Parse for UseTree
  {
    fn parse( input : ParseStream< '_ > ) -> syn::Result< Self >
    {
      use syn::UseTree::*;
      let leading_colon = input.parse()?;
      let tree = input.parse()?;

      let mut glob = false;
      let mut group = false;
      let mut rename = None;
      let use_tree = &mut &tree;
      loop
      {
        match &use_tree
        {
          Name( _e ) =>
          {
            break;
          },
          Path( e ) =>
          {
            *use_tree = e.tree.as_ref();
          },
          Rename( e ) =>
          {
            rename = Some( e.rename.clone() );
            break;
          },
          Glob( _e ) =>
          {
            glob = true;
            break;
          },
          Group( _e ) =>
          {
            group = true;
            break;
          },
        };
      }

      Ok( Self
      {
        leading_colon,
        tree,
        rename,
        glob,
        group,
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
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  pub use orphan::*;
}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use prelude::*;

  pub use private::
  {
    UseTree,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
