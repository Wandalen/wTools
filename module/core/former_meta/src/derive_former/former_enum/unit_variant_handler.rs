use super::*;
use macro_tools::
{
  Result,
  diag,
  generic_params::GenericsRef,
  ident,
  qt,
  syn,
};
use super::EnumVariantHandlerContext;
use convert_case::Case;
use proc_macro2::TokenStream;

pub( crate ) fn handle( ctx : &mut EnumVariantHandlerContext< '_ > ) -> Result< TokenStream >
{
  if let Some( attr ) = &ctx.variant_attrs.subform_scalar
  {
    return diag::return_syn_err!( attr.name.span(), "TEST ERROR: #[subform_scalar] cannot be used on unit variants. V3" );
  }

  let variant_ident = &ctx.variant.ident;
  let enum_name = &ctx.enum_name;
  let vis = &ctx.vis;

  let method_ident = ident::cased_ident_from_ident( variant_ident, Case::Snake );

  let generics_ref = GenericsRef::new( &ctx.generics );
  let fn_signature_generics = generics_ref.impl_generics_tokens_if_any();
  let return_type_generics = generics_ref.ty_generics_tokens_if_any();
  let enum_path_for_construction = generics_ref.type_path_tokens_if_any( enum_name );
  let where_clause_tokens = generics_ref.where_clause_tokens_if_any();

  let generated_method = qt!
  {
    #[ inline( always ) ]
    pub fn #method_ident () -> Self
    {
      Self::#variant_ident
    }
  };

  if ctx.struct_attrs.standalone_constructors.is_some()
  {
    let generated_standalone = qt!
    {
      #[ inline( always ) ]
      #vis fn #method_ident #fn_signature_generics () -> #enum_name #return_type_generics
      #where_clause_tokens
      {
        #enum_path_for_construction :: #variant_ident
      }
    };
    ctx.standalone_constructors.push( generated_standalone );
  }

  Ok( generated_method )
}