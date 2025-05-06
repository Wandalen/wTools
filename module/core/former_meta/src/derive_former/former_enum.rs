#![allow(clippy::wildcard_imports)] // Keep if present

use super::*;
use macro_tools::{ Result, quote::{ format_ident, quote }, syn };
use proc_macro2::TokenStream; // Corrected import for TokenStream
use macro_tools::generic_params::decompose; // Corrected path

// Declare new sibling modules
mod common_emitters;
mod unit_variant_handler;
mod tuple_zero_fields_handler;
mod struct_zero_fields_handler;
mod tuple_single_field_scalar;
mod tuple_single_field_subform;
mod tuple_multi_fields_scalar;
mod struct_single_field_scalar;
mod struct_single_field_subform;
mod struct_multi_fields_scalar;
mod struct_multi_fields_subform;

// Ensure EnumVariantHandlerContext and EnumVariantFieldInfo structs are defined
// or re-exported for use by submodules.
// These will remain in this file.
// qqq : Define EnumVariantFieldInfo struct
#[allow(dead_code)] // Suppress warnings about unused fields
pub(super) struct EnumVariantFieldInfo
{
  pub ident : syn::Ident,
  pub ty : syn::Type,
  pub attrs : FieldAttributes,
  pub is_constructor_arg : bool,
}

// qqq : Define EnumVariantHandlerContext struct
#[allow(dead_code)] // Suppress warnings about unused fields
pub(super) struct EnumVariantHandlerContext< 'a >
{
  pub ast : &'a syn::DeriveInput,
  pub variant : &'a syn::Variant,
  pub struct_attrs : &'a ItemAttributes,
  pub enum_name : &'a syn::Ident,
  pub vis : &'a syn::Visibility,
  pub generics : &'a syn::Generics,
  pub original_input : &'a TokenStream,
  pub variant_attrs : &'a FieldAttributes,
  pub variant_field_info : &'a [EnumVariantFieldInfo],
  pub merged_where_clause : Option< &'a syn::WhereClause >,
  pub methods : &'a mut Vec< TokenStream >,
  pub end_impls : &'a mut Vec< TokenStream >,
  pub standalone_constructors : &'a mut Vec< TokenStream >,
  pub has_debug : bool,
}


pub(super) fn former_for_enum
(
  ast : &syn::DeriveInput,
  data_enum : &syn::DataEnum,
  original_input : &TokenStream,
  has_debug : bool
) -> Result< TokenStream >
{
  let enum_name = &ast.ident;
  let vis = &ast.vis;
  let generics = &ast.generics;
  let struct_attrs = ItemAttributes::from_attrs( ast.attrs.iter() )?;
  // qqq : Ensure ItemAttributes and FieldAttributes are accessible/imported

  let mut methods = Vec::new();
  let mut end_impls = Vec::new();
  let mut standalone_constructors = Vec::new();
  let merged_where_clause = generics.where_clause.as_ref();

  for variant in &data_enum.variants
  {
    let variant_attrs = FieldAttributes::from_attrs( variant.attrs.iter() )?;
    let variant_field_info : Vec<EnumVariantFieldInfo> = match &variant.fields {
        // qqq : Logic to populate variant_field_info (from previous plan)
        syn::Fields::Named(f) => f.named.iter().map(|field| {
            let attrs = FieldAttributes::from_attrs(field.attrs.iter())?;
            let is_constructor_arg = attrs.arg_for_constructor.value(false);
            Ok(EnumVariantFieldInfo {
                ident: field.ident.clone().ok_or_else(|| syn::Error::new_spanned(field, "Named field requires an identifier"))?,
                ty: field.ty.clone(),
                attrs,
                is_constructor_arg,
            })
        }).collect::<Result<_>>()?,
        syn::Fields::Unnamed(f) => f.unnamed.iter().enumerate().map(|(index, field)| {
            let attrs = FieldAttributes::from_attrs(field.attrs.iter())?;
            let is_constructor_arg = attrs.arg_for_constructor.value(false);
            Ok(EnumVariantFieldInfo {
                ident: format_ident!("_{}", index),
                ty: field.ty.clone(),
                attrs,
                is_constructor_arg,
            })
        }).collect::<Result<_>>()?,
        syn::Fields::Unit => vec![],
    };

    let mut ctx = EnumVariantHandlerContext
    {
      ast,
      variant,
      struct_attrs : &struct_attrs,
      enum_name,
      vis,
      generics,
      original_input,
      variant_attrs : &variant_attrs,
      variant_field_info : &variant_field_info,
      merged_where_clause,
      methods : &mut methods,
      end_impls : &mut end_impls,
      standalone_constructors : &mut standalone_constructors,
      has_debug,
    };

    // Dispatch logic directly here
    match &ctx.variant.fields
    {
      syn::Fields::Unit => unit_variant_handler::handle( &mut ctx )?,
      syn::Fields::Unnamed( fields ) => match fields.unnamed.len()
      {
        0 => tuple_zero_fields_handler::handle( &mut ctx )?,
        1 =>
        {
          if ctx.variant_attrs.scalar.is_some() {
              tuple_single_field_scalar::handle( &mut ctx )?
          } else {
              tuple_single_field_subform::handle( &mut ctx )?
          }
        }
        _ =>
        {
          if ctx.variant_attrs.subform_scalar.is_some()
          {
            return Err( syn::Error::new_spanned( ctx.variant, "#[subform_scalar] cannot be used on tuple variants with multiple fields." ) );
          }
          tuple_multi_fields_scalar::handle( &mut ctx )?
        }
      },
      syn::Fields::Named( fields ) => match fields.named.len()
      {
        0 =>
        {
          if ctx.variant_attrs.subform_scalar.is_some()
          {
            return Err( syn::Error::new_spanned( ctx.variant, "#[subform_scalar] is not allowed on zero-field struct variants." ) );
          }
          if !ctx.variant_attrs.scalar.is_some()
          {
            return Err( syn::Error::new_spanned( ctx.variant, "Zero-field struct variants require `#[scalar]` attribute for direct construction." ) );
          }
          struct_zero_fields_handler::handle( &mut ctx )?
        }
        _len =>
        {
          if ctx.variant_attrs.scalar.is_some()
          {
            if fields.named.len() == 1
            {
              struct_single_field_scalar::handle( &mut ctx )?
            }
            else
            {
              struct_multi_fields_scalar::handle( &mut ctx )?
            }
          }
          else
          {
            if fields.named.len() == 1
            {
              struct_single_field_subform::handle( &mut ctx )?
            }
            else
            {
              struct_multi_fields_subform::handle( &mut ctx )?
            }
          }
        }
      }
    }
  } // End of loop

  let ( _enum_generics_with_defaults, enum_generics_impl, enum_generics_ty, _enum_generics_where_punctuated )
    = decompose( generics );

  let result = quote!
  {
      #[ automatically_derived ]
      impl< #enum_generics_impl > #enum_name< #enum_generics_ty >
      where
        #merged_where_clause
      {
          #( #methods )*
      }

      #( #standalone_constructors )*
      #( #end_impls )*
  };

  if has_debug
  {
    let about = format!( "derive : Former\nenum : {}", enum_name );
    diag::report_print( about, original_input, &result );
  }

  Ok( result )
}
