use super::*;
use macro_tools::{ Result, quote::quote, ident::cased_ident_from_ident };
use convert_case::Case;

pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = &ctx.variant.ident;
  let method_name = cased_ident_from_ident(variant_name, Case::Snake);
  let enum_name = ctx.enum_name;
  let vis = ctx.vis;
  let field_type = &ctx.variant_field_info[0].ty;

  let ( _impl_generics, ty_generics, where_clause ) = ctx.generics.split_for_impl();

  // Rule 1d: #[scalar] on single-field tuple variants generates scalar constructor
  let result = quote!
  {
    #[ inline( always ) ]
    #vis fn #method_name ( _0 : impl Into< #field_type > ) -> #enum_name #ty_generics
    #where_clause
    {
      #enum_name :: #variant_name( _0.into() )
    }
  };

  Ok( result )
}
