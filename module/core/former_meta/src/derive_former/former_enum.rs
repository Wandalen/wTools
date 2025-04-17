// File: module/core/former_meta/src/derive_former/former_enum.rs

#![ allow( clippy::wildcard_imports ) ]
use super::*; // Use items from parent module (derive_former.rs)
use macro_tools::
{
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
    let method_name_snake_str = variant_name_str.to_case( Case::Snake );
    let method_name_ident_temp = format_ident!( "{}", method_name_snake_str, span = variant_ident.span() );
    let method_name = ident::ident_maybe_raw( &method_name_ident_temp );

    // Parse attributes *from the variant* itself
    // Using FieldAttributes as it contains the scalar property we need.
    // A dedicated VariantAttributes might be cleaner long-term.
    let variant_attrs = FieldAttributes::from_attrs( variant.attrs.iter() )?;
    let wants_scalar = variant_attrs.scalar.is_some() && variant_attrs.scalar.as_ref().unwrap().setter();

    // Generate method based on the variant's fields
    match &variant.fields
    {
        // Case 1: Unit variant (e.g., `Empty`) - Always Direct constructor
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
            // Sub-case: Single field tuple variant (e.g., `Simple(String)`)
            if fields.unnamed.len() == 1
            {
                let field = fields.unnamed.first().unwrap();
                let inner_type = &field.ty;

                // Determine if the inner type derives Former (heuristic: check if a Former type exists)
                // This is imperfect but a common approach. A better way might involve registry or attributes.
                // let _inner_type_name_str = format!( "{}", quote!{ #inner_type } ).replace( ' ', "" ); // Removed unused
                let inner_former_exists = quote!{ #inner_type::Former }.to_string() != quote!{ < #inner_type as former :: EntityToFormer > :: Former }.to_string(); // Basic check

                if wants_scalar || !inner_former_exists // Treat as scalar if attribute demands OR if inner type likely doesn't have a former
                {
                    // --- Generate Direct Constructor (Scalar Style) ---
                    let static_method = quote!
                    {
                      /// Constructor for the #variant_ident variant (scalar style).
                      /// Takes a value convertible into the inner type #inner_type.
                      #[ inline( always ) ]
                      #vis fn #method_name < #enum_generics_impl > ( value : impl Into< #inner_type > ) -> Self // Add enum generics
                      where #enum_generics_where // Add enum where clause
                      {
                        // Construct the variant, converting the input value
                        Self::#variant_ident( value.into() )
                      }
                    };
                    methods.push( static_method );
                }
                else // Default or explicit subform_scalar -> Generate Subformer
                {
                    // --- Generate Subformer Starter + End Logic ---

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
                      #vis struct #end_struct_name < #enum_generics_impl > // Add enum generics
                      where #enum_generics_where // Add enum where clause
                      {
                        _phantom : core::marker::PhantomData< ( #enum_generics_ty ) >, // Use enum generics
                      }
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
                      for #end_struct_name < #enum_generics_ty > // Add enum generics
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
                      /// Starts forming the #variant_ident variant using a subformer.
                      #[ inline( always ) ]
                      #vis fn #method_name < #enum_generics_impl >() // Add enum generics
                      -> #inner_former_name // Return type is the Former for the inner data type.
                         <
                           #inner_generics_ty // Pass inner type generics
                           // Configure the inner former's definition:
                           #inner_def_name
                           <
                               #inner_generics_ty // Pass inner type generics again
                               (),                             // Context is ().
                               #enum_name< #enum_generics_ty >, // The final type to be Formed is the enum itself.
                               #end_struct_name < #enum_generics_ty > // Use the specialized End struct with enum generics
                           >
                         >
                      where #enum_generics_where // Add enum where clause
                      {
                          // Start the inner former using its `begin` associated function.
                          #inner_former_name::begin( None, None, #end_struct_name::< #enum_generics_ty >::default() ) // Add enum generics to default() call
                      }
                    };

                    methods.push( static_method );
                    end_impls.push( quote!{ #end_struct_def #end_impl } );
                } // End of if/else based on #[scalar]
            }
            // Sub-case: Multi-field tuple variant (e.g., `MultiTuple(i32, String)`)
            else // if fields.unnamed.len() > 1 - Always true here
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

                    let static_method = quote!
                    {
                      /// Constructor for the #variant_ident variant with multiple fields (scalar style).
                      #[ inline( always ) ]
                      #vis fn #method_name < #enum_generics_impl > ( #( #params ),* ) -> Self // Add enum generics
                      where #enum_generics_where // Add enum where clause
                      {
                        Self::#variant_ident( #( #args ),* )
                      }
                    };
                    methods.push( static_method );
                }
                else // Default: Subformer (but unsupported for multi-field tuple)
                {
                    // --- Throw Error ---
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
        // Case 3: Struct variant (e.g., `StructVariant { field: T }`) - NEW LOGIC
        syn::Fields::Named( fields ) =>
        {
          // --- Generate Implicit Former + Subformer Starter + End Logic ---

          // Generate names for the implicit former components based on enum and variant name
          let implicit_former_name = format_ident!( "{}{}Former", enum_name, variant_ident );
          let implicit_storage_name = format_ident!( "{}{}FormerStorage", enum_name, variant_ident );
          let implicit_def_name = format_ident!( "{}{}FormerDefinition", enum_name, variant_ident );
          let implicit_def_types_name = format_ident!( "{}{}FormerDefinitionTypes", enum_name, variant_ident );
          let end_struct_name = format_ident!( "{}{}End", enum_name, variant_ident );

          // Create a temporary syn::ItemStruct to represent the variant's fields
          // This allows reusing the existing former_for_struct logic (partially)
          let variant_struct_fields = fields.named.iter().cloned().collect();
          let variant_struct = syn::ItemStruct
          {
              attrs: vec![], // No attributes needed for this temp struct
              vis: vis.clone(), // Use enum's visibility
              struct_token: Default::default(),
              ident: implicit_former_name.clone(), // Use a derived name
              generics: generics.clone(), // Use enum's generics
              fields: syn::Fields::Named( syn::FieldsNamed { brace_token: Default::default(), named: variant_struct_fields } ),
              semi_token: None,
          };

          // --- Generate Implicit Former Components ---
          // (Leveraging parts of former_for_struct logic might be complex, let's generate directly)

          // 1. Implicit Storage Struct
          let storage_fields_processed : Vec<_> = fields.named.iter()
            .map( |f| FormerField::from_syn( f, true, true ) ) // Treat fields as needing storage and being part of the "formed" (anonymous struct)
            .collect::< Result< _ > >()?;

          let storage_field_definitions = storage_fields_processed.iter().map( |f| f.storage_field_optional() );
          // let _storage_field_defaults = storage_fields_processed.iter().map( |f| f.storage_fields_none() ); // Removed unused

          let implicit_storage_struct = quote!
          {
            #[ derive( Debug, Default ) ]
            #vis struct #implicit_storage_name < #enum_generics_impl > // Add enum generics
            where #enum_generics_where // Add enum where clause
            {
              #( #storage_field_definitions, )*
              _phantom : core::marker::PhantomData< ( #enum_generics_ty ) >, // Use enum generics
            }
          };

          // 2. Implicit StoragePreform
          let storage_preform_fields = storage_fields_processed.iter().map( |f| f.storage_field_preform() ).collect::< Result< Vec<_> > >()?;
          // Collect field names into a Vec first
          let storage_preform_field_names_vec : Vec<_> = storage_fields_processed.iter().map( |f| f.ident ).collect();
          // The "Preformed" type is the anonymous struct `{ field1: T1, field2: T2, ... }`
          // We'll construct this directly in the End::call method.
          // For StoragePreform, we need a placeholder or a tuple of the field types. Let's use a tuple.
          let preformed_tuple_types = fields.named.iter().map( |f| &f.ty );
          let preformed_type = quote!{ ( #( #preformed_tuple_types ),* ) }; // e.g., (String, i32)

          let implicit_storage_preform = quote!
          {
            impl< #enum_generics_impl > former::Storage for #implicit_storage_name < #enum_generics_ty >
            where #enum_generics_where
            {
              type Preformed = #preformed_type;
            }
            impl< #enum_generics_impl > former::StoragePreform for #implicit_storage_name < #enum_generics_ty >
            where #enum_generics_where
            {
              fn preform( mut self ) -> Self::Preformed
              {
                #( #storage_preform_fields )*
                // Return a tuple of the preformed fields using the collected vec
                ( #( #storage_preform_field_names_vec ),* )
              }
            }
          };

          // 3. Implicit DefinitionTypes
          // Use renamed generics: Context2, Formed2
          let ( former_definition_types_generics_with_defaults, former_definition_types_generics_impl, former_definition_types_generics_ty, former_definition_types_generics_where )
            = generic_params::decompose( &generics_of_definition_types_renamed( &generics, enum_name, &enum_generics_ty )? );
          let former_definition_types_phantom = macro_tools::phantom::tuple( &former_definition_types_generics_impl );

          let implicit_def_types = quote!
          {
            #[ derive( Debug ) ]
            #vis struct #implicit_def_types_name < #former_definition_types_generics_with_defaults >
            where #former_definition_types_generics_where
            { _phantom : #former_definition_types_phantom }

            impl < #former_definition_types_generics_impl > ::core::default::Default
            for #implicit_def_types_name < #former_definition_types_generics_ty >
            where #former_definition_types_generics_where
            { fn default() -> Self { Self { _phantom : ::core::marker::PhantomData } } }

            impl < #former_definition_types_generics_impl > former::FormerDefinitionTypes
            for #implicit_def_types_name < #former_definition_types_generics_ty >
            where #former_definition_types_generics_where
            {
              type Storage = #implicit_storage_name < #enum_generics_ty >; // Use enum generics
              type Formed = Formed2; // Use renamed generic Formed2
              type Context = Context2; // Use renamed generic Context2
            }
            impl< #former_definition_types_generics_impl > former::FormerMutator
            for #implicit_def_types_name < #former_definition_types_generics_ty >
            where #former_definition_types_generics_where {} // Default empty mutator
          };

          // 4. Implicit Definition
          // Use renamed generics: Context2, Formed2, End2
          let ( former_definition_generics_with_defaults, former_definition_generics_impl, former_definition_generics_ty, former_definition_generics_where )
            = generic_params::decompose( &generics_of_definition_renamed( &generics, enum_name, &enum_generics_ty, &end_struct_name )? );
          let former_definition_phantom = macro_tools::phantom::tuple( &former_definition_generics_impl );

          let implicit_def = quote!
          {
            #[ derive( Debug ) ]
            #vis struct #implicit_def_name < #former_definition_generics_with_defaults >
            where #former_definition_generics_where
            { _phantom : #former_definition_phantom }

            impl < #former_definition_generics_impl > ::core::default::Default
            for #implicit_def_name < #former_definition_generics_ty >
            where #former_definition_generics_where
            { fn default() -> Self { Self { _phantom : ::core::marker::PhantomData } } }

            impl < #former_definition_generics_impl > former::FormerDefinition
            for #implicit_def_name < #former_definition_generics_ty >
            where
              End2 : former::FormingEnd< #implicit_def_types_name < #former_definition_types_generics_ty > >, // Use renamed End2 and correct DefTypes
              #former_definition_generics_where
            {
              type Types = #implicit_def_types_name < #former_definition_types_generics_ty >; // Use correct DefTypes
              type End = End2; // Use renamed generic End2
              type Storage = #implicit_storage_name < #enum_generics_ty >; // Use enum generics
              type Formed = Formed2; // Use renamed generic Formed2
              type Context = Context2; // Use renamed generic Context2
            }
          };

          // 5. Implicit Former Struct + Setters
          // Use renamed generics: Definition
          // FIX: Correct the call to generics_of_former_renamed and remove the erroneous '?'
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
            = generic_params::decompose( &former_generics_result ); // Decompose the result

          // Safely get where clause predicates without mutable borrow
          let default_where_predicates = syn::punctuated::Punctuated::< syn::WherePredicate, syn::token::Comma >::new();
          let variant_struct_where = variant_struct.generics.where_clause.as_ref().map_or
          (
            &default_where_predicates, // Use reference to the longer-lived default
            | wc | &wc.predicates
          );

          let setters = storage_fields_processed.iter().map( |f|
            {
              // Use the field processing logic, but target the implicit former/storage names
              f.former_field_setter
              (
                &variant_struct.ident, // Use the temp struct ident for context if needed by setter logic
                original_input,
                &variant_struct.generics.params, // Use combined generics for impl
                &variant_struct.generics.params, // Use combined generics for type
                variant_struct_where, // Use safely accessed where clause
                &implicit_former_name, // Target the implicit former
                &former_generics_impl, // Use renamed Definition generic
                &former_generics_ty,   // Use renamed Definition generic
                &former_generics_where,// Use bounds on renamed Definition
                &implicit_storage_name, // Target the implicit storage
              )
            })
            .collect::< Result< Vec<_> > >()?;
          let ( former_field_setters, _namespace_code ) = setters.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
          // We ignore _namespace_code here because sub-sub-forming within enum variants is too complex for now.

          let implicit_former_struct = quote!
          {
            #[ doc = "Implicit former for the struct-like variant" ] // Basic doc
            #vis struct #implicit_former_name < #former_generics_with_defaults > // Uses Definition
            where #former_generics_where // Bounds on Definition
            {
              storage : Definition::Storage,
              context : ::core::option::Option< Definition::Context >,
              on_end : ::core::option::Option< Definition::End >,
            }

            #[ automatically_derived ]
            impl < #former_generics_impl > #implicit_former_name < #former_generics_ty > // Uses Definition
            where #former_generics_where // Bounds on Definition
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
          let end_struct_def = quote!
          {
            #[ derive( Default, Debug ) ]
            #vis struct #end_struct_name < #enum_generics_impl > // Add enum generics
            where #enum_generics_where // Add enum where clause
            {
              _phantom : core::marker::PhantomData< ( #enum_generics_ty ) >, // Use enum generics
            }
          };

          // Use the collected Vec of field names here
          let variant_construction = if fields.named.is_empty()
          { quote! { #enum_name::#variant_ident {} } }
          else
          { quote! { #enum_name::#variant_ident { #( #storage_preform_field_names_vec ),* } } };


          let end_impl = quote!
          {
            #[ automatically_derived ]
            impl< #enum_generics_impl > former::FormingEnd
            <
                // DefinitionTypes of the implicit former: Context=(), Formed=TheEnum<...>
                // Use renamed generics Context2, Formed2
                #implicit_def_types_name< #enum_generics_ty (), #enum_name< #enum_generics_ty > >
            >
            for #end_struct_name < #enum_generics_ty > // Add enum generics
            where // Include where clauses from the enum
              #enum_generics_where
            {
                #[ inline( always ) ]
                fn call
                (
                  &self,
                  sub_storage : #implicit_storage_name< #enum_generics_ty >, // Storage from the implicit former.
                  _context : Option< () >, // Context is () as we start from a static method.
                ) -> #enum_name< #enum_generics_ty > // Returns the final enum instance.
                {
                  // Preform the inner data (tuple) and construct the variant
                  // Use the collected Vec of field names here too
                  let ( #( #storage_preform_field_names_vec ),* ) = former::StoragePreform::preform( sub_storage );
                  #variant_construction
                }
            }
          };

          // --- Generate Static Starter Method ---
          let static_method = quote!
          {
            /// Starts forming the #variant_ident variant using its implicit subformer.
            #[ inline( always ) ]
            #vis fn #method_name < #enum_generics_impl >() // Add enum generics
            -> #implicit_former_name // Return type is the implicit Former for the variant.
               <
                 #enum_generics_ty // Pass enum generics
                 // Configure the implicit former's definition:
                 // Use renamed generics Context2, Formed2, End2
                 #implicit_def_name
                 <
                     #enum_generics_ty // Pass enum generics again
                     (),                             // Context is ().
                     #enum_name< #enum_generics_ty >, // The final type to be Formed is the enum itself.
                     #end_struct_name < #enum_generics_ty > // Use the specialized End struct with enum generics
                 >
               >
            where #enum_generics_where // Add enum where clause
            {
                // Start the implicit former using its `begin` associated function.
                #implicit_former_name::begin( None, None, #end_struct_name::< #enum_generics_ty >::default() ) // Add enum generics to default() call
            }
          };

          methods.push( static_method );
          // Collect all generated components for this variant
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
        #enum_generics_where
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
  let extra : macro_tools::GenericsWithWhere = parse_quote!
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
  let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < Context2 = (), Formed2 = #_enum_name < #enum_generics_ty >, End2 = #end_struct_name < #enum_generics_ty > >
  };
  Ok( generic_params::merge( enum_generics, &extra.into() ) )
}

fn generics_of_former_renamed // Renamed
(
  enum_generics : &syn::Generics,
  implicit_def_name : &syn::Ident,
  // FIX: Remove unused parameter
  // former_definition_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  implicit_storage_name : &syn::Ident,
  enum_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  enum_name : &syn::Ident, // Need enum name for default Formed type
  end_struct_name : &syn::Ident, // Need end struct name for default End type
) -> Result< syn::Generics >
{
   // Construct the correct default type for the Definition parameter
   // FIX: Use the correct generics for the default definition type
   let default_definition_type = quote!
   {
      #implicit_def_name < #enum_generics_ty (), #enum_name < #enum_generics_ty >, #end_struct_name < #enum_generics_ty > >
   };

   // Use Definition
   let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < Definition = #default_definition_type > // Use the correctly constructed default
    where
      Definition : former::FormerDefinition< Storage = #implicit_storage_name < #enum_generics_ty > >,
      Definition::Types : former::FormerDefinitionTypes< Storage = #implicit_storage_name < #enum_generics_ty > >,
  };
  Ok( generic_params::merge( enum_generics, &extra.into() ) )
}