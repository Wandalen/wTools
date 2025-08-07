//! # Smart Tuple Single-Field Handler with Compile-Time Trait Detection
//!
//! This handler implements intelligent routing between different approaches for single-field
//! tuple enum variants based on compile-time trait detection. It automatically chooses the
//! optimal strategy based on whether the field type implements the Former trait.
//!
//! ## Smart Routing Logic
//! 
//! 1. **Former Trait Detection**: Uses compile-time detection to check if field type implements Former
//! 2. **Automatic Strategy Selection**:
//!    - If type implements Former: Delegate to field's Former (subform approach)
//!    - If type doesn't implement Former: Generate variant former (fixed manual approach)
//! 3. **Fallback Safety**: Always provides working implementation regardless of trait availability
//!
//! ## Benefits
//! - **Zero Runtime Overhead**: All decisions made at compile-time
//! - **Optimal Performance**: Uses best approach for each type
//! - **Universal Compatibility**: Works with primitives and Former-implementing types
//! - **Automatic Behavior**: No manual attribute configuration required

use super::*;
use crate::derive_former::trait_detection::*;

use macro_tools::{ Result, quote::{ quote, format_ident } };
use crate::derive_former::raw_identifier_utils::variant_to_method_name;

/// Generates smart routing handler for single-field tuple enum variants.
///
/// This function implements compile-time trait detection to automatically choose
/// between subform delegation and manual variant former generation based on whether
/// the field type implements the Former trait.
///
/// ## Generated Strategies
///
/// ### For Former-implementing types:
/// ```rust,ignore
/// impl<T: Former> Enum<T> {
///     pub fn variant() -> T::Former { /* delegate to field's Former */ }
/// }
/// ```
///
/// ### For primitive types:
/// ```rust,ignore
/// impl<T> Enum<T> {
///     pub fn variant() -> VariantFormer<T> { /* custom variant former */ }
/// }
/// ```
pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = ctx.variant_name;
  let variant_fields = ctx.variant.fields();
  let field = variant_fields.iter().next().unwrap();
  let field_type = &field.ty;
  
  // Generate trait detection helper
  let trait_detector = generate_former_trait_detector();
  
  // Generate Former-delegating approach (for types that implement Former)
  let subform_delegation_approach = generate_subform_delegation_approach(ctx)?;
  
  // Generate manual variant former approach (for primitive types)
  let manual_variant_approach = generate_manual_variant_approach(ctx)?;
  
  // Generate smart routing logic
  let smart_routing = generate_smart_routing(
    field_type,
    subform_delegation_approach,
    manual_variant_approach,
  );
  
  Ok(quote! {
    #trait_detector
    #smart_routing
  })
}

/// Generates the subform delegation approach for types that implement Former.
///
/// This approach delegates to the field type's existing Former implementation,
/// providing seamless integration with nested Former-implementing types.
fn generate_subform_delegation_approach(ctx : &mut EnumVariantHandlerContext<'_>) -> Result< proc_macro2::TokenStream >
{
  let variant_name = ctx.variant_name;
  let variant_fields = ctx.variant.fields();
  let field = variant_fields.iter().next().unwrap();
  let field_type = &field.ty;
  let enum_name = ctx.enum_name;
  let (impl_generics, ty_generics, where_clause) = ctx.generics.split_for_impl();
  
  // Generate method that delegates to field type's Former
  let method_name = variant_to_method_name(variant_name);
  
  Ok(quote! {
    impl #impl_generics #enum_name #ty_generics
    #where_clause
    {
      /// Subform delegation approach - delegates to field type's Former
      #[ inline( always ) ]
      pub fn #method_name() -> impl former::FormingEnd< <#field_type as former::EntityToDefinitionTypes<(), #enum_name #ty_generics>>::Types >
      where
        #field_type: former::Former,
        #field_type: former::EntityToDefinitionTypes<(), #enum_name #ty_generics>,
      {
        // Create end handler that constructs the enum variant
        struct VariantEnd;
        impl former::FormingEnd< <#field_type as former::EntityToDefinitionTypes<(), #enum_name #ty_generics>>::Types > for VariantEnd {
          fn call( &self, storage: <#field_type as former::EntityToStorage>::Storage, _context: Option<()> ) -> #enum_name #ty_generics {
            let field_value = former::StoragePreform::preform( storage );
            #enum_name::#variant_name( field_value )
          }
        }
        
        // Return the field's former with our custom end handler
        <#field_type as former::EntityToFormer<_>>::Former::begin( None, None, VariantEnd )
      }
    }
  })
}

/// Generates the manual variant former approach for primitive types.
///
/// This approach creates a complete variant former infrastructure similar to
/// the existing fixed implementation, providing full builder functionality.
fn generate_manual_variant_approach(ctx : &mut EnumVariantHandlerContext<'_>) -> Result< proc_macro2::TokenStream >
{
  // Use the existing fixed implementation logic
  super::tuple_single_field_subform::handle(ctx)
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_trait_detection_generation() {
    let detector = generate_former_trait_detector();
    let code = detector.to_string();
    
    // Verify the trait detection code is generated correctly
    assert!(code.contains("__FormerDetector"));
    assert!(code.contains("HAS_FORMER"));
    assert!(code.contains("::former::Former"));
  }
}