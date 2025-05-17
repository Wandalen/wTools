// qqq : Implement logic for Tuple(T1) with #[subform_scalar] or default

use super::*;
use macro_tools::{ Result, quote, syn };
use super::EnumVariantHandlerContext;
use proc_macro2::TokenStream; // Import TokenStream
use convert_case::{ Case, Casing }; // Import Case and Casing from convert_case

#[allow(dead_code)] // Suppress warning about unused function
pub( crate ) fn handle( ctx : &mut EnumVariantHandlerContext< '_ > ) -> Result< TokenStream >
{
  // This handler is specifically for Tuple(T1) variants with #[subform_scalar] or default behavior.
  // The main dispatch should ensure this is only called for such variants.

  let variant_ident = &ctx.variant.ident;
  let _enum_ident = &ctx.enum_name;
  let vis = &ctx.vis; // Get visibility

  // Get the single field's type
  let field = ctx.variant_field_info.get(0).ok_or_else(|| {
      syn::Error::new_spanned(ctx.variant, "Tuple variant with subform behavior must have exactly one field.")
  })?;
  let field_ty = &field.ty;

  let type_path_str = quote!{ #field_ty }.to_string().replace(" ", "");
  let is_phantom_data_field = type_path_str.starts_with("core::marker::PhantomData") || type_path_str.starts_with("std::marker::PhantomData");

  let method_ident_string = variant_ident.to_string().to_case( Case::Snake );
  let method_ident = syn::Ident::new( &method_ident_string, variant_ident.span() );

  let mut generated_tokens = TokenStream::new();

  if is_phantom_data_field {
      // If the field is PhantomData, generate a scalar-like constructor for the variant.
      // Enum::variant_name() -> Self { Self::VariantName(core::marker::PhantomData) }
      let variant_construction = quote! { Self::#variant_ident(core::marker::PhantomData) };
      let generated_method = quote!
      {
        #[ inline( always ) ]
        #vis fn #method_ident() -> Self
        {
          #variant_construction
        }
      };
      generated_tokens.extend(generated_method);

      if ctx.struct_attrs.standalone_constructors.is_some() {
          let ( impl_generics, ty_generics, where_clause ) = ctx.generics.split_for_impl(); // Renamed back to ty_generics
          let enum_name_ident = ctx.enum_name;
          let standalone_constructor_name = format_ident!( "{}_{}", enum_name_ident.to_string().to_case( Case::Snake ), method_ident );

          let generated_standalone = quote!
          {
            #[ inline( always ) ]
            #vis fn #standalone_constructor_name #impl_generics () -> #enum_name_ident #ty_generics #where_clause
            {
              #enum_name_ident :: #variant_ident ( core::marker::PhantomData )
            }
          };
          generated_tokens.extend(generated_standalone);
      }
  } else {
      // Original logic for non-PhantomData fields
      let inner_former_name = quote!{ #field_ty::Former };

      let generated_method = quote!
      {
        #[ inline( always ) ]
        #vis fn #method_ident() -> #inner_former_name
        {
          #inner_former_name::default()
        }
      };
      generated_tokens.extend(generated_method);

      if ctx.struct_attrs.standalone_constructors.is_some() {
          let ( impl_generics, _ty_generics, where_clause ) = ctx.generics.split_for_impl(); // Prefixed _ty_generics as it's not used in -> #inner_former_name
          let enum_name_ident = ctx.enum_name;
          // For standalone, the method name is typically just the snake_case variant name if not prefixed by enum
          // However, the original code used #method_ident for standalone too.
          // Let's make it consistent with the PhantomData case for naming.
          let standalone_constructor_name = format_ident!( "{}_{}", enum_name_ident.to_string().to_case( Case::Snake ), method_ident );

          let generated_standalone = quote!
          {
            #[ inline( always ) ]
            #vis fn #standalone_constructor_name #impl_generics () -> #inner_former_name #where_clause // Standalone returns InnerFormer
            {
              #inner_former_name::default()
            }
          };
          generated_tokens.extend(generated_standalone);
      }
  }

  Ok( generated_tokens )
}