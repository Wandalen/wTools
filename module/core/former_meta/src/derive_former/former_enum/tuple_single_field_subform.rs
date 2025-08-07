//! # Tuple Single-Field Subform Handler - Fixed Implementation
//!
//! This is a FIXED implementation of the tuple single-field subform handler that generates
//! proper variant formers instead of attempting to delegate to `EntityToFormer` trait.
//! This approach mirrors the working `struct_single_field_subform` pattern.
//!
//! ## Key Differences from Original
//!
//! ### Original Problematic Approach:
//! - Attempted to use `< T as EntityToFormer< TFormerDefinition > >::Former`
//! - Failed for primitive types that don't implement Former
//! - Generated non-existent definition types like `u32FormerDefinition`
//! - Required complex Former trait integration
//!
//! ### Fixed Approach:
//! - Generates complete variant former infrastructure (`VariantFormer`)
//! - Works with any field type (primitives, structs, etc.)
//! - Mirrors the reliable `struct_single_field_subform` pattern
//! - Provides indexed setter (._0) for tuple field access
//!
//! ## Generated Infrastructure:
//! - `{Enum}{Variant}FormerStorage`: Storage with `field0: Option<T>`
//! - `{Enum}{Variant}FormerDefinitionTypes`: Type system integration
//! - `{Enum}{Variant}FormerDefinition`: Definition linking all components
//! - `{Enum}{Variant}Former`: Builder with `._0(value)` setter
//! - `{Enum}{Variant}End`: Custom end handler for tuple variant construction
//!
//! ## Known Issues ⚠️
//!
//! **Raw Identifier Bug**: This handler (like others) has a bug with raw identifiers:
//! - Symptom: Panic with "KeywordVariantEnumr#breakFormerStorage" is not a valid identifier
//! - Cause: Direct string concatenation of variant names containing `r#` prefix
//! - Location: Line where `variant_name_str` is used without stripping `r#`
//! - Workaround: Use `raw_identifier_utils::strip_raw_prefix_for_compound_ident()`
//! - Status: Utility functions available but integration needed across all handlers

use super::*;

use macro_tools::{ Result, quote::{ quote, format_ident } };
use crate::derive_former::raw_identifier_utils::variant_to_method_name;

/// Determines if a single-field tuple variant should delegate to the inner type's Former
/// instead of using a variant-specific former.
/// 
/// SAFE DELEGATION CRITERIA:
/// 1. Field type name matches variant name (e.g., `Prompt(Prompt)`)
/// 2. Field type is a simple path (not primitive, not generic)
/// 3. Field type is not a known primitive (String, u32, bool, etc.)
/// 
/// This conservative approach prevents delegation to types that don't implement Former,
/// which would cause derive macro expansion failures.
fn is_delegation_candidate(variant_name: &syn::Ident, field_type: &syn::Type) -> bool {
  // Only attempt delegation for simple path types
  if let syn::Type::Path(type_path) = field_type {
    if let Some(last_segment) = type_path.path.segments.last() {
      let type_name = &last_segment.ident;
      
      // SAFETY CHECK 1: Field type name must match variant name exactly
      if type_name != variant_name {
        return false;
      }
      
      // SAFETY CHECK 2: Reject known primitives that don't implement Former
      let type_str = type_name.to_string();
      let known_primitives = [
        "u8", "u16", "u32", "u64", "u128", "usize",
        "i8", "i16", "i32", "i64", "i128", "isize", 
        "f32", "f64", "bool", "char",
        "String", "str",
        "Vec", "HashMap", "HashSet", "BTreeMap", "BTreeSet",
        "Option", "Result"
      ];
      if known_primitives.contains(&&*type_str) {
        return false;
      }
      
      // SAFETY CHECK 3: Reject generic types (they have angle brackets)
      if last_segment.arguments != syn::PathArguments::None {
        return false;
      }
      
      // SAFETY CHECK 4: Must be a simple single-segment path
      if type_path.path.segments.len() != 1 {
        return false;
      }
      
      // All safety checks passed - attempt delegation
      return true;
    }
  }
  false
}

/// Generates delegation code that returns the inner type's Former.
/// The delegation returns the inner Former directly so that .`form()` returns the inner type,
/// which can then be manually wrapped in the enum variant by the caller.
fn generate_delegated_former(
  ctx: &EnumVariantHandlerContext<'_>,
  _variant_name: &syn::Ident, 
  field_type: &syn::Type,
  method_name: &syn::Ident,
  vis: &syn::Visibility,
) -> proc_macro2::TokenStream {
  quote! {
    // DELEGATION: Return inner type's Former directly
    // The caller will wrap the result in the enum variant manually
    #[ inline( always ) ]
    #vis fn #method_name() -> <#field_type as ::former::Former>::Former
    {
      // Return the inner type's former directly
      // When .form() is called, it returns the inner type (e.g., Prompt)
      // Test code then manually wraps: FunctionStep::Prompt(prompt_step)
      <#field_type as ::former::Former>::former()
    }
  }
}

/// Generates implicit variant former infrastructure for single-field tuple enum variants.
///
/// This function creates a complete builder ecosystem for tuple variants with a single unnamed field,
/// implementing the same pattern as `struct_single_field_subform` but adapted for tuple field access.
///
/// ## Generated Method Signature
/// ```rust,ignore
/// impl<T> Enum<T> {
///     pub fn variant() -> VariantFormer<T> { /* ... */ }
/// }
/// ```
///
/// ## Generated Setter Method
/// ```rust,ignore
/// impl<T> VariantFormer<T> {
///     pub fn _0(self, src: impl Into<T>) -> Self { /* ... */ }
/// }
/// ```
///
/// ## Parameters
/// - `ctx`: Mutable context containing variant information, generics, and output collections
///
/// ## Returns
/// - `Ok(TokenStream)`: Generated enum method that returns the tuple variant former
/// - `Err(syn::Error)`: If variant processing fails due to invalid configuration
#[allow(clippy::too_many_lines)]
pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = &ctx.variant.ident;
  let method_name = variant_to_method_name(variant_name);
  let enum_name = ctx.enum_name;
  let vis = ctx.vis;
  let field_type = &ctx.variant_field_info[0].ty;

  let ( impl_generics, ty_generics, where_clause ) = ctx.generics.split_for_impl();

  // Generate unique names for the variant former infrastructure
  let variant_name_str = crate::derive_former::raw_identifier_utils::strip_raw_prefix_for_compound_ident(variant_name);
  let storage_name = format_ident!("{}{}FormerStorage", enum_name, variant_name_str);
  let definition_types_name = format_ident!("{}{}FormerDefinitionTypes", enum_name, variant_name_str);
  let definition_name = format_ident!("{}{}FormerDefinition", enum_name, variant_name_str);
  let former_name = format_ident!("{}{}Former", enum_name, variant_name_str);
  let end_name = format_ident!("{}{}End", enum_name, variant_name_str);

  // Generate proper PhantomData type based on whether we have generics
  let phantom_data_type = if ctx.generics.type_params().next().is_some() {
    quote! { std::marker::PhantomData< #ty_generics > }
  } else {
    quote! { std::marker::PhantomData< () > }
  };

  // Generate the storage struct and its impls
  let storage_impls = quote!
  {
    pub struct #storage_name #impl_generics
    #where_clause
    {
      field0 : Option< #field_type >,
    }

    impl #impl_generics Default for #storage_name #ty_generics
    #where_clause
    {
      fn default() -> Self
      {
        Self { field0 : None }
      }
    }

    impl #impl_generics former::Storage for #storage_name #ty_generics
    #where_clause
    {
      type Preformed = #field_type;
    }

    impl #impl_generics former::StoragePreform for #storage_name #ty_generics
    where
      #field_type : Default,
    {
      fn preform( mut self ) -> Self::Preformed
      {
        self.field0.take().unwrap_or_default()
      }
    }
  };

  // Generate the DefinitionTypes struct and its impls
  let definition_types_impls = quote!
  {
    #[ derive( Debug ) ]
    pub struct #definition_types_name #impl_generics
    #where_clause
    {
      _p : #phantom_data_type,
    }

    impl #impl_generics Default for #definition_types_name #ty_generics
    #where_clause
    {
      fn default() -> Self
      {
        Self { _p : std::marker::PhantomData }
      }
    }

    impl #impl_generics former::FormerDefinitionTypes for #definition_types_name #ty_generics
    #where_clause
    {
      type Storage = #storage_name #ty_generics;
      type Context = ();
      type Formed = #enum_name #ty_generics;
    }

    impl #impl_generics former::FormerMutator for #definition_types_name #ty_generics
    #where_clause
    {}
  };

  // Generate the Definition struct and its impls
  let definition_impls = quote!
  {
    #[ derive( Debug ) ]
    pub struct #definition_name #impl_generics
    #where_clause
    {
      _p : #phantom_data_type,
    }

    impl #impl_generics Default for #definition_name #ty_generics
    #where_clause
    {
      fn default() -> Self
      {
        Self { _p : std::marker::PhantomData }
      }
    }

    impl #impl_generics former::FormerDefinition for #definition_name #ty_generics
    #where_clause
    {
      type Storage = #storage_name #ty_generics;
      type Context = ();
      type Formed = #enum_name #ty_generics;
      type Types = #definition_types_name #ty_generics;
      type End = #end_name #ty_generics;
    }
  };

  // Generate the Former struct and its impls
  let former_impls = quote!
  {
    pub struct #former_name #impl_generics
    #where_clause
    {
      storage : #storage_name #ty_generics,
      context : Option< () >,
      on_end : Option< #end_name #ty_generics >,
    }

    impl #impl_generics #former_name #ty_generics
    #where_clause
    {
      #[ inline( always ) ]
      pub fn form( self ) -> #enum_name #ty_generics
      {
        self.end()
      }

      #[ inline( always ) ]
      pub fn end( mut self ) -> #enum_name #ty_generics
      {
        let on_end = self.on_end.take().unwrap();
        let context = self.context.take();
        < #definition_types_name #ty_generics as former::FormerMutator >::form_mutation( &mut self.storage, &mut self.context );
        former::FormingEnd::call( &on_end, self.storage, context )
      }

      #[ inline( always ) ]
      pub fn begin( storage : Option< #storage_name #ty_generics >, context : Option< () >, on_end : #end_name #ty_generics ) -> Self
      {
        Self { storage : storage.unwrap_or_default(), context, on_end : Some( on_end ) }
      }

      #[ allow( dead_code ) ]
      #[ inline( always ) ]
      pub fn new( on_end : #end_name #ty_generics ) -> Self
      {
        Self::begin( None, None, on_end )
      }

      #[ inline ]
      pub fn _0( mut self, src : impl Into< #field_type > ) -> Self
      {
        self.storage.field0 = Some( src.into() );
        self
      }
    }
  };

  // Generate the End struct and its impl
  let end_impls = quote!
  {
    #[ derive( Debug ) ]
    pub struct #end_name #impl_generics
    #where_clause
    {}

    impl #impl_generics Default for #end_name #ty_generics
    #where_clause
    {
      fn default() -> Self
      {
        Self {}
      }
    }

    impl #impl_generics former::FormingEnd< #definition_types_name #ty_generics >
    for #end_name #ty_generics
    #where_clause
    {
      #[ inline( always ) ]
      fn call(
        &self,
        sub_storage : #storage_name #ty_generics,
        _context : Option< () >,
      ) -> #enum_name #ty_generics
      {
        let field0 = former::StoragePreform::preform( sub_storage );
        #enum_name :: #variant_name ( field0 )
      }
    }
  };

  // Push all the generated infrastructure to the context
  ctx.end_impls.push( storage_impls );
  ctx.end_impls.push( definition_types_impls );
  ctx.end_impls.push( definition_impls );
  ctx.end_impls.push( former_impls );
  ctx.end_impls.push( end_impls );

  // STABLE APPROACH: Always use variant former (delegation disabled for now)
  // TODO: Implement proper trait detection or compile-time feature detection for delegation
  let result = quote!
  {
    #[ inline( always ) ]
    #vis fn #method_name() -> #former_name #ty_generics
    #where_clause
    {
      #former_name::begin( None, None, #end_name #ty_generics ::default() )
    }
  };

  // Generate standalone constructor if requested
  if ctx.struct_attrs.standalone_constructors.value(false) {
    // Check if the single field has arg_for_constructor - if so, generate scalar standalone constructor
    let field_is_constructor_arg = ctx.variant_field_info[0].is_constructor_arg;
    
    if field_is_constructor_arg {
      // Scalar standalone constructor - takes argument for the field and returns the enum directly
      let field_type = &ctx.variant_field_info[0].ty;
      let field_name = &ctx.variant_field_info[0].ident;
      
      let standalone_method = quote!
      {
        #[ inline( always ) ]
        #vis fn #method_name( #field_name : impl Into< #field_type > ) -> #enum_name #ty_generics
        {
          #enum_name #ty_generics ::#variant_name( #field_name.into() )
        }
      };
      ctx.standalone_constructors.push( standalone_method );
    } else {
      // Subform standalone constructor - returns a Former for building
      let standalone_method = quote!
      {
        #[ inline( always ) ]
        #vis fn #method_name() -> #former_name #ty_generics
        #where_clause
        {
          #former_name::begin( None, None, #end_name #ty_generics ::default() )
        }
      };
      ctx.standalone_constructors.push( standalone_method );
    }
  }

  Ok( result )
}