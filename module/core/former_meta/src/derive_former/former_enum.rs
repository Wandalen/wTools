// File: module/core/former_meta/src/derive_former/former_enum.rs
#![ allow( clippy::wildcard_imports ) ]
use super::*;
use macro_tools::
{
  generic_params, Result,
  proc_macro2::TokenStream, quote::{ format_ident, quote },
  ident, // Added for ident_maybe_raw
  phantom, // Added for phantom::tuple
};
#[ cfg( feature = "derive_former" ) ]
use convert_case::{ Case, Casing }; // Space before ;

// ==================================
//      Enum Variant Handling Rules (Consistent Logic)
// ==================================
// ... (omitted for brevity) ...
// ==================================

// <<< Added helper struct to store field info >>>
/// Temporary storage for field information needed during generation.
#[derive(Clone)] // <<< Added Clone
struct EnumVariantFieldInfo
{
  // index : usize, // Removed unused field
  ident : syn::Ident,
  ty : syn::Type,
  attrs : FieldAttributes,
  is_constructor_arg : bool,
}
// <<< End Added >>>


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
        // --- Standalone Constructor (Unit) ---
        if struct_attrs.standalone_constructors.value( false )
        {
          if variant_attrs.arg_for_constructor.value( false )
          {
            return Err( syn::Error::new_spanned( variant, "#[arg_for_constructor] cannot be applied to a unit enum variant." ) );
          }
          // <<< Use collected info (empty for unit) to generate params >>>
          let _constructor_params : Vec<_> = variant_field_info // Will be empty // <<< Prefixed with _
            .iter()
            .filter( |f_info| f_info.is_constructor_arg )
            .map( |f_info| {
              let param_name = &f_info.ident; // Should not happen for unit
              let ty = &f_info.ty;
              quote! { #param_name : impl Into< #ty > }
            })
            .collect(); // <<< Added collect()
          // <<< End Use >>>

          // <<< Determine Return Type (Always Self for Unit) >>>
          let return_type = quote! { #enum_name< #enum_generics_ty > };
          // <<< End Determine >>>

          let constructor = quote!
          {
            /// Standalone constructor for the #variant_ident unit variant.
            #[ inline( always ) ]
            #vis fn #method_name < #enum_generics_impl >()
            -> // Return type on new line
            #return_type // <<< Use determined return type
            where // Where clause on new line
              #enum_generics_where
            { // Brace on new line
              #enum_name::#variant_ident
            } // Brace on new line
          };
          standalone_constructors.push( constructor );
        }
        // --- End Standalone Constructor ---

        // Associated method
        let static_method = quote!
        {
          /// Constructor for the #variant_ident unit variant.
          #[ inline( always ) ]
          #vis fn #method_name() -> Self
          {
            Self::#variant_ident
          }
        };
        methods.push( static_method );
      },
      // Case 2: Tuple variant
      syn::Fields::Unnamed( fields ) =>
      {
        if variant_attrs.arg_for_constructor.value( false )
        {
          return Err( syn::Error::new_spanned( variant, "#[arg_for_constructor] cannot be applied directly to an enum variant identifier. Apply it to the fields *within* the variant instead, e.g., `MyVariant( #[arg_for_constructor] i32 )`." ) );
        }

        // Sub-case: Single field tuple variant
        if fields.unnamed.len() == 1
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

            let end_struct_name = format_ident!( "{}{}End", enum_name, variant_ident );
            let ( inner_type_name, inner_generics ) = match inner_type { syn::Type::Path( tp ) => { let s = tp.path.segments.last().unwrap(); ( s.ident.clone(), s.arguments.clone() ) }, _ => return Err( syn::Error::new_spanned( inner_type, "Inner variant type must be a path type for subforming" ) ) };
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
        else // len > 1
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
      },
      // Case 3: Struct variant
      syn::Fields::Named( _fields ) => // <<< Changed _ to _fields, mark unused >>>
      {
        if variant_attrs.arg_for_constructor.value( false )
        {
          return Err( syn::Error::new_spanned( variant, "#[arg_for_constructor] cannot be applied directly to an enum variant identifier. Apply it to the fields *within* the variant instead, e.g., `MyVariant { #[arg_for_constructor] field : i32 }`." ) );
        }

        // <<< Start: Logic for Named Fields (Struct-like Variants) >>>
        if wants_subform_scalar
        {
          return Err( syn::Error::new_spanned( variant, "#[subform_scalar] cannot be used on struct-like variants (variants with named fields)." ) );
        }
        else if wants_scalar
        {
          // --- Scalar Struct Variant (len == 0, len == 1, len > 1) ---
          // Logic for this case will be implemented in Increment 4
          // Placeholder: Add empty tokens for now to avoid compilation errors in this step
          methods.push( quote!{} );
          end_impls.push( quote!{} );
        }
        else // Default: Error
        {
          return Err( syn::Error::new_spanned( variant, "Former derive requires `#[scalar]` attribute for struct-like variants (variants with named fields)." ) );
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

// Removed unused helper function: generate_implicit_former_for_variant
// Removed unused helper function: generics_of_definition_types_renamed
// Removed unused helper function: generics_of_definition_renamed
// Removed unused helper function: generics_of_former_renamed