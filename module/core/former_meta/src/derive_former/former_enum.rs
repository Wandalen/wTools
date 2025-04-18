// File: module/core/former_meta/src/derive_former/former_enum.rs
#![ allow( clippy::wildcard_imports ) ]
use super::*; // Use items from parent module (derive_former.rs)
use macro_tools::
{
  generic_params, Result,
  proc_macro2::TokenStream, quote::{ format_ident, quote },
  ident, // Added for ident_maybe_raw
  phantom, // Added for phantom::tuple
  // typ, // Removed unused import
};
// CORRECTED: Re-added Casing import
#[ cfg( feature = "derive_former" ) ]
use convert_case::{ Case, Casing };

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

// ==================================
//        Main Generation Logic
// ==================================

/// Generate the Former ecosystem for an enum.
#[ allow( clippy::too_many_lines ) ]
pub(super) fn former_for_enum // Make it pub(super)
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

  // Initialize vectors to collect generated code pieces
  let mut methods = Vec::new();
  let mut end_impls = Vec::new(); // Needed again for subform variants

  // Iterate through each variant of the enum
  for variant in &data_enum.variants
  {
    let variant_ident = &variant.ident;

    // Generate the snake_case method name, handling potential keywords
    let variant_name_str = variant_ident.to_string();
    // CORRECTED: Reverted to using Snake case
    let method_name_snake_str = variant_name_str.to_case( Case::Snake );
    let method_name_ident_temp = format_ident!( "{}", method_name_snake_str, span = variant_ident.span() );
    let method_name = ident::ident_maybe_raw( &method_name_ident_temp );

    // Parse attributes *from the variant* itself
    let variant_attrs = FieldAttributes::from_attrs( variant.attrs.iter() )?;
    let wants_scalar = variant_attrs.scalar.is_some() && variant_attrs.scalar.as_ref().unwrap().setter();
    let wants_subform_scalar = variant_attrs.subform_scalar.is_some(); // Check for explicit subform_scalar

    // --- Prepare merged where clause for this variant's generated impls ---
    // Start with the enum's where clause. We might add more bounds later if needed
    // specifically by the traits we implement (like FormingEnd often needs Default, Debug etc.)
    // For now, we primarily rely on propagating the enum's constraints.
    // FIX: Removed `mut` as it's not mutated currently.
    let merged_where_clause = enum_generics_where.clone();
    // Example of adding common bounds (adjust as needed based on trait requirements):
    // use std::collections::HashSet;
    // let mut merged_bounds_set = HashSet::new();
    // for pred in &enum_generics_where { merged_bounds_set.insert(quote!{#pred}.to_string()); }
    // for param in generics.params.iter() {
    //     if let syn::GenericParam::Type(tp) = param {
    //         let ident = &tp.ident;
    //         let common_bounds: Vec<syn::WherePredicate> = vec![
    //             syn::parse_quote! { #ident: core::fmt::Debug },
    //             syn::parse_quote! { #ident: core::default::Default },
    //             // Add other common bounds required by FormingEnd, StoragePreform etc.
    //         ];
    //         for bound in common_bounds {
    //             if merged_bounds_set.insert(quote!{#bound}.to_string()) {
    //                 merged_where_clause.push(bound);
    //             }
    //         }
    //     }
    // }
    // --- End merged where clause preparation ---


    // Generate method based on the variant's fields
    match &variant.fields
    {
        // Case 1: Unit variant (e.g., `Empty`) - Always Direct constructor
        syn::Fields::Unit =>
        {
            // FIX: Removed generics from method signature
            let static_method = quote!
            {
              /// Constructor for the #variant_ident unit variant.
              #[ inline( always ) ]
              #vis fn #method_name() -> Self
              // where #enum_generics_where // Removed where clause from method
              {
                Self::#variant_ident
              }
            };
            methods.push( static_method );
        },
        // Case 2: Tuple variant (e.g., `Simple(String)`, `MultiTuple(i32, String)`)
        syn::Fields::Unnamed( fields ) =>
        {
            // Sub-case: Single field tuple variant (e.g., `Simple(String)`)
            if fields.unnamed.len() == 1
            {
                let field = fields.unnamed.first().unwrap();
                let inner_type = &field.ty;

                // Check if the inner type likely has a Former derived (simplistic check)
                // A more robust check would involve trying to resolve the path `::Former`
                // but that's complex in proc macros. This assumes simple paths.
                let inner_former_exists = if let syn::Type::Path( tp ) = inner_type
                {
                  tp.path.segments.last().map_or( false, | seg |
                  {
                    // Heuristic: if it's not a primitive type maybe it has a former
                    !matches!( seg.ident.to_string().as_str(), "bool" | "char" | "str" | "String" | "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "f32" | "f64" )
                  })
                }
                else
                {
                  false // Not a path, unlikely to have a derived Former
                };

                if wants_scalar || ( !wants_subform_scalar && !inner_former_exists )
                {
                    // --- Generate Direct Constructor (Scalar Style) ---
                    // FIX: Removed generics from method signature
                    let static_method = quote!
                    {
                      /// Constructor for the #variant_ident variant (scalar style).
                      /// Takes a value convertible into the inner type #inner_type.
                      #[ inline( always ) ]
                      #vis fn #method_name( value : impl Into< #inner_type > ) -> Self
                      // where #enum_generics_where // Removed where clause from method
                      {
                        Self::#variant_ident( value.into() )
                      }
                    };
                    methods.push( static_method );
                }
                else // Default or explicit subform_scalar -> Generate Subformer
                {
                    // --- Generate Subformer Starter + End Logic ---

                    let end_struct_name = format_ident!( "{}{}End", enum_name, variant_ident );

                    // Attempt to extract name and generics from the inner type path
                    let ( inner_type_name, inner_generics ) = match inner_type
                    {
                        syn::Type::Path( type_path ) =>
                        {
                          let segment = type_path.path.segments.last().ok_or_else( || syn::Error::new_spanned( inner_type, "Cannot derive name from type path") )?;
                          ( segment.ident.clone(), segment.arguments.clone() )
                        },
                        _ => return Err( syn::Error::new_spanned( inner_type, "Inner variant type must be a path type (like MyStruct or MyStruct<T>) to derive Former" ) ),
                    };

                    let inner_former_name = format_ident!( "{}Former", inner_type_name );
                    let inner_storage_name = format_ident!( "{}FormerStorage", inner_type_name );
                    let inner_def_name = format_ident!( "{}FormerDefinition", inner_type_name );
                    let inner_def_types_name = format_ident!( "{}FormerDefinitionTypes", inner_type_name );

                    // Extract type arguments from inner_generics PathArguments
                    let inner_generics_ty : syn::punctuated::Punctuated<syn::GenericArgument, syn::token::Comma> = match &inner_generics
                    {
                        syn::PathArguments::AngleBracketed( args ) => args.args.clone(),
                        _ => syn::punctuated::Punctuated::new(), // Handle cases without generics like `MyStruct`
                    };
                    // Add comma if generics are present
                    let inner_generics_ty_comma = if inner_generics_ty.is_empty() { quote!{} } else { quote!{ #inner_generics_ty, } };

                    // Create PhantomData type using enum's generics
                    let phantom_field_type = phantom::tuple( &enum_generics_ty );

                    // Define the End struct with enum's generics and merged where clause
                    let end_struct_def = quote!
                    {
                      #[ derive( Default, Debug ) ]
                      #vis struct #end_struct_name < #enum_generics_impl >
                      // Use the potentially merged where clause here
                      where #merged_where_clause
                      {
                        _phantom : #phantom_field_type,
                      }
                    };

                    // Implement FormingEnd for the End struct
                    let end_impl = quote!
                    {
                      #[ automatically_derived ]
                      // Use enum's impl generics here
                      impl< #enum_generics_impl > former::FormingEnd
                      <
                          // Use DefinitionTypes of the *inner* type's former
                          // Specify its generics, context=(), formed=Enum<EnumGenerics>
                          #inner_def_types_name< #inner_generics_ty_comma (), #enum_name< #enum_generics_ty > >
                      >
                      // Use enum's type generics here
                      for #end_struct_name < #enum_generics_ty >
                      // Use the potentially merged where clause here
                      where
                        #merged_where_clause
                      {
                          #[ inline( always ) ]
                          fn call
                          (
                            &self,
                            // Storage is from the *inner* type's former, specialized with its generics
                            sub_storage : #inner_storage_name< #inner_generics_ty >,
                            _context : Option< () >, // Context is () from static method
                          ) -> #enum_name< #enum_generics_ty > // Returns the Enum type specialized with its generics
                          {
                            // Preform the inner data and wrap it in the enum variant
                            let data = former::StoragePreform::preform( sub_storage );
                            #enum_name::#variant_ident( data )
                          }
                      }
                    };

                    // Define the static starter method on the enum
                    // FIX: Removed generics and where clause from method signature
                    let static_method = quote!
                    {
                      /// Starts forming the #variant_ident variant using a subformer.
                      #[ inline( always ) ]
                      #vis fn #method_name ()
                      // Return type is the *inner* type's former...
                      -> #inner_former_name
                         <
                           #inner_generics_ty_comma // ...specialized with its own generics...
                           // ...and configured with a definition that uses the specialized End struct.
                           #inner_def_name
                           <
                               #inner_generics_ty_comma // Inner type generics
                               (),                    // Context = ()
                               #enum_name< #enum_generics_ty >, // Formed = Enum<EnumGenerics>
                               #end_struct_name < #enum_generics_ty > // End = Specialized End<EnumGenerics>
                           >
                         >
                      // where #merged_where_clause // Removed where clause from method
                      {
                          // Start the inner former using its `begin` associated function.
                          // The End struct passed depends on the enum's generics.
                          #inner_former_name::begin( None, None, #end_struct_name::< #enum_generics_ty >::default() )
                      }
                    };

                    methods.push( static_method );
                    end_impls.push( quote!{ #end_struct_def #end_impl } ); // Collect End struct and its impl
                }
            }
            // Sub-case: Multi-field tuple variant
            else
            {
                if wants_scalar
                {
                    // --- Generate Direct Constructor (Multi-Arg) ---
                    let mut params = Vec::new();
                    let mut args = Vec::new();
                    for ( i, field ) in fields.unnamed.iter().enumerate()
                    {
                        let param_name = format_ident!( "field{}", i );
                        let field_type = &field.ty;
                        params.push( quote! { #param_name : impl Into< #field_type > } );
                        args.push( quote! { #param_name.into() } );
                    }

                    // FIX: Removed generics from method signature
                    let static_method = quote!
                    {
                      /// Constructor for the #variant_ident variant with multiple fields (scalar style).
                      #[ inline( always ) ]
                      #vis fn #method_name( #( #params ),* ) -> Self
                      // where #enum_generics_where // Removed where clause from method
                      {
                        Self::#variant_ident( #( #args ),* )
                      }
                    };
                    methods.push( static_method );
                }
                else // Default: Subformer (unsupported for multi-field tuple)
                {
                    return Err
                    (
                      syn::Error::new_spanned
                      (
                        variant,
                        "Former derive on enums does not support the default subformer pattern for multi-field tuple variants.\nAdd the `#[ scalar ]` attribute to the variant, e.g., `#[ derive( Former ) ] enum MyEnum { #[ scalar ] MyVariant( T1, T2 ) }`, to generate a static constructor method `MyEnum::my_variant( T1, T2 ) -> MyEnum` instead."
                      )
                    );
                }
            }
        },
        // Case 3: Struct variant
        syn::Fields::Named( fields ) =>
        {
          // --- Generate Implicit Former + Subformer Starter + End Logic ---

          let implicit_former_name = format_ident!( "{}{}Former", enum_name, variant_ident );
          let implicit_storage_name = format_ident!( "{}{}FormerStorage", enum_name, variant_ident );
          let implicit_def_name = format_ident!( "{}{}FormerDefinition", enum_name, variant_ident );
          let implicit_def_types_name = format_ident!( "{}{}FormerDefinitionTypes", enum_name, variant_ident );
          let end_struct_name = format_ident!( "{}{}End", enum_name, variant_ident );

          let variant_struct_fields = fields.named.iter().cloned().collect();
          let variant_struct = syn::ItemStruct
          {
              attrs: vec![],
              vis: vis.clone(),
              struct_token: Default::default(),
              ident: implicit_former_name.clone(),
              generics: generics.clone(), // Use enum's generics for the implicit struct
              fields: syn::Fields::Named( syn::FieldsNamed { brace_token: Default::default(), named: variant_struct_fields } ),
              semi_token: None,
          };

          // --- Generate Implicit Former Components ---

          // 1. Implicit Storage Struct
          let storage_fields_processed : Vec<_> = fields.named.iter()
            .map( |f| FormerField::from_syn( f, true, true ) )
            .collect::< Result< _ > >()?;

          let storage_field_definitions = storage_fields_processed.iter().map( |f| f.storage_field_optional() );
          let storage_field_defaults = storage_fields_processed.iter().map( |f| f.storage_fields_none() );

          // Use enum's generics for storage phantom data
          let phantom_field_type_storage = phantom::tuple( &enum_generics_ty );
          let implicit_storage_struct = quote!
          {
            #[ derive( Debug ) ]
            #vis struct #implicit_storage_name < #enum_generics_impl >
            where #enum_generics_where // Use enum's where clause
            {
              #( #storage_field_definitions, )*
              _phantom : #phantom_field_type_storage,
            }
            impl< #enum_generics_impl > ::core::default::Default for #implicit_storage_name < #enum_generics_ty >
            where #enum_generics_where // Use enum's where clause
            {
              #[ inline( always ) ]
              fn default() -> Self { Self { #( #storage_field_defaults, )* _phantom: ::core::marker::PhantomData } }
            }
          };

          // 2. Implicit StoragePreform
          let storage_preform_fields = storage_fields_processed.iter().map( |f| f.storage_field_preform() ).collect::< Result< Vec<_> > >()?;
          let storage_preform_field_names_vec : Vec<_> = storage_fields_processed.iter().map( |f| f.ident ).collect();
          // The preformed type is a tuple of the *actual* field types from the variant
          let preformed_tuple_types = fields.named.iter().map( |f| &f.ty );
          let preformed_type = quote!{ ( #( #preformed_tuple_types ),* ) };

          let implicit_storage_preform = quote!
          {
            impl< #enum_generics_impl > former::Storage for #implicit_storage_name < #enum_generics_ty >
            where #enum_generics_where // Use enum's where clause
            {
              type Preformed = #preformed_type;
            }
            impl< #enum_generics_impl > former::StoragePreform for #implicit_storage_name < #enum_generics_ty >
            where #enum_generics_where // Use enum's where clause
            {
              fn preform( mut self ) -> Self::Preformed
              {
                #( #storage_preform_fields )*
                ( #( #storage_preform_field_names_vec ),* )
              }
            }
          };

          // 3. Implicit DefinitionTypes
          // Use helper to generate generics like <'a, T, Context2=(), Formed2=Enum<'a, T>>
          let ( former_definition_types_generics_with_defaults, former_definition_types_generics_impl, former_definition_types_generics_ty, former_definition_types_generics_where )
            = generic_params::decompose( &generics_of_definition_types_renamed( &generics, enum_name, &enum_generics_ty )? );
          let former_definition_types_phantom = macro_tools::phantom::tuple( &former_definition_types_generics_impl );

          let implicit_def_types = quote!
          {
            #[ derive( Debug ) ]
            #vis struct #implicit_def_types_name < #former_definition_types_generics_with_defaults >
            where #former_definition_types_generics_where // Merged where clause
            { _phantom : #former_definition_types_phantom }

            impl < #former_definition_types_generics_impl > ::core::default::Default
            for #implicit_def_types_name < #former_definition_types_generics_ty >
            where #former_definition_types_generics_where // Merged where clause
            { fn default() -> Self { Self { _phantom : ::core::marker::PhantomData } } }

            impl < #former_definition_types_generics_impl > former::FormerDefinitionTypes
            for #implicit_def_types_name < #former_definition_types_generics_ty >
            where #former_definition_types_generics_where // Merged where clause
            {
              type Storage = #implicit_storage_name < #enum_generics_ty >; // Storage uses enum generics
              type Formed = Formed2; // Use renamed generic
              type Context = Context2; // Use renamed generic
            }
            impl< #former_definition_types_generics_impl > former::FormerMutator
            for #implicit_def_types_name < #former_definition_types_generics_ty >
            where #former_definition_types_generics_where {} // Merged where clause
          };

          // 4. Implicit Definition
          // Use helper to generate generics like <'a, T, Context2=(), Formed2=Enum<'a, T>, End2=EnumVariantEnd<'a, T>>
          let ( former_definition_generics_with_defaults, former_definition_generics_impl, former_definition_generics_ty, former_definition_generics_where )
            = generic_params::decompose( &generics_of_definition_renamed( &generics, enum_name, &enum_generics_ty, &end_struct_name )? );
          let former_definition_phantom = macro_tools::phantom::tuple( &former_definition_generics_impl );

          let implicit_def = quote!
          {
            #[ derive( Debug ) ]
            #vis struct #implicit_def_name < #former_definition_generics_with_defaults >
            where #former_definition_generics_where // Merged where clause
            { _phantom : #former_definition_phantom }

            impl < #former_definition_generics_impl > ::core::default::Default
            for #implicit_def_name < #former_definition_generics_ty >
            where #former_definition_generics_where // Merged where clause
            { fn default() -> Self { Self { _phantom : ::core::marker::PhantomData } } }

            impl < #former_definition_generics_impl > former::FormerDefinition
            for #implicit_def_name < #former_definition_generics_ty >
            where
              End2 : former::FormingEnd< #implicit_def_types_name < #former_definition_types_generics_ty > >, // Use renamed End2
              #former_definition_generics_where // Merged where clause
            {
              type Types = #implicit_def_types_name < #former_definition_types_generics_ty >;
              type End = End2; // Use renamed End2
              type Storage = #implicit_storage_name < #enum_generics_ty >; // Storage uses enum generics
              type Formed = Formed2; // Use renamed Formed2
              type Context = Context2; // Use renamed Context2
            }
          };

          // 5. Implicit Former Struct + Setters
          // Use helper to generate generics like <'a, T, Definition=...> where Definition : ...
          let former_generics_result = generics_of_former_renamed
          (
            &generics,
            &implicit_def_name,
            &implicit_storage_name,
            &enum_generics_ty,
            enum_name,
            &end_struct_name
          )?;
          let ( former_generics_with_defaults, former_generics_impl, former_generics_ty, former_generics_where )
            = generic_params::decompose( &former_generics_result );

          // Get where clause from the original enum generics
          let default_where_predicates = syn::punctuated::Punctuated::< syn::WherePredicate, syn::token::Comma >::new();
          let variant_struct_where = variant_struct.generics.where_clause.as_ref().map_or
          (
            &default_where_predicates,
            | wc | &wc.predicates
          );

          // Generate setters using the implicit former's details
          let setters = storage_fields_processed.iter().map( |f|
            {
              f.former_field_setter
              (
                &variant_struct.ident, // Use the implicit former's name as the item context for setters
                original_input,
                &variant_struct.generics.params, // Use enum generics for the struct context
                &variant_struct.generics.params, // Use enum generics for the struct context
                variant_struct_where,            // Use enum where clause
                &implicit_former_name,           // The former being defined
                &former_generics_impl,           // Its impl generics
                &former_generics_ty,             // Its type generics
                &former_generics_where,          // Its where clause
                &implicit_storage_name,          // Its storage
              )
            })
            .collect::< Result< Vec<_> > >()?;
          let ( former_field_setters, _namespace_code ) = setters.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

          let implicit_former_struct = quote!
          {
            #[ doc = "Implicit former for the struct-like variant" ]
            #vis struct #implicit_former_name < #former_generics_with_defaults >
            where #former_generics_where // Use former's where clause
            {
              storage : Definition::Storage,
              context : ::core::option::Option< Definition::Context >,
              on_end : ::core::option::Option< Definition::End >,
            }

            #[ automatically_derived ]
            impl < #former_generics_impl > #implicit_former_name < #former_generics_ty >
            where #former_generics_where // Use former's where clause
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
              ( storage : ::core::option::Option< Definition::Storage >, context : ::core::option::Option< Definition::Context >, on_end : Definition::End ) -> Self
              { Self { storage : storage.unwrap_or_default(), context, on_end : ::core::option::Option::Some( on_end ) } }
              #[ inline( always ) ] pub fn new( on_end : Definition::End ) -> Self { Self::begin( None, None, on_end ) }

              #( #former_field_setters )*
            }
          };

          // --- Generate End Struct and Impl ---
          let phantom_field_type_end = phantom::tuple( &enum_generics_ty );
          let end_struct_def = quote!
          {
            #[ derive( Default, Debug ) ]
            #vis struct #end_struct_name < #enum_generics_impl >
            where #merged_where_clause // Use merged bounds
            {
              _phantom : #phantom_field_type_end,
            }
          };

          // Construct the final enum variant using field names
          let variant_construction = if fields.named.is_empty()
          { quote! { #enum_name::#variant_ident {} } }
          else
          { quote! { #enum_name::#variant_ident { #( #storage_preform_field_names_vec ),* } } };

          let end_impl = quote!
          {
            #[ automatically_derived ]
            impl< #enum_generics_impl > former::FormingEnd
            <
                // Use DefinitionTypes of the *implicit* former
                #implicit_def_types_name< #enum_generics_ty (), #enum_name< #enum_generics_ty > >
            >
            for #end_struct_name < #enum_generics_ty >
            where
              #merged_where_clause // Use merged bounds
            {
                #[ inline( always ) ]
                fn call
                (
                  &self,
                  // Storage is from the *implicit* former
                  sub_storage : #implicit_storage_name< #enum_generics_ty >,
                  _context : Option< () >, // Context is () from static method
                ) -> #enum_name< #enum_generics_ty > // Returns the Enum type
                {
                  // Preform the tuple of fields from the implicit storage
                  let ( #( #storage_preform_field_names_vec ),* ) = former::StoragePreform::preform( sub_storage );
                  // Construct the enum variant using the field names
                  #variant_construction
                }
            }
          };

          // --- Generate Static Starter Method ---
          // FIX: Removed generics and where clause from method signature
          let static_method = quote!
          {
            /// Starts forming the #variant_ident variant using its implicit subformer.
            #[ inline( always ) ]
            #vis fn #method_name ()
            // Return type is the *implicit* former...
            -> #implicit_former_name
               <
                 #enum_generics_ty // ...specialized with the enum's generics...
                 // ...and configured with a definition that uses the specialized End struct.
                 #implicit_def_name
                 <
                     #enum_generics_ty // Enum generics
                     (),                    // Context = ()
                     #enum_name< #enum_generics_ty >, // Formed = Enum<EnumGenerics>
                     #end_struct_name < #enum_generics_ty > // End = Specialized End<EnumGenerics>
                 >
               >
            // where #merged_where_clause // Removed where clause from method
            {
                // Start the implicit former using its `begin` associated function.
                // The End struct passed depends on the enum's generics.
                #implicit_former_name::begin( None, None, #end_struct_name::< #enum_generics_ty >::default() )
            }
          };

          methods.push( static_method );
          end_impls.push
          (
            quote!
            {
              #implicit_storage_struct
              #implicit_storage_preform
              #implicit_def_types
              #implicit_def
              #implicit_former_struct
              #end_struct_def
              #end_impl
            }
          );

        } // End syn::Fields::Named
    } // End match variant.fields

  } // End variant loop

  // Assemble the final impl block containing the generated static methods
  let result = quote!
  {
      // Implement the static methods on the enum.
      #[ automatically_derived ]
      impl< #enum_generics_impl > #enum_name< #enum_generics_ty >
      where
        #enum_generics_where // <<< USE ORIGINAL ENUM BOUNDS HERE
      {
          #( #methods )* // Splice the collected methods here
      }

      // Define the End structs, implicit formers, etc., outside the enum impl block.
      #( #end_impls )*
  };

  if has_debug // Print generated code if #[debug] is present on the enum
  {
    let about = format!( "derive : Former\nenum : {enum_name}" );
    diag::report_print( about, original_input, &result );
  }

  Ok( result )
}

// Helper functions to generate generics for implicit definitions
// (These are simplified versions of what's used for structs)
// Renamed versions to avoid conflicts with struct helpers if they existed in the same scope.

fn generics_of_definition_types_renamed // Renamed
(
  enum_generics : &syn::Generics,
  _enum_name : &syn::Ident,
  enum_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
) -> Result< syn::Generics >
{
  // Use Context2, Formed2
  let extra : macro_tools::GenericsWithWhere = syn::parse_quote!
  {
    < Context2 = (), Formed2 = #_enum_name < #enum_generics_ty > >
  };
  Ok( generic_params::merge( enum_generics, &extra.into() ) )
}

fn generics_of_definition_renamed // Renamed
(
  enum_generics : &syn::Generics,
  _enum_name : &syn::Ident,
  enum_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  end_struct_name : &syn::Ident,
) -> Result< syn::Generics >
{
  // Use Context2, Formed2, End2
  let extra : macro_tools::GenericsWithWhere = syn::parse_quote!
  {
    < Context2 = (), Formed2 = #_enum_name < #enum_generics_ty >, End2 = #end_struct_name < #enum_generics_ty > >
  };
  Ok( generic_params::merge( enum_generics, &extra.into() ) )
}

fn generics_of_former_renamed // Renamed
(
  enum_generics : &syn::Generics,
  implicit_def_name : &syn::Ident,
  implicit_storage_name : &syn::Ident,
  enum_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  enum_name : &syn::Ident, // Need enum name for default Formed type
  end_struct_name : &syn::Ident, // Need end struct name for default End type
) -> Result< syn::Generics >
{
   let default_definition_type = quote!
   {
      #implicit_def_name < #enum_generics_ty (), #enum_name < #enum_generics_ty >, #end_struct_name < #enum_generics_ty > >
   };

   // Use Definition
   let extra : macro_tools::GenericsWithWhere = syn::parse_quote!
  {
    < Definition = #default_definition_type > // Use the correctly constructed default
    where
      Definition : former::FormerDefinition< Storage = #implicit_storage_name < #enum_generics_ty > >,
      Definition::Types : former::FormerDefinitionTypes< Storage = #implicit_storage_name < #enum_generics_ty > >,
  };
  Ok( generic_params::merge( enum_generics, &extra.into() ) )
}