//!
//! Macro helpers around derive macro and structure [`syn::DeriveInput`].
//!

/// Internal namespace.
pub( crate ) mod private
{
  use super::super::*;
  use syn::punctuated::Punctuated;

//   struct Wrap< T >( pub T );
//   // impl quote::ToTokens for Wrap< syn::Data >
//   // {
//   //   fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
//   //   {
//   //     match self.0
//   //     {
//   //       syn::Data::Struct( ref data_struct ) =>
//   //       {
//   //         qt! { #data_struct }.to_tokens( tokens );
//   //       },
//   //       syn::Data::Enum( ref data_enum ) =>
//   //       {
//   //         qt! { #data_enum }.to_tokens( tokens );
//   //       },
//   //       syn::Data::Union( ref data_union ) =>
//   //       {
//   //         qt! { #data_union }.to_tokens( tokens );
//   //       },
//   //     }
//   //   }
//   // }
// impl quote::ToTokens for Wrap<syn::Data> {
//     fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//         match &self.0 {
//             syn::Data::Struct(data_struct) => {
//                 // Manually construct the representation for structs
//                 let fields_tokens = data_struct.fields.iter().map(|field| {
//                     quote::quote!(#field)
//                 });
//                 let ident = &data_struct.ident;
//                 tokens.extend(quote::quote! {
//                     struct #ident {
//                         #(#fields_tokens),*
//                     }
//                 });
//             },
//             syn::Data::Enum(data_enum) => {
//                 // Manually construct the representation for enums
//                 let variants_tokens = data_enum.variants.iter().map(|variant| {
//                     quote::quote!(#variant)
//                 });
//                 tokens.extend(quote::quote! {
//                     enum #data_enum.ident {
//                         #(#variants_tokens),*
//                     }
//                 });
//             },
//             syn::Data::Union(data_union) => {
//                 // Manually construct the representation for unions
//                 let fields_tokens = data_union.fields.named.iter().map(|field| {
//                     quote::quote!(#field)
//                 });
//                 tokens.extend(quote::quote! {
//                     union #data_union.ident {
//                         #(#fields_tokens),*
//                     }
//                 });
//             },
//         }
//     }
// }

  /// # Example
  ///
  /// ```rust, ignore
  /// let ast = match syn::parse::< syn::DeriveInput >( input )
  /// {
  ///   Ok( syntax_tree ) => syntax_tree,
  ///   Err( err ) => return Err( err ),
  /// };
  /// let fields = derive.data_named_fields( &ast );
  /// ```

  pub fn data_named_fields< 'a >( ast : &'a syn::DeriveInput ) -> crate::Result< &'a Punctuated< syn::Field, syn::token::Comma > >
  {

    let fields = match ast.data
    {
      syn::Data::Struct( ref data_struct ) => match data_struct.fields
      {
        syn::Fields::Named( ref fields_named ) =>
        {
          &fields_named.named
        },
        _ => return Err( syn_err!( ast, "Unknown format of data, expected syn::Fields::Named( ref fields_named )\n  {}", qt!{ #ast } ) ),
      },
      _ => return Err( syn_err!( ast, "Unknown format of data, expected syn::Data::Struct( ref data_struct )\n  {}", qt!{ #ast } ) ),
    };

    Ok( fields )
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

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    data_named_fields,
  };

}

/// Parented namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::protected as derive;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };

}
