use super::*;
use macro_tools::{ Result, quote::quote };

pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = &ctx.variant.ident;
  let vis = ctx.vis;
  let fields = &ctx.variant_field_info;

  let field_types = fields.iter().map( | f | &f.ty );

  let result = quote!
  {
    #[ inline( always ) ]
    #vis fn #variant_name() -> ( #( < #field_types as former::Former >::Former ),* )
    {
      ( #( < #field_types as former::Former >::former() ),* )
    }
  };

  Ok( result )
}