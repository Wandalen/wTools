/// Internal namespace.
mod internal
{

//   #[ allow( unused_imports ) ]
//   use quote::{ quote };
//   #[ allow( unused_imports ) ]
//   use crate::prelude::*;
//   // use crate::prelude::tree_print;
//
//   ///
//   /// 3 parts which are result of splitting with name.
//   ///
//
//   #[ derive( Debug ) ]
//   pub struct SplitsWithName
//   {
//     /// Code before name.
//     pub prefix : proc_macro2::TokenStream,
//     /// Name.
//     pub name : proc_macro2::TokenStream,
//     /// Code after name.
//     pub postfix : proc_macro2::TokenStream,
//   }
//
//   impl quote::ToTokens for SplitsWithName
//   {
//     fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
//     {
//       self.prefix.to_tokens( tokens );
//       self.name.to_tokens( tokens );
//       self.postfix.to_tokens( tokens );
//     }
//   }
//
//   ///
//   /// Trait to get name of an syntax element.
//   ///
//
//   pub trait SplitWithName
//   {
//     /// Split with name.
//     fn split_item_with_name( &self ) -> Option< SplitsWithName >;
//   }
//
//   impl SplitWithName for syn::Item
//   {
//
//     fn split_item_with_name( &self ) -> Option< SplitsWithName >
//     {
//
//       match self
//       {
//         // syn::Item::Const( item ) => item.name(),
//         // syn::Item::Enum( item ) => item.name(),
//         // syn::Item::ExternCrate( item ) => item.name(),
//         syn::Item::Fn( item ) => item.split_item_with_name(),
//         // {
//         //   let attrs = &item.attrs;
//         //   let prefix = quote!{ #( #attrs )* #item.vis };
//         //   let name = quote!{  };
//         //   let postfix = quote!{  };
//         //   Some( SplitsWithName { prefix, name, postfix } )
//         // }
//         // syn::Item::ForeignMod( item ) => item.name(),
//         // syn::Item::Impl( item ) => item.name(),
//         // syn::Item::Macro( item ) => item.name(),
//         // syn::Item::Macro2( item ) => item.name(),
//         // syn::Item::Mod( item ) => item.name(),
//         // syn::Item::Static( item ) => item.name(),
//         // syn::Item::Struct( item ) => item.name(),
//         // syn::Item::Trait( item ) => item.name(),
//         // syn::Item::TraitAlias( item ) => item.name(),
//         // syn::Item::Type( item ) => item.name(),
//         // syn::Item::Union( item ) => item.name(),
//         // syn::Item::Use( item ) => item.name(),
//         // syn::Item::Verbatim( item ) => item.name(),
//         _ => None,
//       }
//
//     }
//
//   }
//
// //   impl SplitWithName for syn::Path
// //   {
// //     fn split_item_with_name( &self ) -> Option< SplitsWithName >
// //     {
// //       let first = self.segments.first();
// //       if first.is_none()
// //       {
// //         return "".into()
// //       }
// //       let first = first.unwrap();
// //       first.ident.to_string()
// //     }
// //   }
// //
// //   impl SplitWithName for syn::ItemConst
// //   {
// //     fn split_item_with_name( &self ) -> Option< SplitsWithName >
// //     {
// //       self.ident.to_string()
// //     }
// //   }
// //
// //   impl SplitWithName for syn::ItemEnum
// //   {
// //     fn split_item_with_name( &self ) -> Option< SplitsWithName >
// //     {
// //       self.ident.to_string()
// //     }
// //   }
// //
// //   impl SplitWithName for syn::ItemExternCrate
// //   {
// //     fn split_item_with_name( &self ) -> Option< SplitsWithName >
// //     {
// //       self.ident.to_string()
// //     }
// //   }
//
//   impl SplitWithName for syn::ItemFn
//   {
//     fn split_item_with_name( &self ) -> Option< SplitsWithName >
//     {
//       let attrs = &self.attrs;
//       let vis = &self.vis;
//       let constness = &self.sig.constness;
//       let asyncness = &self.sig.asyncness;
//       let unsafety = &self.sig.unsafety;
//       let abi = &self.sig.abi;
//       let fn_token = &self.sig.fn_token;
//       let ident = &self.sig.ident;
//       let generics = &self.sig.generics;
//       // let paren_token = &self.sig.paren_token;
//       let inputs = &self.sig.inputs;
//       let variadic = &self.sig.variadic;
//       let output = &self.sig.output;
//       let block = &self.block;
//
//       let prefix = quote!
//       {
//         #( #attrs )*
//         #vis
//         #constness
//         #asyncness
//         #unsafety
//         #abi
//         #fn_token
//       };
//       let name = quote!{ #ident };
//       let args = if inputs.is_empty()
//       {
//         quote!
//         {
//           #variadic
//         }
//       }
//       else
//       {
//         quote!
//         {
//           #inputs,
//           #variadic
//         }
//       };
//       let postfix = quote!
//       {
//         #generics
//         (
//           #args
//         )
//         #output
//         #block
//       };
//       let result = Some( SplitsWithName { prefix, name, postfix } );
//       tree_print!( result.as_ref().unwrap() );
//       result
//     }
//   }
//
// //   // impl SplitWithName for syn::ItemForeignMod
// //   // {
// //   //   fn split_item_with_name( &self ) -> Option< SplitsWithName >
// //   //   {
// //   //     self.ident.to_string()
// //   //   }
// //   // }
// //
// //   impl SplitWithName for syn::ItemImpl
// //   {
// //     fn split_item_with_name( &self ) -> Option< SplitsWithName >
// //     {
// //       if self.trait_.is_none()
// //       {
// //         return "".into()
// //       }
// //       let t = self.trait_.as_ref().unwrap();
// //       t.1.name()
// //     }
// //   }
// //
// //   impl SplitWithName for syn::ItemMacro
// //   {
// //     fn split_item_with_name( &self ) -> Option< SplitsWithName >
// //     {
// //       if self.ident.is_none()
// //       {
// //         return "".to_string()
// //       }
// //       let ident = self.ident.as_ref().unwrap();
// //       ident.to_string()
// //     }
// //   }
// //
// //   impl SplitWithName for syn::ItemMacro2
// //   {
// //     fn split_item_with_name( &self ) -> Option< SplitsWithName >
// //     {
// //       self.ident.to_string()
// //     }
// //   }
// //
// //   impl SplitWithName for syn::ItemMod
// //   {
// //     fn split_item_with_name( &self ) -> Option< SplitsWithName >
// //     {
// //       self.ident.to_string()
// //     }
// //   }
// //
// //   impl SplitWithName for syn::ItemStatic
// //   {
// //     fn split_item_with_name( &self ) -> Option< SplitsWithName >
// //     {
// //       self.ident.to_string()
// //     }
// //   }
// //
// //   impl SplitWithName for syn::ItemStruct
// //   {
// //     fn split_item_with_name( &self ) -> Option< SplitsWithName >
// //     {
// //       self.ident.to_string()
// //     }
// //   }
// //
// //   impl SplitWithName for syn::ItemTrait
// //   {
// //     fn split_item_with_name( &self ) -> Option< SplitsWithName >
// //     {
// //       self.ident.to_string()
// //     }
// //   }
// //
// //   impl SplitWithName for syn::ItemTraitAlias
// //   {
// //     fn split_item_with_name( &self ) -> Option< SplitsWithName >
// //     {
// //       self.ident.to_string()
// //     }
// //   }
// //
// //   impl SplitWithName for syn::ItemType
// //   {
// //     fn split_item_with_name( &self ) -> Option< SplitsWithName >
// //     {
// //       self.ident.to_string()
// //     }
// //   }
// //
// //   impl SplitWithName for syn::ItemUnion
// //   {
// //     fn split_item_with_name( &self ) -> Option< SplitsWithName >
// //     {
// //       self.ident.to_string()
// //     }
// //   }
// //
// //   // impl SplitWithName for syn::ItemUse
// //   // {
// //   //   fn split_item_with_name( &self ) -> Option< SplitsWithName >
// //   //   {
// //   //     self.ident.to_string()
// //   //   }
// //   // }
// //
// //   // impl SplitWithName for syn::ItemVerbatim
// //   // {
// //   //   fn split_item_with_name( &self ) -> Option< SplitsWithName >
// //   //   {
// //   //     self.ident.to_string()
// //   //   }
// //   // }

}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  // use super::internal as i;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
  // pub use i::SplitWithName;
}
