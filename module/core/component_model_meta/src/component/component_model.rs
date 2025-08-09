//! Component model unified derive macro implementation

use macro_tools::prelude::*;
use macro_tools::attr;

/// Generate `ComponentModel` derive implementation
/// 
/// This macro combines all existing component model derives:
/// - `Assign`: Basic component assignment
/// - `ComponentsAssign`: Multiple component assignment from tuples  
/// - `ComponentFrom`: Create objects from single components
/// - `FromComponents`: Create objects from multiple components
#[allow(clippy::too_many_lines, clippy::manual_let_else, clippy::explicit_iter_loop)]
pub fn component_model( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream, syn::Error >
{
  let parsed = syn::parse::<syn::DeriveInput>( input )?;
  
  // Extract debug attribute if present (Design Rule: Proc Macros Must Have debug Attribute)
  let debug = attr::has_debug( parsed.attrs.iter() )?;
  
  let struct_name = &parsed.ident;
  let generics = &parsed.generics;
  let ( impl_generics, ty_generics, where_clause ) = generics.split_for_impl();
  
  // Only work with structs for now
  let data_struct = match &parsed.data
  {
    syn::Data::Struct( data_struct ) => data_struct,
    _ => return Err( syn_err!( parsed.span(), "ComponentModel can only be applied to structs" ) ),
  };

  // Extract field information
  let fields = match &data_struct.fields
  {
    syn::Fields::Named( fields ) => &fields.named,
    _ => return Err( syn_err!( parsed.span(), "ComponentModel requires named fields" ) ),
  };

  let mut result = proc_macro2::TokenStream::new();

  // Collect unique field types to avoid conflicts
  let mut seen_types = std::collections::HashSet::new();
  let mut unique_fields = Vec::new();
  
  for field in fields.iter()
  {
    let field_type = &field.ty;
    let type_string = quote::quote!( #field_type ).to_string();
    
    if seen_types.insert( type_string )
    {
      unique_fields.push( field );
    }
  }

  // Generate field-specific methods for ALL fields to avoid type ambiguity
  for field in fields.iter()
  {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    
    // Generate field-specific assignment methods to avoid type ambiguity
    let field_name_str = field_name.to_string();
    let clean_field_name = if field_name_str.starts_with("r#") {
      field_name_str.trim_start_matches("r#")
    } else {
      &field_name_str
    };
    let set_method_name = syn::Ident::new( &format!( "{}_set", clean_field_name ), field_name.span() );
    let with_method_name = syn::Ident::new( &format!( "{}_with", clean_field_name ), field_name.span() );
    
    let field_specific_methods = if generics.params.is_empty() {
      quote::quote!
      {
        impl #struct_name
        {
          /// Field-specific setter method to avoid type ambiguity
          #[ inline( always ) ]
          pub fn #set_method_name < IntoT >( &mut self, component : IntoT )
          where
            IntoT : Into< #field_type >
          {
            self.#field_name = component.into();
          }
          
          /// Field-specific builder method for fluent pattern
          #[ inline( always ) ]
          #[ must_use ]
          pub fn #with_method_name < IntoT >( mut self, component : IntoT ) -> Self
          where
            IntoT : Into< #field_type >
          {
            self.#field_name = component.into();
            self
          }
        }
      }
    } else {
      quote::quote!
      {
        impl #impl_generics #struct_name #ty_generics 
        #where_clause
        {
          /// Field-specific setter method to avoid type ambiguity
          #[ inline( always ) ]
          pub fn #set_method_name < IntoT >( &mut self, component : IntoT )
          where
            IntoT : Into< #field_type >
          {
            self.#field_name = component.into();
          }
          
          /// Field-specific builder method for fluent pattern  
          #[ inline( always ) ]
          #[ must_use ]
          pub fn #with_method_name < IntoT >( mut self, component : IntoT ) -> Self
          where
            IntoT : Into< #field_type >
          {
            self.#field_name = component.into();
            self
          }
        }
      }
    };
    
    result.extend( field_specific_methods );
  }

  // Generate Assign implementations only for unique field types to avoid conflicts
  for field in unique_fields.iter()
  {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    
    // Check if this is a popular type that needs special handling
    let _type_str = quote::quote!( #field_type ).to_string();
    let popular_impls = crate::popular_types::generate_popular_type_assigns( 
      struct_name, 
      field_name, 
      field_type,
      generics,
      &impl_generics,
      &ty_generics,
      where_clause
    );
    
    if popular_impls.is_empty()
    {
      // Generate standard Assign implementation using Into trait for non-popular types
      let assign_impl = if generics.params.is_empty() {
        quote::quote!
        {
          impl< IntoT > component_model_types::Assign< #field_type, IntoT > for #struct_name
          where
            IntoT : Into< #field_type >
          {
            #[ inline( always ) ]
            fn assign( &mut self, component : IntoT )
            {
              self.#field_name = component.into();
            }
          }
        }
      } else {
        quote::quote!
        {
          impl< #impl_generics, IntoT > component_model_types::Assign< #field_type, IntoT > for #struct_name #ty_generics
          where
            IntoT : Into< #field_type >,
            #where_clause
          {
            #[ inline( always ) ]
            fn assign( &mut self, component : IntoT )
            {
              self.#field_name = component.into();
            }
          }
        }
      };
      
      result.extend( assign_impl );
    }
    else
    {
      // For popular types, generate specific implementations instead of generic Into
      for impl_tokens in popular_impls
      {
        result.extend( impl_tokens );
      }
    }
  }
  
  // Generate ComponentFrom implementations for unique field types
  for field in unique_fields.iter()
  {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    
    let _component_from_impl = quote::quote!
    {
      impl From< &#struct_name #ty_generics > for #field_type
      where
        #field_type : Clone,
        #where_clause
      {
        #[ inline( always ) ]
        fn from( src : &#struct_name #ty_generics ) -> Self
        {
          src.#field_name.clone()
        }
      }
    };
    
    // For now, skip to avoid conflicts with existing From implementations
    // TODO: Add proper conflict detection and resolution
    // result.extend( component_from_impl );
  }

  if debug
  {
    eprintln!( "Generated ComponentModel implementation:\n{result}" );
  }

  Ok( result )
}