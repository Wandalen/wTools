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
//      Generic Handling Strategy
// ==================================
//
// IMPORTANT NOTE ON GENERICS:
//
// Handling generics in enum variants for the `Former` derive involves several complexities,
// primarily concerning the interaction between the enum's own generic parameters (e.g., `Enum<T>`)
// and the generics potentially present in the data type held by a variant (e.g., `Variant(Inner<T>)`
// or `Variant(Inner<U>)`).
//
// The core challenges and the chosen strategy are:
//
// 1.  **Extracting Bounds from Inner Types is Unreliable:** Attempting to determine the necessary
//     trait bounds for a generic parameter (`T` or `U`) solely by inspecting the inner type
//     (e.g., `Inner<T>`) within the variant is generally not feasible or reliable in a procedural macro.
//     The macro only sees the *use* of the type, not its definition, and thus cannot know the
//     bounds `Inner` requires for its generic parameters. The previous attempt to implement
//     `generics_of_type` demonstrated this difficulty, leading to compilation errors.
//
// 2.  **Focus on Propagating Enum Generics:** The correct approach is to focus on the generics
//     defined *on the enum itself*. These generics (`enum Enum<T: Bound>`) and their associated
//     `where` clauses *must* be correctly propagated to all generated code that depends on them.
//
// 3.  **Merging Generics for Implementations:** When generating `impl` blocks (like `impl FormingEnd`
//     for the specialized `End` struct or `impl FormerMutator` for implicit definition types),
//     we often need to combine the enum's generics with *additional* generics introduced by the
//     macro's infrastructure (e.g., `Definition`, `Context`, `Formed`, `End`).
//     **For this purpose, `macro_tools::generic_params::merge` MUST be used.** It correctly
//     combines two complete `syn::Generics` structures (including their `where` clauses).
//
// 4.  **Bound Requirements:** The necessary bounds for the *inner type's* generics (e.g., the bounds
//     `Inner` requires for `T` or `U`) are implicitly handled by the Rust compiler *after* the macro
//     generates the code. If the generated code attempts to use the inner type in a way that
//     violates its bounds (because the enum's generics/bounds passed down are insufficient),
//     the compiler will produce the appropriate error. The macro's responsibility is to correctly
//     apply the *enum's* bounds where needed.
//
// 5.  **`macro_tools::generic_params::merge` Issues:** If any issues arise with the merging logic itself
//     (e.g., incorrect handling of `where` clauses by the `merge` function), those issues must be
//     addressed within the `macro_tools` crate, as it is the designated tool for this task.
//
// In summary: We propagate the enum's generics and bounds. We use `generic_params::merge`
// to combine these with macro-internal generics when generating implementations. We rely on
// the Rust compiler to enforce the bounds required by the inner data types used in variants.
//
// ==================================

/// Temporary storage for field information needed during generation.
#[derive(Clone)]
struct EnumVariantFieldInfo
{
  // index : usize, // Removed unused field
  ident : syn::Ident,
  ty : syn::Type,
  attrs : FieldAttributes,
  is_constructor_arg : bool,
}

/// Generate the Former ecosystem for an enum.
#[ allow( clippy::too_many_lines ) ]
pub(super) fn former_for_enum
(
  ast : &syn::DeriveInput,
  data_enum : &syn::DataEnum,
  original_input : &proc_macro::TokenStream,
  has_debug : bool,
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

    let variant_field_info: Vec<EnumVariantFieldInfo> = match &variant.fields
    {
        syn::Fields::Named( f ) => f.named.iter().enumerate().map( | ( _index, field ) | // Space around |
        {
            let attrs = FieldAttributes::from_attrs( field.attrs.iter() )?;
            let is_constructor_arg = attrs.arg_for_constructor.value( false );
            Ok( EnumVariantFieldInfo
            {
                // index, // Removed assignment to unused field
                ident: field.ident.clone().ok_or_else( || syn::Error::new_spanned( field, "Named field requires an identifier" ) )?, // Space around ||
                ty: field.ty.clone(),
                attrs,
                is_constructor_arg,
            })
        }).collect::< Result< _ > >()?,
        syn::Fields::Unnamed( f ) => f.unnamed.iter().enumerate().map( | ( index, field ) | // Space around |
        {
            let attrs = FieldAttributes::from_attrs( field.attrs.iter() )?;
            let is_constructor_arg = attrs.arg_for_constructor.value( false );
            Ok( EnumVariantFieldInfo
            {
                ident: format_ident!( "_{}", index ), // Synthesize identifier - Note: still uses index here!
                ty: field.ty.clone(),
                attrs,
                is_constructor_arg,
            })
        }).collect::< Result< _ > >()?,
        syn::Fields::Unit => vec![],
    };

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
          let _constructor_params = variant_field_info // Will be empty // Prefixed with _
            .iter()
            .filter( | f_info | f_info.is_constructor_arg ) // Space around |
            .map( | f_info | // Space around |
            {
              let param_name = &f_info.ident; // Should not happen for unit
              let ty = &f_info.ty;
              quote! { #param_name : impl Into< #ty > }
            });
          // <<< End Use >>>

          let constructor = quote!
          {
            /// Standalone constructor for the #variant_ident unit variant.
            #[ inline( always ) ]
            #vis fn #method_name < #enum_generics_impl >()
            -> // Return type on new line
            #enum_name< #enum_generics_ty >
            where // Where clause on new line
              #enum_generics_where
            {
              #enum_name::#variant_ident
            }
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
          let field_info = &variant_field_info[ 0 ]; // Get the collected info
          let inner_type = &field_info.ty;
          // let _field_attrs = &field_info.attrs; // <<< Use parsed attrs from field_info (Marked unused for now)

          // Determine if the inner type likely has its own Former (heuristic)
          let inner_former_exists = if let syn::Type::Path( tp ) = inner_type { tp.path.segments.last().is_some_and( | seg | !matches!( seg.ident.to_string().as_str(), "bool" | "char" | "str" | "String" | "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "f32" | "f64" ) ) } else { false }; // Space around |

          if wants_scalar || ( !wants_subform_scalar && !inner_former_exists ) // Space around ||
          {
            // --- Scalar Tuple(1) Variant ---
            let implicit_former_name = format_ident!( "{}{}Former", enum_name, variant_ident );
            let implicit_storage_name = format_ident!( "{}{}FormerStorage", enum_name, variant_ident );
            let implicit_def_name = format_ident!( "{}{}FormerDefinition", enum_name, variant_ident );
            let implicit_def_types_name = format_ident!( "{}{}FormerDefinitionTypes", enum_name, variant_ident );
            let end_struct_name = format_ident!( "{}{}End", enum_name, variant_ident );

            // Generate the implicit former components (Storage, Defs, Former, End)
            let ( implicit_former_components, _ ) = generate_implicit_former_for_variant
            (
              vis,
              enum_name,
              variant_ident,
              &variant.fields, // Pass fields here
              generics,
              &implicit_former_name,
              &implicit_storage_name,
              &implicit_def_name,
              &implicit_def_types_name,
              &end_struct_name,
              original_input,
              &variant_field_info, // <<< Pass full info
            )?;
            end_impls.push( implicit_former_components ); // Add generated components

            // --- Standalone Constructor (Scalar Tuple(1)) --- Option 2 Logic ---
            if struct_attrs.standalone_constructors.value( false )
            {
              // Generate constructor parameters based on #[arg_for_constructor]
              let constructor_params = variant_field_info
                .iter()
                .filter( | f_info | f_info.is_constructor_arg ) // Space around |
                .map( | f_info | // Space around |
                {
                  let param_name = &f_info.ident; // Will be _0
                  let ty = &f_info.ty;
                  quote! { #param_name : impl Into< #ty > }
                });

              // Determine if all fields are args (for Tuple(1), just check the single field)
              let all_fields_are_args = field_info.is_constructor_arg;

              // Determine return type and body based on Option 2 rule
              let ( return_type, constructor_body ) = if all_fields_are_args
              {
                // Return Self
                let return_type = quote! { #enum_name< #enum_generics_ty > };
                let arg_name = format_ident!( "_0" ); // The single argument name
                let body = quote! { #enum_name::#variant_ident( #arg_name.into() ) };
                ( return_type, body )
              }
              else
              {
                // Return Former
                let former_return_type = quote!
                {
                  #implicit_former_name
                  <
                    #enum_generics_ty // Enum generics
                    #implicit_def_name // Implicit definition
                    <
                      #enum_generics_ty // Enum generics
                      (), // Context
                      #enum_name< #enum_generics_ty >, // Formed
                      #end_struct_name < #enum_generics_ty > // End
                    >
                  >
                };
                // Initialize storage only if the field is an argument
                let initial_storage_code = if field_info.is_constructor_arg
                {
                  let param_name = format_ident!( "_0" );
                  quote! { ::core::option::Option::Some( #implicit_storage_name :: < #enum_generics_ty > { _0 : ::core::option::Option::Some( #param_name.into() ), _phantom : ::core::marker::PhantomData } ) }
                } else { quote! { ::core::option::Option::None } };
                let former_body = quote!
                {
                  #implicit_former_name::begin
                  ( // Paren on new line
                    #initial_storage_code,
                    None, // Context
                    #end_struct_name::< #enum_generics_ty >::default() // End
                  )
                };
                ( former_return_type, former_body )
              };

              // Generate the constructor function code
              let constructor = quote!
              {
                /// Standalone constructor for the #variant_ident variant.
                #[ inline( always ) ]
                #vis fn #method_name < #enum_generics_impl >
                (
                  #( #constructor_params ),*
                )
                ->
                #return_type // Use determined return type
                where
                  #enum_generics_where
                {
                  #constructor_body // Use determined body
                }
              };
              standalone_constructors.push( constructor );
            }
            // --- End Standalone Constructor ---

            // Associated method (still returns Self directly for scalar)
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
            let end_struct_name = format_ident!( "{}{}End", enum_name, variant_ident );
            let ( inner_type_name, inner_generics ) = match inner_type { syn::Type::Path( tp ) => { let s = tp.path.segments.last().unwrap(); ( s.ident.clone(), s.arguments.clone() ) }, _ => return Err( syn::Error::new_spanned( inner_type, "Inner variant type must be a path type" ) ) };
            let inner_former_name = format_ident!( "{}Former", inner_type_name );
            let inner_storage_name = format_ident!( "{}FormerStorage", inner_type_name );
            let inner_def_name = format_ident!( "{}FormerDefinition", inner_type_name );
            let inner_def_types_name = format_ident!( "{}FormerDefinitionTypes", inner_type_name );
            let inner_generics_ty : syn::punctuated::Punctuated<_,_> = match &inner_generics { syn::PathArguments::AngleBracketed( args ) => args.args.clone(), _ => syn::punctuated::Punctuated::default() };
            let inner_generics_ty_comma = if inner_generics_ty.is_empty() { quote!{} } else { quote!{ #inner_generics_ty, } };

            // --- Standalone Constructor (Subform Tuple(1)) --- Option 2 Logic ---
            if struct_attrs.standalone_constructors.value( false )
            {
              // Generate constructor parameters based on #[arg_for_constructor]
              let constructor_params = variant_field_info
                .iter()
                .filter( | f_info | f_info.is_constructor_arg ) // Space around |
                .map( | f_info | // Space around |
                {
                  let param_name = &f_info.ident; // Will be _0
                  let ty = &f_info.ty;
                  quote! { #param_name : impl Into< #ty > }
                });

              // Determine if all fields are args (for Tuple(1), just check the single field)
              let all_fields_are_args = field_info.is_constructor_arg;

              // Determine return type and body based on Option 2 rule
              let ( return_type, constructor_body ) = if all_fields_are_args
              {
                // Return Self
                let return_type = quote! { #enum_name< #enum_generics_ty > };
                let arg_name = format_ident!( "_0" ); // The single argument name
                let body = quote! { #enum_name::#variant_ident( #arg_name.into() ) };
                ( return_type, body )
              }
              else
              {
                // Return Inner Former
                let former_return_type = quote!
                {
                  #inner_former_name // Use the inner type's former
                  <
                    #inner_generics_ty_comma // Inner type generics
                    #inner_def_name // Inner definition
                    <
                      #inner_generics_ty_comma // Inner type generics
                      (), // Context
                      #enum_name< #enum_generics_ty >, // Formed (Outer Enum)
                      #end_struct_name < #enum_generics_ty > // End (Outer Enum's End)
                    >
                  >
                };
                // Initialize inner storage only if the field is an argument
                let initial_storage_code = if field_info.is_constructor_arg
                {
                  let param_name = format_ident!( "_0" );
                  // Assume inner storage field is also named _0 for tuple variants
                  quote! { ::core::option::Option::Some( #inner_storage_name :: < #inner_generics_ty > { _0 : ::core::option::Option::Some( #param_name.into() ) /* Add _phantom if needed */ } ) }
                } else { quote! { ::core::option::Option::None } };
                let former_body = quote!
                {
                  #inner_former_name::begin
                  ( // Paren on new line
                    #initial_storage_code,
                    None, // Context
                    #end_struct_name::< #enum_generics_ty >::default() // End
                  )
                };
                ( former_return_type, former_body )
              };

              // Generate the constructor function code
              let constructor = quote!
              {
                /// Standalone constructor for the #variant_ident subform variant.
                #[ inline( always ) ]
                #vis fn #method_name < #enum_generics_impl >
                (
                  #( #constructor_params ),*
                )
                ->
                #return_type // Use determined return type
                where
                  #enum_generics_where
                {
                  #constructor_body // Use determined body
                }
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
              {
                _phantom : #phantom_field_type,
              }
            };
            let end_impl = quote!
            {
              #[ automatically_derived ]
              impl< #enum_generics_impl > former::FormingEnd
              < // Angle bracket on new line
                #inner_def_types_name< #inner_generics_ty_comma (), #enum_name< #enum_generics_ty > >
              >
              for #end_struct_name < #enum_generics_ty >
              where // Where clause on new line
                #merged_where_clause
              {
                #[ inline( always ) ]
                fn call
                ( // Paren on new line
                  &self,
                  sub_storage : #inner_storage_name< #inner_generics_ty >,
                  _context : Option< () >,
                ) // Paren on new line
                -> // Return type on new line
                #enum_name< #enum_generics_ty >
                {
                  let data = former::StoragePreform::preform( sub_storage );
                  #enum_name::#variant_ident( data )
                }
              }
            };
            let static_method = quote!
            {
              /// Starts forming the #variant_ident variant using a subformer.
              #[ inline( always ) ]
              #vis fn #method_name ()
              -> // Return type on new line
              #inner_former_name
              < // Angle bracket on new line
                #inner_generics_ty_comma
                #inner_def_name
                <
                  #inner_generics_ty_comma (), #enum_name< #enum_generics_ty >, #end_struct_name < #enum_generics_ty >
                >
              > // Angle bracket on new line
              {
                #inner_former_name::begin( None, None, #end_struct_name::< #enum_generics_ty >::default() )
              }
            };
            methods.push( static_method );
            end_impls.push( quote!{ #end_struct_def #end_impl } );
          }
        }
        // Sub-case: Multi-field tuple variant
        else
        if wants_scalar
        {
          // --- Scalar Tuple(N) Variant ---
          let implicit_former_name = format_ident!( "{}{}Former", enum_name, variant_ident );
          let implicit_storage_name = format_ident!( "{}{}FormerStorage", enum_name, variant_ident );
          let implicit_def_name = format_ident!( "{}{}FormerDefinition", enum_name, variant_ident );
          let implicit_def_types_name = format_ident!( "{}{}FormerDefinitionTypes", enum_name, variant_ident );
          let end_struct_name = format_ident!( "{}{}End", enum_name, variant_ident );

          // Generate the implicit former components (Storage, Defs, Former, End)
          let ( implicit_former_components, _ ) = generate_implicit_former_for_variant
          (
            vis,
            enum_name,
            variant_ident,
            &variant.fields, // Pass fields here
            generics,
            &implicit_former_name,
            &implicit_storage_name,
            &implicit_def_name,
            &implicit_def_types_name,
            &end_struct_name,
            original_input,
            &variant_field_info, // <<< Pass full info
          )?;
          end_impls.push( implicit_former_components ); // Add generated components

          // --- Standalone Constructor (Tuple(N)) --- Option 2 Logic ---
          // Note: This block handles variants previously considered "Scalar Tuple(N)"
          // but now follows the general Option 2 logic based solely on #[arg_for_constructor].
          if struct_attrs.standalone_constructors.value( false )
          {
            // Generate constructor parameters based *only* on #[arg_for_constructor]
            let constructor_params = variant_field_info
              .iter()
              .filter( | f_info | f_info.is_constructor_arg ) // Space around |
              .map( | f_info | // Space around |
              {
                let param_name = &f_info.ident; // Will be _0, _1, ...
                let ty = &f_info.ty;
                quote! { #param_name : impl Into< #ty > }
              });

            // Determine if all fields are args
            let all_fields_are_args = variant_field_info.iter().all( | f_info | f_info.is_constructor_arg ); // Space around |

            // Determine return type and body based on Option 2 rule
            let ( return_type, constructor_body ) = if all_fields_are_args
            {
              // Return Self
              let return_type = quote! { #enum_name< #enum_generics_ty > };
              let construction_args = variant_field_info.iter().map( | f_info | // Space around |
              {
                let param_name = &f_info.ident;
                quote! { #param_name.into() }
              });
              let body = quote! { #enum_name::#variant_ident( #( #construction_args ),* ) };
              ( return_type, body )
            }
            else
            {
              // Return Implicit Former
              let former_return_type = quote!
              {
                #implicit_former_name
                < // Angle bracket on new line
                  #enum_generics_ty // Enum generics
                  #implicit_def_name // Implicit definition
                  <
                    #enum_generics_ty // Enum generics
                    (), // Context
                    #enum_name< #enum_generics_ty >, // Formed
                    #end_struct_name < #enum_generics_ty > // End
                  >
                > // Angle bracket on new line
              };
              // Initialize storage based on constructor args
              let initial_storage_fields = variant_field_info
                .iter()
                .map( | f_info | // Space around |
                {
                  let field_ident = &f_info.ident;
                  if f_info.is_constructor_arg
                  {
                    quote! { #field_ident : ::core::option::Option::Some( #field_ident.into() ) }
                  }
                  else
                  {
                    quote! { #field_ident : ::core::option::Option::None }
                  }
                });
              let initial_storage_code = quote!
              {
                ::core::option::Option::Some
                ( // Paren on new line
                  #implicit_storage_name :: < #enum_generics_ty > // Add generics
                  {
                    #( #initial_storage_fields, )*
                    _phantom : ::core::marker::PhantomData // Add phantom if needed
                  }
                ) // Paren on new line
              };
              let former_body = quote!
              {
                #implicit_former_name::begin
                ( // Paren on new line
                  #initial_storage_code,
                  None, // Context
                  #end_struct_name::< #enum_generics_ty >::default() // End
                )
              };
              ( former_return_type, former_body )
            };

            // Generate the constructor function code
            let constructor = quote!
            {
              /// Standalone constructor for the #variant_ident variant.
              #[ inline( always ) ]
              #vis fn #method_name < #enum_generics_impl >
              (
                #( #constructor_params ),*
              )
              ->
              #return_type // Use determined return type
              where
                #enum_generics_where
              {
                #constructor_body // Use determined body
              }
            };
            standalone_constructors.push( constructor );
          }
          // --- End Standalone Constructor ---

          // Associated method (returns implicit former)
          let static_method = quote!
          {
            /// Starts forming the #variant_ident variant using its implicit former.
            #[ inline( always ) ]
            #vis fn #method_name ()
            -> // Return type on new line
            #implicit_former_name
            < // Angle bracket on new line
              #enum_generics_ty
              #implicit_def_name
              <
                #enum_generics_ty (), #enum_name< #enum_generics_ty >, #end_struct_name < #enum_generics_ty >
              >
            > // Angle bracket on new line
            {
              #implicit_former_name::begin( None, None, #end_struct_name::< #enum_generics_ty >::default() )
            }
          };
          methods.push( static_method );
        }
        else // Default: Subformer (unsupported)
        {
          return Err( syn::Error::new_spanned( variant, "Former derive on enums does not support the default subformer pattern for multi-field tuple variants.\nAdd the `#[ scalar ]` attribute to the variant..." ) );
        }
      },
      // Case 3: Struct variant
      syn::Fields::Named( _ ) => // <<< Changed fields to _
      {
        if variant_attrs.arg_for_constructor.value( false )
        {
          return Err( syn::Error::new_spanned( variant, "#[arg_for_constructor] cannot be applied directly to an enum variant identifier. Apply it to the fields *within* the variant instead, e.g., `MyVariant { #[arg_for_constructor] field : i32 }`." ) );
        }

        // Define names and generate implicit components *before* branching on wants_scalar
        let implicit_former_name = format_ident!( "{}{}Former", enum_name, variant_ident );
        let implicit_storage_name = format_ident!( "{}{}FormerStorage", enum_name, variant_ident );
        let implicit_def_name = format_ident!( "{}{}FormerDefinition", enum_name, variant_ident );
        let implicit_def_types_name = format_ident!( "{}{}FormerDefinitionTypes", enum_name, variant_ident );
        let end_struct_name = format_ident!( "{}{}End", enum_name, variant_ident );

        let ( implicit_former_components, _ ) = generate_implicit_former_for_variant
        (
          vis,
          enum_name,
          variant_ident,
          &variant.fields, // Pass fields here
          generics,
          &implicit_former_name,
          &implicit_storage_name,
          &implicit_def_name,
          &implicit_def_types_name,
          &end_struct_name,
          original_input,
          &variant_field_info, // <<< Pass full info
        )?;
        end_impls.push( implicit_former_components ); // Add generated components

        // Generate associated method based on scalar/subform
        if wants_scalar
        {
          // --- Scalar Struct Variant --- Associated Method ---

          // Associated method (returns Self directly) // <<< Standalone constructor moved below if/else
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
            #vis fn #method_name
            ( // Paren on new line
              #( #params ),*
            ) // Paren on new line
            -> Self
            {
              Self::#variant_ident { #( #args ),* }
            }
          };
          methods.push( static_method );
        }
        else // Default: Subformer
        {
          // --- Subform Struct Variant --- Associated Method ---
          // Names are already defined before the if/else block
          // Implicit components are already generated and pushed before the if/else block

          // <<< Redundant generation removed >>>

          // Associated method (returns implicit former)
          let static_method = quote!
          {
            /// Starts forming the #variant_ident variant using its implicit subformer.
            #[ inline( always ) ]
            #vis fn #method_name ()
            -> // Return type on new line
            #implicit_former_name
            < // Angle bracket on new line
              #enum_generics_ty
              #implicit_def_name
              <
                #enum_generics_ty (), #enum_name< #enum_generics_ty >, #end_struct_name < #enum_generics_ty >
              >
            > // Angle bracket on new line
            {
              #implicit_former_name::begin( None, None, #end_struct_name::< #enum_generics_ty >::default() )
            }
          };
          methods.push( static_method );
          // Implicit former components are already pushed to end_impls by the helper function
        }

        // --- Standalone Constructor (Named Fields) --- Option 2 Logic ---
        // This logic now applies regardless of `wants_scalar`
        if struct_attrs.standalone_constructors.value( false )
        {
          // Generate constructor parameters based *only* on #[arg_for_constructor]
          let constructor_params = variant_field_info
            .iter()
            .filter( | f_info | f_info.is_constructor_arg ) // Space around |
            .map( | f_info | // Space around |
            {
              let param_name = &f_info.ident;
              let ty = &f_info.ty;
              quote! { #param_name : impl Into< #ty > }
            });

          // Determine if all fields are args
          let all_fields_are_args = variant_field_info.iter().all( | f_info | f_info.is_constructor_arg ); // Space around |

          // Determine return type and body based on Option 2 rule
          let ( return_type, constructor_body ) = if all_fields_are_args
          {
            // Return Self
            let return_type = quote! { #enum_name< #enum_generics_ty > };
            let construction_args = variant_field_info.iter().map( | f_info | // Space around |
            {
              let field_ident = &f_info.ident; // Use the actual field identifier
              let param_name = ident::ident_maybe_raw( field_ident ); // Handle raw idents if needed
              quote! { #field_ident : #param_name.into() }
            });
            let body = quote! { #enum_name::#variant_ident { #( #construction_args ),* } };
            ( return_type, body )
          }
          else
          {
            // Return Implicit Former
            let former_return_type = quote!
            {
              #implicit_former_name
              < // Angle bracket on new line
                #enum_generics_ty // Enum generics
                #implicit_def_name // Implicit definition
                <
                  #enum_generics_ty // Enum generics
                  (), // Context
                  #enum_name< #enum_generics_ty >, // Formed
                  #end_struct_name < #enum_generics_ty > // End
                >
              >
            };
            // Initialize storage based on constructor args
            let initial_storage_fields = variant_field_info
              .iter()
              .map( | f_info | // Space around |
              {
                let field_ident = &f_info.ident;
                let param_name = ident::ident_maybe_raw( field_ident );
                if f_info.is_constructor_arg
                {
                  quote! { #field_ident : ::core::option::Option::Some( #param_name.into() ) }
                }
                else
                {
                  quote! { #field_ident : ::core::option::Option::None }
                }
              });
            let initial_storage_code = quote!
            {
              ::core::option::Option::Some
              ( // Paren on new line
                #implicit_storage_name :: < #enum_generics_ty > // Add generics
                {
                  #( #initial_storage_fields, )*
                  _phantom : ::core::marker::PhantomData // Add phantom if needed
                }
              ) // Paren on new line
            };
            let former_body = quote!
            {
              #implicit_former_name::begin
              ( // Paren on new line
                #initial_storage_code,
                None, // Context
                #end_struct_name::< #enum_generics_ty >::default() // End
              )
            };
            ( former_return_type, former_body )
          };

          // Generate the constructor function code
          let constructor = quote!
          {
            /// Standalone constructor for the #variant_ident variant.
            #[ inline( always ) ]
            #vis fn #method_name < #enum_generics_impl >
            (
              #( #constructor_params ),*
            )
            ->
            #return_type // Use determined return type
            where
              #enum_generics_where
            {
              #constructor_body // Use determined body
            }
          };
          standalone_constructors.push( constructor );
        }
        // --- End Standalone Constructor ---

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
      {
          #( #methods )* // Splice the collected methods here
      }

      // Define the End structs, implicit formers, etc., outside the enum impl block.
      #( #end_impls )*

      #( #standalone_constructors )*
  };

  if has_debug // Print generated code if #[debug] is present on the enum
  {
    let about = format!( "derive : Former\nenum : {enum_name}" );
    diag::report_print( about, original_input, &result );
  }

  Ok( result )
}

/// Helper function to generate the implicit former infrastructure for a variant.
/// Returns a tuple: (`TokenStream` for all components`TokenStream`am for setters only)
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn generate_implicit_former_for_variant
(
  vis : &syn::Visibility,
  enum_name : &syn::Ident,
  variant_ident : &syn::Ident,
  fields : &syn::Fields,
  generics : &syn::Generics,
  implicit_former_name : &syn::Ident,
  implicit_storage_name : &syn::Ident,
  implicit_def_name : &syn::Ident,
  implicit_def_types_name : &syn::Ident,
  end_struct_name : &syn::Ident,
  _original_input : &proc_macro::TokenStream,
  variant_field_info : &[EnumVariantFieldInfo], // <<< Changed parameter
) -> Result< ( TokenStream, TokenStream ) >
{
  // --- Use pre-collected field data ---
  let field_data_vec = variant_field_info; // <<< Use passed info
  let ( _enum_generics_with_defaults, enum_generics_impl, enum_generics_ty, enum_generics_where ) = generic_params::decompose( generics );
  // --- End Use ---


  // --- Generate code snippets using the owned FieldData ---
  let storage_field_definitions = field_data_vec.iter().map( | f_data | // Space around |
  {
      let ident = &f_data.ident;
      let ty = &f_data.ty;
      let is_optional = typ::is_optional( ty ); // <<< Calculate is_optional
      let ty2 = if is_optional { quote! { #ty } } else { quote! { ::core::option::Option< #ty > } };
      quote! { pub #ident : #ty2 }
  });

  let storage_field_defaults = field_data_vec.iter().map( | f_data | // Space around |
  {
      let ident = &f_data.ident;
      quote! { #ident : ::core::option::Option::None }
  });

  let phantom_field_type_storage = phantom::tuple( &enum_generics_ty );

  let implicit_storage_struct = quote!
  {
    #[ derive( Debug ) ]
    #vis struct #implicit_storage_name < #enum_generics_impl >
    where // Where clause on new line
      #enum_generics_where
    {
      #( #storage_field_definitions, )*
      _phantom : #phantom_field_type_storage,
    }
    impl< #enum_generics_impl > ::core::default::Default
    for #implicit_storage_name < #enum_generics_ty >
    where // Where clause on new line
      #enum_generics_where
    {
      #[ inline( always ) ]
      fn default() -> Self
      {
        Self { #( #storage_field_defaults, )* _phantom: ::core::marker::PhantomData }
      }
    }
  };

  let storage_preform_fields = field_data_vec.iter().map( |f_data|
  {
      let ident = &f_data.ident;
      let ty = &f_data.ty;
      let is_optional = typ::is_optional( ty ); // <<< Calculate is_optional




      // Get the default value expression directly if present
      let default : Option< &syn::Expr > = f_data.attrs.config
        .as_ref()
        .map( | attr | &attr.default ) // Space around |
        .and_then( | prop | prop.ref_internal() ); // Space around |
      // <<< End Correction >>>


      if is_optional
      {
          let _else = match default
          {
              None => quote! { ::core::option::Option::None },
              Some( default_val ) => quote! { ::core::option::Option::Some( ::core::convert::Into::into( #default_val ) ) },
          };
          Ok( quote!
          {
              let #ident = if self.#ident.is_some()
              {
                  ::core::option::Option::Some( self.#ident.take().unwrap() )
              }
              else
              {
                  #_else
              };
          })
      }
      else
      {
          let _else = match default
          {
              None =>
              {
                  let panic_msg = format!( "Field '{ident}' isn't initialized" );
                  quote!
                  {
                      {
                          trait MaybeDefault< T > { fn maybe_default( self : &Self ) -> T { panic!( #panic_msg ) } }
                          impl< T > MaybeDefault< T > for &::core::marker::PhantomData< T > {}
                          impl< T > MaybeDefault< T > for ::core::marker::PhantomData< T > where T : ::core::default::Default, { fn maybe_default( self : &Self ) -> T { T::default() } }
                          ( &::core::marker::PhantomData::< #ty > ).maybe_default()
                      }
                  }
              },
              Some( default_val ) => quote! { ::core::convert::Into::into( #default_val ) },
          };
          Ok( quote!
          {
              let #ident = if self.#ident.is_some()
              {
                  self.#ident.take().unwrap()
              }
              else
              {
                  #_else
              };
          })
      }
  }).collect::< Result< Vec< _ > > >()?; // <<< Collect Result

  let storage_preform_field_names_vec : Vec<_> = field_data_vec.iter().map( | f | &f.ident ).collect(); // Space around |

  // Determine the preformed type and variant construction based on field kind
  let ( preformed_type, variant_construction ) = match fields
  {
      syn::Fields::Named( _ ) => // Use _ as we use field_data_vec now
      {
          let preformed_tuple_types = field_data_vec.iter().map( | f | &f.ty ); // Space around |
          (
              quote!{ ( #( #preformed_tuple_types ),* ) }, // Preformed is a tuple for named fields
              quote!{ #enum_name::#variant_ident { #( #storage_preform_field_names_vec ),* } }
          )
      },
      syn::Fields::Unnamed( _ ) => // Use _ as we use field_data_vec now
      {
          let field_types = field_data_vec.iter().map( | f | &f.ty ); // Space around |
          (
              quote!{ ( #( #field_types ),* ) }, // Preformed is a tuple for unnamed fields
              quote!{ #enum_name::#variant_ident( #( #storage_preform_field_names_vec ),* ) }
          )
      },
      syn::Fields::Unit => unreachable!(),
  };


  let implicit_storage_preform = quote!
  {
    impl< #enum_generics_impl > former::Storage
    for #implicit_storage_name < #enum_generics_ty >
    where // Where clause on new line
      #enum_generics_where
    {
      type Preformed = #preformed_type;
    }
    impl< #enum_generics_impl > former::StoragePreform
    for #implicit_storage_name < #enum_generics_ty >
    where // Where clause on new line
      #enum_generics_where
    {
      fn preform( mut self ) -> Self::Preformed
      {
        #( #storage_preform_fields )*
        ( #( #storage_preform_field_names_vec ),* )
      }
    }
  };

  let ( former_definition_types_generics_with_defaults, former_definition_types_generics_impl, former_definition_types_generics_ty, former_definition_types_generics_where )
    = generic_params::decompose( &generics_of_definition_types_renamed( generics, enum_name, &enum_generics_ty ) );
  let former_definition_types_phantom = macro_tools::phantom::tuple( &former_definition_types_generics_impl );

  let implicit_def_types = quote!
  {
    #[ derive( Debug ) ]
    #vis struct #implicit_def_types_name < #former_definition_types_generics_with_defaults >
    where // Where clause on new line
      #former_definition_types_generics_where
    {
      _phantom : #former_definition_types_phantom
    }
    impl < #former_definition_types_generics_impl > ::core::default::Default
    for #implicit_def_types_name < #former_definition_types_generics_ty >
    where // Where clause on new line
      #former_definition_types_generics_where
    {
      fn default() -> Self { Self { _phantom : ::core::marker::PhantomData } }
    }
    impl < #former_definition_types_generics_impl > former::FormerDefinitionTypes
    for #implicit_def_types_name < #former_definition_types_generics_ty >
    where // Where clause on new line
      #former_definition_types_generics_where
    {
      type Storage = #implicit_storage_name < #enum_generics_ty >;
      type Formed = Formed2;
      type Context = Context2;
    }
    impl< #former_definition_types_generics_impl > former::FormerMutator
    for #implicit_def_types_name < #former_definition_types_generics_ty >
    where // Where clause on new line
      #former_definition_types_generics_where {}
  };

  let ( former_definition_generics_with_defaults, former_definition_generics_impl, former_definition_generics_ty, former_definition_generics_where )
    = generic_params::decompose( &generics_of_definition_renamed( generics, enum_name, &enum_generics_ty, end_struct_name ) );
  let former_definition_phantom = macro_tools::phantom::tuple( &former_definition_generics_impl );

  let implicit_def = quote!
  {
    #[ derive( Debug ) ]
    #vis struct #implicit_def_name < #former_definition_generics_with_defaults >
    where // Where clause on new line
      #former_definition_generics_where
    {
      _phantom : #former_definition_phantom
    }
    impl < #former_definition_generics_impl > ::core::default::Default
    for #implicit_def_name < #former_definition_generics_ty >
    where // Where clause on new line
      #former_definition_generics_where
    {
      fn default() -> Self { Self { _phantom : ::core::marker::PhantomData } }
    }
    impl < #former_definition_generics_impl > former::FormerDefinition
    for #implicit_def_name < #former_definition_generics_ty >
    where // Where clause on new line
      End2 : former::FormingEnd< #implicit_def_types_name < #former_definition_types_generics_ty > >,
      #former_definition_generics_where
    {
      type Types = #implicit_def_types_name < #former_definition_types_generics_ty >;
      type End = End2;
      type Storage = #implicit_storage_name < #enum_generics_ty >;
      type Formed = Formed2;
      type Context = Context2;
    }
  };

  let former_generics_result = generics_of_former_renamed( generics, implicit_def_name, implicit_storage_name, &enum_generics_ty, enum_name, end_struct_name );
  let ( former_generics_with_defaults, former_generics_impl, former_generics_ty, former_generics_where )
    = generic_params::decompose( &former_generics_result );

  // --- Generate setters using owned FieldData ---
  let former_field_setters = field_data_vec.iter().map( | f_data | // Space around |
  {
      let field_ident = &f_data.ident;
      let ty = &f_data.ty; // Use original type for setter input
      let is_optional = typ::is_optional( ty ); // <<< Calculate is_optional
      let non_optional_typ = if is_optional { typ::parameter_first( ty )? } else { ty }; // <<< Calculate non_optional_ty
      // Use field identifier as setter name for implicit former (e.g., _0, _1 for tuple variants)
      let setter_name = &f_data.ident;
      let doc = format!( "Setter for the '{field_ident}' field." );

      Ok( quote!
      {
          #[ doc = #doc ]
          #[ inline ]
          pub fn #setter_name< Src >( mut self, src : Src ) -> Self
          where // Where clause on new line
              Src : ::core::convert::Into< #non_optional_typ >, // <<< Use calculated non_optional_typ
          {
              debug_assert!( self.storage.#field_ident.is_none() );
              self.storage.#field_ident = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
              self
          }
      })
  }).collect::< Result< Vec< _ > > >()?; // <<< Collect Result
  // --- End setter generation ---

  let implicit_former_struct = quote!
  {
    #[ doc = "Implicit former for the struct-like variant" ]
    #vis struct #implicit_former_name < #former_generics_with_defaults >
    where // Where clause on new line
      #former_generics_where
    {
      storage : Definition::Storage,
      context : ::core::option::Option< Definition::Context >,
      on_end : ::core::option::Option< Definition::End >,
    }
    #[ automatically_derived ]
    impl < #former_generics_impl > #implicit_former_name < #former_generics_ty >
    where // Where clause on new line
      #former_generics_where
    {
      #[ inline( always ) ] pub fn form( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed { self.end() }
      #[ inline( always ) ] pub fn end( mut self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      {
        let on_end = self.on_end.take().unwrap();
        let mut context = self.context.take();
        < Definition::Types as former::FormerMutator >::form_mutation( &mut self.storage, &mut context );
        former::FormingEnd::< Definition::Types >::call( &on_end, self.storage, context )
      }
      #[ inline( always ) ] pub fn begin
      ( // Paren on new line
        storage : ::core::option::Option< Definition::Storage >,
        context : ::core::option::Option< Definition::Context >,
        on_end : Definition::End
      ) // Paren on new line
      -> Self
      {
        Self { storage : storage.unwrap_or_default(), context, on_end : ::core::option::Option::Some( on_end ) }
      }
      #[ inline( always ) ] pub fn new( on_end : Definition::End ) -> Self
      {
        Self::begin( None, None, on_end )
      }
      #( #former_field_setters )*
    }
  };

  let phantom_field_type_end = phantom::tuple( &enum_generics_ty );
  let end_struct_def = quote!
  {
    #[ derive( Default, Debug ) ]
    #vis struct #end_struct_name < #enum_generics_impl >
    where // Where clause on new line
      #enum_generics_where // Use original enum where clause
    {
      _phantom : #phantom_field_type_end,
    }
  };

  let end_impl = quote!
  {
    #[ automatically_derived ]
    impl< #enum_generics_impl > former::FormingEnd
    < // Angle bracket on new line
      #implicit_def_types_name< #enum_generics_ty (), #enum_name< #enum_generics_ty > >
    > // Angle bracket on new line
    for #end_struct_name < #enum_generics_ty >
    where // Where clause on new line
      #enum_generics_where // Use original enum where clause
    {
      #[ inline( always ) ]
      fn call
      ( // Paren on new line
        &self,
        sub_storage : #implicit_storage_name< #enum_generics_ty >,
        _context : Option< () >,
      ) // Paren on new line
      -> // Return type on new line
      #enum_name< #enum_generics_ty >
      {
        let ( #( #storage_preform_field_names_vec ),* ) = former::StoragePreform::preform( sub_storage );
        #variant_construction
      }
    }
  };

  let all_components = quote!
  {
      #implicit_storage_struct
      #implicit_storage_preform
      #implicit_def_types
      #implicit_def
      #implicit_former_struct
      #end_struct_def
      #end_impl
  };

  Ok( ( all_components, quote!( #( #former_field_setters )* ) ) )
}


// Helper functions to generate generics for implicit definitions
// (These are simplified versions of what's used for structs)
// Renamed versions to avoid conflicts with struct helpers if they existed in the same scope.

fn generics_of_definition_types_renamed // Renamed
(
  enum_generics : &syn::Generics,
  _enum_name : &syn::Ident,
  enum_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
) -> syn::Generics
{
  // Use Context2, Formed2
  let extra : macro_tools::GenericsWithWhere = syn::parse_quote!
  {
    < Context2 = (), Formed2 = #_enum_name < #enum_generics_ty > >
  };
  generic_params::merge( enum_generics, &extra.into() )
}

fn generics_of_definition_renamed // Renamed
(
  enum_generics : &syn::Generics,
  _enum_name : &syn::Ident,
  enum_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  end_struct_name : &syn::Ident,
) -> syn::Generics
{
  // Use Context2, Formed2, End2
  let extra : macro_tools::GenericsWithWhere = syn::parse_quote!
  {
    < Context2 = (), Formed2 = #_enum_name < #enum_generics_ty >, End2 = #end_struct_name < #enum_generics_ty > >
  };
  generic_params::merge( enum_generics, &extra.into() )
}

fn generics_of_former_renamed // Renamed
(
  enum_generics : &syn::Generics,
  implicit_def_name : &syn::Ident,
  implicit_storage_name : &syn::Ident,
  enum_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  enum_name : &syn::Ident, // Need enum name for default Formed type
  end_struct_name : &syn::Ident, // Need end struct name for default End type
) -> syn::Generics
{
   let default_definition_type = quote!
   {
      #implicit_def_name < #enum_generics_ty (), #enum_name < #enum_generics_ty >, #end_struct_name < #enum_generics_ty > >
   };

   // Use Definition
   let extra : macro_tools::GenericsWithWhere = syn::parse_quote!
  {
    < Definition = #default_definition_type > // Use the correctly constructed default
    where // Where clause on new line
      Definition : former::FormerDefinition< Storage = #implicit_storage_name < #enum_generics_ty > >,
      Definition::Types : former::FormerDefinitionTypes< Storage = #implicit_storage_name < #enum_generics_ty > >,
  };
  generic_params::merge( enum_generics, &extra.into() )
}