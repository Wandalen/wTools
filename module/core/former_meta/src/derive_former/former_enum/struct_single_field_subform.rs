// qqq : Implement logic for Struct { f1:T1 } with #[subform_scalar] or default

use super::*;
use macro_tools::{ Result, quote, syn };
use super::EnumVariantHandlerContext;
use proc_macro2::TokenStream; // Import TokenStream
use convert_case::{ Case, Casing }; // Import Case and Casing from convert_case

#[allow(dead_code)] // Suppress warning about unused function
pub( crate ) fn handle( ctx : &mut EnumVariantHandlerContext< '_ > ) -> Result< TokenStream >
{
  // This handler is specifically for Struct { f1: T1 } variants with #[subform_scalar] or default behavior.
  // The main dispatch should ensure this is only called for such variants.

  let variant_ident = &ctx.variant.ident;
  let enum_ident = &ctx.enum_name;
  let vis = &ctx.vis;

  // Decompose generics for use in signatures (impl_generics and ty_generics are needed from local decomposition)
  let ( _def_generics, impl_generics, ty_generics, _local_where_clause_option_unused ) = // Renamed to avoid confusion
  macro_tools::generic_params::decompose( ctx.generics );

  // Use merged_where_clause from the context for any top-level item's where clause (like standalone fns or VariantFormer struct)
  let top_level_where_clause = match ctx.merged_where_clause 
  { // Use ctx.merged_where_clause
    Some( clause ) => quote! { where #clause }, // Add `where` keyword if clause exists
    None => quote! {},
  };

  // Get the single field's info
  let field_info = ctx.variant_field_info.first().ok_or_else( 
  || 
  {
    syn::Error::new_spanned( ctx.variant, "Struct variant with subform behavior must have exactly one field for this handler." )
  })?;
  let field_name_original = &field_info.ident; // This is the original field name from the enum variant
  let field_ty = &field_info.ty;

  // Generate the name for the implicit variant former, make it generic if enum is generic
  let variant_former_name_str = format!( "{enum_ident}{variant_ident}Former" );
  let variant_former_ident = format_ident!( "{}", variant_former_name_str );
  let variant_former_name_generic = if ctx.generics.params.is_empty() 
  {
    quote! { #variant_former_ident }
  } 
  else 
  {
    quote! { #variant_former_ident< #ty_generics > }
  };

  // Correctly create method_ident for the accessor method, handling raw identifiers
  let method_ident = 
  {
    let name_str = variant_ident.to_string();
    // Raw identifier check (consistent with other handlers)
    if let Some( core_name ) = name_str.strip_prefix( "r#" ) 
    {
      let snake_core_name = core_name.to_case( Case::Snake );
      syn::Ident::new_raw( &snake_core_name, variant_ident.span() )
    } 
    else 
    {
      let snake_name = name_str.to_case( Case::Snake );
      let is_keyword = matches!( snake_name.as_str(), "as" | "async" | "await" | "break" | "const" | "continue" | "crate" | "dyn" | "else" | "enum" | "extern" | "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return" | "Self" | "self" | "static" | "struct" | "super" | "trait" | "true" | "type" | "unsafe" | "use" | "where" | "while" | "union" );
      if is_keyword 
      {
        syn::Ident::new_raw( &snake_name, variant_ident.span() )
      } 
      else 
      {
        syn::Ident::new( &snake_name, variant_ident.span() )
      }
    }
  };

  // Generate the static method: Enum::variant_name() -> VariantFormer<...>
  // Signature needs to be generic if the enum is generic.
  // The return type `Self` for the static method is not correct here, it should be the VariantFormer type.
  let generated_method = quote!
  {
    #[ inline( always ) ]
    pub fn #method_ident () -> #variant_former_name_generic // Return type is the implicit variant former
    {
      #variant_former_name_generic::default()
    }
  };

  // Generate standalone constructor if #[standalone_constructors] is present
  if ctx.struct_attrs.standalone_constructors.is_some()
  {
    let fn_signature_generics = if ctx.generics.params.is_empty() { quote!{} } else { quote!{ < #impl_generics > } };
    // Standalone constructor also returns the VariantFormer
    let generated_standalone = quote!
    {
      #[ inline( always ) ]
      #vis fn #method_ident #fn_signature_generics () -> #variant_former_name_generic
      #top_level_where_clause // Use the correctly formed where clause
      {
        #variant_former_name_generic::default()
      }
    };
    ctx.standalone_constructors.push(generated_standalone);
  }

  // Generate a MINIMAL definition for the implicit VariantFormer struct
  // This is NOT a full Former implementation, just enough to resolve type errors.
  let former_fields_def = quote! { pub #field_name_original : #field_ty };
  // let former_fields_init = quote! { #field_name_original : Default::default() }; // Unused, commented out

  let variant_former_def = quote!
  {
    #[derive(Debug, Default)] // Add Default for .default() call
    #vis struct #variant_former_ident< #impl_generics > // Make former struct generic
    #top_level_where_clause // Use the correctly formed where clause
    {
      #former_fields_def,
      // If T is a parameter, PhantomData might be needed if T is not used in fields
      // For MixedEnum { Complex { data: i32 } }, T is not used, so no PhantomData needed for this specific case.
      // If Complex was Complex { data: T }, then PhantomData might be needed if T is not Default.
    }
    // Basic impl to satisfy construction, not a full Former impl
    // impl< #impl_generics > #variant_former_name_generic // This would be for impl Former
    // #where_clause
    // {
    //   // pub fn new() -> Self { Self { #former_fields_init } } // Example constructor
    // }
  };
  ctx.end_impls.push(variant_former_def); // Add to end_impls to be emitted at top level

  Ok( generated_method ) // Return only the static method for the main impl block
}