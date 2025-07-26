use super::*;
use macro_tools::{ Result, quote::quote };

pub fn handle( _ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = & _ctx.variant.ident;
  let enum_name = _ctx.enum_name;
  let vis = _ctx.vis;

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
