// File: module/core/former_meta/src/derive_former/former_enum.rs
#![ allow( clippy::wildcard_imports ) ]
use super::*;

mod unit;
use unit::handle_unit_variant;

mod tuple_zero;
use tuple_zero::handle_tuple_zero_variant;

mod struct_zero;
use struct_zero::handle_struct_zero_variant;

// Add module declaration and use statement for struct_non_zero
mod struct_non_zero;
use struct_non_zero::handle_struct_non_zero_variant;

use macro_tools::
{
  generic_params, Result,
  proc_macro2::TokenStream, quote::{ format_ident, quote },
  ident, // Added for ident_maybe_raw
  // phantom, // Removed unused import
  diag, // Added for report_print
  // punctuated, // Removed unused import
  // parse_quote, // FIX: Removed unused import
};
#[ cfg( feature = "derive_former" ) ]
use convert_case::{ Case, Casing }; // Space before ;
// FIX: Added necessary imports
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{ GenericArgument, GenericParam, TypeParam, ConstParam, LifetimeParam, /* Type, */ Expr }; // FIX: Removed unused Type import


// ==================================
//      Enum Variant Handling Rules (Consistent Logic)
// ==================================
// ... (rules documentation remains the same) ...
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
  let ( _enum_generics_with_defaults, enum_generics_impl, enum_generics_ty, _enum_generics_where_punctuated ) // Use _ for unused where punctuated
  = generic_params::decompose( generics );
  // Use the Option<&WhereClause> directly from generics by calling .as_ref()
  let merged_where_clause = generics.where_clause.as_ref(); // FIX: Use .as_ref() here

  // --- DEBUG PRINT 1 ---
  // ...
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
    let variant_ident = &variant.ident; // Renamed from _variant_ident

    // --- DEBUG PRINT 2 ---
    // ...
    // --- END DEBUG PRINT 2 ---


    // Generate the snake_case method name, handling potential keywords
    let variant_name_str = variant_ident.to_string(); // Renamed from _variant_name_str
    let method_name_snake_str = variant_name_str.to_case( Case::Snake ); // Renamed from _method_name_snake_str
    let method_name_ident_temp = format_ident!( "{}", method_name_snake_str, span = variant_ident.span() ); // Renamed from _method_name_ident_temp
    let method_name = ident::ident_maybe_raw( &method_name_ident_temp ); // Renamed from _method_name

    // Parse attributes *from the variant* itself
    let variant_attrs = FieldAttributes::from_attrs( variant.attrs.iter() )?;
    let wants_scalar = variant_attrs.scalar.is_some() && variant_attrs.scalar.as_ref().unwrap().setter(); // Renamed from _wants_scalar
    let wants_subform_scalar = variant_attrs.subform_scalar.is_some(); // Renamed from _wants_subform_scalar

    // --- Prepare merged where clause for this variant's generated impls ---
    // let merged_where_clause = enum_generics_where.clone(); // Clone the Option<&WhereClause> // Removed redundant clone

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
          // Pass Option<&WhereClause> directly
          merged_where_clause, // FIX: Pass directly
        )?;
      },
      // Case 2: Tuple variant
      syn::Fields::Unnamed( fields ) =>
      {
        // --- DEBUG PRINT 3b ---
        // ...
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
              // Pass Option<&WhereClause> directly
              merged_where_clause, // FIX: Pass directly
            )?;
          }
          // Sub-case: Single field tuple variant
          1 =>
          {
            // Removed the placeholder error
            let field_info = &variant_field_info[0]; // Get the collected info
            let inner_type = &field_info.ty;

            // Determine behavior based on attributes
            if wants_scalar
            {
              // --- Scalar Tuple(1) Variant ---
              // --- Standalone Constructor (Scalar Tuple(1)) ---
              if struct_attrs.standalone_constructors.value( false )
              {
                let constructor_params : Vec<_> = variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
                let all_fields_are_args = !variant_field_info.is_empty() && variant_field_info.iter().all( |f| f.is_constructor_arg );
                let return_type = if all_fields_are_args { quote! { #enum_name< #enum_generics_ty > } } else { return Err( syn::Error::new_spanned( variant, "#[scalar] on single-field variant implies all fields are constructor args, but #[arg_for_constructor] is missing." ) ); };
                let param_name = format_ident!( "_0" ); // Param name for tuple variant
                let constructor = quote!
                {
                  /// Standalone constructor for the #variant_ident variant (scalar style).
                  #[ inline( always ) ]
                  #vis fn #method_name < #enum_generics_impl >
                  ( // Paren on new line
                    #( #constructor_params ),*
                  ) // Paren on new line
                  -> // Return type on new line
                  #return_type
                  where // Where clause on new line
                    #merged_where_clause // FIX: Use correct variable
                  { // Brace on new line
                    Self::#variant_ident( #param_name.into() )
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
                { // Brace on new line
                  Self::#variant_ident( #param_name.into() )
                } // Brace on new line
              };
              methods.push( static_method );
            }
            else // Default or explicit subform_scalar -> Generate Subformer
            {
              // --- Subform Tuple(1) Variant ---
              if wants_subform_scalar
              {
                if !matches!( inner_type, syn::Type::Path( _ ) ) { return Err( syn::Error::new_spanned( inner_type, "#[subform_scalar] can only be applied to variants holding a path type (e.g., MyStruct, Option<T>), not tuples, references, etc." ) ); }
              }
              else // Default case
              {
                 if !matches!( inner_type, syn::Type::Path( _ ) ) { return Err( syn::Error::new_spanned( inner_type, "Default subforming requires the single field of a tuple variant to be a path type (e.g., MyStruct, Option<T>)." ) ); }
              }

              let end_struct_name = format_ident!( "{}{}End", enum_name, variant_ident );
              let ( inner_type_name, inner_generics ) = match inner_type { syn::Type::Path( tp ) => { let s = tp.path.segments.last().unwrap(); ( s.ident.clone(), s.arguments.clone() ) }, _ => unreachable!() };
              let inner_former_name = format_ident!( "{}Former", inner_type_name );
              let inner_storage_name = format_ident!( "{}FormerStorage", inner_type_name );
              let inner_def_name = format_ident!( "{}FormerDefinition", inner_type_name );
              let inner_def_types_name = format_ident!( "{}FormerDefinitionTypes", inner_type_name );
              // FIX: Convert GenericArgument to GenericParam
              let inner_generics_params : Punctuated<GenericParam, Comma> = match &inner_generics
              {
                syn::PathArguments::AngleBracketed( args ) => args.args.iter().map( |arg| match arg {
                  // FIX: Extract ident correctly for Type and Const
                  GenericArgument::Type( ty ) => match ty {
                      syn::Type::Path( p ) => GenericParam::Type( TypeParam { ident: p.path.get_ident().unwrap().clone(), attrs: vec![], colon_token: None, bounds: Punctuated::new(), eq_token: None, default: None } ),
                      _ => panic!("Unsupported generic argument type for TypeParam ident extraction"),
                  },
                  GenericArgument::Lifetime( lt ) => GenericParam::Lifetime( LifetimeParam::new( lt.clone() ) ),
                  GenericArgument::Const( c ) => match c {
                      Expr::Path( p ) => GenericParam::Const( ConstParam { ident: p.path.get_ident().unwrap().clone(), attrs: vec![], const_token: Default::default(), colon_token: Default::default(), ty: parse_quote!(_), eq_token: None, default: None } ), // Assume type _ if not easily extractable
                      _ => panic!("Unsupported const expression for ConstParam ident extraction"),
                  },
                  _ => panic!("Unsupported generic argument type"), // Or return error
                }).collect(),
                _ => Punctuated::new(),
              };
              let mut inner_generics_ty_punctuated = inner_generics_params.clone(); // Use the converted params
              if !inner_generics_ty_punctuated.empty_or_trailing() { inner_generics_ty_punctuated.push_punct( Default::default() ); }

              // FIX: Helper for conditional comma based on enum generics
              let comma_if_enum_generics = if enum_generics_ty.is_empty() { quote!{} } else { quote!{ , } };


              // --- Standalone Constructor (Subform Tuple(1)) ---
              if struct_attrs.standalone_constructors.value( false )
              {
                let constructor_params : Vec<_> = variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
                let all_fields_are_args = !variant_field_info.is_empty() && variant_field_info.iter().all( |f| f.is_constructor_arg );
                // FIX: Correct return type generation
                let return_type = if all_fields_are_args
                {
                   quote! { #enum_name< #enum_generics_ty > }
                }
                else
                { // FIX: Added comma_if_enum_generics
                  quote! { #inner_former_name < #inner_generics_ty_punctuated #inner_def_name < #inner_generics_ty_punctuated () #comma_if_enum_generics #enum_name< #enum_generics_ty >, #end_struct_name < #enum_generics_ty > > > }
                };
                // FIX: Use inner_generics_ty_punctuated in storage init
                let initial_storage_code = if field_info.is_constructor_arg { let param_name = format_ident!( "_0" ); quote! { ::core::option::Option::Some( #inner_storage_name :: < #inner_generics_ty_punctuated > { _0 : ::core::option::Option::Some( #param_name.into() ) } ) } } else { quote! { ::core::option::Option::None } };
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
                    #merged_where_clause // FIX: Use correct variable
                  { // Brace on new line
                    #inner_former_name::begin
                    ( // Paren on new line
                      #initial_storage_code,
                      None, // Context
                      #end_struct_name::< #enum_generics_ty >::default() // End
                    ) // Paren on new line
                  } // Brace on new line
                };
                standalone_constructors.push( constructor );
              }
              // --- End Standalone Constructor ---

              // Associated method logic
              let phantom_field_type = macro_tools::phantom::tuple( &generics.params ); // FIX: Use qualified path and correct generics
              let end_struct_def = quote!
              {
                #[ derive( Default, Debug ) ]
                #vis struct #end_struct_name < #enum_generics_impl >
                where // Where clause on new line
                  #merged_where_clause // FIX: Use correct variable
                { // Brace on new line
                  _phantom : #phantom_field_type,
                } // Brace on new line
              };
              let end_impl = quote!
              {
                #[ automatically_derived ]
                impl< #enum_generics_impl > former::FormingEnd
                < // Angle bracket on new line
                  // FIX: Correct generics usage and add comma_if_enum_generics
                  #inner_def_types_name< #inner_generics_ty_punctuated () #comma_if_enum_generics #enum_name< #enum_generics_ty > >
                > // Angle bracket on new line
                for #end_struct_name < #enum_generics_ty >
                where // Where clause on new line
                  #merged_where_clause // FIX: Use correct variable
                { // Brace on new line
                  #[ inline( always ) ]
                  fn call
                  ( // Paren on new line
                    &self,
                    sub_storage : #inner_storage_name< #inner_generics_ty_punctuated >, // FIX: Use punctuated version
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
                < // Angle bracket on new line
                  #inner_generics_ty_punctuated // FIX: Use punctuated version
                  #inner_def_name
                  < // Angle bracket on new line
                    #inner_generics_ty_punctuated () #comma_if_enum_generics #enum_name< #enum_generics_ty >, #end_struct_name < #enum_generics_ty > // FIX: Use punctuated version and add comma
                  > // Angle bracket on new line
                > // Angle bracket on new line
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
                let constructor_params : Vec<_> = variant_field_info.iter().map( |f_info| { let param_name = &f_info.ident; let ty = &f_info.ty; quote! { #param_name : impl Into< #ty > } } ).collect();
                let return_type = quote! { #enum_name< #enum_generics_ty > };
                let mut direct_construction_args = Vec::new();
                for field_info_inner in &variant_field_info { let param_name = &field_info_inner.ident; direct_construction_args.push( quote! { #param_name.into() } ); }
                let constructor = quote!
                {
                  /// Standalone constructor for the #variant_ident variant with multiple fields (scalar style).
                  #[ inline( always ) ]
                  #vis fn #method_name < #enum_generics_impl >
                  ( // Paren on new line
                    #( #constructor_params ),*
                  ) // Paren on new line
                  -> // Return type on new line
                  #return_type
                  where // Where clause on new line
                    #merged_where_clause // FIX: Use correct variable
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
              for field_info in &variant_field_info { let param_name = &field_info.ident; let field_type = &field_info.ty; params.push( quote! { #param_name : impl Into< #field_type > } ); args.push( quote! { #param_name.into() } ); }
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
        // ...
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
                    // Pass Option<&WhereClause> directly
                    merged_where_clause, // FIX: Pass directly
                )?;
            }
            // Sub-case: Single field (Struct(1)) or Multi-field (Struct(N))
            _ => // len >= 1
            {
              // Call the extracted handler for non-zero struct variants
              println!( "DEBUG: Calling handle_struct_non_zero_variant for variant: {}", variant.ident ); // Debug print
              handle_struct_non_zero_variant
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
                // Pass Option<&WhereClause> directly
                merged_where_clause, // FIX: Pass directly
              )?;
            }
        }
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
        #merged_where_clause // FIX: Use the Option<&WhereClause> variable here
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