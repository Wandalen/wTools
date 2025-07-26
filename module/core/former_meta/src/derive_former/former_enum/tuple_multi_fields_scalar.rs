use super::*;
use macro_tools::{Result, quote::quote};

pub fn handle(_ctx: &mut EnumVariantHandlerContext<'_>) -> Result<proc_macro2::TokenStream> {
  let variant_name = &_ctx.variant.ident;
  let enum_name = _ctx.enum_name;
  let vis = _ctx.vis;
  let fields = &_ctx.variant_field_info;

  let field_types = fields.iter().map(|f| &f.ty);
  let field_names = fields.iter().map(|f| &f.ident);

  let field_types_clone_1 = field_types.clone();
  let field_names_clone_1 = field_names.clone();
  let field_names_clone_2 = field_names.clone();

  let result = quote! {
    #[ inline( always ) ]
    #vis fn #variant_name( #( #field_names_clone_1 : impl Into< #field_types_clone_1 > ),* ) -> #enum_name
    {
      #enum_name::#variant_name( #( #field_names_clone_2.into() ),* )
    }
  };

  Ok(result)
}
