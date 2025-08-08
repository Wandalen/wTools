//! # Tuple Single-Field Subform Handler - Fixed Implementation
//!
//! This is a FIXED implementation of the tuple single-field subform handler that generates
//! proper variant formers instead of attempting to delegate to EntityToFormer trait.
//! This approach mirrors the working struct_single_field_subform pattern.
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
//! - Mirrors the reliable struct_single_field_subform pattern
//! - Provides indexed setter (._0) for tuple field access
//!
//! ## Generated Infrastructure:
//! - `{Enum}{Variant}FormerStorage`: Storage with `field0: Option< T >`
//! - `{Enum}{Variant}FormerDefinitionTypes`: Type system integration
//! - `{Enum}{Variant}FormerDefinition`: Definition linking all components
//! - `{Enum}{Variant}Former`: Builder with `._0(value)` setter
//! - `{Enum}{Variant}End`: Custom end handler for tuple variant construction

use super::*;

use macro_tools::{ Result, quote::{ quote, format_ident }, ident::cased_ident_from_ident, generic_params::GenericsRef };
use convert_case::Case;

/// Generates implicit variant former infrastructure for single-field tuple enum variants.
///
/// This function creates a complete builder ecosystem for tuple variants with a single unnamed field,
/// implementing the same pattern as struct_single_field_subform but adapted for tuple field access.
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
pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result<  proc_macro2::TokenStream  >
{
  let variant_name = &ctx.variant.ident;
  let method_name = cased_ident_from_ident(variant_name, Case::Snake);
  let enum_name = ctx.enum_name;
  let vis = ctx.vis;
  let field_type = &ctx.variant_field_info[0].ty;

  let ( impl_generics, ty_generics, where_clause ) = ctx.generics.split_for_impl();

  // Generate unique names for the variant former infrastructure
  let variant_name_str = variant_name.to_string();
  let storage_name = format_ident!(\"{}{}FormerStorage\", enum_name, variant_name_str);
  let definition_types_name = format_ident!(\"{}{}FormerDefinitionTypes\", enum_name, variant_name_str);
  let definition_name = format_ident!(\"{}{}FormerDefinition\", enum_name, variant_name_str);
  let former_name = format_ident!(\"{}{}Former\", enum_name, variant_name_str);
  let end_name = format_ident!(\"{}{}End\", enum_name, variant_name_str);

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
      field0 : Option<  #field_type  >,
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
      context : Option<  ()  >,
      on_end : Option<  #end_name #ty_generics  >,
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
      pub fn begin( storage : Option<  #storage_name #ty_generics  >, context : Option<  ()  >, on_end : #end_name #ty_generics ) -> Self
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
        _context : Option<  ()  >,
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

  // Generate the method that returns the implicit variant former
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

  Ok( result )
}