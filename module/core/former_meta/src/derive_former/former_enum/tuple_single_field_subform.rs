// qqq : Implement logic for Tuple(T1) with #[subform_scalar] or default

use super::*;
use macro_tools::{ Result, quote, syn };
use super::EnumVariantHandlerContext;
use proc_macro2::TokenStream; // Import TokenStream
use convert_case::{ Case, Casing }; // Import Case and Casing from convert_case

#[allow(dead_code)] // Suppress warning about unused function
pub( crate ) fn handle( ctx : &mut EnumVariantHandlerContext< '_ > ) -> Result< TokenStream >
{
  // This handler is specifically for Tuple(T1) variants with #[subform_scalar] or default behavior.
  // The main dispatch should ensure this is only called for such variants.

  let variant_ident = &ctx.variant.ident;
  let _enum_ident = &ctx.enum_name;
  let vis = &ctx.vis; // Get visibility

  // Get the single field's type
  let field = ctx.variant_field_info.get(0).ok_or_else(|| {
      syn::Error::new_spanned(ctx.variant, "Tuple variant with subform behavior must have exactly one field.")
  })?;
  let field_ty = &field.ty;

  // Check if the field type is a path (e.g., MyStruct) and derives Former
  // qqq : Need a way to check if a type derives Former. This might require
  // inspecting the type's definition or relying on a helper from macro_tools.
  // For now, assume the type is a path and generate the former name.
  // A proper check should be added here later.

  let inner_former_name = quote!{ #field_ty::Former }; // Assuming Former is derived and accessible

  // Convert variant identifier to snake_case for the method name using convert_case
  let method_ident_string = variant_ident.to_string().to_case( Case::Snake );
  let method_ident = syn::Ident::new( &method_ident_string, variant_ident.span() ); // Create new Ident with correct span

  // Generate the static method: Enum::variant_name() -> InnerFormer<...>
  let generated_method = quote!
  {
    #[ inline( always ) ]
    pub fn #method_ident() -> #inner_former_name // Return type is the inner former
    {
      #inner_former_name::default() // Assuming the inner former has a default constructor
      // qqq : Need to handle cases where the inner former doesn't have Default
    }
  };

  let mut generated_tokens = generated_method;

  // Generate standalone constructor if #[standalone_constructors] is present on the enum
  if ctx.struct_attrs.standalone_constructors.is_some()
  {
    let generated_standalone = quote!
    {
      #[ inline( always ) ]
      #vis fn #method_ident() -> #inner_former_name // Return type is the inner former
      {
        #inner_former_name::default() // Assuming the inner former has a default constructor
        // qqq : Need to handle cases where the inner former doesn't have Default
      }
    };
    generated_tokens.extend(generated_standalone);
  }

  Ok( generated_tokens )
}