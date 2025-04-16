// File: module/core/former_meta/src/derive_former/former_enum.rs

#![ allow( clippy::wildcard_imports ) ]
use super::*; // Use items from parent module (derive_former.rs)
// Removed Itertools - not used here
use macro_tools::
{
  // Removed attr, diag, generic_args, typ, derive - not used directly here
  generic_params, Result,
  proc_macro2::TokenStream, quote::{ format_ident, quote },
  ident,
};
#[ cfg( feature = "derive_former" ) ]
use convert_case::{ Case, Casing };


/// Generate the Former ecosystem for an enum.
#[ allow( clippy::too_many_lines ) ]
pub(super) fn former_for_enum // Make it pub(super)
(
  ast : &syn::DeriveInput,
  data_enum : &syn::DataEnum,
  _original_input : &proc_macro::TokenStream,
  _has_debug : bool, // qqq : xxx : make sure debug works
) -> Result< TokenStream >
{
  let enum_name = &ast.ident;
  let vis = &ast.vis;
  let generics = &ast.generics;
  let ( _enum_generics_with_defaults, enum_generics_impl, enum_generics_ty, enum_generics_where )
  = generic_params::decompose( generics );

  // Initialize vectors to collect generated code pieces
  let mut methods = Vec::new();
  let mut end_impls = Vec::new(); // Need this again for End structs

  // Iterate through each variant of the enum
  for variant in &data_enum.variants
  {
    let variant_ident = &variant.ident;

    // Generate the snake_case method name, handling potential keywords
    let variant_name_str = variant_ident.to_string();
    let method_name_snake_str = variant_name_str.to_case( Case::Snake );
    let method_name_ident_temp = format_ident!( "{}", method_name_snake_str, span = variant_ident.span() );
    let method_name = ident::ident_maybe_raw( &method_name_ident_temp );

    // Generate method based on the variant's fields
    match &variant.fields
    {
        // Case 1: Unit variant (e.g., `Empty`) - Keep direct constructor
        syn::Fields::Unit =>
        {
            let static_method = quote!
            {
              /// Constructor for the #variant_ident unit variant.
              #[ inline( always ) ]
              #vis fn #method_name() -> Self // Self refers to the enum type
              {
                // Directly construct the unit variant
                Self::#variant_ident
              }
            };
            methods.push( static_method );
        },
        // Case 2: Tuple variant (e.g., `Simple(String)`, `MultiTuple(i32, String)`)
        syn::Fields::Unnamed( fields ) =>
        {
            // Sub-case: Single field tuple variant (e.g., `Simple(String)`) - Generate Subformer Starter
            if fields.unnamed.len() == 1
            {
                let field = fields.unnamed.first().unwrap();
                let inner_type = &field.ty;

                // Generate name for the specialized End struct for this variant.
                let end_struct_name = format_ident!( "{}{}End", enum_name, variant_ident );

                // Extract the base name and generics of the inner type (assuming it's a path).
                let ( inner_type_name, inner_generics ) = match inner_type
                {
                    syn::Type::Path( type_path ) =>
                    {
                      let segment = type_path.path.segments.last().ok_or_else( || syn::Error::new_spanned( inner_type, "Cannot derive name from type path") )?;
                      ( segment.ident.clone(), segment.arguments.clone() ) // Get name and any generics like <T>
                    },
                    _ => return Err( syn::Error::new_spanned( inner_type, "Inner variant type must be a path type (like MyStruct or MyStruct<T>) to derive Former" ) ),
                };

                // Generate names for the inner type's Former components.
                let inner_former_name = format_ident!( "{}Former", inner_type_name );
                let inner_storage_name = format_ident!( "{}FormerStorage", inner_type_name );
                let inner_def_name = format_ident!( "{}FormerDefinition", inner_type_name );
                let inner_def_types_name = format_ident!( "{}FormerDefinitionTypes", inner_type_name );

                // Extract type generics from inner type arguments if present. Use owned Punctuated.
                let inner_generics_ty : syn::punctuated::Punctuated<syn::GenericArgument, syn::token::Comma> = match &inner_generics
                {
                    syn::PathArguments::AngleBracketed( args ) => args.args.clone(), // Clone the args
                    _ => syn::punctuated::Punctuated::new(), // Return an empty owned list
                };
                // TODO: Properly handle inner_generics_impl and inner_generics_where if needed

                // Generate the definition for the specialized End struct.
                let end_struct_def = quote!
                {
                  #[ derive( Default, Debug ) ]
                  #vis struct #end_struct_name;
                };

                // Generate the `impl FormingEnd` block for the specialized End struct.
                let end_impl = quote!
                {
                  #[ automatically_derived ]
                  impl< #enum_generics_impl > former::FormingEnd
                  <
                      // DefinitionTypes of the inner former: Context=(), Formed=TheEnum<...>
                      #inner_def_types_name< #inner_generics_ty (), #enum_name< #enum_generics_ty > >
                  >
                  for #end_struct_name
                  where // Include where clauses from the enum
                    #enum_generics_where // TODO: Add where clauses from inner type if needed
                  {
                      #[ inline( always ) ]
                      fn call
                      (
                        &self,
                        sub_storage : #inner_storage_name< #inner_generics_ty >, // Storage from the inner former.
                        _context : Option< () >, // Context is () as we start from a static method.
                      ) -> #enum_name< #enum_generics_ty > // Returns the final enum instance.
                      {
                        // Preform the inner data and wrap it in the correct enum variant.
                        let data = former::StoragePreform::preform( sub_storage );
                        #enum_name::#variant_ident( data )
                      }
                  }
                };

                // Generate the static method on the enum that returns the subformer
                let static_method = quote!
                {
                  /// Starts forming the #variant_ident variant.
                  #[ inline( always ) ]
                  #vis fn #method_name() // Use the generated (potentially raw) method name.
                  -> #inner_former_name // Return type is the Former for the inner data type.
                     <
                       #inner_generics_ty // Pass inner type generics
                       // Configure the inner former's definition:
                       #inner_def_name
                       <
                           #inner_generics_ty // Pass inner type generics again
                           (),                             // Context is ().
                           #enum_name< #enum_generics_ty >, // The final type to be Formed is the enum itself.
                           #end_struct_name                // Use the specialized End struct.
                       >
                     >
                  {
                      // Start the inner former using its `begin` associated function.
                      #inner_former_name::begin( None, None, #end_struct_name::default() )
                  }
                };

                methods.push( static_method );
                end_impls.push( quote!{ #end_struct_def #end_impl } );

            }
            // Sub-case: Multi-field tuple variant (e.g., `MultiTuple(i32, String)`)
            else if fields.unnamed.len() > 1
            {
                // TODO: Implement logic for multi-field tuple variants (direct constructor)
                // For now, skip them
                continue;
            }
            // else: fields.unnamed.len() == 0 - should not happen for Unnamed, ignore or error?
        },
        // Case 3: Struct variant (e.g., `StructVariant { field: T }`)
        syn::Fields::Named( _fields ) =>
        {
          // Struct variants are not supported by this pattern
          return Err
          (
            syn::Error::new_spanned
            (
              &variant.fields,
              "Former derive on enums does not support struct variants." // Simplified error
            )
          );
        }
    } // End match variant.fields

  } // End variant loop

  // Assemble the final impl block containing the generated static methods
  let result = quote!
  {
      // Implement the static methods on the enum.
      #[ automatically_derived ]
      impl< #enum_generics_impl > #enum_name< #enum_generics_ty >
      where
        #enum_generics_where
      {
          #( #methods )* // Splice the collected methods here
      }

      // Define the End structs and their implementations outside the enum impl block.
      #( #end_impls )*
  };

  Ok( result )
}