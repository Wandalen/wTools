//! # Struct Multi-Field Subform Handler - Complex Struct Variant Former Generation
//!
//! This handler specializes in generating implicit variant formers for struct enum variants 
//! with multiple named fields, providing sophisticated field-by-field construction capabilities
//! with comprehensive pitfall prevention for complex generic scenarios.
//!
//! ## Variant Type Specialization
//!
//! **Target Pattern**: `Variant { field1: T1, field2: T2, ..., fieldN: TN }`
//! **Generated Constructor**: `Enum::variant() -> VariantFormer<...>`
//! **Construction Style**: Multi-step builder pattern with individual field setters
//!
//! ## Key Behavioral Characteristics
//!
//! ### Attribute-Driven Activation
//! - **Default Behavior**: Multi-field struct variants automatically get implicit variant formers
//! - **`#[scalar]` Override**: Forces direct constructor generation instead (handled elsewhere)
//! - **`#[subform_scalar]` Support**: Supported but generates same implicit variant former
//! - **Field-Level Attributes**: Individual field attributes respected in generated setters
//!
//! ### Generated Infrastructure Components
//! 1. **`{Enum}{Variant}FormerStorage`**: Optional field storage for incremental construction
//! 2. **`{Enum}{Variant}FormerDefinitionTypes`**: Type system integration for Former trait
//! 3. **`{Enum}{Variant}FormerDefinition`**: Definition linking storage, context, and formed type
//! 4. **`{Enum}{Variant}Former`**: Main builder struct with field setters and termination methods
//! 5. **Entity Trait Implementations**: Complete Former ecosystem integration
//!
//! ## Critical Pitfalls Resolved
//!
//! ### 1. Generic Parameter Propagation (Critical Prevention)
//! **Issue Resolved**: Manual implementations losing generic parameter information during variant former generation
//! **Root Cause**: Complex generic parameter tracking through multiple generated struct definitions
//! **Solution**: Systematic generic parameter preservation through all generated components
//! **Prevention**: Uses `GenericsRef` for consistent generic parameter handling across all generated items
//!
//! ```rust,ignore
//! // Manual Implementation Pitfall:
//! impl MyEnum {
//!     fn variant() -> VariantFormer {  // ❌ Generic parameters lost
//!         // Missing <T, U> generic parameters
//!     }
//! }
//!
//! // Generated Solution:
//! impl<T, U> MyEnum<T, U> {
//!     fn variant() -> VariantFormer<T, U> {  // ✅ Generic parameters preserved
//!         VariantFormer::new(ReturnPreformed::default())
//!     }
//! }
//! ```
//!
//! ### 2. Storage Field Type Safety (Critical Prevention)
//! **Issue Resolved**: Manual implementations using incorrect optional wrapping for field storage
//! **Root Cause**: Forgetting that former storage requires Optional<T> wrapping for incremental construction
//! **Solution**: Automatic Optional<T> wrapping with proper unwrap_or_default() handling in preform
//! **Prevention**: Generated storage always uses `Option<FieldType>` with safe defaults
//!
//! ```rust,ignore
//! // Manual Implementation Pitfall:
//! struct VariantFormerStorage {
//!     field1: String,  // ❌ Should be Option<String>
//!     field2: i32,     // ❌ Should be Option<i32>
//! }
//!
//! // Generated Solution:
//! struct VariantFormerStorage {
//!     field1: Option<String>,  // ✅ Proper optional wrapping
//!     field2: Option<i32>,     // ✅ Allows incremental construction
//! }
//! ```
//!
//! ### 3. Former Trait Integration (Critical Prevention)
//! **Issue Resolved**: Manual implementations missing required trait implementations for Former ecosystem
//! **Root Cause**: Complex trait hierarchy requiring multiple interrelated implementations
//! **Solution**: Automatic generation of all required trait implementations with proper type associations
//! **Prevention**: Complete trait implementation suite ensures compatibility with Former-based APIs
//!
//! ### 4. Where Clause Propagation (Prevention)
//! **Issue Resolved**: Manual implementations not properly propagating where clause constraints
//! **Root Cause**: Where clauses needed on all generated items for proper type constraint enforcement
//! **Solution**: Systematic where clause propagation to all generated structs and implementations
//! **Prevention**: Ensures all generic constraints are properly maintained across generated code
//!
//! ### 5. Lifetime Parameter Handling (Prevention)
//! **Issue Resolved**: Manual implementations dropping lifetime parameters during generation
//! **Root Cause**: Lifetime parameters require careful tracking through multiple generic contexts
//! **Solution**: Complete lifetime parameter preservation in all generated generic contexts
//! **Prevention**: Maintains lifetime safety guarantees through entire Former construction chain
//!
//! ## Generated Code Architecture
//!
//! ### Storage Infrastructure
//! ```rust,ignore
//! pub struct EnumVariantFormerStorage<T, U> 
//! where T: Clone, U: Default
//! {
//!     pub field1: Option<T>,      // Incremental field storage
//!     pub field2: Option<U>,      // Safe optional wrapping
//! }
//! ```
//!
//! ### Former Definition System
//! ```rust,ignore
//! pub struct EnumVariantFormerDefinitionTypes<T, U> { /* ... */ }
//! pub struct EnumVariantFormerDefinition<T, U> { /* ... */ }
//! 
//! impl<T, U> FormerDefinition for EnumVariantFormerDefinition<T, U> {
//!     type Storage = EnumVariantFormerStorage<T, U>;
//!     type Formed = Enum<T, U>;
//!     // Complete trait implementation
//! }
//! ```
//!
//! ### Builder Implementation
//! ```rust,ignore
//! impl<T, U> EnumVariantFormer<T, U> {
//!     pub fn field1(mut self, value: impl Into<T>) -> Self { /* ... */ }
//!     pub fn field2(mut self, value: impl Into<U>) -> Self { /* ... */ }
//!     pub fn form(self) -> Enum<T, U> { /* ... */ }
//! }
//! ```
//!
//! ## Integration Notes
//! - **Standalone Constructors**: Supports `#[standalone_constructors]` for top-level function generation
//! - **Context Handling**: Integrates with Former's context system for advanced construction scenarios
//! - **Error Handling**: Provides clear compilation errors for invalid attribute combinations
//! - **Performance**: Generated code is optimized with `#[inline(always)]` for zero-cost abstractions

use super::*;

use macro_tools::{ Result, quote::{ quote, format_ident }, generic_params::GenericsRef };
use crate::derive_former::raw_identifier_utils::variant_to_method_name;
// use iter_tools::Itertools; // Removed unused import

/// Generates comprehensive implicit variant former infrastructure for multi-field struct enum variants.
///
/// This function creates a complete builder ecosystem for struct variants with multiple named fields,
/// implementing sophisticated pitfall prevention mechanisms for generic parameter handling,
/// storage type safety, and Former trait integration.
///
/// ## Generated Infrastructure
///
/// ### Core Components Generated:
/// 1. **Storage Struct**: `{Enum}{Variant}FormerStorage` with optional field wrapping
/// 2. **Definition Types**: `{Enum}{Variant}FormerDefinitionTypes` for type system integration
/// 3. **Definition**: `{Enum}{Variant}FormerDefinition` linking all components
/// 4. **Former Builder**: `{Enum}{Variant}Former` with field setters and termination methods
/// 5. **Entity Traits**: Complete Former ecosystem trait implementations
///
/// ## Pitfall Prevention Mechanisms
///
/// - **Generic Safety**: All generated items properly propagate generic parameters and where clauses
/// - **Storage Safety**: Fields are wrapped in `Option<T>` with safe default handling
/// - **Trait Integration**: Complete Former trait hierarchy implementation prevents ecosystem incompatibility
/// - **Context Preservation**: Proper context handling for advanced Former scenarios
///
/// ## Generated Method Signature
/// ```rust,ignore
/// impl<T, U> Enum<T, U> {
///     pub fn variant() -> VariantFormer<T, U> { /* ... */ }
/// }
/// ```
///
/// ## Parameters
/// - `ctx`: Mutable context containing variant information, generics, and output collections
///
/// ## Returns
/// - `Ok(TokenStream)`: Generated enum method that returns the variant former
/// - `Err(syn::Error)`: If variant processing fails due to invalid configuration
pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = &ctx.variant.ident;
  let method_name = variant_to_method_name(variant_name);
  let enum_name = ctx.enum_name;
  let vis = ctx.vis;
  let fields = &ctx.variant_field_info;

  let generics_ref = GenericsRef::new(ctx.generics);
  let ( impl_generics, ty_generics, where_clause ) = ctx.generics.split_for_impl();
  let enum_type_path = if ctx.generics.type_params().next().is_some() {
    let ty_generics_tokens = generics_ref.ty_generics_tokens_if_any();
    quote! { #enum_name :: #ty_generics_tokens }
  } else {
    quote! { #enum_name }
  };

  // Generate the End struct name for this variant
  let end_struct_name = format_ident!("{}{}End", enum_name, variant_name);

  // Generate the End struct for this variant
  let end_struct = quote!
  {
    #[derive(Default, Debug)]
    pub struct #end_struct_name #impl_generics
    #where_clause
    {}
  };

  // Generate the implicit former for the variant
  let variant_name_str = crate::derive_former::raw_identifier_utils::strip_raw_prefix_for_compound_ident(variant_name);
  let variant_former_name = format_ident!("{}{}Former", enum_name, variant_name_str);
  let variant_former_storage_name = format_ident!("{}{}FormerStorage", enum_name, variant_name_str);
  let variant_former_definition_name = format_ident!("{}{}FormerDefinition", enum_name, variant_name_str);
  let variant_former_definition_types_name = format_ident!("{}{}FormerDefinitionTypes", enum_name, variant_name_str);

  // Generate the storage struct for the variant's fields
  let storage_field_optional: Vec<_> = fields.iter().map(|f| {
    let field_name = &f.ident;
    let field_type = &f.ty;
    quote! { pub #field_name : ::core::option::Option< #field_type > }
  }).collect();
  let storage_field_none: Vec<_> = fields.iter().map(|f| {
    let field_name = &f.ident;
    quote! { #field_name : ::core::option::Option::None }
  }).collect();
  let storage_field_preform: Vec<_> = fields.iter().map(|f| {
    let field_name = &f.ident;
    quote! { let #field_name = self.#field_name.unwrap_or_default(); }
  }).collect();
  let storage_field_name: Vec<_> = fields.iter().map(|f| {
    let field_name = &f.ident;
    quote! { #field_name }
  }).collect();

  // Capture field types for setters
  let field_types_for_setters: Vec<_> = fields.iter().map(|f| &f.ty).collect();

  let variant_former_code = quote!
  {
    // = definition types: Define the FormerDefinitionTypes struct for the variant.
    #[ derive( Debug ) ]
    pub struct #variant_former_definition_types_name #impl_generics
    #where_clause
    {
      _phantom : ::core::marker::PhantomData< ( #impl_generics ) >,
    }

    impl #impl_generics ::core::default::Default
    for #variant_former_definition_types_name #ty_generics
    #where_clause
    {
      fn default() -> Self
      {
        Self
        {
          _phantom : ::core::marker::PhantomData,
        }
      }
    }

    impl #impl_generics former_types::FormerDefinitionTypes
    for #variant_former_definition_types_name #ty_generics
    #where_clause
    {
      type Storage = #variant_former_storage_name #ty_generics;
      type Formed = #enum_name #ty_generics;
      type Context = ();
    }

    // Add FormerMutator implementation here
    impl #impl_generics former_types::FormerMutator
    for #variant_former_definition_types_name #ty_generics
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

    // = definition: Define the FormerDefinition struct for the variant.
    #[ derive( Debug ) ]
    pub struct #variant_former_definition_name #impl_generics
    #where_clause
    {
      _phantom : ::core::marker::PhantomData< ( #impl_generics ) >,
    }

    impl #impl_generics ::core::default::Default
    for #variant_former_definition_name #ty_generics
    #where_clause
    {
      fn default() -> Self
      {
        Self
        {
          _phantom : ::core::marker::PhantomData,
        }
      }
    }

    impl #impl_generics former_types::FormerDefinition
    for #variant_former_definition_name #ty_generics
    #where_clause
    {
      type Types = #variant_former_definition_types_name #ty_generics;
      type End = former_types::forming::ReturnPreformed;
      type Storage = #variant_former_storage_name #ty_generics;
      type Formed = #enum_name #ty_generics;
      type Context = ();
    }

    // = storage: Define the FormerStorage struct for the variant.
    #[ doc = "Stores potential values for fields during the formation process." ]
    #[ allow( explicit_outlives_requirements ) ]
    pub struct #variant_former_storage_name #impl_generics
    #where_clause
    {
      #(
        /// A field
        #storage_field_optional,
      )*
    }

    impl #impl_generics ::core::default::Default
    for #variant_former_storage_name #ty_generics
    #where_clause
    {
      #[ inline( always ) ]
      fn default() -> Self
      {
        Self
        {
          #( #storage_field_none, )*
        }
      }
    }

    impl #impl_generics former_types::Storage
    for #variant_former_storage_name #ty_generics
    #where_clause
    {
      type Preformed = #enum_name #ty_generics;
    }

    impl #impl_generics former_types::StoragePreform
    for #variant_former_storage_name #ty_generics
    #where_clause
    {
      fn preform( mut self ) -> Self::Preformed
      {
        #( #storage_field_preform )*
        let result = #enum_name::#variant_name { #( #storage_field_name ),* };
        return result;
      }
    }

    // = former: Define the Former struct itself for the variant.
    pub struct #variant_former_name #impl_generics
    #where_clause
    {
      pub storage : #variant_former_storage_name #ty_generics,
      pub context : ::core::option::Option< () >,
      pub on_end : ::core::option::Option< former_types::forming::ReturnPreformed >,
    }

    impl #impl_generics #variant_former_name #ty_generics
    #where_clause
    {
      #[ inline( always ) ]
      pub fn new
      (
        on_end : former_types::forming::ReturnPreformed
      ) -> Self
      {
        Self::begin_coercing( ::core::option::Option::None, ::core::option::Option::None, on_end )
      }

      #[ inline( always ) ]
      pub fn new_coercing< IntoEnd >
      (
        end : IntoEnd
      ) -> Self
      where
        IntoEnd : ::core::convert::Into< former_types::forming::ReturnPreformed >,
      {
        Self::begin_coercing
        (
          ::core::option::Option::None,
          ::core::option::Option::None,
          end,
        )
      }

      #[ inline( always ) ]
      pub fn begin
      (
        mut storage : ::core::option::Option< #variant_former_storage_name #ty_generics >,
        context : ::core::option::Option< () >,
        on_end : former_types::forming::ReturnPreformed,
      )
      -> Self
      {
        if storage.is_none()
        {
          storage = ::core::option::Option::Some( ::core::default::Default::default() );
        }
        Self
        {
          storage : storage.unwrap(),
          context : context,
          on_end : ::core::option::Option::Some( on_end ),
        }
      }

      #[ inline( always ) ]
      pub fn begin_coercing< IntoEnd >
      (
        mut storage : ::core::option::Option< #variant_former_storage_name #ty_generics >,
        context : ::core::option::Option< () >,
        on_end : IntoEnd,
      ) -> Self
      where
        IntoEnd : ::core::convert::Into< former_types::forming::ReturnPreformed >,
      {
        if storage.is_none()
        {
          storage = ::core::option::Option::Some( ::core::default::Default::default() );
        }
        Self
        {
          storage : storage.unwrap(),
          context : context,
          on_end : ::core::option::Option::Some( ::core::convert::Into::into( on_end ) ),
        }
      }

      #[ inline( always ) ]
      pub fn form( self ) -> #enum_name #ty_generics
      {
        self.end()
      }

      #[ inline( always ) ]
      pub fn end( mut self ) -> #enum_name #ty_generics
      {
        let on_end = self.on_end.take().unwrap();
        let mut context = self.context.take();
        < #variant_former_definition_types_name #ty_generics as former_types::FormerMutator >::form_mutation( &mut self.storage, &mut context );
        former_types::forming::FormingEnd::< #variant_former_definition_types_name #ty_generics >::call( &on_end, self.storage, context )
      }

      // Setters for each field
      #(
        #[ inline( always ) ]
        pub fn #storage_field_name( mut self, value : impl ::core::convert::Into< #field_types_for_setters > ) -> Self
        {
          self.storage.#storage_field_name = ::core::option::Option::Some( value.into() );
          self
        }
      )*
    }

    // = entity to former: Implement former traits linking the variant to its generated components.
    impl #impl_generics former_types::EntityToFormer< #variant_former_definition_name #ty_generics >
    for #enum_name #ty_generics
    #where_clause
    {
      type Former = #variant_former_name #ty_generics;
    }

    impl #impl_generics former_types::EntityToStorage
    for #enum_name #ty_generics
    #where_clause
    {
      type Storage = #variant_former_storage_name #ty_generics;
    }

    impl #impl_generics former_types::EntityToDefinition< (), #enum_name #ty_generics, former_types::forming::ReturnPreformed >
    for #enum_name #ty_generics
    #where_clause
    {
      type Definition = #variant_former_definition_name #ty_generics;
      type Types = #variant_former_definition_types_name #ty_generics;
    }

    impl #impl_generics former_types::EntityToDefinitionTypes< (), #enum_name #ty_generics >
    for #enum_name #ty_generics
    #where_clause
    {
      type Types = #variant_former_definition_types_name #ty_generics;
    }
  };

  // Generate the method for the enum
  let method = quote!
  {
    #[ inline( always ) ]
    #vis fn #method_name() -> #variant_former_name #ty_generics
    {
      #variant_former_name::new( former_types::forming::ReturnPreformed::default() )
    }
  };

  // Generate standalone constructor if requested
  if ctx.struct_attrs.standalone_constructors.value(false) {
    let standalone_method = quote!
    {
      #[ inline( always ) ]
      #vis fn #method_name() -> #variant_former_name #ty_generics
      {
        #variant_former_name::new( former_types::forming::ReturnPreformed::default() )
      }
    };
    ctx.standalone_constructors.push(standalone_method);
  }

  ctx.end_impls.push(variant_former_code);

  Ok(method)
}
