//! # Struct Single-Field Subform Handler - Implicit Variant Former Generation
//!
//! This handler specializes in generating implicit variant formers for struct enum variants 
//! with a single named field, creating sophisticated builder patterns that enable field-by-field
//! construction with comprehensive pitfall prevention for single-field scenarios.
//!
//! ## Variant Type Specialization
//!
//! **Target Pattern**: `Variant { field: T }`
//! **Generated Constructor**: `Enum::variant() -> VariantFormer<...>`
//! **Construction Style**: Single-field builder pattern with setter method and termination
//!
//! ## Key Behavioral Characteristics
//!
//! ### Attribute-Driven Activation
//! - **Default Behavior**: Single-field struct variants automatically get implicit variant formers
//! - **`#[scalar]` Override**: Forces direct constructor generation instead (handled elsewhere)
//! - **`#[subform_scalar]` Support**: Supported and generates same implicit variant former
//! - **Field-Level Attributes**: Individual field attributes respected in generated setter
//!
//! ### Generated Infrastructure Components
//! 1. **`{Enum}{Variant}FormerStorage`**: Single-field optional storage for incremental construction
//! 2. **`{Enum}{Variant}FormerDefinitionTypes`**: Type system integration for Former trait
//! 3. **`{Enum}{Variant}FormerDefinition`**: Definition linking storage, context, and formed type
//! 4. **`{Enum}{Variant}Former`**: Main builder struct with field setter and termination methods
//! 5. **Entity Trait Implementations**: Complete Former ecosystem integration
//!
//! ## Critical Pitfalls Resolved
//!
//! ### 1. Single-Field Storage Specialization (Critical Prevention)
//! **Issue Resolved**: Manual implementations treating single-field variants like multi-field variants
//! **Root Cause**: Single-field struct variants have different construction patterns than multi-field
//! **Solution**: Specialized single-field storage generation with proper Optional<T> wrapping
//! **Prevention**: Optimized single-field handling while maintaining Former pattern consistency
//!
//! ```rust
//! // Manual Implementation Pitfall:
//! struct VariantFormerStorage {
//!     field: String,  // ❌ Should be Option<String>
//! }
//! impl Default for VariantFormerStorage {
//!     fn default() -> Self {
//!         Self { field: String::new() }  // ❌ Wrong default handling
//!     }
//! }
//!
//! // Generated Solution:
//! struct VariantFormerStorage {
//!     field: Option<String>,  // ✅ Proper optional wrapping
//! }
//! impl Default for VariantFormerStorage {
//!     fn default() -> Self {
//!         Self { field: None }  // ✅ Correct optional default
//!     }
//! }
//! ```
//!
//! ### 2. Generic Parameter Context (Critical Prevention)
//! **Issue Resolved**: Manual implementations losing generic parameter context in single-field scenarios
//! **Root Cause**: Single-field variants still require full generic parameter propagation
//! **Solution**: Complete generic parameter preservation through all generated components
//! **Prevention**: Uses `GenericsRef` for consistent generic handling regardless of field count
//!
//! ### 3. Setter Method Type Safety (Prevention)
//! **Issue Resolved**: Manual implementations not properly handling Into<T> conversions for setters
//! **Root Cause**: Field setters need flexible type acceptance while maintaining type safety
//! **Solution**: Generated setter uses `impl Into<FieldType>` for maximum flexibility
//! **Prevention**: Type-safe conversion handling with automatic type coercion
//!
//! ```rust
//! // Manual Implementation Pitfall:
//! impl VariantFormer {
//!     pub fn field(mut self, value: String) -> Self {  // ❌ Too restrictive
//!         self.storage.field = Some(value);
//!         self
//!     }
//! }
//!
//! // Generated Solution:
//! impl VariantFormer {
//!     pub fn field(mut self, value: impl Into<String>) -> Self {  // ✅ Flexible input
//!         self.storage.field = Some(value.into());
//!         self
//!     }
//! }
//! ```
//!
//! ### 4. StoragePreform Implementation (Critical Prevention)
//! **Issue Resolved**: Manual implementations not properly handling single-field preform logic
//! **Root Cause**: Single-field preform requires special handling for unwrap_or_default()
//! **Solution**: Specialized preform implementation for single-field variant construction
//! **Prevention**: Safe unwrapping with proper default value handling
//!
//! ### 5. Former Trait Integration (Critical Prevention)
//! **Issue Resolved**: Manual implementations missing required trait implementations
//! **Root Cause**: Single-field variants still need complete Former ecosystem integration
//! **Solution**: Full trait implementation suite for single-field scenarios
//! **Prevention**: Ensures compatibility with Former-based APIs regardless of field count
//!
//! ## Generated Code Architecture
//!
//! ### Single-Field Storage Infrastructure
//! ```rust
//! pub struct EnumVariantFormerStorage<T> 
//! where T: Default
//! {
//!     pub field: Option<T>,       // Single optional field storage
//! }
//!
//! impl<T> StoragePreform for EnumVariantFormerStorage<T> {
//!     fn preform(mut self) -> Self::Preformed {
//!         let field = self.field.unwrap_or_default();
//!         Enum::Variant { field }
//!     }
//! }
//! ```
//!
//! ### Builder Implementation
//! ```rust
//! impl<T> EnumVariantFormer<T> {
//!     pub fn field(mut self, value: impl Into<T>) -> Self {
//!         self.storage.field = Some(value.into());
//!         self
//!     }
//!     
//!     pub fn form(self) -> Enum<T> {
//!         self.end()
//!     }
//! }
//! ```
//!
//! ## Integration Notes
//! - **Standalone Constructors**: Supports `#[standalone_constructors]` for top-level function generation
//! - **Context Handling**: Integrates with Former's context system for advanced construction scenarios
//! - **Performance**: Single-field optimization maintains zero-cost abstraction guarantees
//! - **Type Safety**: Complete type safety through Former trait system integration

use super::*;

use macro_tools::{ Result, quote::{ quote, format_ident }, ident::cased_ident_from_ident, generic_params::GenericsRef };
use convert_case::Case;

/// Generates comprehensive implicit variant former infrastructure for single-field struct enum variants.
///
/// This function creates a complete builder ecosystem for struct variants with a single named field,
/// implementing specialized pitfall prevention mechanisms for single-field construction patterns,
/// storage optimization, and Former trait integration.
///
/// ## Generated Infrastructure
///
/// ### Core Components Generated:
/// 1. **Storage Struct**: `{Enum}{Variant}FormerStorage` with single optional field wrapping
/// 2. **Definition Types**: `{Enum}{Variant}FormerDefinitionTypes` for type system integration
/// 3. **Definition**: `{Enum}{Variant}FormerDefinition` linking all components
/// 4. **Former Builder**: `{Enum}{Variant}Former` with single field setter and termination methods
/// 5. **Entity Traits**: Complete Former ecosystem trait implementations
///
/// ## Single-Field Specialization
///
/// - **Optimized Storage**: Single optional field storage with specialized default handling
/// - **Type-Safe Setter**: Generated setter accepts `impl Into<FieldType>` for maximum flexibility
/// - **Efficient Preform**: Specialized preform logic for single-field variant construction
/// - **Complete Integration**: Full Former trait hierarchy implementation for ecosystem compatibility
///
/// ## Generated Method Signature
/// ```rust
/// impl<T> Enum<T> {
///     pub fn variant() -> VariantFormer<T> { /* ... */ }
/// }
/// ```
///
/// ## Parameters
/// - `ctx`: Mutable context containing variant information, generics, and output collections
///
/// ## Returns
/// - `Ok(TokenStream)`: Generated enum method that returns the single-field variant former
/// - `Err(syn::Error)`: If variant processing fails due to invalid configuration
pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = &ctx.variant.ident;
  let method_name = cased_ident_from_ident(variant_name, Case::Snake);
  let enum_name = ctx.enum_name;
  let vis = ctx.vis;
  let field = &ctx.variant_field_info[0];
  let field_name = &field.ident;
  let field_type = &field.ty;

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
  let variant_former_name = format_ident!("{}{}Former", enum_name, variant_name);
  let variant_former_storage_name = format_ident!("{}{}FormerStorage", enum_name, variant_name);
  let variant_former_definition_name = format_ident!("{}{}FormerDefinition", enum_name, variant_name);
  let variant_former_definition_types_name = format_ident!("{}{}FormerDefinitionTypes", enum_name, variant_name);

  // Generate the storage struct for the variant's fields
  let storage_field_optional = quote! { pub #field_name : ::core::option::Option< #field_type > };
  let storage_field_none = quote! { #field_name : ::core::option::Option::None };
  let storage_field_preform = quote! { let #field_name = self.#field_name.unwrap_or_default(); };
  let storage_field_name = quote! { #field_name };

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
      /// A field
      #storage_field_optional,
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
          #storage_field_none,
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
        #storage_field_preform
        let result = #enum_name::#variant_name { #field_name };
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

      // Setter for the single field
      #[ inline( always ) ]
      pub fn #field_name( mut self, value : impl ::core::convert::Into< #field_type > ) -> Self
      {
        self.storage.#field_name = ::core::option::Option::Some( value.into() );
        self
      }
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
    let constructor_name_str = method_name.to_string();
    let base_name = constructor_name_str.strip_prefix("r#").unwrap_or(&constructor_name_str);
    let standalone_name = format_ident!("{}_variant", base_name);

    let standalone_method = quote!
    {
      #[ inline( always ) ]
      #vis fn #standalone_name() -> #variant_former_name #ty_generics
      {
        #variant_former_name::new( former_types::forming::ReturnPreformed::default() )
      }
    };
    ctx.standalone_constructors.push(standalone_method);
  }

  ctx.end_impls.push(variant_former_code);

  Ok(method)
}
