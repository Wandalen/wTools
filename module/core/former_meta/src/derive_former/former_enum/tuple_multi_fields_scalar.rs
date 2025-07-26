use super::*;
use macro_tools::{ Result, quote::quote };

pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = &ctx.variant.ident;
  let enum_name = ctx.enum_name;
  let vis = ctx.vis;
  let fields = &ctx.variant_field_info;

  let field_types = fields.iter().map( | f | &f.ty );
  let field_names : Vec<_> = fields.iter().map( | f | &f.ident ).collect();

  let result = quote!
  {
    #[ inline( always ) ]
    #vis fn #variant_name( #( #field_names : impl Into< #field_types > ),* ) -> #enum_name
    {
      #enum_name::#variant_name( #( #field_names.into() ),* )
    }
  };

  Ok( result )
}
