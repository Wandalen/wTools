use super::*;

use macro_tools::{ Result, quote::{ quote, format_ident }, ident::cased_ident_from_ident, generic_params::GenericsRef };
use former_types::forming::FormerBegin;
use convert_case::Case;

pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = &ctx.variant.ident;
  let method_name = cased_ident_from_ident(variant_name, Case::Snake);
  let enum_name = ctx.enum_name;
  let vis = ctx.vis;
  let field_type = &ctx.variant_field_info[0].ty;

  let generics_ref = GenericsRef::new(ctx.generics);
  let ( impl_generics, ty_generics, where_clause ) = ctx.generics.split_for_impl();
  let enum_type_path = if ctx.generics.type_params().next().is_some() {
    let ty_generics_tokens = generics_ref.ty_generics_tokens_if_any();
    quote! { #enum_name :: #ty_generics_tokens }
  } else {
    quote! { #enum_name }
  };

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

  // Construct the FormerDefinition type for the field_type
  let field_type_path = if let syn::Type::Path(type_path) = field_type {
      type_path
  } else {
      return Err(syn::Error::new_spanned(field_type, "Field type must be a path to derive Former"));
  };

  let field_type_base_ident = &field_type_path.path.segments.last().unwrap().ident;
  let field_type_generics = &field_type_path.path.segments.last().unwrap().arguments;
  let field_former_definition_type = format_ident!("{}{}Definition", field_type_base_ident, "Former");


  // Generate a custom definition types for the enum result
  let enum_end_definition_types = format_ident!("{}{}EndDefinitionTypes", enum_name, variant_name_string);
  
  let end_definition_types = quote!
  {
    #[derive(Default, Debug)]
    pub struct #enum_end_definition_types #impl_generics
    #where_clause
    {}
    
    impl #impl_generics former_types::FormerDefinitionTypes for #enum_end_definition_types #ty_generics
    #where_clause
    {
      type Storage = < #field_former_definition_type as former_types::definition::FormerDefinition >::Storage;
      type Context = < #field_former_definition_type as former_types::definition::FormerDefinition >::Context;
      type Formed = #enum_name #ty_generics;
    }
  };

  // Generate the FormingEnd implementation
  let end_impl = quote!
  {
    impl #impl_generics former_types::forming::FormingEnd<
      #enum_end_definition_types #ty_generics
    > for #end_struct_name #ty_generics
    #where_clause
    {
      #[ inline( always ) ]
      fn call(
        &self,
        sub_storage: < #field_former_definition_type as former_types::definition::FormerDefinition >::Storage,
        _context: Option< < #field_former_definition_type as former_types::definition::FormerDefinition >::Context >,
      ) -> #enum_name #ty_generics
      {
        let inner = former_types::storage::StoragePreform::preform( sub_storage );
        #enum_name::#variant_name( inner )
      }
    }
  };

  // Push the End struct and its implementation to the appropriate collections
  ctx.end_impls.push( end_definition_types );
  ctx.end_impls.push( end_struct );
  ctx.end_impls.push( end_impl );

  // Rule 3d.i: When the field type implements Former, return its former
  // and create the infrastructure to convert the formed inner type to the enum variant
  let method = if ctx.variant_attrs.subform_scalar.is_some() {
    // Rule 2d: #[subform_scalar] means configured former with custom End
    quote!
    {
      #[ inline( always ) ]
      #vis fn #method_name() -> < #field_type as former_types::definition::EntityToFormer< #field_former_definition_type #field_type_generics > >::Former
      {
        < #field_type as former_types::definition::EntityToFormer< #field_former_definition_type #field_type_generics > >::Former::former_begin( None, None, #end_struct_name::default() )
      }
    }
  } else {
    // Rule 3d: Default behavior - return a configured former with custom End
    quote!
    {
      #[ inline( always ) ]
      #vis fn #method_name() -> < #field_type as former_types::definition::EntityToFormer< #field_former_definition_type #field_type_generics > >::Former
      {
        < #field_type as former_types::definition::EntityToFormer< #field_former_definition_type #field_type_generics > >::Former::former_begin( None, None, #end_struct_name::default() )
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
      #vis fn #standalone_name() -> < #field_type as former_types::definition::EntityToFormer< #field_former_definition_type #field_type_generics > >::Former
      {
        < #field_type as former_types::definition::EntityToFormer< #field_former_definition_type #field_type_generics > >::Former::former_begin( None, None, former_types::forming::ReturnPreformed :: default() )
      }
    };

    ctx.methods.push( standalone_method );
  }

  Ok( method )
}
