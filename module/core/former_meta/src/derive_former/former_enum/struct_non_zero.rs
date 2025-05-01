// File: module/core/former_meta/src/derive_former/former_enum/struct_non_zero.rs
use super::*; // Use items from parent module (former_enum)

use macro_tools::
{
  generic_params, Result,
  quote::{ format_ident, quote }, // Removed unused TokenStream
  ident,
  parse_quote,
};
use syn::
{
  self,
  Fields,
  Error,
  GenericParam,
  TypeParam,
  ConstParam,
  LifetimeParam,
  GenericArgument,
  Expr,
  punctuated::Punctuated,
  token::Comma,
};
use convert_case::{ Case, Casing };

/// Handles the generation of code for struct variants with non-zero fields.
#[ allow( clippy::too_many_lines ) ] // Keep this one for now
pub( super ) fn handle_struct_non_zero_variant< 'a >
(
  ctx : &mut EnumVariantHandlerContext< 'a >, // Changed signature to use context struct
) -> Result< () >
{
  let variant_ident = &ctx.variant.ident;
  // Generate the snake_case method name, handling potential keywords
  let variant_name_str = variant_ident.to_string();
  let method_name_snake_str = variant_name_str.to_case( Case::Snake );
  let method_name_ident_temp = format_ident!( "{}", method_name_snake_str, span = variant_ident.span() );
  let method_name = ident::ident_maybe_raw( &method_name_ident_temp );

  let ( _enum_generics_with_defaults, enum_generics_impl, enum_generics_ty, _enum_generics_where_punctuated ) // Use _ for unused where punctuated
  = generic_params::decompose( ctx.generics );
  // Use the passed Option<&WhereClause>
  let enum_generics_where = ctx.merged_where_clause;

  // Check if the attribute is present using .is_some()
  let wants_subform_scalar = ctx.variant_attrs.subform_scalar.is_some();
  let wants_scalar = ctx.variant_attrs.scalar.is_some();

  // FIX: Helper for conditional comma
  let comma_if_enum_generics = if enum_generics_ty.is_empty() { quote!{} } else { quote!{ , } };


  match &ctx.variant.fields
  {
    Fields::Named( fields ) =>
    { // Opening brace for Fields::Named arm (line 59)
      // --- DEBUG PRINT 3d ---
      // ...
      // --- END DEBUG PRINT 3d ---

      if wants_subform_scalar
      {
          // ... (subform_scalar logic remains the same, but needs comma fix below) ...
          if fields.named.len() > 1
          {
            return Err( syn::Error::new_spanned( ctx.variant, "#[subform_scalar] cannot be used on struct-like variants with multiple fields." ) );
          }
          // Handle single-field subform_scalar case (similar to tuple(1) subform)
          let field_info = &ctx.variant_field_info[0];
          let inner_type = &field_info.ty;
          if !matches!( inner_type, syn::Type::Path( _ ) )
          {
            return Err( syn::Error::new_spanned( inner_type, "#[subform_scalar] can only be applied to variants holding a path type (e.g., MyStruct, Option<T>), not tuples, references, etc." ) );
          }

          let end_struct_name = format_ident!( "{}{}End", ctx.enum_name, variant_ident );
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


          // --- Standalone Constructor (Subform Struct(1)) ---
          if ctx.struct_attrs.standalone_constructors.value( false )
          {
              let constructor_params : Vec<_> = ctx.variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
              let all_fields_are_args = !ctx.variant_field_info.is_empty() && ctx.variant_field_info.iter().all( |f| f.is_constructor_arg );
              // FIX: Correct return type generation
              let enum_name = ctx.enum_name; // Assign ctx.enum_name to local variable
              let return_type = if all_fields_are_args
              {
                 quote! { #enum_name< #enum_generics_ty > } // Use local variable #enum_name
              }
              else
              { // FIX: Added comma_if_enum_generics
                quote! { #inner_former_name < #inner_generics_ty_punctuated #inner_def_name < #inner_generics_ty_punctuated (), #comma_if_enum_generics #enum_name< #enum_generics_ty >, #end_struct_name < #enum_generics_ty > > > } // Use local variable #enum_name
              };
              // FIX: Use inner_generics_ty_punctuated in storage init
              let initial_storage_code = if field_info.is_constructor_arg { let fi = &field_info.ident; let pn = ident::ident_maybe_raw( fi ); quote! { ::core::option::Option::Some( #inner_storage_name :: < #inner_generics_ty_punctuated > { #fi : ::core::option::Option::Some( #pn.into() ) } ) } } else { quote! { ::core::option::Option::None } };
              let vis = ctx.vis; // Assign ctx.vis to local variable
              let constructor = quote!
              {
                  /// Standalone constructor for the #variant_ident subform variant.
                  #[ inline( always ) ]
                  #vis fn #method_name < #enum_generics_impl > // Use local variable #vis
                  ( // Paren on new line
                    #( #constructor_params ),*
                  ) // Paren on new line
                  -> // Return type on new line
                  #return_type
                  where // Where clause on new line
                    #enum_generics_where
                  { // Brace on new line
                    #inner_former_name::begin
                    ( // Paren on new line
                      #initial_storage_code,
                      None, // Context
                      #end_struct_name::< #enum_generics_ty >::default() // End
                    ) // Paren on new line
                  } // Brace on new line
              };
              ctx.standalone_constructors.push( constructor );
          }
          // --- End Standalone Constructor ---

          // Associated method logic
          let phantom_field_type = macro_tools::phantom::tuple( &ctx.generics.params ); // FIX: Use qualified path and correct generics
          let field_ident = &field_info.ident; // Get the single field's ident
          let vis = ctx.vis; // Assign ctx.vis to local variable
          ctx.end_impls.push( quote!
          {
            #[ derive( Default, Debug ) ]
            #vis struct #end_struct_name < #enum_generics_impl > // Use local variable #vis
            where // Where clause on new line
              #enum_generics_where
            { // Brace on new line
              _phantom : #phantom_field_type,
            } // Brace on new line
          });
          ctx.end_impls.push( quote!
          {
            #[ automatically_derived ]
            impl< #enum_generics_impl > former::FormingEnd
            < // Angle bracket on new line
              // FIX: Correct generics usage and add comma_if_enum_generics
              // Access def_types_name from ctx? No, it's derived locally.
              #inner_def_types_name< #inner_generics_ty_punctuated (), #comma_if_enum_generics #enum_name< #enum_generics_ty > > // Use local variable #enum_name
            > // Angle bracket on new line
            for #end_struct_name < #enum_generics_ty >
            where // Where clause on new line
              #enum_generics_where
            { // Brace on new line
              #[ inline( always ) ]
              fn call
              ( // Paren on new line
                &self,
                sub_storage : #inner_storage_name< #inner_generics_ty_punctuated >, // FIX: Use punctuated version
                _context : Option< () >,
              ) // Paren on new line
              -> // Return type on new line
              #enum_name< #enum_generics_ty > // Use local variable #enum_name
              { // Brace on new line
                // FIX: Handle single vs multi-field preformed type
                let data = former::StoragePreform::preform( sub_storage );
                #enum_name::#variant_ident{ #field_ident : data } // Use local variable #enum_name
              } // Brace on new line
            } // Brace on new line
          });
          let vis = ctx.vis; // Assign ctx.vis to local variable
          let static_method = quote!
          {
            /// Starts forming the #variant_ident variant using its implicit former.
            #[ inline( always ) ]
            #vis fn #method_name () // Use local variable #vis
            -> // Return type on new line
            #inner_former_name
            < // Angle bracket on new line
              #inner_generics_ty_punctuated // FIX: Use punctuated version
              #inner_def_name
              < // Angle bracket on new line
                #inner_generics_ty_punctuated (), #comma_if_enum_generics #enum_name< #enum_generics_ty >, #end_struct_name < #enum_generics_ty > > // Use local variable #enum_name
              > // Angle bracket on new line
            > // Angle bracket on new line
            { // Brace on new line
              #inner_former_name::begin( None, None, #end_struct_name::< #enum_generics_ty >::default() )
            } // Brace on new line
          };
          ctx.methods.push( static_method );

      }
      else if wants_scalar
      {
          // --- Scalar Struct(N) Variant ---
          // --- Standalone Constructor (Scalar Struct(N)) ---
          if ctx.struct_attrs.standalone_constructors.value( false )
          {
              let constructor_params : Vec<_> = ctx.variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
              let enum_name = ctx.enum_name; // Assign ctx.enum_name to local variable
              let return_type = quote! { #enum_name< #enum_generics_ty > }; // Use local variable #enum_name
              let direct_construction_args = ctx.variant_field_info.iter().map( |f| { let fi = &f.ident; let pn = ident::ident_maybe_raw( fi ); quote! { #fi : #pn.into() } } );
              let vis = ctx.vis; // Assign ctx.vis to local variable
              let constructor = quote!
              {
                  /// Standalone constructor for the #variant_ident struct variant (scalar style).
                  #[ inline( always ) ]
                  #vis fn #method_name < #enum_generics_impl > // Use local variable #vis
                  ( // Paren on new line
                    #( #constructor_params ),*
                  ) // Paren on new line
                  -> // Return type on new line
                  #return_type
                  where // Where clause on new line
                    #enum_generics_where
                  { // Brace on new line
                    Self::#variant_ident { #( #direct_construction_args ),* }
                  } // Brace on new line
              };
              ctx.standalone_constructors.push( constructor );
          }
          // --- End Standalone Constructor ---

          // Associated method (direct constructor)
          let mut params = Vec::new();
          let mut args = Vec::new();
          // FIX: Iterate over ctx.variant_field_info directly (remove &)
          for field_info in ctx.variant_field_info
          {
              let field_ident = &field_info.ident;
              let param_name = ident::ident_maybe_raw( field_ident );
              let field_type = &field_info.ty;
              params.push( quote! { #param_name : impl Into< #field_type > } );
              args.push( quote! { #field_ident : #param_name.into() } );
          }
          let vis = ctx.vis; // Assign ctx.vis to local variable
          let static_method = quote!
          {
              /// Constructor for the #variant_ident struct variant (scalar style).
              #[ inline( always ) ]
              #vis fn #method_name // Use local variable #vis
              ( // Paren on new line
                #( #params ),*
              ) // Paren on new line
              -> Self
              { // Brace on new line
                Self::#variant_ident { #( #args ),* }
              } // Brace on new line
          };
          ctx.methods.push( static_method );
      }
      else // Default: Subformer (Implicit Former)
      {
          // --- Subform Struct(N) Variant ---
          // Generate implicit former ecosystem for this variant

          // Storage struct name: EnumNameVariantNameFormerStorage
          let storage_struct_name = format_ident!( "{}{}FormerStorage", ctx.enum_name, variant_ident );
          // DefinitionTypes struct name
          let def_types_name = format_ident!( "{}{}FormerDefinitionTypes", ctx.enum_name, variant_ident );
          // Definition struct name
          let def_name = format_ident!( "{}{}FormerDefinition", ctx.enum_name, variant_ident );
          // End struct name
          let end_struct_name = format_ident!( "{}{}End", ctx.enum_name, variant_ident );
          // Former struct name
          let former_name = format_ident!( "{}{}Former", ctx.enum_name, variant_ident );

          // --- Generate Storage ---
          let phantom_field_type = macro_tools::phantom::tuple( &ctx.generics.params ); // FIX: Use qualified path and correct generics
          let storage_fields = ctx.variant_field_info.iter().map( |f_info|
          {
            let field_ident = &f_info.ident;
            let field_type = &f_info.ty;
            quote! { pub #field_ident : ::core::option::Option< #field_type > }
          });
          let default_assignments = ctx.variant_field_info.iter().map( |f_info|
          {
            let field_ident = &f_info.ident;
            quote! { #field_ident : ::core::option::Option::None }
        });
        // Push Storage struct definition
        let vis = ctx.vis; // Assign ctx.vis to local variable
        ctx.end_impls.push( quote!
        {
          #[ derive( Debug ) ] // Removed Default derive here
          #vis struct #storage_struct_name < #enum_generics_impl > // Use local variable #vis
          where // Where clause on new line
            #enum_generics_where
          { // Brace on new line
              #( #storage_fields, )*
              _phantom : #phantom_field_type,
            } // Brace on new line
          });
          // Push Default impl for Storage
          ctx.end_impls.push( quote!
          {
            impl< #enum_generics_impl > ::core::default::Default
            for #storage_struct_name < #enum_generics_ty >
            where // Where clause on new line
              #enum_generics_where // FIX: Use correct variable
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
          });

          // --- Generate Storage Impls ---
          let field_types : Vec<_> = ctx.variant_field_info.iter().map( |f_info| &f_info.ty ).collect(); // Collect types
          // Push former::Storage impl
          ctx.end_impls.push( quote!
          {
            impl< #enum_generics_impl > former::Storage
            for #storage_struct_name < #enum_generics_ty >
            where // Where clause on new line
              #enum_generics_where // FIX: Use correct variable
            { // Brace on new line
              type Preformed = ( #( #field_types ),* ); // Preformed type is a tuple of field types
            } // Brace on new line
          });
          let preform_field_assignments = ctx.variant_field_info.iter().map( |f_info|
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
          let preformed_tuple_elements_vec : Vec<_> = ctx.variant_field_info.iter().map( |f_info|
          {
            let field_ident = &f_info.ident;
            quote! { #field_ident }
          }).collect();
          // Push former::StoragePreform impl
          ctx.end_impls.push( quote!
          {
            impl< #enum_generics_impl > former::StoragePreform
            for #storage_struct_name < #enum_generics_ty >
            where // Where clause on new line
              #enum_generics_where // FIX: Use correct variable
            { // Brace on new line
              fn preform( mut self ) -> Self::Preformed
              { // Brace on new line
                #( let #preformed_tuple_elements_vec = #preform_field_assignments; )*
                ( #( #preformed_tuple_elements_vec ),* ) // Return the tuple
              } // Brace on new line
            } // Brace on new line
          });

          // --- Generate DefinitionTypes ---
          // FIX: Correctly merge generics and handle commas
          let mut def_types_generics_impl_punctuated : Punctuated<GenericParam, Comma> = ctx.generics.params.clone();
          if !def_types_generics_impl_punctuated.is_empty() && !def_types_generics_impl_punctuated.trailing_punct() { def_types_generics_impl_punctuated.push_punct( Default::default() ); } // Add trailing comma if needed
          def_types_generics_impl_punctuated.push( parse_quote!( Context2 = () ) );
          let enum_name = ctx.enum_name; // Assign ctx.enum_name to local variable
          def_types_generics_impl_punctuated.push( parse_quote!( Formed2 = #enum_name< #enum_generics_ty > ) ); // Use local variable #enum_name
          let ( _def_types_generics_with_defaults, def_types_generics_impl, def_types_generics_ty, def_types_generics_where ) = generic_params::decompose( &syn::Generics { params: def_types_generics_impl_punctuated, ..ctx.generics.clone() } );
          let def_types_phantom = macro_tools::phantom::tuple( &def_types_generics_impl ); // FIX: Use qualified path
          // Push DefinitionTypes struct definition
          let vis = ctx.vis; // Assign ctx.vis to local variable
          ctx.end_impls.push( quote!
          {
            #[ derive( Debug ) ]
            #vis struct #def_types_name < #def_types_generics_impl > // Use local variable #vis
            where // Where clause on new line
              #def_types_generics_where
            { // Brace on new line
              _phantom : #def_types_phantom,
            } // Brace on new line
          });
          // Push Default impl for DefinitionTypes
          ctx.end_impls.push( quote!
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
          });
          // Push former::FormerDefinitionTypes impl
          ctx.end_impls.push( quote!
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
          });
          // Push former::FormerMutator impl
          ctx.end_impls.push( quote!
          {
            impl< #def_types_generics_impl > former::FormerMutator
            for #def_types_name < #def_types_generics_ty >
            where // Where clause on new line
              #def_types_generics_where
            { // Brace on new line
              // Default empty mutator
            } // Brace on new line
          });

          // --- Generate Definition ---
          // FIX: Correctly merge generics and handle commas
          let mut def_generics_impl_punctuated : Punctuated<GenericParam, Comma> = ctx.generics.params.clone();
          if !def_generics_impl_punctuated.is_empty() && !def_generics_impl_punctuated.trailing_punct() { def_generics_impl_punctuated.push_punct( Default::default() ); } // Add trailing comma if needed
          def_generics_impl_punctuated.push( parse_quote!( Context2 = () ) );
          let enum_name = ctx.enum_name; // Assign ctx.enum_name to local variable
          def_generics_impl_punctuated.push( parse_quote!( Formed2 = #enum_name< #enum_generics_ty > ) ); // Use local variable #enum_name
          def_generics_impl_punctuated.push( parse_quote!( End2 = #end_struct_name< #enum_generics_ty > ) );
          let def_generics_syn = syn::Generics { params: def_generics_impl_punctuated, ..ctx.generics.clone() };
          let ( _def_generics_with_defaults, def_generics_impl, def_generics_ty, def_generics_where ) = generic_params::decompose( &def_generics_syn );
          let def_phantom = macro_tools::phantom::tuple( &def_generics_impl ); // FIX: Use qualified path
          // Push Definition struct definition
          let vis = ctx.vis; // Assign ctx.vis to local variable
          ctx.end_impls.push( quote!
          {
            #[ derive( Debug ) ]
            #vis struct #def_name < #def_generics_impl > // Use local variable #vis
            where // Where clause on new line
              // FIX: Correctly reference DefinitionTypes with its generics
              End2 : former::FormingEnd< #def_types_name< #enum_generics_ty #comma_if_enum_generics Context2, Formed2 > >, // Note: Formed2 already uses #enum_name
              #def_generics_where // Includes original enum where clause
            { // Brace on new line
              _phantom : #def_phantom,
            } // Brace on new line
          });
          // Push Default impl for Definition
          ctx.end_impls.push( quote!
          {
            impl< #def_generics_impl > ::core::default::Default
            for #def_name < #def_generics_ty >
            where // Where clause on new line
              #def_generics_where
            { // Brace on new line
              fn default() -> Self
              { // Brace on new line
                Self { _phantom : ::core::marker::PhantomData }
              } // Brace on new line
            } // Brace on new line
          });
          // Push former::FormerDefinition impl
          ctx.end_impls.push( quote!
          {
            impl< #def_generics_impl > former::FormerDefinition
            for #def_name < #def_generics_ty >
            where // Where clause on new line
              // FIX: Correctly reference DefinitionTypes with its generics
              End2 : former::FormingEnd< #def_types_name< #enum_generics_ty #comma_if_enum_generics Context2, Formed2 > >, // Note: Formed2 already uses #enum_name
              #def_generics_where
            { // Brace on new line
              type Storage = #storage_struct_name< #enum_generics_ty >;
              type Context = Context2;
              type Formed = Formed2; // Note: Formed2 already uses #enum_name
              // FIX: Correctly reference DefinitionTypes with its generics
              type Types = #def_types_name< #enum_generics_ty #comma_if_enum_generics Context2, Formed2 >; // Note: Formed2 already uses #enum_name
              type End = End2;
            } // Brace on new line
          });

          // --- Generate Former Struct ---
          let mut former_generics = ctx.generics.clone();
          // FIX: Correctly add Definition generic parameter and handle commas
          let enum_name = ctx.enum_name; // Assign ctx.enum_name to local variable
          former_generics.params.push( parse_quote!( Definition = #def_name< #enum_generics_ty #comma_if_enum_generics (), #enum_name<#enum_generics_ty>, #end_struct_name<#enum_generics_ty> > ) ); // Use local variable #enum_name
          let former_where_clause = former_generics.make_where_clause();
          former_where_clause.predicates.push( parse_quote!{ Definition : former::FormerDefinition< Storage = #storage_struct_name< #enum_generics_ty > > } );
          former_where_clause.predicates.push( parse_quote!{ Definition::Types : former::FormerDefinitionTypes< Storage = #storage_struct_name< #enum_generics_ty > > } );
          if let Some( enum_where ) = &ctx.generics.where_clause
          {
            for predicate in &enum_where.predicates
            {
              former_where_clause.predicates.push( predicate.clone() );
            }
          }
          let ( _former_generics_with_defaults, former_generics_impl, former_generics_ty, former_generics_where ) = generic_params::decompose( &former_generics );
          // Push Former struct definition
          let vis = ctx.vis; // Assign ctx.vis to local variable
          ctx.end_impls.push( quote!
          {
            #[ doc = "Former for the #variant_ident variant." ]
            #vis struct #former_name < #former_generics_impl > // Use local variable #vis
            where // Where clause on new line
              #former_generics_where
            { // Brace on new line
              /// Temporary storage for all fields during the formation process.
              pub storage : Definition::Storage,
              /// Optional context.
              pub context : ::core::option::Option< Definition::Context >,
              /// Optional handler for the end of formation.
              pub on_end : ::core::option::Option< Definition::End >,
              // Add phantom data for Definition generic
              _phantom_def : ::core::marker::PhantomData< Definition >,
            } // Brace on new line
          });

          // --- Generate Former Impl + Setters ---
          let setters = ctx.variant_field_info.iter().map( |f_info|
          {
            let field_ident = &f_info.ident;
            let field_type = &f_info.ty;
            let setter_name = ident::ident_maybe_raw( field_ident );
            quote!
            {
              #[ inline ]
              pub fn #setter_name< Src >( mut self, src : Src ) -> Self
              where Src : ::core::convert::Into< #field_type >
              { // Brace on new line
                debug_assert!( self.storage.#field_ident.is_none() );
                self.storage.#field_ident = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
                self
              } // Brace on new line
            }
          });
          // Push Former impl block
          ctx.end_impls.push( quote!
          {
            #[ automatically_derived ]
            impl< #former_generics_impl > #former_name < #former_generics_ty >
            where // Where clause on new line
              #former_generics_where
            { // Brace on new line
              // Standard former methods (new, begin, form, end)
              #[ inline( always ) ] pub fn new( on_end : Definition::End ) -> Self { Self::begin( None, None, on_end ) }
              #[ inline( always ) ] pub fn new_coercing< IntoEnd >( end : IntoEnd ) -> Self where IntoEnd : Into< Definition::End > { Self::begin_coercing( None, None, end ) }
              #[ inline( always ) ] pub fn begin ( mut storage : ::core::option::Option< Definition::Storage >, context : ::core::option::Option< Definition::Context >, on_end : Definition::End ) -> Self
              { // Brace on new line
                if storage.is_none() { storage = Some( Default::default() ); }
                Self { storage : storage.unwrap(), context, on_end : Some( on_end ), _phantom_def : ::core::marker::PhantomData } // Added phantom data init
              } // Brace on new line
              #[ inline( always ) ] pub fn begin_coercing< IntoEnd > ( mut storage : ::core::option::Option< Definition::Storage >, context : ::core::option::Option< Definition::Context >, on_end : IntoEnd ) -> Self where IntoEnd : Into< Definition::End >
              { // Brace on new line
                if storage.is_none() { storage = Some( Default::default() ); }
                Self { storage : storage.unwrap(), context, on_end : Some( on_end.into() ), _phantom_def : ::core::marker::PhantomData } // Added phantom data init
              } // Brace on new line
              #[ inline( always ) ] pub fn form( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed { self.end() }
              #[ inline( always ) ] pub fn end( mut self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
              { // Added opening brace for end() body
                let context = self.context.take();
                let on_end = self.on_end.take().unwrap();
                // Apply mutator if needed (assuming default empty mutator for now)
                // < Definition::Types as former::FormerMutator >::form_mutation( &mut self.storage, &mut self.context );
                on_end.call( self.storage, context )
              } // Added closing brace for end() body

              // Field setters
              #( #setters )*
            } // Brace on new line for impl block
          }); // Closing parenthesis for push

          // --- Generate End Struct ---
          let phantom_field_type = macro_tools::phantom::tuple( &ctx.generics.params ); // FIX: Use qualified path and correct generics
          // Push End struct definition
          let vis = ctx.vis; // Assign ctx.vis to local variable
          ctx.end_impls.push( quote!
          {
            #[ derive( Default, Debug ) ]
            #vis struct #end_struct_name < #enum_generics_impl > // Use local variable #vis
            where // Where clause on new line
              #enum_generics_where
            { // Brace on new line
              _phantom : #phantom_field_type,
            } // Brace on new line
          });

          // --- Generate End Impl ---
          let tuple_indices = ( 0..ctx.variant_field_info.len() ).map( syn::Index::from );
          let field_idents_for_construction : Vec<_> = ctx.variant_field_info.iter().map( |f| &f.ident ).collect();
          // Push End impl block
          ctx.end_impls.push( quote!
          {
            #[ automatically_derived ]
            impl< #enum_generics_impl > former::FormingEnd
            < // Angle bracket on new line
              // FIX: Correct generics usage and add comma_if_enum_generics
              #def_types_name< #enum_generics_ty #comma_if_enum_generics (), #enum_name< #enum_generics_ty > > // Use local variable #enum_name
            > // Angle bracket on new line
            for #end_struct_name < #enum_generics_ty >
            where // Where clause on new line
              #enum_generics_where
            { // Brace on new line
              #[ inline( always ) ]
              fn call
              ( // Paren on new line
                &self,
                sub_storage : #storage_struct_name< #enum_generics_ty >,
                _context : Option< () >,
              ) // Paren on new line
              -> // Return type on new line
              #enum_name< #enum_generics_ty > // Use local variable #enum_name
              { // Brace on new line
                // FIX: Handle single vs multi-field preformed type
                let preformed_tuple = former::StoragePreform::preform( sub_storage ); // Renamed to avoid conflict
                #enum_name::#variant_ident // Use local variable #enum_name
                { // Brace on new line
                  #( #field_idents_for_construction : preformed_tuple.#tuple_indices ),* // Use preformed_tuple
                } // Brace on new line
              } // Brace on new line
            } // Brace on new line
          });

          // --- Generate Static Method ---
          // Push static method for Former
          let vis = ctx.vis; // Assign ctx.vis to local variable
          let static_method = quote!
          {
            /// Starts forming the #variant_ident variant using its implicit former.
            #[ inline( always ) ]
            #vis fn #method_name () // Use local variable #vis
            -> // Return type on new line
            #former_name
            < // Angle bracket on new line
              #enum_generics_ty, // Enum generics
              // Default definition
              #def_name< #enum_generics_ty #comma_if_enum_generics (), #enum_name< #enum_generics_ty >, #end_struct_name< #enum_generics_ty > > // Use local variable #enum_name
            > // Angle bracket on new line
            { // Brace on new line
              #former_name::begin( None, None, #end_struct_name::< #enum_generics_ty >::default() )
            } // Brace on new line
          };
          ctx.methods.push( static_method );

          // --- Generate Standalone Constructor (Subform Struct(N)) ---
          if ctx.struct_attrs.standalone_constructors.value( false )
          {
              let constructor_params : Vec<_> = ctx.variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
              let all_fields_are_args = !ctx.variant_field_info.is_empty() && ctx.variant_field_info.iter().all( |f| f.is_constructor_arg );
              // FIX: Added comma in return type generics
              let enum_name = ctx.enum_name; // Assign ctx.enum_name to local variable
              let return_type = if all_fields_are_args { quote! { #enum_name< #enum_generics_ty > } } else { quote! { #former_name < #enum_generics_ty, #def_name< #enum_generics_ty #comma_if_enum_generics (), #enum_name< #enum_generics_ty >, #end_struct_name< #enum_generics_ty > > > } }; // Use local variable #enum_name
              let initial_storage_assignments = ctx.variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let fi = &f.ident; let pn = ident::ident_maybe_raw( fi ); quote! { #fi : ::core::option::Option::Some( #pn.into() ) } } ); // Filter only constructor args
              let initial_storage_code = if constructor_params.is_empty() { quote! { ::core::option::Option::None } } else { quote! { ::core::option::Option::Some( #storage_struct_name :: < #enum_generics_ty > { #( #initial_storage_assignments, )* ..Default::default() } ) } }; // Use ..Default::default()
              let constructor_body = if all_fields_are_args { let construction_args = ctx.variant_field_info.iter().map( |f| { let fi = &f.ident; let pn = ident::ident_maybe_raw( fi ); quote! { #fi : #pn.into() } } ); quote! { #enum_name::#variant_ident { #( #construction_args ),* } } } else { quote! { #former_name::begin( #initial_storage_code, None, #end_struct_name::< #enum_generics_ty >::default() ) } }; // Use local variable #enum_name
              let vis = ctx.vis; // Assign ctx.vis to local variable
              let constructor = quote!
              {
                  /// Standalone constructor for the #variant_ident subform variant.
                  #[ inline( always ) ]
                  #vis fn #method_name < #enum_generics_impl > // Use local variable #vis
                  ( // Paren on new line
                    #( #constructor_params ),*
                  ) // Paren on new line
                  -> // Return type on new line
                  #return_type
                  where // Where clause on new line
                    #enum_generics_where
                  { // Brace on new line
                    #constructor_body
                  } // Brace on new line
              };
              ctx.standalone_constructors.push( constructor );
          }
          // --- End Standalone Constructor ---


      } // End Default: Subformer
    } // Closing brace for Fields::Named arm (matches brace at line 59)
    _ => return Err( Error::new_spanned( ctx.variant, "Former derive macro only supports named fields for struct variants" ) ), // Added error handling for non-named fields
  } // Added closing brace for match statement (matches brace at line 56)
  Ok( () )
} // Closing brace for function (matches brace at line 33)
