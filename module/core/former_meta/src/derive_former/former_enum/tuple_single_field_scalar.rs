use super::*;
use macro_tools::{ Result, quote::quote, ident::cased_ident_from_ident, generic_params::GenericsRef };
use convert_case::Case;

pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = &ctx.variant.ident;
  let method_name = cased_ident_from_ident(variant_name, Case::Snake);
  let enum_name = ctx.enum_name;
  let vis = ctx.vis;
  let field_type = &ctx.variant_field_info[0].ty;

  let generics_ref = GenericsRef::new( ctx.generics );
  let ty_generics = generics_ref.ty_generics_tokens_if_any();

  // Rule 1d: #[scalar] on single-field tuple variants generates scalar constructor
  let result = quote!
  {
    #[ inline( always ) ]
    #vis fn #method_name ( _0 : impl Into< #field_type > ) -> #enum_name #ty_generics
    {
      #enum_name #ty_generics :: #variant_name( _0.into() )
    }
  };

  Ok( result )
}
