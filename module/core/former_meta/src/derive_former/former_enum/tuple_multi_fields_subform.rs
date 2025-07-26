use super::*;
use macro_tools::{Result, quote::quote};

#[allow(dead_code)]
pub fn handle(ctx: &mut EnumVariantHandlerContext<'_>) -> Result<proc_macro2::TokenStream> {
  let variant_name = &ctx.variant.ident;
  let vis = ctx.vis;
  let fields = &ctx.variant_field_info;

  let field_types = fields.iter().map(|f| &f.ty);
  let _field_names = fields.iter().map(|f| &f.ident);

  let field_types_clone_1 = field_types.clone();
  let field_types_clone_2 = field_types.clone();

  let result = quote! {
    #[ inline( always ) ]
    #vis fn #variant_name() -> ( #( < #field_types_clone_1 as former::Former >::Former ),* )
    {
      // This is a placeholder. The actual implementation will generate an implicit former.
      // For now, return a tuple of default formers.
      ( #( < #field_types_clone_2 as former::Former >::former() ),* )
    }
  };

  Ok(result)
}
