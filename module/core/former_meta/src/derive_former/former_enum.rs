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

    // Merge bounds: Start with enum's where clause + add common bounds needed by former components
    use std::collections::HashSet;
    let mut merged_bounds_set = HashSet::new();
    let mut merged_where_clause = enum_generics_where.clone();

    for pred in &enum_generics_where {
        merged_bounds_set.insert(quote!{#pred}.to_string());
    }

    for param in generics.params.iter() {
        if let syn::GenericParam::Type(tp) = param {
            let ident = &tp.ident;
            // CORRECTED: Add bounds required by the manual implementation/tests
            // Note: This is still a simplification. A robust solution would inspect trait bounds properly.
            let common_bounds: Vec<syn::WherePredicate> = vec![
                parse_quote! { #ident: core::fmt::Debug },
                parse_quote! { #ident: core::default::Default },
                parse_quote! { #ident: core::cmp::PartialEq },
                parse_quote! { #ident: core::clone::Clone },
                // parse_quote! { #ident: BoundA }, // Removed dummy bounds
                // parse_quote! { #ident: BoundB }, // Removed dummy bounds
            ];
            for bound in common_bounds {
                let bound_str = quote!{#bound}.to_string();
                if merged_bounds_set.insert(bound_str) {
                    merged_where_clause.push(bound);
                }
            }
        }
    }


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
              #vis fn #method_name < #enum_generics_impl >() -> Self
              where #enum_generics_where // Use original enum where clause
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

                let inner_former_exists = quote!{ #inner_type::Former }.to_string() != quote!{ < #inner_type as former :: EntityToFormer > :: Former }.to_string();

                if wants_scalar || ( !wants_subform_scalar && !inner_former_exists )
                {
                    // --- Generate Direct Constructor (Scalar Style) ---
                    let static_method = quote!
                    {
                      /// Constructor for the #variant_ident variant (scalar style).
                      /// Takes a value convertible into the inner type #inner_type.
                      #[ inline( always ) ]
                      #vis fn #method_name < #enum_generics_impl > ( value : impl Into< #inner_type > ) -> Self
                      where #enum_generics_where
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

                    let inner_generics_ty : syn::punctuated::Punctuated<syn::GenericArgument, syn::token::Comma> = match &inner_generics
                    {
                        syn::PathArguments::AngleBracketed( args ) => args.args.clone(),
                        _ => syn::punctuated::Punctuated::new(),
                    };
                    let inner_generics_ty_comma = if inner_generics_ty.is_empty() { quote!{} } else { quote!{ #inner_generics_ty, } };

                    let phantom_field_type = phantom::tuple( &enum_generics_ty );

                    let end_struct_def = quote!
                    {
                      #[ derive( Default, Debug ) ]
                      #vis struct #end_struct_name < #enum_generics_impl >
                      where #merged_where_clause
                      {
                        _phantom : #phantom_field_type,
                      }
                    };

                    let end_impl = quote!
                    {
                      #[ automatically_derived ]
                      impl< #enum_generics_impl > former::FormingEnd
                      <
                          #inner_def_types_name< #inner_generics_ty_comma (), #enum_name< #enum_generics_ty > >
                      >
                      for #end_struct_name < #enum_generics_ty >
                      where
                        #merged_where_clause
                      {
                          #[ inline( always ) ]
                          fn call
                          (
                            &self,
                            sub_storage : #inner_storage_name< #inner_generics_ty >,
                            _context : Option< () >,
                          ) -> #enum_name< #enum_generics_ty >
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
                      -> #inner_former_name
                         <
                           #inner_generics_ty_comma
                           #inner_def_name
                           <
                               #inner_generics_ty_comma
                               (),
                               #enum_name< #enum_generics_ty >,
                               #end_struct_name < #enum_generics_ty >
                           >
                         >
                      where #merged_where_clause
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
                      #vis fn #method_name < #enum_generics_impl > ( #( #params ),* ) -> Self
                      where #enum_generics_where
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
              generics: generics.clone(),
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

          let phantom_field_type_storage = phantom::tuple( &enum_generics_ty );
          let implicit_storage_struct = quote!
          {
            #[ derive( Debug ) ]
            #vis struct #implicit_storage_name < #enum_generics_impl >
            where #enum_generics_where
            {
              #( #storage_field_definitions, )*
              _phantom : #phantom_field_type_storage,
            }
            impl< #enum_generics_impl > ::core::default::Default for #implicit_storage_name < #enum_generics_ty >
            where #enum_generics_where
            {
              #[ inline( always ) ]
              fn default() -> Self { Self { #( #storage_field_defaults, )* _phantom: ::core::marker::PhantomData } }
            }
          };

          // 2. Implicit StoragePreform
          let storage_preform_fields = storage_fields_processed.iter().map( |f| f.storage_field_preform() ).collect::< Result< Vec<_> > >()?;
          let storage_preform_field_names_vec : Vec<_> = storage_fields_processed.iter().map( |f| f.ident ).collect();
          let preformed_tuple_types = fields.named.iter().map( |f| &f.ty );
          let preformed_type = quote!{ ( #( #preformed_tuple_types ),* ) };

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
                ( #( #storage_preform_field_names_vec ),* )
              }
            }
          };

          // 3. Implicit DefinitionTypes
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
              type Storage = #implicit_storage_name < #enum_generics_ty >;
              type Formed = Formed2;
              type Context = Context2;
            }
            impl< #former_definition_types_generics_impl > former::FormerMutator
            for #implicit_def_types_name < #former_definition_types_generics_ty >
            where #former_definition_types_generics_where {}
          };

          // 4. Implicit Definition
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

          // 5. Implicit Former Struct + Setters
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

          let default_where_predicates = syn::punctuated::Punctuated::< syn::WherePredicate, syn::token::Comma >::new();
          let variant_struct_where = variant_struct.generics.where_clause.as_ref().map_or
          (
            &default_where_predicates,
            | wc | &wc.predicates
          );

          let setters = storage_fields_processed.iter().map( |f|
            {
              f.former_field_setter
              (
                &variant_struct.ident,
                original_input,
                &variant_struct.generics.params,
                &variant_struct.generics.params,
                variant_struct_where,
                &implicit_former_name,
                &former_generics_impl,
                &former_generics_ty,
                &former_generics_where,
                &implicit_storage_name,
              )
            })
            .collect::< Result< Vec<_> > >()?;
          let ( former_field_setters, _namespace_code ) = setters.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

          let implicit_former_struct = quote!
          {
            #[ doc = "Implicit former for the struct-like variant" ]
            #vis struct #implicit_former_name < #former_generics_with_defaults >
            where #former_generics_where
            {
              storage : Definition::Storage,
              context : ::core::option::Option< Definition::Context >,
              on_end : ::core::option::Option< Definition::End >,
            }

            #[ automatically_derived ]
            impl < #former_generics_impl > #implicit_former_name < #former_generics_ty >
            where #former_generics_where
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
            where #merged_where_clause
            {
              _phantom : #phantom_field_type_end,
            }
          };

          let variant_construction = if fields.named.is_empty()
          { quote! { #enum_name::#variant_ident {} } }
          else
          { quote! { #enum_name::#variant_ident { #( #storage_preform_field_names_vec ),* } } };

          let end_impl = quote!
          {
            #[ automatically_derived ]
            impl< #enum_generics_impl > former::FormingEnd
            <
                #implicit_def_types_name< #enum_generics_ty (), #enum_name< #enum_generics_ty > >
            >
            for #end_struct_name < #enum_generics_ty >
            where
              #merged_where_clause
            {
                #[ inline( always ) ]
                fn call
                (
                  &self,
                  sub_storage : #implicit_storage_name< #enum_generics_ty >,
                  _context : Option< () >,
                ) -> #enum_name< #enum_generics_ty >
                {
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
            #vis fn #method_name ()
            -> #implicit_former_name
               <
                 #enum_generics_ty
                 #implicit_def_name
                 <
                     #enum_generics_ty
                     (),
                     #enum_name< #enum_generics_ty >,
                     #end_struct_name < #enum_generics_ty >
                 >
               >
            where #merged_where_clause
            {
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
   let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < Definition = #default_definition_type > // Use the correctly constructed default
    where
      Definition : former::FormerDefinition< Storage = #implicit_storage_name < #enum_generics_ty > >,
      Definition::Types : former::FormerDefinitionTypes< Storage = #implicit_storage_name < #enum_generics_ty > >,
  };
  Ok( generic_params::merge( enum_generics, &extra.into() ) )
}