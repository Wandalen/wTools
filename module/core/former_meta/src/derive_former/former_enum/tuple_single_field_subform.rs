use super::*;
use macro_tools::{ Result, quote::quote };

pub fn handle( _ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = & _ctx.variant.ident;
  let _enum_name = _ctx.enum_name;
  let vis = _ctx.vis;
  let field_type = & _ctx.variant_field_info[ 0 ].ty;

  let result = quote!
  {
    #[ inline( always ) ]
    #vis fn #variant_name() -> < #field_type as former::Former >::Former
    {
      < #field_type as former::Former >::former()
    }
  };

  Ok( result )
}
