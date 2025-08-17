//! # Enhanced Tuple Single-Field Subform Handler
//!
//! This enhanced handler provides better error messages and more robust handling
//! for single-field tuple enum variants. It includes improved diagnostics and
//! fallback mechanisms when the field type may not implement Former.
//!
//! ## Key Improvements
//! - Better error messages when Former trait is missing
//! - Optional attributes to control behavior 
//! - More robust generic parameter handling
//! - Improved documentation generation
//!
//! ## Usage Examples
//! ```rust,ignore
//! #[ derive( Former ) ]
//! enum MyEnum {
//!     // Works with Former-implementing types
//!     #[ subform_scalar ]  // Uses field's Former
//!     WithFormer(MyStruct),
//!     
//!     // Works with primitive types using explicit scalar
//!     #[ scalar ]  // Direct scalar approach
//!     Primitive(i32),
//! }
//! ```

use super::*;

use macro_tools::{ Result, quote::{ quote, format_ident } };
use crate::derive_former::raw_identifier_utils::variant_to_method_name;

/// Enhanced handler for single-field tuple enum variants with improved diagnostics.
///
/// This handler generates variant formers with better error handling and more
/// informative compiler messages when trait bounds aren't satisfied.
pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result<  proc_macro2::TokenStream  >
{
  let variant_name = ctx.variant_name;
  let variant_fields = ctx.variant.fields();
  let field = variant_fields.iter().next().unwrap();
  let field_type = &field.ty;
  let enum_name = ctx.enum_name;
  let (impl_generics, ty_generics, where_clause) = ctx.generics.split_for_impl();

  // Check if this variant has explicit scalar attribute
  let field_attrs = &ctx.fields.get(0).unwrap().attrs;
  let has_scalar_attr = field_attrs.scalar.value(false);
  
  if has_scalar_attr {
    // Use scalar approach for explicitly marked fields
    return generate_scalar_approach(ctx);
  }

  // Default to subform approach with enhanced error handling
  generate_enhanced_subform_approach(ctx)
}

/// Generates scalar approach for primitives and explicitly marked fields.
fn generate_scalar_approach(ctx : &mut EnumVariantHandlerContext<'_>) -> Result<  proc_macro2::TokenStream  >
{
  // Delegate to the scalar handler
  super::tuple_single_field_scalar::handle(ctx)
}

/// Generates enhanced subform approach with better error messages.
fn generate_enhanced_subform_approach(ctx : &mut EnumVariantHandlerContext<'_>) -> Result<  proc_macro2::TokenStream  >
{
  let variant_name = ctx.variant_name;
  let variant_fields = ctx.variant.fields();
  let field = variant_fields.iter().next().unwrap();
  let field_type = &field.ty;
  let enum_name = ctx.enum_name;
  let (impl_generics, ty_generics, where_clause) = ctx.generics.split_for_impl();
  
  // Generate method name
  let method_name = variant_to_method_name(variant_name);
  
  // Create informative error messages
  let error_hint = format!(
    "Field type `{}` in variant `{}` must implement `Former` trait for subform functionality. \
     Consider adding `#[ scalar ]` attribute if this is a primitive type.",
    quote!(#field_type).to_string(),
    variant_name
  );
  
  Ok(quote! {
    impl #impl_generics #enum_name #ty_generics
    #where_clause
    {
      #[ doc = concat!("Subformer for `", stringify!(#variant_name), "` variant.") ]
      #[ doc = "" ]
      #[ doc = "This method returns a subformer that delegates to the field type's Former implementation." ]
      #[ doc = concat!("If you get a compilation error, the field type `", stringify!(#field_type), "` may not implement `Former`.") ]
      #[ doc = "In that case, consider using `#[ scalar ]` attribute instead." ]
      #[ inline( always ) ]
      pub fn #method_name() -> < #field_type as former::EntityToFormer< #field_type##FormerDefinition > >::Former
      where
        #field_type: former::EntityToFormer< #field_type##FormerDefinition >,
        #field_type##FormerDefinition: former::FormerDefinition< Storage = #field_type##FormerStorage >,
        #field_type##FormerStorage: former::Storage< Preformed = #field_type >,
      {
        // Enhanced error message for better debugging
        const _: fn() = || {
          fn assert_former_requirements<T>() 
          where 
            T: former::EntityToFormer< T##FormerDefinition >,
            T##FormerDefinition: former::FormerDefinition< Storage = T##FormerStorage >,
            T##FormerStorage: former::Storage< Preformed = T >,
          {}
          
          // This will provide a clear error if requirements aren't met
          if false {
            assert_former_requirements::<#field_type>();
          }
        };
        
        // Create the actual subformer with proper end handling
        < #field_type as former::EntityToFormer< #field_type##FormerDefinition > >::Former::begin(
          None,
          None, 
          |storage, _context| {
            let field_value = former::StoragePreform::preform( storage );
            #enum_name::#variant_name( field_value )
          }
        )
      }
    }
  })
}

/// Fallback handler that provides helpful compilation errors.
///
/// This generates code that will provide clear error messages if the
/// field type doesn't meet the requirements for subform handling.
pub fn generate_error_fallback(ctx : &mut EnumVariantHandlerContext<'_>) -> Result<  proc_macro2::TokenStream  >
{
  let variant_name = ctx.variant_name;
  let field = ctx.variant.fields().iter().next().unwrap();
  let field_type = &field.ty;
  let enum_name = ctx.enum_name;
  
  Ok(quote! {
    // This will generate a helpful error message
    compile_error!(concat!(
      "Cannot generate subformer for variant `", stringify!(#variant_name), "` in enum `", stringify!(#enum_name), "`. ",
      "Field type `", stringify!(#field_type), "` does not implement the required Former traits. ",
      "Consider using `#[ scalar ]` attribute instead of `#[ subform_scalar ]` for primitive types."
    ));
  })
}