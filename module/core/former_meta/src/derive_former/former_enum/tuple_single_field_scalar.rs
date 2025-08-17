//! # Tuple Single-Field Scalar Handler - Direct Constructor Generation
//!
//! This handler specializes in generating direct scalar constructors for tuple enum variants 
//! with a single unnamed field marked with the `#[ scalar ]` attribute, providing efficient 
//! direct construction patterns that bypass the Former pattern for simple single-field scenarios.
//!
//! ## Variant Type Specialization
//!
//! **Target Pattern**: `Variant(T)` with `#[ scalar ]` attribute
//! **Generated Constructor**: `Enum::variant(T) -> Enum`
//! **Construction Style**: Direct function call with single parameter
//!
//! ## Key Behavioral Characteristics
//!
//! ### Attribute-Driven Activation
//! - **`#[ scalar ]` Required**: Single-field tuple variants with explicit `#[ scalar ]` attribute
//! - **Default Behavior**: Without `#[ scalar ]`, these variants get inner type formers
//! - **`#[ subform_scalar ]` Conflict**: Cannot be combined with `#[ subform_scalar ]`
//! - **Field-Level Attributes**: Field attributes not applicable for scalar construction
//!
//! ### Generated Method Characteristics
//! - **Parameter Type**: Single parameter with `impl Into<FieldType>` flexibility
//! - **Generic Safety**: Complete generic parameter and where clause propagation 
//! - **Performance**: Direct construction without Former overhead
//! - **Type Safety**: Compile-time type checking for field type
//!
//! ## Critical Pitfalls Resolved
//!
//! ### 1. Single-Field Parameter Handling (Critical Prevention)
//! **Issue Resolved**: Manual implementations not properly handling single tuple field parameter
//! **Root Cause**: Single-field tuple construction requires careful parameter type handling
//! **Solution**: Generated parameter with Into<T> conversion support for maximum flexibility
//! **Prevention**: Automated parameter handling with type safety guarantees
//!
//! ```rust,ignore
//! // Manual Implementation Pitfall:
//! impl MyEnum {
//!     fn variant(field: String) -> Self {  // ❌ Fixed type, no generics, no Into<T>
//!         MyEnum::Variant(field)
//!     }
//! }
//!
//! // Generated Solution:
//! impl<T> MyEnum<T> {
//!     fn variant(_0: impl Into<T>) -> MyEnum<T> {  // ✅ Generic with Into<T>
//!         MyEnum::Variant(_0.into())
//!     }
//! }
//! ```
//!
//! ### 2. Generic Parameter Context (Critical Prevention)
//! **Issue Resolved**: Manual implementations losing generic parameter context in single-field scenarios
//! **Root Cause**: Single-field tuple variants still require full generic parameter propagation
//! **Solution**: Complete generic parameter preservation through `GenericsRef` infrastructure
//! **Prevention**: Ensures all generic constraints are properly maintained
//!
//! ### 3. Tuple Field Naming (Prevention)
//! **Issue Resolved**: Manual implementations using inconsistent parameter naming for tuple fields
//! **Root Cause**: Tuple fields are positional and should use consistent index-based naming
//! **Solution**: Generated parameter uses standardized `_0` naming convention
//! **Prevention**: Consistent naming pattern eliminates confusion and maintains conventions
//!
//! ### 4. Into<T> Conversion Safety (Prevention)
//! **Issue Resolved**: Manual implementations not providing flexible type conversion for parameters
//! **Root Cause**: Direct parameter types are too restrictive for practical usage
//! **Solution**: Parameter accepts `impl Into<FieldType>` for maximum flexibility
//! **Prevention**: Type-safe conversion handling with automatic type coercion
//!
//! ```rust,ignore
//! // Manual Implementation Pitfall:
//! fn variant(s: String) -> MyEnum {  // ❌ Only accepts String
//!     MyEnum::Variant(s)
//! }
//!
//! // Generated Solution:
//! fn variant(_0: impl Into<String>) -> MyEnum {  // ✅ Accepts &str, String, etc.
//!     MyEnum::Variant(_0.into())
//! }
//! ```
//!
//! ### 5. Where Clause Propagation (Prevention)
//! **Issue Resolved**: Manual implementations not properly propagating where clause constraints
//! **Root Cause**: Generic constraints needed for proper type checking in single-field scenarios
//! **Solution**: Systematic where clause propagation to generated constructor method
//! **Prevention**: Ensures all generic constraints are properly maintained
//!
//! ## Generated Code Architecture
//!
//! ### Direct Constructor Pattern
//! ```rust,ignore
//! impl<T> Enum<T> where T: Clone {
//!     pub fn variant(_0: impl Into<T>) -> Enum<T> {
//!         Enum::Variant(_0.into())
//!     }
//! }
//! ```
//!
//! ### Generic Parameter Handling
//! - **Generic Preservation**: All enum generic parameters maintained in method signature
//! - **Where Clause**: All enum where clauses propagated to method
//! - **Type Path**: Proper enum type path construction with generic parameters
//! - **Parameter Flexibility**: Single parameter accepts `impl Into<FieldType>`
//!
//! ## Integration Notes
//! - **Performance Optimized**: Direct construction bypasses Former overhead for maximum efficiency
//! - **Attribute Validation**: Compile-time validation ensures proper attribute usage
//! - **Generic Safety**: Complete type safety through generic parameter propagation
//! - **Conversion Flexibility**: Parameter accepts flexible input types through Into<T> conversion
//! - **Naming Consistency**: Uses standardized `_0` parameter naming for tuple field convention

use super::*;
use macro_tools::{ Result, quote::quote };
use crate::derive_former::raw_identifier_utils::variant_to_method_name;

/// Generates direct scalar constructor for single-field tuple enum variants with `#[ scalar ]` attribute.
///
/// This function creates efficient direct constructors for tuple variants with a single unnamed field,
/// implementing comprehensive pitfall prevention for parameter handling, generic propagation,
/// and type conversion flexibility while maintaining zero-cost abstraction guarantees.
///
/// ## Generated Infrastructure
///
/// ### Direct Constructor Method:
/// - **Single Parameter**: Tuple field becomes function parameter with `impl Into<FieldType>`
/// - **Generic Propagation**: Complete generic parameter and where clause preservation
/// - **Type Conversion**: Flexible input type through Into<T> trait usage
/// - **Performance**: Direct construction without Former pattern overhead
///
/// ## Pitfall Prevention Features
///
/// - **Parameter Safety**: Uses standardized `_0` parameter naming for tuple field convention
/// - **Generic Context**: Complete generic parameter preservation through proper type path construction
/// - **Type Flexibility**: Parameter accepts `impl Into<T>` for maximum usability
/// - **Naming Consistency**: Maintains tuple field naming conventions
///
/// ## Generated Method Signature
/// ```rust,ignore
/// impl<T> Enum<T> where T: Clone {
///     pub fn variant(_0: impl Into<T>) -> Enum<T> {
///         Enum::Variant(_0.into())
///     }
/// }
/// ```
///
/// ## Parameters
/// - `ctx`: Mutable context containing variant information, generics, and output collections
///
/// ## Returns
/// - `Ok(TokenStream)`: Generated direct constructor method for the single-field tuple variant
/// - `Err(syn::Error)`: If variant processing fails due to invalid configuration
pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result<  proc_macro2::TokenStream  >
{
  let variant_name = &ctx.variant.ident;
  let method_name = variant_to_method_name(variant_name);
  let enum_name = ctx.enum_name;
  let vis = ctx.vis;
  let field_type = &ctx.variant_field_info[0].ty;

  let ( _impl_generics, ty_generics, where_clause ) = ctx.generics.split_for_impl();

  // Rule 1d: #[ scalar ] on single-field tuple variants generates scalar constructor
  let enum_type_path = if ctx.generics.type_params().next().is_some() {
    quote! { #enum_name #ty_generics }
  } else {
    quote! { #enum_name }
  };

  let result = quote!
  {
    #[ inline( always ) ]
    #vis fn #method_name ( _0 : impl Into< #field_type > ) -> #enum_name #ty_generics
    #where_clause
    {
      #enum_type_path :: #variant_name( _0.into() )
    }
  };

  Ok( result )
}
