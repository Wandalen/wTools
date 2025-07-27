use super::*;
use macro_tools::{ Result, quote::{ quote, format_ident }, ident::cased_ident_from_ident, generic_params::GenericsRef };
use convert_case::Case;

pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = &ctx.variant.ident;
  let method_name = cased_ident_from_ident(variant_name, Case::Snake);
  let enum_name = ctx.enum_name;
  let vis = ctx.vis;
  let field_type = &ctx.variant_field_info[0].ty;
  
  let generics_ref = GenericsRef::new( ctx.generics );
  let impl_generics = generics_ref.impl_generics_tokens_if_any();
  let ty_generics = generics_ref.ty_generics_tokens_if_any();
  let where_clause = generics_ref.where_clause_tokens_if_any();
  
  // Generate the End struct name for this variant
  // Use the original variant name to avoid issues with raw identifiers
  let variant_name_string = variant_name.to_string();
  let end_struct_name = format_ident!("{}{}End", enum_name, variant_name_string);
  
  // Generate the End struct for this variant (for both Rule 2d and 3d)
  let end_struct = quote!
  {
    #[derive(Default, Debug)]
    pub struct #end_struct_name #impl_generics
    #where_clause
    {}
  };
  
  // Generate the FormingEnd implementation  
  // Looking at the manual test, we need to implement FormingEnd for the field type's definition types
  let field_definition_type = quote! { < #field_type as former::Former >::Definition };
  
  let end_impl = quote!
  {
    impl #impl_generics former::FormingEnd< 
      #field_definition_type 
    > for #end_struct_name #ty_generics
    #where_clause
    {
      #[ inline( always ) ]
      fn call(
        &self,
        sub_storage: < #field_definition_type as former::FormerDefinition >::Storage,
        _context: Option< < #field_definition_type as former::FormerDefinition >::Context >,
      ) -> #enum_name #ty_generics
      {
        let inner = former::StoragePreform::preform( sub_storage );
        #enum_name :: #variant_name ( inner )
      }
    }
  };
  
  // Push the End struct and its implementation to the appropriate collections
  ctx.end_impls.push( end_struct );
  ctx.end_impls.push( end_impl );
  
  // Rule 3d.i: When the field type implements Former, return its former
  // and create the infrastructure to convert the formed inner type to the enum variant
  let method = if ctx.variant_attrs.subform_scalar.is_some() {
    // Rule 2d: #[subform_scalar] means configured former with custom End
    quote!
    {
      #[ inline( always ) ]
      #vis fn #method_name() -> < #field_type as former::Former >::Former
      {
        < #field_type as former::Former >::Former::begin( None, None, #end_struct_name :: default() )
      }
    }
  } else {
    // Rule 3d: Default behavior - return a configured former with custom End
    quote!
    {
      #[ inline( always ) ]
      #vis fn #method_name() -> < #field_type as former::Former >::Former
      {
        < #field_type as former::Former >::Former::begin( None, None, #end_struct_name :: default() )
      }
    }
  };
  
  // Generate standalone constructor if requested (for both Rule 2d and 3d)
  if ctx.struct_attrs.standalone_constructors.value(false) {
    // Strip raw identifier prefix if present
    let method_name_str = method_name.to_string();
    let base_name = method_name_str.strip_prefix("r#").unwrap_or(&method_name_str);
    let standalone_name = format_ident!("{}_variant", base_name);
    
    // Add the standalone constructor as a static method on the enum
    let standalone_method = quote!
    {
      #[ inline( always ) ]
      #vis fn #standalone_name() -> < #field_type as former::Former >::Former
      {
        < #field_type as former::Former >::Former::begin( None, None, #end_struct_name :: default() )
      }
    };
    
    ctx.methods.push( standalone_method );
  }

  Ok( method )
}
