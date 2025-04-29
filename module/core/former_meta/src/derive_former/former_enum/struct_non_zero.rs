use super::*;

use macro_tools::
{
  generic_params, Result,
  proc_macro2::TokenStream, quote::{ format_ident, quote },
  ident,
  phantom,
  // diag, // Removed unused import
  parse_quote,
};
use syn::
{
  self,
  // DeriveInput, // Removed unused import
  // Variant, // Removed unused import
  // Visibility, // Removed unused import
  // Generics, // Removed unused import
  // Ident, // Removed unused import
  // Type, // Removed unused import
  Fields,
  Error,
};
// use proc_macro::TokenStream as ProcTokenStream; // Removed unused import
use convert_case::{ Case, Casing };

/// Handles the generation of code for struct variants with non-zero fields.
#[ allow( unused_variables ) ] // qqq : remove after implementing
pub fn handle_struct_non_zero_variant< 'a > // Added explicit lifetime 'a
(
  ast : &'a syn::DeriveInput, // Added lifetime 'a
  variant : &'a syn::Variant, // Added lifetime 'a
  struct_attrs : &'a ItemAttributes, // Added lifetime 'a
  enum_name : &'a syn::Ident, // Added lifetime 'a
  vis : &'a syn::Visibility, // Added lifetime 'a
  generics : &'a syn::Generics, // Added lifetime 'a
  original_input : &'a proc_macro::TokenStream, // Added lifetime 'a
  has_debug : bool,
  methods : &mut Vec<TokenStream>,
  end_impls : &mut Vec<TokenStream>,
  standalone_constructors : &mut Vec<TokenStream>,
  variant_attrs : &'a FieldAttributes, // Added lifetime 'a
  variant_field_info : &'a Vec<EnumVariantFieldInfo>, // Added lifetime 'a
  merged_where_clause : Option< &'a syn::WhereClause >, // Changed type back to Option<&'a WhereClause>
) -> Result< () >
{
  // qqq : reconstruct local variables needed from former_for_enum
  let variant_ident = &variant.ident;
  let method_name = format_ident!( "{}", variant_ident.to_string().to_case( Case::Snake ) );
  let ( enum_generics_impl, enum_generics_ty, enum_generics_where ) = generics.split_for_impl();
  // Check if the attribute is present using .is_some()
  let wants_subform_scalar = variant_attrs.subform_scalar.is_some();
  let wants_scalar = variant_attrs.scalar.is_some();

  match &variant.fields
  {
    Fields::Named( fields ) =>
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
          for field_info in variant_field_info
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
          // Pass generics.params to phantom::tuple
          let phantom_field_type = phantom::tuple( &generics.params );
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
          let field_types : Vec<_> = variant_field_info.iter().map( |f_info| &f_info.ty ).collect(); // Collect types
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
          // Removed line: let enum_generics_ty_no_comma = ...
          let def_generics_impl = generic_params::merge( &generics, &parse_quote!{ < Context2 = (), Formed2 = #enum_name< #enum_generics_ty >, End2 = #end_struct_name< #enum_generics_ty > > } );
          let ( _def_generics_with_defaults, def_generics_impl, def_generics_ty, def_generics_where ) = generic_params::decompose( &def_generics_impl );
          let def_phantom = phantom::tuple( &def_generics_impl );
          let def_struct = quote!
          {
            #[ derive( Debug ) ]
            #vis struct #def_name < #def_generics_impl >
            where // Where clause on new line
              // Use enum_generics_ty directly
              End2 : former::FormingEnd< #def_types_name< #enum_generics_ty, Context2, Formed2 > >,
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
              // Use enum_generics_ty directly
              End2 : former::FormingEnd< #def_types_name< #enum_generics_ty, Context2, Formed2 > >,
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
              // Use enum_generics_ty directly
              End2 : former::FormingEnd< #def_types_name< #enum_generics_ty, Context2, Formed2 > >,
              #def_generics_where
            { // Brace on new line
              type Storage = #storage_struct_name< #enum_generics_ty >;
              type Context = Context2;
              type Formed = Formed2;
              // Use enum_generics_ty directly
              type Types = #def_types_name< #enum_generics_ty, Context2, Formed2 >;
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
          end_impls.push( storage_def );
          end_impls.push( storage_default_impl );
          end_impls.push( storage_trait_impl );
          end_impls.push( storage_preform_impl );
          end_impls.push( def_types_struct );
          end_impls.push( def_types_default_impl );
          end_impls.push( def_types_former_impl );
          end_impls.push( def_types_mutator_impl );
          end_impls.push( def_struct );
          end_impls.push( def_default_impl );
          end_impls.push( def_former_impl );
          end_impls.push( former_struct_def ); // <<< Added Former struct definition

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
    _ => return Err( Error::new_spanned( variant, "Former derive macro only supports named fields for struct variants" ) ), // Added error handling for non-named fields
  }
  Ok( () )
}