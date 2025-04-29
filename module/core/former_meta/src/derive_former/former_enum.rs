// File: module/core/former_meta/src/derive_former/former_enum.rs
#![ allow( clippy::wildcard_imports ) ]
use super::*;

mod unit;
use unit::handle_unit_variant;

mod tuple_zero;
use tuple_zero::handle_tuple_zero_variant;

mod struct_zero;
use struct_zero::handle_struct_zero_variant;

use macro_tools::
{
  generic_params, Result,
  proc_macro2::TokenStream, quote::{ format_ident, quote },
  ident, // Added for ident_maybe_raw
  phantom, // Added for phantom::tuple
  diag, // Added for report_print
  // punctuated, // Removed unused import
  parse_quote, // Added for parse_quote
};
#[ cfg( feature = "derive_former" ) ]
use convert_case::{ Case, Casing }; // Space before ;


// ==================================
//      Enum Variant Handling Rules (Consistent Logic)
// ==================================
//
// This macro implements the `Former` derive for enums based on the following consistent rules:
//
// 1.  **`#[scalar]` Attribute:**
//     *   **Unit Variant:** Generates `Enum::variant() -> Enum`.
//     *   **Zero-Field Variant Generates `Enum::variant() -> Enum`.
//     *   **Single-Field Variant (Tuple or Struct):** Generates `Enum::variant(InnerType) -> Enum`.
//     *   **Multi-Field Variant (Tuple or Struct):** Generates `Enum::variant(Field1Type, Field2Type, ...) -> Enum`.
//     *   **Error Cases:** Cannot be combined with `#[subform_scalar]`.
//
// 2.  **`#[subform_scalar]` Attribute:**
//     *   **Unit Variant:** Error.
//     *   **Zero-Field Variant (Tuple or Struct):** Error.
//     *   **Single-Field Variant (Tuple or Struct):** Generates `Enum::variant() -> InnerFormer<...>` (where `InnerFormer` is the former for the field's type). Requires the field type to be a path type deriving `Former`.
//     *   **Multi-Field Variant (Tuple or Struct):** Generates `Enum::variant() -> InnerFormer<...>` (where `InnerFormer` is the former for the field's type). Requires the field type to be a path type deriving `Former`.
//
// 3.  **Default Behavior (No Attribute):**
//     *   **Unit Variant:** Generates `Enum::variant() -> Enum`.
//     *   **Zero-Field Variant Generates `Enum::variant() -> Enum`.
//     *   **Single-Field Variant (Tuple or Struct):** Generates `Enum::variant() -> InnerFormer<...>` (where `InnerFormer` is the former for the field's type). Requires the field type to be a path type deriving `Former`.
//     *   **Multi-Field Variant (Tuple or Struct):** Generates `Enum::variant() -> InnerFormer<...>` (where `InnerFormer` is the former for the field's type). Requires the field type to be a path type deriving `Former`.
//
// Body attribute `standalone_constructors` creates stand-alone, top-level constructors for struct/enum. for struct it's always single function, for enum it's as many functions as enum has vartianys.
//
// ==================================

/// Temporary storage for field information needed during generation.
#[derive(Clone)] // <<< Added Clone
struct EnumVariantFieldInfo
{
  // index : usize, // Removed unused field
  ident : syn::Ident,
  ty : syn::Type,
  #[allow(dead_code)] // Keep attrs field even if unused for now
  attrs : FieldAttributes,
  is_constructor_arg : bool,
}

/// Generate the Former ecosystem for an enum.
#[ allow( clippy::too_many_lines ) ]
pub(super) fn former_for_enum
(
  ast : &syn::DeriveInput,
  data_enum : &syn::DataEnum,
  original_input : &proc_macro::TokenStream, // Added original_input
  has_debug : bool, // Added has_debug
) -> Result< TokenStream >
{
  let enum_name = &ast.ident;
  let vis = &ast.vis;
  let generics = &ast.generics;
  let ( _enum_generics_with_defaults, enum_generics_impl, enum_generics_ty, enum_generics_where )
  = generic_params::decompose( generics );

  // --- DEBUG PRINT 1 ---
  // println!( "Former Enum Debug: Processing Enum: {}", enum_name );
  // println!( "  - Generics Impl: {}", quote!{ #enum_generics_impl } );
  // println!( "  - Generics Ty: {}", quote!{ #enum_generics_ty } );
  // println!( "  - Generics Where: {}", quote!{ #enum_generics_where } );
  // --- END DEBUG PRINT 1 ---


  // Parse struct-level attributes
  let struct_attrs = ItemAttributes::from_attrs( ast.attrs.iter() )?;

  // Initialize vectors to collect generated code pieces
  let mut methods = Vec::new();
  let mut end_impls = Vec::new();
  let mut standalone_constructors = Vec::new(); // <<< Vector to store standalone constructors

  // Iterate through each variant of the enum
  for variant in &data_enum.variants
  {
    let variant_ident = &variant.ident;

    // --- DEBUG PRINT 2 ---
    // println!( "Former Enum Debug: Processing Variant: {}", variant_ident );
    // --- END DEBUG PRINT 2 ---


    // Generate the snake_case method name, handling potential keywords
    let variant_name_str = variant_ident.to_string();
    let method_name_snake_str = variant_name_str.to_case( Case::Snake );
    let method_name_ident_temp = format_ident!( "{}", method_name_snake_str, span = variant_ident.span() );
    let method_name = ident::ident_maybe_raw( &method_name_ident_temp );

    // Parse attributes *from the variant* itself
    let variant_attrs = FieldAttributes::from_attrs( variant.attrs.iter() )?;
    let wants_scalar = variant_attrs.scalar.is_some() && variant_attrs.scalar.as_ref().unwrap().setter();
    let wants_subform_scalar = variant_attrs.subform_scalar.is_some();

    // --- Prepare merged where clause for this variant's generated impls ---
    let merged_where_clause = enum_generics_where.clone();

    // <<< Added: Collect detailed field info for the current variant >>>
    let variant_field_info: Vec<EnumVariantFieldInfo> = match &variant.fields {
        syn::Fields::Named(f) => f.named.iter().enumerate().map(|(_index, field)| { // <<< Use _index
            let attrs = FieldAttributes::from_attrs(field.attrs.iter())?;
            let is_constructor_arg = attrs.arg_for_constructor.value(false);
            Ok(EnumVariantFieldInfo {
                // index, // Removed assignment to unused field
                ident: field.ident.clone().ok_or_else(|| syn::Error::new_spanned(field, "Named field requires an identifier"))?,
                ty: field.ty.clone(),
                attrs, // Store parsed field attributes
                is_constructor_arg,
            })
        }).collect::<Result<_>>()?,
        syn::Fields::Unnamed(f) => f.unnamed.iter().enumerate().map(|(index, field)| {
            let attrs = FieldAttributes::from_attrs(field.attrs.iter())?;
            let is_constructor_arg = attrs.arg_for_constructor.value(false);
            Ok(EnumVariantFieldInfo {
                // index, // Removed assignment to unused field
                ident: format_ident!("_{}", index), // Synthesize identifier - Note: still uses index here!
                ty: field.ty.clone(),
                attrs, // Store parsed field attributes
                is_constructor_arg,
            })
        }).collect::<Result<_>>()?,
        syn::Fields::Unit => vec![],
    };
    // <<< End Added >>>


    // Generate method based on the variant's fields
    match &variant.fields
    {
      // Case 1: Unit variant
      syn::Fields::Unit =>
      {
        handle_unit_variant
        (
          ast,
          variant,
          &struct_attrs,
          enum_name,
          vis,
          generics,
          original_input,
          has_debug,
          &mut methods,
          &mut end_impls,
          &mut standalone_constructors,
          &variant_attrs,
          &variant_field_info,
          &merged_where_clause,
        )?;
      },
      // Case 2: Tuple variant
      syn::Fields::Unnamed( fields ) =>
      {
        // ... (Tuple variant logic - unchanged) ...
        // --- DEBUG PRINT 3b ---
        // println!( "Former Enum Debug: Variant {} - Unnamed Case ({} fields)", variant_ident, fields.unnamed.len() );
        // --- END DEBUG PRINT 3b ---

        if variant_attrs.arg_for_constructor.value( false )
        {
          return Err( syn::Error::new_spanned( variant, "#[arg_for_constructor] cannot be applied directly to an enum variant identifier. Apply it to the fields *within* the variant instead, e.g., `MyVariant( #[arg_for_constructor] i32 )`." ) );
        }

        match fields.unnamed.len()
        {
          // Sub-case: Zero fields (treat like Unit variant)
          0 =>
          {
            handle_tuple_zero_variant
            (
              ast,
              variant,
              &struct_attrs,
              enum_name,
              vis,
              generics,
              original_input,
              has_debug,
              &mut methods,
              &mut end_impls,
              &mut standalone_constructors,
              &variant_attrs,
              &variant_field_info,
              &merged_where_clause,
            )?;
          }
          // Sub-case: Single field tuple variant
          1 =>
          {
            let field_info = &variant_field_info[0]; // Get the collected info
            let inner_type = &field_info.ty;
            // let _field_attrs = &field_info.attrs; // <<< Use parsed attrs from field_info (Marked unused for now)

            // Determine behavior based on attributes
            if wants_scalar
            {
              // --- Scalar Tuple(1) Variant ---
              // --- Standalone Constructor (Scalar Tuple(1)) ---
              if struct_attrs.standalone_constructors.value( false )
              {
                // <<< Use collected info to generate params and COLLECT >>>
                let constructor_params : Vec<_> = variant_field_info
                  .iter()
                  .filter( |f_info| f_info.is_constructor_arg )
                  .map( |f_info| {
                    let param_name = &f_info.ident;
                    let ty = &f_info.ty;
                    quote! { #param_name : impl Into< #ty > }
                  })
                  .collect(); // <<< Added collect()
                // <<< End Use >>>

                // <<< Determine Return Type (Option 2) >>>
                let all_fields_are_args = !variant_field_info.is_empty() && variant_field_info.iter().all( |f| f.is_constructor_arg );
                let return_type = if all_fields_are_args
                {
                  quote! { #enum_name< #enum_generics_ty > } // Return Self
                }
                else
                {
                  // This case shouldn't happen for scalar single-field, but handle defensively
                  return Err( syn::Error::new_spanned( variant, "#[scalar] on single-field variant implies all fields are constructor args, but #[arg_for_constructor] is missing." ) );
                };
                // <<< End Determine >>>

                let mut direct_construction_args = Vec::new(); // For returning Self
                for field_info_inner in &variant_field_info
                {
                  let param_name = &field_info_inner.ident;
                  direct_construction_args.push( quote! { #param_name.into() } ); // For Self construction
                }

                let constructor = quote!
                {
                  /// Standalone constructor for the #variant_ident variant (scalar style).
                  #[ inline( always ) ]
                  #vis fn #method_name < #enum_generics_impl >
                  ( // Paren on new line
                    #( #constructor_params ),* // <<< Use generated params
                  ) // Paren on new line
                  -> // Return type on new line
                  #return_type // <<< Use determined return type
                  where // Where clause on new line
                    #enum_generics_where
                  { // Brace on new line
                    Self::#variant_ident( #( #direct_construction_args ),* )
                  } // Brace on new line
                };
                standalone_constructors.push( constructor );
              }
              // --- End Standalone Constructor ---

              // Associated method (returns Self directly for scalar)
              let param_name = format_ident!( "_0" );
              let static_method = quote!
              {
                /// Constructor for the #variant_ident variant (scalar style).
                #[ inline( always ) ]
                #vis fn #method_name( #param_name : impl Into< #inner_type > ) -> Self
                {
                  Self::#variant_ident( #param_name.into() )
                }
              };
              methods.push( static_method );
            }
            else // Default or explicit subform_scalar -> Generate Subformer
            {
              // --- Subform Tuple(1) Variant ---
              if wants_subform_scalar
              {
                // Check if inner type is a path type, required for subform_scalar
                if !matches!( inner_type, syn::Type::Path( _ ) )
                {
                  return Err( syn::Error::new_spanned( inner_type, "#[subform_scalar] can only be applied to variants holding a path type (e.g., MyStruct, Option<T>), not tuples, references, etc." ) );
                }
              }
              // If !wants_scalar and !wants_subform_scalar, it's the default case, which is subformer.
              else // Default case requires path type check as well
              {
                 if !matches!( inner_type, syn::Type::Path( _ ) )
                 {
                   return Err( syn::Error::new_spanned( inner_type, "Default subforming requires the single field of a tuple variant to be a path type (e.g., MyStruct, Option<T>)." ) );
                 }
              }

              let end_struct_name = format_ident!( "{}{}End", enum_name, variant_ident );
              let ( inner_type_name, inner_generics ) = match inner_type { syn::Type::Path( tp ) => { let s = tp.path.segments.last().unwrap(); ( s.ident.clone(), s.arguments.clone() ) }, _ => unreachable!() }; // Already checked path type
              let inner_former_name = format_ident!( "{}Former", inner_type_name );
              let inner_storage_name = format_ident!( "{}FormerStorage", inner_type_name );
              let inner_def_name = format_ident!( "{}FormerDefinition", inner_type_name );
              let inner_def_types_name = format_ident!( "{}FormerDefinitionTypes", inner_type_name );
              let inner_generics_ty : syn::punctuated::Punctuated<_,_> = match &inner_generics { syn::PathArguments::AngleBracketed( args ) => args.args.clone(), _ => syn::punctuated::Punctuated::default() };
              let inner_generics_ty_comma = if inner_generics_ty.is_empty() { quote!{} } else { quote!{ #inner_generics_ty, } };

              // --- Standalone Constructor (Subform Tuple(1)) ---
              if struct_attrs.standalone_constructors.value( false )
              {
                // <<< Use collected info to generate params and COLLECT >>>
                let constructor_params : Vec<_> = variant_field_info
                  .iter()
                  .filter( |f_info| f_info.is_constructor_arg )
                  .map( |f_info| {
                    let param_name = &f_info.ident;
                    let ty = &f_info.ty;
                    quote! { #param_name : impl Into< #ty > }
                  })
                  .collect(); // <<< Added collect()
                // <<< End Use >>>

                // <<< Determine Return Type (Option 2) >>>
                let all_fields_are_args = !variant_field_info.is_empty() && variant_field_info.iter().all( |f| f.is_constructor_arg );
                let return_type = if all_fields_are_args
                {
                  quote! { #enum_name< #enum_generics_ty > } // Return Self
                }
                else
                {
                  // Return Inner Former
                  quote!
                  {
                    #inner_former_name
                    <
                      #inner_generics_ty_comma // Inner type generics
                      #inner_def_name // Inner definition
                      <
                        #inner_generics_ty_comma // Inner type generics
                        (), // Context
                        #enum_name< #enum_generics_ty >, // Formed
                        #end_struct_name < #enum_generics_ty > // End
                      >
                    >
                  }
                };
                // <<< End Determine >>>

                // Initialize storage only if there's an argument
                let initial_storage_code = if field_info.is_constructor_arg // <<< Use field_info here
                {
                  let param_name = format_ident!( "_0" );
                  // Assume storage field is also named _0 for tuple variants
                  quote!
                  {
                    ::core::option::Option::Some
                    (
                      #inner_storage_name :: < #inner_generics_ty > // Add generics if inner type has them
                      {
                        _0 : ::core::option::Option::Some( #param_name.into() ),
                        // Add _phantom if needed by storage
                      }
                    )
                  }
                } else { quote! { ::core::option::Option::None } };

                let constructor = quote!
                {
                  /// Standalone constructor for the #variant_ident subform variant.
                  #[ inline( always ) ]
                  #vis fn #method_name < #enum_generics_impl >
                  ( // Paren on new line
                    #( #constructor_params ),* // <<< Use generated params
                  ) // Paren on new line
                  -> // Return type on new line
                  #return_type // <<< Use determined return type
                  where // Where clause on new line
                    #enum_generics_where
                  { // Brace on new line
                    // <<< Logic to return Self or Former needs to be added in Increment 3d >>>
                    #inner_former_name::begin // Placeholder: assumes returns Former for now
                    (
                      #initial_storage_code,
                      None, // Context
                      #end_struct_name::< #enum_generics_ty >::default() // End
                    )
                  } // Brace on new line
                };
                standalone_constructors.push( constructor );
              }
              // --- End Standalone Constructor ---

              // Associated method logic (remains the same)
              let phantom_field_type = phantom::tuple( &enum_generics_ty );
              let end_struct_def = quote!
              {
                #[ derive( Default, Debug ) ]
                #vis struct #end_struct_name < #enum_generics_impl >
                where // Where clause on new line
                  #merged_where_clause
                { // Brace on new line
                  _phantom : #phantom_field_type,
                } // Brace on new line
              };
              let end_impl = quote!
              {
                #[ automatically_derived ]
                impl< #enum_generics_impl > former::FormingEnd
                <
                  #inner_def_types_name< #inner_generics_ty_comma (), #enum_name< #enum_generics_ty > >
                >
                for #end_struct_name < #enum_generics_ty >
                where // Where clause on new line
                  #merged_where_clause
                { // Brace on new line
                  #[ inline( always ) ]
                  fn call
                  ( // Paren on new line
                    &self,
                    sub_storage : #inner_storage_name< #inner_generics_ty >,
                    _context : Option< () >,
                  ) // Paren on new line
                  -> // Return type on new line
                  #enum_name< #enum_generics_ty >
                  { // Brace on new line
                    let data = former::StoragePreform::preform( sub_storage );
                    #enum_name::#variant_ident( data )
                  } // Brace on new line
                } // Brace on new line
              };
              let static_method = quote!
              {
                /// Starts forming the #variant_ident variant using a subformer.
                #[ inline( always ) ]
                #vis fn #method_name ()
                -> // Return type on new line
                #inner_former_name
                <
                  #inner_generics_ty_comma
                  #inner_def_name
                  <
                    #inner_generics_ty_comma (), #enum_name< #enum_generics_ty >, #end_struct_name < #enum_generics_ty >
                  >
                >
                { // Brace on new line
                  #inner_former_name::begin( None, None, #end_struct_name::< #enum_generics_ty >::default() )
                } // Brace on new line
              };
              methods.push( static_method );
              end_impls.push( quote!{ #end_struct_def #end_impl } );
            }
          }
          // Sub-case: Multi-field tuple variant
          _ => // len > 1
          {
            // <<< Start: Corrected logic for multi-field tuple variants >>>
            if wants_subform_scalar
            {
              return Err( syn::Error::new_spanned( variant, "#[subform_scalar] cannot be used on tuple variants with multiple fields." ) );
            }
            else if wants_scalar
            {
              // --- Scalar Tuple(N) Variant ---
              // --- Standalone Constructor (Scalar Tuple(N)) ---
              if struct_attrs.standalone_constructors.value( false )
              {
                // <<< Use collected info to generate params and COLLECT >>>
                let constructor_params : Vec<_> = variant_field_info
                  .iter()
                  // .filter( |f_info| f_info.is_constructor_arg ) // All fields are args for scalar
                  .map( |f_info| {
                    let param_name = &f_info.ident;
                    let ty = &f_info.ty;
                    quote! { #param_name : impl Into< #ty > }
                  })
                  .collect(); // <<< Added collect()
                // <<< End Use >>>

                // <<< Determine Return Type (Option 2) >>>
                // For scalar variants, all fields are implicitly constructor args, so always return Self
                let return_type = quote! { #enum_name< #enum_generics_ty > };
                // <<< End Determine >>>

                let mut direct_construction_args = Vec::new(); // For returning Self
                for field_info_inner in &variant_field_info
                {
                  let param_name = &field_info_inner.ident;
                  direct_construction_args.push( quote! { #param_name.into() } ); // For Self construction
                }

                let constructor = quote!
                {
                  /// Standalone constructor for the #variant_ident variant with multiple fields (scalar style).
                  #[ inline( always ) ]
                  #vis fn #method_name < #enum_generics_impl >
                  ( // Paren on new line
                    #( #constructor_params ),* // <<< Use generated params
                  ) // Paren on new line
                  -> // Return type on new line
                  #return_type // <<< Use determined return type
                  where // Where clause on new line
                    #enum_generics_where
                  { // Brace on new line
                    Self::#variant_ident( #( #direct_construction_args ),* )
                  } // Brace on new line
                };
                standalone_constructors.push( constructor );
              }
              // --- End Standalone Constructor ---

              // Associated method (returns Self directly)
              let mut params = Vec::new();
              let mut args = Vec::new();
              for field_info in &variant_field_info
              {
                let param_name = &field_info.ident;
                let field_type = &field_info.ty;
                params.push( quote! { #param_name : impl Into< #field_type > } );
                args.push( quote! { #param_name.into() } );
              }
              let static_method = quote!
              {
                /// Constructor for the #variant_ident variant with multiple fields (scalar style).
                #[ inline( always ) ]
                #vis fn #method_name
                ( // Paren on new line
                  #( #params ),*
                ) // Paren on new line
                -> Self
                { // Brace on new line
                  Self::#variant_ident( #( #args ),* )
                } // Brace on new line
              };
              methods.push( static_method );
              // No implicit former components needed for direct constructor
            }
            else // Default: Error
            {
              return Err( syn::Error::new_spanned( variant, "Former derive requires `#[scalar]` attribute for tuple variants with multiple fields." ) );
            }
            // <<< End: Corrected logic for multi-field tuple variants >>>
          }
        }
      },
      // Case 3: Struct variant
      syn::Fields::Named( fields ) => // <<< Use fields variable >>>
      {
        // --- DEBUG PRINT 3c ---
        // println!( "Former Enum Debug: Variant {} - Named Case ({} fields)", variant_ident, fields.named.len() );
        // --- END DEBUG PRINT 3c ---

        if variant_attrs.arg_for_constructor.value( false )
        {
          return Err( syn::Error::new_spanned( variant, "#[arg_for_constructor] cannot be applied directly to an enum variant identifier. Apply it to the fields *within* the variant instead, e.g., `MyVariant { #[arg_for_constructor] field : i32 }`." ) );
        }

        // <<< Start: Logic for Named Fields (Struct-like Variants) >>>
        println!( "DEBUG: Processing Named fields for variant: {}", variant.ident ); // Debug print
        match fields.named.len()
        {
            // Sub-case: Zero fields (Struct(0))
            0 =>
            {
                println!( "DEBUG: Calling handle_struct_zero_variant for variant: {}", variant.ident ); // Debug print
                handle_struct_zero_variant
                (
                    ast,
                    variant,
                    &struct_attrs,
                    enum_name,
                    vis,
                    generics,
                    original_input,
                    has_debug,
                    &mut methods,
                    &mut end_impls,
                    &mut standalone_constructors,
                    &variant_attrs,
                    &variant_field_info,
                    &merged_where_clause,
                )?;
            }
            // Sub-case: Single field (Struct(1))
            1 =>
            {
                let field_info = &variant_field_info[0];
                let inner_type = &field_info.ty;

                if wants_scalar
                {
                    // --- Scalar Struct(1) Variant ---
                    // --- Standalone Constructor (Scalar Struct(1)) ---
                     if struct_attrs.standalone_constructors.value( false )
                     {
                        let constructor_params : Vec<_> = variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
                        let all_fields_are_args = !variant_field_info.is_empty() && variant_field_info.iter().all( |f| f.is_constructor_arg );
                        let return_type = if all_fields_are_args { quote! { #enum_name< #enum_generics_ty > } } else { /* Should error if not all args */ return Err( syn::Error::new_spanned( variant, "#[scalar] on single-field variant implies all fields are constructor args, but #[arg_for_constructor] is missing." ) ); };
                        let field_ident = &field_info.ident;
                        let param_name = ident::ident_maybe_raw( field_ident );
                        let constructor = quote!
                        {
                            /// Standalone constructor for the #variant_ident variant (scalar style).
                            #[ inline( always ) ]
                            #vis fn #method_name < #enum_generics_impl > ( #( #constructor_params ),* ) -> #return_type where #enum_generics_where
                            { Self::#variant_ident { #field_ident : #param_name.into() } }
                        };
                        standalone_constructors.push( constructor );
                     }
                    // --- End Standalone Constructor ---

                    // Associated method (direct constructor)
                    let field_ident = &field_info.ident;
                    let param_name = ident::ident_maybe_raw( field_ident );
                    let static_method = quote!
                    {
                        /// Constructor for the #variant_ident variant (scalar style).
                        #[ inline( always ) ]
                        #vis fn #method_name( #param_name : impl Into< #inner_type > ) -> Self
                        { Self::#variant_ident { #field_ident : #param_name.into() } }
                    };
                    methods.push( static_method );
                }
                else // Default or explicit subform_scalar -> Generate Subformer
                {
                    // --- Subform Struct(1) Variant ---
                    if wants_subform_scalar
                    {
                        if !matches!( inner_type, syn::Type::Path( _ ) ) { return Err( syn::Error::new_spanned( inner_type, "#[subform_scalar] can only be applied to variants holding a path type (e.g., MyStruct, Option<T>), not tuples, references, etc." ) ); }
                    }
                    else // Default case
                    {
                        if !matches!( inner_type, syn::Type::Path( _ ) ) { return Err( syn::Error::new_spanned( inner_type, "Default subforming requires the single field of a struct-like variant to be a path type (e.g., MyStruct, Option<T>)." ) ); }
                    }

                    let end_struct_name = format_ident!( "{}{}End", enum_name, variant_ident );
                    let ( inner_type_name, inner_generics ) = match inner_type { syn::Type::Path( tp ) => { let s = tp.path.segments.last().unwrap(); ( s.ident.clone(), s.arguments.clone() ) }, _ => unreachable!() };
                    let inner_former_name = format_ident!( "{}Former", inner_type_name );
                    let inner_storage_name = format_ident!( "{}FormerStorage", inner_type_name );
                    let inner_def_name = format_ident!( "{}FormerDefinition", inner_type_name );
                    let inner_def_types_name = format_ident!( "{}FormerDefinitionTypes", inner_type_name );
                    let inner_generics_ty : syn::punctuated::Punctuated<_,_> = match &inner_generics { syn::PathArguments::AngleBracketed( args ) => args.args.clone(), _ => syn::punctuated::Punctuated::default() };
                    let inner_generics_ty_comma = if inner_generics_ty.is_empty() { quote!{} } else { quote!{ #inner_generics_ty, } };

                    // --- Standalone Constructor (Subform Struct(1)) ---
                    if struct_attrs.standalone_constructors.value( false )
                    {
                        let constructor_params : Vec<_> = variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
                        let all_fields_are_args = !variant_field_info.is_empty() && variant_field_info.iter().all( |f| f.is_constructor_arg );
                        let return_type = if all_fields_are_args { quote! { #enum_name< #enum_generics_ty > } } else { quote! { #inner_former_name < #inner_generics_ty_comma #inner_def_name < #inner_generics_ty_comma (), #enum_name< #enum_generics_ty >, #end_struct_name < #enum_generics_ty > > > } };
                        let initial_storage_code = if field_info.is_constructor_arg { let fi = &field_info.ident; let pn = ident::ident_maybe_raw( fi ); quote! { ::core::option::Option::Some( #inner_storage_name :: < #inner_generics_ty > { #fi : ::core::option::Option::Some( #pn.into() ) } ) } } else { quote! { ::core::option::Option::None } };
                        let constructor = quote!
                        {
                            /// Standalone constructor for the #variant_ident subform variant.
                            #[ inline( always ) ]
                            #vis fn #method_name < #enum_generics_impl >
                            ( // Paren on new line
                              #( #constructor_params ),*
                            ) // Paren on new line
                            -> // Return type on new line
                            #return_type
                            where // Where clause on new line
                              #enum_generics_where
                            { // Brace on new line
                              #inner_former_name::begin
                              (
                                #initial_storage_code,
                                None, // Context
                                #end_struct_name::< #enum_generics_ty >::default() // End
                              )
                            } // Brace on new line
                        };
                        standalone_constructors.push( constructor );
                    }
                    // --- End Standalone Constructor ---

                    // Associated method logic
                    let phantom_field_type = phantom::tuple( &enum_generics_ty );
                    let field_ident = &field_info.ident; // Get the single field's ident
                    let end_struct_def = quote!
                    {
                      #[ derive( Default, Debug ) ]
                      #vis struct #end_struct_name < #enum_generics_impl >
                      where // Where clause on new line
                        #merged_where_clause
                      { // Brace on new line
                        _phantom : #phantom_field_type,
                      } // Brace on new line
                    };
                    let end_impl = quote!
                    {
                      #[ automatically_derived ]
                      impl< #enum_generics_impl > former::FormingEnd
                      <
                        #inner_def_types_name< #inner_generics_ty_comma (), #enum_name< #enum_generics_ty > >
                      >
                      for #end_struct_name < #enum_generics_ty >
                      where // Where clause on new line
                        #merged_where_clause
                      { // Brace on new line
                        #[ inline( always ) ]
                        fn call
                        ( // Paren on new line
                          &self,
                          sub_storage : #inner_storage_name< #inner_generics_ty >,
                          _context : Option< () >,
                        ) // Paren on new line
                        -> // Return type on new line
                        #enum_name< #enum_generics_ty >
                        { // Brace on new line
                          let data = former::StoragePreform::preform( sub_storage );
                          #enum_name::#variant_ident{ #field_ident : data } // Construct struct variant
                        } // Brace on new line
                      } // Brace on new line
                    };
                    let static_method = quote!
                    {
                      /// Starts forming the #variant_ident variant using a subformer (default behavior).
                      #[ inline( always ) ]
                      #vis fn #method_name ()
                      -> // Return type on new line
                      #inner_former_name
                      <
                        #inner_generics_ty_comma
                        #inner_def_name
                        <
                          #inner_generics_ty_comma (), #enum_name< #enum_generics_ty >, #end_struct_name < #enum_generics_ty >
                        >
                      >
                      { // Brace on new line
                        #inner_former_name::begin( None, None, #end_struct_name::< #enum_generics_ty >::default() )
                      } // Brace on new line
                    };
                    methods.push( static_method );
                    end_impls.push( quote!{ #end_struct_def #end_impl } );
                }
            }
            // Sub-case: Multi-field (Struct(N))
            _ => // len > 1
            {
                // --- DEBUG PRINT 3d ---
                // println!( "Former Enum Debug: Variant {} - Named Case ({} fields) - Subformer Path", variant_ident, fields.named.len() );
                // --- END DEBUG PRINT 3d ---

                if wants_subform_scalar
                {
                    return Err( syn::Error::new_spanned( variant, "#[subform_scalar] cannot be used on struct-like variants with multiple fields." ) );
                }
                else if wants_scalar
                {
                    // --- Scalar Struct(N) Variant ---
                    // --- Standalone Constructor (Scalar Struct(N)) ---
                    if struct_attrs.standalone_constructors.value( false )
                    {
                        let constructor_params : Vec<_> = variant_field_info.iter().map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
                        let return_type = quote! { #enum_name< #enum_generics_ty > };
                        let direct_construction_args = variant_field_info.iter().map( |f| { let fi = &f.ident; let pn = ident::ident_maybe_raw( fi ); quote! { #fi : #pn.into() } } );
                        let constructor = quote!
                        {
                            /// Standalone constructor for the #variant_ident struct variant (scalar style).
                            #[ inline( always ) ]
                            #vis fn #method_name < #enum_generics_impl > ( #( #constructor_params ),* ) -> #return_type where #enum_generics_where
                            { Self::#variant_ident { #( #direct_construction_args ),* } }
                        };
                        standalone_constructors.push( constructor );
                    }
                    // --- End Standalone Constructor ---

                    // Associated method (direct constructor)
                    let mut params = Vec::new();
                    let mut args = Vec::new();
                    for field_info in &variant_field_info
                    {
                        let field_ident = &field_info.ident;
                        let param_name = ident::ident_maybe_raw( field_ident );
                        let field_type = &field_info.ty;
                        params.push( quote! { #param_name : impl Into< #field_type > } );
                        args.push( quote! { #field_ident : #param_name.into() } );
                    }
                    let static_method = quote!
                    {
                        /// Constructor for the #variant_ident struct variant (scalar style).
                        #[ inline( always ) ]
                        #vis fn #method_name ( #( #params ),* ) -> Self
                        { Self::#variant_ident { #( #args ),* } }
                    };
                    methods.push( static_method );
                }
                else // Default: Subformer
                {
                    // --- Subform Struct(N) Variant ---
                    // Generate implicit former ecosystem for this variant

                    // Storage struct name: EnumNameVariantNameFormerStorage
                    let storage_struct_name = format_ident!( "{}{}FormerStorage", enum_name, variant_ident );
                    // DefinitionTypes struct name
                    let def_types_name = format_ident!( "{}{}FormerDefinitionTypes", enum_name, variant_ident );
                    // Definition struct name
                    let def_name = format_ident!( "{}{}FormerDefinition", enum_name, variant_ident );
                    // End struct name
                    let end_struct_name = format_ident!( "{}{}End", enum_name, variant_ident );
                    // Former struct name
                    let former_name = format_ident!( "{}{}Former", enum_name, variant_ident );

                    // --- Generate Storage --- (Increment 1)
                    let phantom_field_type = phantom::tuple( &enum_generics_ty );
                    let storage_fields = variant_field_info.iter().map( |f_info|
                    {
                      let field_ident = &f_info.ident;
                      let field_type = &f_info.ty;
                      quote! { pub #field_ident : ::core::option::Option< #field_type > }
                    });
                    let default_assignments = variant_field_info.iter().map( |f_info|
                    {
                      let field_ident = &f_info.ident;
                      quote! { #field_ident : ::core::option::Option::None }
                    });
                    let storage_def = quote!
                    {
                      #[ doc = "Storage for the implicit former of the #variant_ident variant." ]
                      #[ allow( explicit_outlives_requirements ) ] // qqq : check if needed
                      #vis struct #storage_struct_name < #enum_generics_impl >
                      where // Where clause on new line
                        #merged_where_clause
                      { // Brace on new line
                        #( #storage_fields, )*
                        _phantom : #phantom_field_type,
                      } // Brace on new line
                    };
                    let storage_default_impl = quote!
                    {
                      impl< #enum_generics_impl > ::core::default::Default
                      for #storage_struct_name < #enum_generics_ty >
                      where // Where clause on new line
                        #merged_where_clause
                      { // Brace on new line
                        #[ inline( always ) ]
                        fn default() -> Self
                        { // Brace on new line
                          Self
                          { // Brace on new line
                            #( #default_assignments, )*
                            _phantom : ::core::marker::PhantomData,
                          } // Brace on new line
                        } // Brace on new line
                      } // Brace on new line
                    };

                    // --- Generate Storage Impls --- (Increment 2)
                    let field_types = variant_field_info.iter().map( |f_info| &f_info.ty );
                    let storage_trait_impl = quote!
                    {
                      impl< #enum_generics_impl > former::Storage
                      for #storage_struct_name < #enum_generics_ty >
                      where // Where clause on new line
                        #merged_where_clause
                      { // Brace on new line
                        type Preformed = ( #( #field_types ),* ); // Preformed type is a tuple of field types
                      } // Brace on new line
                    };
                    let preform_field_assignments = variant_field_info.iter().map( |f_info|
                    {
                      let field_ident = &f_info.ident;
                      let field_type = &f_info.ty;
                      quote!
                      {
                        if self.#field_ident.is_some()
                        {
                          self.#field_ident.take().unwrap()
                        }
                        else
                        {
                          {
                            trait MaybeDefault< T > { fn maybe_default( self : &Self ) -> T { panic!( "Field '{}' isn't initialized", stringify!( #field_ident ) ) } }
                            impl< T > MaybeDefault< T > for &::core::marker::PhantomData< T > {}
                            impl< T > MaybeDefault< T > for ::core::marker::PhantomData< T > where T : ::core::default::Default { fn maybe_default( self : &Self ) -> T { T::default() } }
                            ( &::core::marker::PhantomData::< #field_type > ).maybe_default()
                          }
                        }
                      }
                    });
                    let preformed_tuple_elements_vec : Vec<_> = variant_field_info.iter().map( |f_info|
                    {
                      let field_ident = &f_info.ident;
                      quote! { #field_ident }
                    }).collect();
                    let storage_preform_impl = quote!
                    {
                      impl< #enum_generics_impl > former::StoragePreform
                      for #storage_struct_name < #enum_generics_ty >
                      where // Where clause on new line
                        #merged_where_clause
                      { // Brace on new line
                        fn preform( mut self ) -> Self::Preformed
                        { // Brace on new line
                          #( let #preformed_tuple_elements_vec = #preform_field_assignments; )*
                          ( #( #preformed_tuple_elements_vec ),* ) // Return the tuple
                        } // Brace on new line
                      } // Brace on new line
                    };

                    // --- Generate DefinitionTypes --- (Increment 3)
                    let def_types_generics_impl = generic_params::merge( &generics, &parse_quote!{ < Context2 = (), Formed2 = #enum_name< #enum_generics_ty > > } );
                    let ( _def_types_generics_with_defaults, def_types_generics_impl, def_types_generics_ty, def_types_generics_where ) = generic_params::decompose( &def_types_generics_impl );
                    let def_types_phantom = phantom::tuple( &def_types_generics_impl );
                    let def_types_struct = quote!
                    {
                      #[ derive( Debug ) ]
                      #vis struct #def_types_name < #def_types_generics_impl >
                      where // Where clause on new line
                        #def_types_generics_where
                      { // Brace on new line
                        _phantom : #def_types_phantom,
                      } // Brace on new line
                    };
                    let def_types_default_impl = quote!
                    {
                      impl< #def_types_generics_impl > ::core::default::Default
                      for #def_types_name < #def_types_generics_ty >
                      where // Where clause on new line
                        #def_types_generics_where
                      { // Brace on new line
                        fn default() -> Self
                        { // Brace on new line
                          Self { _phantom : ::core::marker::PhantomData }
                        } // Brace on new line
                      } // Brace on new line
                    };
                    let def_types_former_impl = quote!
                    {
                      impl< #def_types_generics_impl > former::FormerDefinitionTypes
                      for #def_types_name < #def_types_generics_ty >
                      where // Where clause on new line
                        #def_types_generics_where
                      { // Brace on new line
                        type Storage = #storage_struct_name< #enum_generics_ty >;
                        type Context = Context2;
                        type Formed = Formed2;
                      } // Brace on new line
                    };
                    let def_types_mutator_impl = quote!
                    {
                      impl< #def_types_generics_impl > former::FormerMutator
                      for #def_types_name < #def_types_generics_ty >
                      where // Where clause on new line
                        #def_types_generics_where
                      { // Brace on new line
                        // Default empty mutator
                      } // Brace on new line
                    };

                    // --- Generate Definition --- (Increment 4)
                    let enum_generics_ty_no_comma = { let mut ty = enum_generics_ty.clone(); ty.pop_punct(); ty };
                    let def_generics_impl = generic_params::merge( &generics, &parse_quote!{ < Context2 = (), Formed2 = #enum_name< #enum_generics_ty >, End2 = #end_struct_name< #enum_generics_ty > > } );
                    let ( _def_generics_with_defaults, def_generics_impl, def_generics_ty, def_generics_where ) = generic_params::decompose( &def_generics_impl );
                    let def_phantom = phantom::tuple( &def_generics_impl );
                    let def_struct = quote!
                    {
                      #[ derive( Debug ) ]
                      #vis struct #def_name < #def_generics_impl >
                      where // Where clause on new line
                        End2 : former::FormingEnd< #def_types_name< #enum_generics_ty_no_comma, Context2, Formed2 > >,
                        #def_generics_where // Includes original enum where clause
                      { // Brace on new line
                        _phantom : #def_phantom,
                      } // Brace on new line
                    };
                    let def_default_impl = quote!
                    {
                      impl< #def_generics_impl > ::core::default::Default
                      for #def_name < #def_generics_ty >
                      where // Where clause on new line
                        End2 : former::FormingEnd< #def_types_name< #enum_generics_ty_no_comma, Context2, Formed2 > >,
                        #def_generics_where
                      { // Brace on new line
                        fn default() -> Self
                        { // Brace on new line
                          Self { _phantom : ::core::marker::PhantomData }
                        } // Brace on new line
                      } // Brace on new line
                    };
                    let def_former_impl = quote!
                    {
                      impl< #def_generics_impl > former::FormerDefinition
                      for #def_name < #def_generics_ty >
                      where // Where clause on new line
                        End2 : former::FormingEnd< #def_types_name< #enum_generics_ty_no_comma, Context2, Formed2 > >,
                        #def_generics_where
                      { // Brace on new line
                        type Storage = #storage_struct_name< #enum_generics_ty >;
                        type Context = Context2;
                        type Formed = Formed2;
                        type Types = #def_types_name< #enum_generics_ty_no_comma, Context2, Formed2 >;
                        type End = End2;
                      } // Brace on new line
                    };

                    // --- Generate Former Struct --- (Increment 5)
                    let mut former_generics = generics.clone();
                    former_generics.params.push( parse_quote!( Definition = #def_name< #enum_generics_ty > ) );
                    let former_where_clause = former_generics.make_where_clause();
                    former_where_clause.predicates.push( parse_quote!{ Definition : former::FormerDefinition< Storage = #storage_struct_name< #enum_generics_ty > > } );
                    former_where_clause.predicates.push( parse_quote!{ Definition::Types : former::FormerDefinitionTypes< Storage = #storage_struct_name< #enum_generics_ty > > } );
                    if let Some( enum_where ) = &generics.where_clause
                    {
                      for predicate in &enum_where.predicates
                      {
                        former_where_clause.predicates.push( predicate.clone() );
                      }
                    }
                    let ( _former_generics_with_defaults, former_generics_impl, _former_generics_ty, former_generics_where ) = generic_params::decompose( &former_generics );
                    let former_struct_def = quote!
                    {
                      #[ doc = "Former for the #variant_ident variant." ]
                      #vis struct #former_name < #former_generics_impl > // Use decomposed impl generics
                      where // Where clause on new line
                        #former_generics_where // Use decomposed where clause
                      { // Brace on new line
                        /// Temporary storage for all fields during the formation process.
                        pub storage : Definition::Storage,
                        /// Optional context.
                        pub context : ::core::option::Option< Definition::Context >,
                        /// Optional handler for the end of formation.
                        pub on_end : ::core::option::Option< Definition::End >,
                      } // Brace on new line
                    };

                    // --- Collect generated code ---
                    end_impls.push( quote!
                    {
                       #storage_def #storage_default_impl #storage_trait_impl #storage_preform_impl
                       #def_types_struct #def_types_default_impl #def_types_former_impl #def_types_mutator_impl
                       #def_struct #def_default_impl #def_former_impl
                       #former_struct_def // <<< Added Former struct definition
                    });

                    // --- Force Debug Print for EnumG4::V1 ---
                    // if enum_name == "EnumG4" && variant_ident == "V1"
                    // {
                    //   let about = format!( "derive : Former\nenum : {enum_name}::V1 (Forced Debug)" );
                    //   let variant_code = quote!
                    //   {
                    //     #storage_def #storage_default_impl #storage_trait_impl #storage_preform_impl
                    //     #def_types_struct #def_types_default_impl #def_types_former_impl #def_types_mutator_impl
                    //     #def_struct #def_default_impl #def_former_impl
                    //     #former_struct_def
                    //   };
                    //   diag::report_print( about, original_input, &variant_code );
                    // }
                    // --- End Force Debug Print ---

                    // Placeholder for the rest of the implicit former generation (Increments 6-10)
                    // methods.push( quote!{ /* TODO: Add static method for subformer */ } );
                    // end_impls.push( quote!{ /* TODO: Add Former impl, End impls */ } );
                    // standalone_constructors.push( quote!{ /* TODO: Add standalone constructor */ } );
                }
            }
        }
        // <<< End: Logic for Named Fields (Struct-like Variants) >>>
      } // End syn::Fields::Named
    } // End match variant.fields

  } // End variant loop

  // Assemble the final impl block containing the generated static methods
  let result = quote!
  {
      // Implement the static methods on the enum.
      #[ automatically_derived ]
      impl< #enum_generics_impl > #enum_name< #enum_generics_ty >
      where // Where clause on new line
        #enum_generics_where
      { // Brace on new line
          #( #methods )* // Splice the collected methods here
      } // Brace on new line

      // Define the End structs, implicit formers, etc., outside the enum impl block.
      #( #end_impls )*

      // <<< Added: Splice standalone constructors here >>>
      #( #standalone_constructors )*
  };

  if has_debug // Print generated code if #[debug] is present on the enum
  {
    let about = format!( "derive : Former\nenum : {enum_name}" );
    diag::report_print( about, original_input, &result );
  }

  Ok( result )
}
