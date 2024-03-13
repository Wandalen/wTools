//!
//! Tait to getn name of an Item.
//!

/// Internal namespace.
pub( crate ) mod private
{

  ///
  /// Trait to get name of an syntax element.
  ///

  pub trait Name
  {
    /// Get name.
    fn name( &self ) -> String;
  }

  impl Name for syn::Item
  {
    fn name( &self ) -> String
    {
      match self
      {
        syn::Item::Const( item ) => item.name(),
        syn::Item::Enum( item ) => item.name(),
        syn::Item::ExternCrate( item ) => item.name(),
        syn::Item::Fn( item ) => item.name(),
        // syn::Item::ForeignMod( item ) => item.name(),
        syn::Item::Impl( item ) => item.name(),
        syn::Item::Macro( item ) => item.name(),
        // syn::Item::Macro2( item ) => item.name(),
        syn::Item::Mod( item ) => item.name(),
        syn::Item::Static( item ) => item.name(),
        syn::Item::Struct( item ) => item.name(),
        syn::Item::Trait( item ) => item.name(),
        syn::Item::TraitAlias( item ) => item.name(),
        syn::Item::Type( item ) => item.name(),
        syn::Item::Union( item ) => item.name(),
        // syn::Item::Use( item ) => item.name(),
        // syn::Item::Verbatim( item ) => item.name(),
        _ => "".into(),
      }
    }
  }

  impl Name for syn::Path
  {
    fn name( &self ) -> String
    {
      let first = self.segments.first();
      if first.is_none()
      {
        return "".into()
      }
      let first = first.unwrap();
      first.ident.to_string()
    }
  }

  impl Name for syn::ItemConst
  {
    fn name( &self ) -> String
    {
      self.ident.to_string()
    }
  }

  impl Name for syn::ItemEnum
  {
    fn name( &self ) -> String
    {
      self.ident.to_string()
    }
  }

  impl Name for syn::ItemExternCrate
  {
    fn name( &self ) -> String
    {
      self.ident.to_string()
    }
  }

  impl Name for syn::ItemFn
  {
    fn name( &self ) -> String
    {
      self.sig.ident.to_string()
    }
  }

  // impl Name for syn::ItemForeignMod
  // {
  //   fn name( &self ) -> String
  //   {
  //     self.ident.to_string()
  //   }
  // }

  impl Name for syn::ItemImpl
  {
    fn name( &self ) -> String
    {
      if self.trait_.is_none()
      {
        return "".into()
      }
      let t = self.trait_.as_ref().unwrap();
      t.1.name()
    }
  }

  impl Name for syn::ItemMacro
  {
    fn name( &self ) -> String
    {
      if self.ident.is_none()
      {
        return "".to_string()
      }
      let ident = self.ident.as_ref().unwrap();
      ident.to_string()
    }
  }

  // impl Name for syn::ItemMacro2
  // {
  //   fn name( &self ) -> String
  //   {
  //     self.ident.to_string()
  //   }
  // }

  impl Name for syn::ItemMod
  {
    fn name( &self ) -> String
    {
      self.ident.to_string()
    }
  }

  impl Name for syn::ItemStatic
  {
    fn name( &self ) -> String
    {
      self.ident.to_string()
    }
  }

  impl Name for syn::ItemStruct
  {
    fn name( &self ) -> String
    {
      self.ident.to_string()
    }
  }

  impl Name for syn::ItemTrait
  {
    fn name( &self ) -> String
    {
      self.ident.to_string()
    }
  }

  impl Name for syn::ItemTraitAlias
  {
    fn name( &self ) -> String
    {
      self.ident.to_string()
    }
  }

  impl Name for syn::ItemType
  {
    fn name( &self ) -> String
    {
      self.ident.to_string()
    }
  }

  impl Name for syn::ItemUnion
  {
    fn name( &self ) -> String
    {
      self.ident.to_string()
    }
  }

  // impl Name for syn::ItemUse
  // {
  //   fn name( &self ) -> String
  //   {
  //     self.ident.to_string()
  //   }
  // }

  // impl Name for syn::ItemVerbatim
  // {
  //   fn name( &self ) -> String
  //   {
  //     self.ident.to_string()
  //   }
  // }

//
//     Const(ItemConst),
//     Enum(ItemEnum),
//     ExternCrate(ItemExternCrate),
//     Fn(ItemFn),
//     ForeignMod(ItemForeignMod),
//     Impl(ItemImpl),
//     Macro(ItemMacro),
//     Macro2(ItemMacro2),
//     Mod(ItemMod),
//     Static(ItemStatic),
//     Struct(ItemStruct),
//     Trait(ItemTrait),
//     TraitAlias(ItemTraitAlias),
//     Type(ItemType),
//     Union(ItemUnion),
//     Use(ItemUse),
//     Verbatim(TokenStream),
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
  pub use super::protected as name;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::Name;
}
