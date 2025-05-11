use super::*;
use macro_tools::
{
  Result,
  diag, // For diag::return_syn_err!
  generic_params::GenericsRef, // For enhanced generics handling
  ident, // For proposed ident::new_ident_from_cased_str
  tokens::qt, // For qt! macro, if preferred over quote::quote!
  syn,
  quote::quote_spanned, // Keep for specific span control if needed, or replace with qt!
};
use super::EnumVariantHandlerContext;
use convert_case::{ Case, Casing }; // Keep for Case::Snake
use proc_macro2::TokenStream;

pub( crate ) fn handle( ctx : &mut EnumVariantHandlerContext< '_ > ) -> Result< TokenStream >
{
  // Handle #[subform_scalar] attribute error
  // Assumes ctx.variant_attrs.subform_scalar is an Option<(AttributeValue, Span)> or similar
  // For now, using ctx.variant.span() as a placeholder if specific attribute span isn't easily available.
  // This part depends on how FieldAttributes is structured and if it stores spans for attributes.
  // If `ctx.variant_attrs.subform_scalar` is simply an `Option<bool>` or `Option<SomeMarkerStruct>`,
  // we might need to iterate attributes here to find the span, or use a broader span.
  // For this refactoring, we'll assume `FieldAttributes` can provide a span for `subform_scalar` if present.
  // If not, `ctx.variant.span()` is a fallback.
  if let Some( attr_property ) = &ctx.variant_attrs.subform_scalar // Assuming FieldAttributes stores it as Option<AttributeProperty>
  {
    // If AttributeProperty has a span() method or field:
    // return diag::return_syn_err!( attr_property.span(), "Attribute `subform_scalar` is not applicable to unit variants" );
    // Otherwise, using variant span as a fallback:
    return diag::return_syn_err!( ctx.variant.span(), "Attribute `subform_scalar` is not applicable to unit variants" );
  }

  let variant_ident = &ctx.variant.ident;
  let enum_name = &ctx.enum_name; // This is syn::Ident
  let vis = &ctx.vis;

  // Generate method_ident (for static method and standalone constructor)
  let variant_ident_str = variant_ident.to_string();
  let is_raw_prefix = variant_ident_str.starts_with( "r#" );
  let core_name_str = if is_raw_prefix { &variant_ident_str[ 2.. ] } else { &variant_ident_str };
  let snake_case_name = core_name_str.to_case( Case::Snake );

  // Use the proposed (conceptual) macro_tools utility
  // This will fail to compile until Increment 6 implements this utility.
  let method_ident = ident::new_ident_from_cased_str(
      &snake_case_name,
      variant_ident.span(),
      is_raw_prefix
  )?;

  // Prepare generics using the proposed (conceptual) GenericsRef enhancements
  // These will also fail to compile until Increment 6.
  let generics_ref = GenericsRef::new_borrowed( &ctx.generics );
  let fn_signature_generics = generics_ref.impl_generics_tokens_if_any()?;
  let return_type_generics = generics_ref.ty_generics_tokens_if_any()?;
  let enum_path_for_construction = generics_ref.type_path_tokens_if_any( enum_name )?;
  let where_clause_tokens = generics_ref.where_clause_tokens_if_any()?;

  // Generate the static constructor method on the enum itself
  let generated_method = qt!
  {
    #[ inline( always ) ]
    pub fn #method_ident () -> Self
    {
      Self::#variant_ident
    }
  };

  // Generate standalone constructor if #[standalone_constructors] is present
  if ctx.struct_attrs.standalone_constructors.is_some()
  {
    let generated_standalone = qt!
    {
      #[ inline( always ) ]
      #vis fn #method_ident #fn_signature_generics () -> #enum_name #return_type_generics
      #where_clause_tokens
      {
        #enum_path_for_construction :: #variant_ident
      }
    };
    ctx.standalone_constructors.push( generated_standalone );
  }

  Ok( generated_method )
}