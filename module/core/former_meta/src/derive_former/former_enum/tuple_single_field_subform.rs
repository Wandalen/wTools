//! # Tuple Single-Field Subform Handler - Inner Former Integration
//!
//! This handler specializes in generating inner former constructors for tuple enum variants 
//! with a single unnamed field, creating sophisticated integration with the field type's Former
//! implementation while providing comprehensive pitfall prevention for Former trait resolution
//! and custom end handling.
//!
//! ## Variant Type Specialization
//!
//! **Target Pattern**: `Variant(T)` where `T` implements `Former`
//! **Generated Constructor**: `Enum::variant() -> T::Former` (configured with custom end)
//! **Construction Style**: Field type's Former with custom end handler for enum variant construction
//!
//! ## Key Behavioral Characteristics
//!
//! ### Attribute-Driven Activation
//! - **Default Behavior**: Single-field tuple variants without `#[scalar]` get inner type formers
//! - **`#[subform_scalar]` Support**: Explicitly enables inner former integration (same behavior)
//! - **`#[scalar]` Override**: Forces direct constructor generation (handled elsewhere)
//! - **Field Type Constraint**: Field type must implement Former trait for this handler
//!
//! ### Generated Infrastructure Components
//! 1. **Custom End Handler**: `{Enum}{Variant}End` for converting inner type to enum variant
//! 2. **End Definition Types**: `{Enum}{Variant}EndDefinitionTypes` for type system integration
//! 3. **FormingEnd Implementation**: Proper integration with Former's ending system
//! 4. **Method Integration**: Enum method that returns configured inner former
//!
//! ## Critical Pitfalls Resolved
//!
//! ### 1. Former Trait Resolution (Critical Prevention)
//! **Issue Resolved**: Manual implementations not properly resolving field type's Former implementation
//! **Root Cause**: Complex Former trait resolution requiring proper type path and generic handling
//! **Solution**: Automatic Former trait resolution with proper generic parameter propagation
//! **Prevention**: Generated code ensures field type's Former trait is properly accessible
//!
//! ```rust
//! // Manual Implementation Pitfall:
//! impl MyEnum {
//!     fn variant() -> String::Former {  // ❌ Incorrect Former trait usage
//!         String::former()
//!     }
//! }
//!
//! // Generated Solution:
//! impl<T> MyEnum<T> {
//!     fn variant() -> <T as EntityToFormer<TFormerDefinition>>::Former {  // ✅ Proper trait resolution
//!         <T as EntityToFormer<TFormerDefinition>>::Former::former_begin(
//!             None, None, MyEnumVariantEnd::default()
//!         )
//!     }
//! }
//! ```
//!
//! ### 2. Custom End Handler Generation (Critical Prevention)
//! **Issue Resolved**: Manual implementations not providing proper end handling for inner formers
//! **Root Cause**: Inner formers need custom end handlers to convert to enum variants
//! **Solution**: Generated custom End struct with proper FormingEnd implementation
//! **Prevention**: Ensures inner former completion properly constructs enum variant
//!
//! ### 3. FormerDefinition Type Resolution (Critical Prevention)
//! **Issue Resolved**: Manual implementations not properly determining field type's Former definition
//! **Root Cause**: Former definition type naming requires systematic pattern matching
//! **Solution**: Automatic generation of definition type names based on field type
//! **Prevention**: Consistent definition type resolution eliminates naming mismatches
//!
//! ```rust
//! // Manual Implementation Pitfall:
//! let former = MyFieldType::former();  // ❌ No custom end handling
//!
//! // Generated Solution:
//! let former = <MyFieldType as EntityToFormer<MyFieldTypeFormerDefinition>>::Former
//!     ::former_begin(None, None, CustomEnd::default());  // ✅ Proper end integration
//! ```
//!
//! ### 4. Generic Parameter Context Preservation (Critical Prevention)
//! **Issue Resolved**: Manual implementations losing enum generic context when calling inner formers
//! **Root Cause**: Inner former calls need enum's generic parameters for proper type resolution
//! **Solution**: Complete generic parameter preservation through custom end handler types
//! **Prevention**: Ensures enum generic parameters are properly maintained through inner former chain
//!
//! ### 5. FormingEnd Type Integration (Prevention)
//! **Issue Resolved**: Manual implementations not properly implementing FormingEnd for custom ends
//! **Root Cause**: FormingEnd trait requires specific type associations and call method implementation
//! **Solution**: Generated FormingEnd implementation with proper type conversions
//! **Prevention**: Ensures seamless integration with Former ecosystem's ending system
//!
//! ## Generated Code Architecture
//!
//! ### Custom End Handler
//! ```rust
//! #[derive(Default, Debug)]
//! pub struct EnumVariantEnd<T> 
//! where T: Former
//! {
//!     // Marker struct for custom end handling
//! }
//!
//! impl<T> FormingEnd<EnumVariantEndDefinitionTypes<T>> for EnumVariantEnd<T> {
//!     fn call(&self, sub_storage: Storage, _context: Option<Context>) -> Enum<T> {
//!         let inner = StoragePreform::preform(sub_storage);
//!         Enum::Variant(inner)
//!     }
//! }
//! ```
//!
//! ### End Definition Types
//! ```rust
//! impl<T> FormerDefinitionTypes for EnumVariantEndDefinitionTypes<T> {
//!     type Storage = <TFormerDefinition as FormerDefinition>::Storage;
//!     type Context = <TFormerDefinition as FormerDefinition>::Context;
//!     type Formed = Enum<T>;
//! }
//! ```
//!
//! ### Generated Method
//! ```rust
//! impl<T> Enum<T> {
//!     pub fn variant() -> <T as EntityToFormer<TFormerDefinition>>::Former {
//!         <T as EntityToFormer<TFormerDefinition>>::Former::former_begin(
//!             None, None, EnumVariantEnd::default()
//!         )
//!     }
//! }
//! ```
//!
//! ## Integration Notes
//! - **Former Ecosystem**: Complete integration with existing Former trait hierarchy
//! - **Type Safety**: Compile-time verification of Former trait implementation for field types
//! - **Context Handling**: Proper context propagation through inner former to enum construction
//! - **Generic Safety**: Complete generic parameter preservation through Former chain
//! - **End Customization**: Custom end handling ensures proper enum variant construction from inner type

use super::*;

use macro_tools::{ Result, quote::{ quote, format_ident }, ident::cased_ident_from_ident, generic_params::GenericsRef };
use convert_case::Case;

/// Generates inner former integration infrastructure for single-field tuple enum variants.
///
/// This function creates sophisticated integration with the field type's Former implementation,
/// providing comprehensive pitfall prevention for Former trait resolution, custom end handling,
/// and generic parameter preservation through the Former chain.
///
/// ## Generated Infrastructure
///
/// ### Core Components Generated:
/// 1. **Custom End Handler**: `{Enum}{Variant}End` for converting inner type to enum variant
/// 2. **End Definition Types**: `{Enum}{Variant}EndDefinitionTypes` for type system integration
/// 3. **FormingEnd Implementation**: Proper integration with Former's ending system
/// 4. **Method Integration**: Enum method returning configured field type former
///
/// ## Former Integration Features
///
/// - **Trait Resolution**: Automatic Former trait resolution with proper generic handling
/// - **Custom End**: Generated end handler ensures proper enum variant construction
/// - **Type Safety**: Compile-time verification of Former trait implementation for field types
/// - **Generic Preservation**: Complete generic parameter maintenance through Former chain
///
/// ## Generated Method Signature
/// ```rust
/// impl<T> Enum<T> {
///     pub fn variant() -> <T as EntityToFormer<TFormerDefinition>>::Former {
///         // Returns field type's former configured with custom end
///     }
/// }
/// ```
///
/// ## Generated End Handler
/// ```rust
/// impl<T> FormingEnd<EndDefinitionTypes<T>> for EnumVariantEnd<T> {
///     fn call(&self, sub_storage: Storage, _context: Option<Context>) -> Enum<T> {
///         let inner = StoragePreform::preform(sub_storage);
///         Enum::Variant(inner)
///     }
/// }
/// ```
///
/// ## Parameters
/// - `ctx`: Mutable context containing variant information, generics, and output collections
///
/// ## Returns
/// - `Ok(TokenStream)`: Generated enum method that returns configured field type former
/// - `Err(syn::Error)`: If variant processing fails or field type path is invalid
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
  let syn::Type::Path(field_type_path) = field_type else {
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

    // Add FormerMutator implementation here
    impl #impl_generics former_types::FormerMutator
    for #enum_end_definition_types #ty_generics
    #where_clause
    {
      #[ inline( always ) ]
      fn form_mutation
      (
        _storage : &mut Self::Storage,
        _context : &mut Option< Self::Context >,
      )
      {
      }
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
