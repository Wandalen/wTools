use super::*;
use macro_tools::{ Result, quote::quote };

pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = &ctx.variant.ident;
  let enum_name = ctx.enum_name;
  let vis = ctx.vis;

  let result = quote!
  {
    #[ inline( always ) ]
    #vis fn #variant_name() -> #enum_name
    {
      #enum_name::#variant_name()
    }
  };

  Ok( result )
}
