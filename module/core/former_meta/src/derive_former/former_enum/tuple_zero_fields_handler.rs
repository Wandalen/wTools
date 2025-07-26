use super::*;
use macro_tools::{ Result, quote::quote, ident::cased_ident_from_ident, syn_err };
use convert_case::Case;

pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = &ctx.variant.ident;
  let method_name = cased_ident_from_ident(variant_name, Case::Snake);
  let enum_name = ctx.enum_name;
  let vis = ctx.vis;

  // Rule 2b: #[subform_scalar] on zero-field tuple variants should cause a compile error
  if ctx.variant_attrs.subform_scalar.is_some() {
    return Err(syn_err!(
      ctx.variant,
      "#[subform_scalar] cannot be used on zero-field tuple variants."
    ));
  }

  // For zero-field tuple variants, Rules 1b and 3b both generate the same direct constructor
  let result = quote!
  {
    #[ inline( always ) ]
    #vis fn #method_name() -> #enum_name
    {
      #enum_name::#variant_name()
    }
  };

  Ok( result )
}
