// File: module/core/former_meta/src/derive_former/former_enum/struct_non_zero.rs
use super::*; // Use items from parent module (former_enum)
use syn::token::{ Const, Colon };

use macro_tools::
{
  generic_params, Result,
  quote::{ format_ident, quote },
  ident,
  parse_quote,
  punctuated, // Import punctuated utilities
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
  WherePredicate, // Import WherePredicate
};
use convert_case::{ Case, Casing };

/// Handles the generation of code for struct variants with non-zero fields.
#[ allow( clippy::too_many_lines ) ] // Keep this one for now
pub( super ) fn handle_struct_non_zero_variant
(
  ctx : &mut EnumVariantHandlerContext< '_ >,
) -> Result< () >
{
  // Extract necessary fields from context into local variables
  let variant = &ctx.variant;
  let variant_ident = &variant.ident;
  let variant_attrs = &ctx.variant_attrs;
  let struct_attrs = &ctx.struct_attrs;
  let generics = &ctx.generics;
  let variant_field_info = &ctx.variant_field_info;
  let vis = &ctx.vis;
  let enum_name = &ctx.enum_name;

  // Define field_types here to make it available in multiple scopes
  let field_types : Vec<syn::Type> = variant_field_info.iter().map( |f_info| f_info.ty.clone() ).collect(); // Collect owned types

  // Generate the snake_case method name, handling potential keywords
  let variant_name_str = variant_ident.to_string();
  let method_name_snake_str = variant_name_str.to_case( Case::Snake );
  // Use format_ident! instead of parse_quote! for robust identifier creation
  let method_name_ident_temp = format_ident!( "{}", method_name_snake_str, span = variant_ident.span() );
  let method_name = ident::ident_maybe_raw( &method_name_ident_temp );

  let ( _enum_generics_with_defaults, enum_generics_impl, enum_generics_ty_with_comma, _enum_generics_where_punctuated )
  = generic_params::decompose( generics );
  // Use the Option<&WhereClause> directly from generics by calling .as_ref()
  let _enum_generics_where_clause = ctx.merged_where_clause; // Renamed for clarity, prefixed with _

  // Create a version of enum_generics_ty *without* the trailing comma for use in type names
  let enum_generics_ty_no_comma : Punctuated<GenericParam, Comma> = enum_generics_ty_with_comma.into_iter().collect(); // Use into_iter().collect()


  // Check if the attribute is present using .is_some()
  let wants_subform_scalar = variant_attrs.subform_scalar.is_some();
  let wants_scalar = variant_attrs.scalar.is_some();

  // Helper for conditional comma - Removed, logic embedded below


  match &variant.fields
  {
    Fields::Named( fields ) =>
    {
      if wants_subform_scalar
      {
          if fields.named.len() > 1
          {
            return Err( syn::Error::new_spanned( variant, "#[subform_scalar] cannot be used on struct-like variants with multiple fields." ) );
          }
          // Handle single-field subform_scalar case (similar to tuple(1) subform)
          let field_info = &variant_field_info[0];
          let inner_type = &field_info.ty;
          if !matches!( inner_type, syn::Type::Path( _ ) )
          {
            return Err( syn::Error::new_spanned( inner_type, "#[subform_scalar] can only be applied to variants holding a path type (e.g., MyStruct, Option<T>), not tuples, references, etc." ) );
          }

          let end_struct_name = format_ident!( "{}{}End", enum_name, variant_ident );
          let ( inner_type_name, inner_generics ) = match inner_type { syn::Type::Path( tp ) => { let s = tp.path.segments.last().unwrap(); ( s.ident.clone(), s.arguments.clone() ) }, _ => unreachable!() };
          let inner_former_name = format_ident!( "{}Former", inner_type_name );
          let inner_storage_name = format_ident!( "{}FormerStorage", inner_type_name );
          let inner_def_name = format_ident!( "{}FormerDefinition", inner_type_name );
          let inner_def_types_name = format_ident!( "{}FormerDefinitionTypes", inner_type_name );
          // Convert GenericArgument to GenericParam
          let inner_generics_params : Punctuated<GenericParam, Comma> = match &inner_generics
          {
            syn::PathArguments::AngleBracketed( args ) => args.args.iter().map( |arg| match arg {
              // Extract ident correctly for Type and Const
              GenericArgument::Type( ty ) => match ty {
                  syn::Type::Path( p ) => GenericParam::Type( TypeParam { ident: p.path.get_ident().unwrap().clone(), attrs: vec![], colon_token: None, bounds: Punctuated::new(), eq_token: None, default: None } ),
                  _ => panic!("Unsupported generic argument type for TypeParam ident extraction"),
              },
              GenericArgument::Lifetime( lt ) => GenericParam::Lifetime( LifetimeParam::new( lt.clone() ) ),
              GenericArgument::Const( c ) => match c {
                  Expr::Path( p ) => GenericParam::Const( ConstParam { ident: p.path.get_ident().unwrap().clone(), attrs: vec![], const_token: Const::default(), colon_token: Colon::default(), ty: parse_quote!(_), eq_token: None, default: None } ),
                  &_ => panic!("Unsupported const expression for ConstParam ident extraction"),
                },
              _ => panic!("Unsupported generic argument type"), // Or return error
            }).collect(),
            _ => Punctuated::new(),
          };
          // Create versions with and without trailing comma
          let mut inner_generics_ty_punctuated_with_comma = inner_generics_params.clone();
          punctuated::ensure_trailing_comma( &mut inner_generics_ty_punctuated_with_comma );
          let inner_generics_ty_punctuated_no_comma = inner_generics_params.iter().cloned().collect::< Punctuated< _, Comma > >();


          // --- Standalone Constructor (Subform Struct(1)) ---
          if struct_attrs.standalone_constructors.value( false )
          {
              let constructor_params : Vec<_> = variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
              let all_fields_are_args = !variant_field_info.is_empty() && variant_field_info.iter().all( |f| f.is_constructor_arg );

              // Construct return type generics list
              let mut return_type_def_generics_vec : Vec<GenericParam> = inner_generics_ty_punctuated_no_comma.iter().cloned().collect(); // Use iter().cloned()
              let context_arg : GenericArgument = parse_quote!( () );
              let formed_arg : GenericArgument = parse_quote!( #enum_name< #enum_generics_ty_no_comma > );
              let end_arg : GenericArgument = parse_quote!( #end_struct_name < #enum_generics_ty_no_comma > );
              let def_param : GenericParam = parse_quote!( Def = #inner_def_name < #inner_generics_ty_punctuated_no_comma #context_arg, #formed_arg, #end_arg > ); // Simplified
              return_type_def_generics_vec.push( def_param );
              let return_type_def_generics = Punctuated::<_, Comma>::from_iter( return_type_def_generics_vec );

              let return_type = if all_fields_are_args
              {
                 quote! { #enum_name< #enum_generics_ty_no_comma > } // Use no_comma
              }
              else
              {
                // Use no_comma for inner generics type name
                quote! { #inner_former_name < #return_type_def_generics > } // Use constructed list
              };
              // Use no_comma for inner generics type name in storage init
              let initial_storage_code = if field_info.is_constructor_arg
              {
                let fi = &field_info.ident;
                let pn = ident::ident_maybe_raw( fi );
                quote!
                {
                  ::core::option::Option::Some
                  (
                    #inner_storage_name :: < #inner_generics_ty_punctuated_no_comma > // Use no_comma
                    {
                      #fi : ::core::option::Option::Some( #pn.into() )
                    }
                  )
                }
              }
              else
              {
                quote! { ::core::option::Option::None }
              };
              let constructor = {
                  let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                      if where_clause.predicates.is_empty() {
                          quote! {}
                      } else {
                          let predicates = &where_clause.predicates;
                          quote! { where #predicates }
                      }
                  } else {
                      quote! {}
                  };
                  quote!
                  {
                      /// Standalone constructor for the #variant_ident subform variant.
                      #[ inline( always ) ]
                      #vis fn #method_name < #enum_generics_impl >
                      (
                        #( #constructor_params ),*
                      )
                      ->
                      #return_type
                      #where_clause_tokens
                      {
                        #inner_former_name::begin
                        (
                          #initial_storage_code,
                          None, // Context
                          #end_struct_name::< #enum_generics_ty_no_comma >::default() // Use no_comma
                        )
                      }
                  }
              };
              ctx.standalone_constructors.push( constructor );
             }
             // --- End Standalone Constructor ---

             // Associated method logic
             let phantom_field_type = macro_tools::phantom::tuple( &generics.params ); // Use qualified path and correct generics
             // let _field_ident = &field_info.ident; // Removed unused variable per clippy
             let end_struct_tokens = {
                 let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 quote!
                 {
                   #[ derive( Default, Debug ) ]
                   #vis struct #end_struct_name < #enum_generics_impl >
                   #where_clause_tokens
                   {
                     _phantom : #phantom_field_type,
                   }
                 }
             };
             ctx.end_impls.push( end_struct_tokens );
             // Generate token stream for struct field assignments in call function
             let field_assignments_tokens = {
                 let mut tokens = quote! {};
                 let tuple_indices = ( 0..variant_field_info.len() ).map( syn::Index::from );
                 let field_idents_for_construction : Vec<_> = variant_field_info.iter().map( |f| &f.ident ).collect();
                 for (field_ident, tuple_index) in field_idents_for_construction.iter().zip(tuple_indices) {
                     tokens.extend(quote! { #field_ident : preformed_tuple.#tuple_index, });
                 }
                 tokens
             };
             // Generate token stream for the type within the angle brackets for FormingEnd
             // Construct the punctuated list for DefinitionTypes generics
             let mut forming_end_def_types_generics_vec : Vec<GenericParam> = inner_generics_ty_punctuated_no_comma.iter().cloned().collect(); // Use iter().cloned()
             let context_param : GenericParam = parse_quote!( Context2 ); // Use generic parameter directly
             let formed_param : GenericParam = parse_quote!( Formed2 ); // Use generic parameter directly
             forming_end_def_types_generics_vec.push( context_param );
             forming_end_def_types_generics_vec.push( formed_param );
             let forming_end_def_types_generics = Punctuated::<_, Comma>::from_iter( forming_end_def_types_generics_vec );

             let forming_end_type_tokens = quote! {
                 #inner_def_types_name< #forming_end_def_types_generics > // Use constructed list
             };
             let forming_end_impl_tokens = {
                 let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 quote!
                 {
                   #[ automatically_derived ]
                   impl< #enum_generics_impl > former::FormingEnd
                   <
                     // Correct generics usage and add comma_if_enum_generics
                     #forming_end_type_tokens
                   >
                   for #end_struct_name < #enum_generics_ty_no_comma > // Use no_comma
                   #where_clause_tokens
                   {
                     #[ inline( always ) ]
                     fn call
                     (
                       &self,
                       sub_storage : #inner_storage_name< #inner_generics_ty_punctuated_no_comma >, // Use no_comma
                       _context : Option< () >,
                     )
                     ->
                     #enum_name< #enum_generics_ty_no_comma > // Use no_comma
                     {
                       // Handle single vs multi-field preformed type
                       let preformed_tuple = former::StoragePreform::preform( sub_storage );
                       #enum_name::#variant_ident
                       {
                         #field_assignments_tokens
                       }
                     }
                   }
                 }
             };
             ctx.end_impls.push( forming_end_impl_tokens );

             // Construct the generics list for the static method return type
             let mut static_method_return_generics_vec : Vec<GenericParam> = inner_generics_ty_punctuated_no_comma.iter().cloned().collect(); // Use iter().cloned()
             let context_arg : GenericArgument = parse_quote!( () );
             let formed_arg : GenericArgument = parse_quote!( #enum_name< #enum_generics_ty_no_comma > );
             let end_arg : GenericArgument = parse_quote!( #end_struct_name < #enum_generics_ty_no_comma > );
             let def_param : GenericParam = parse_quote!( Def = #inner_def_name < #inner_generics_ty_punctuated_no_comma #context_arg, #formed_arg, #end_arg > ); // Simplified
             static_method_return_generics_vec.push( def_param );
             let static_method_return_generics = Punctuated::<_, Comma>::from_iter( static_method_return_generics_vec );

             let static_method = quote!
             {
               /// Starts forming the #variant_ident variant using its implicit former.
               #[ inline( always ) ]
               #vis fn #method_name ()
               ->
               #inner_former_name // Use no_comma for inner generics type name
               <
                 #static_method_return_generics // Use constructed list
               >
               {
                 #inner_former_name::begin( None, None, #end_struct_name::< #enum_generics_ty_no_comma >::default() ) // Use no_comma
               }
             };
             ctx.methods.push( static_method );

         }
         else if wants_scalar
         {
             // --- Scalar Struct(N) Variant ---
             // --- Standalone Constructor (Scalar Struct(N)) ---
             if struct_attrs.standalone_constructors.value( false )
             {
                 let constructor_params : Vec<_> = variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
                 let return_type = {
                   quote! { #enum_name< #enum_generics_ty_no_comma > } // Use no_comma
                 };
                 let direct_construction_args = variant_field_info.iter().map( |f| { let fi = &f.ident; let pn = ident::ident_maybe_raw( fi ); quote! { #fi : #pn.into() } } );
                 let constructor = {
                     let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                         if where_clause.predicates.is_empty() {
                             quote! {}
                         } else {
                             let predicates = &where_clause.predicates;
                             quote! { where #predicates }
                         }
                     } else {
                         quote! {}
                     };
                     quote!
                     {
                         /// Standalone constructor for the #variant_ident struct variant (scalar style).
                         #[ inline( always ) ]
                         #vis fn #method_name < #enum_generics_impl >
                         (
                           #( #constructor_params ),*
                         )
                         ->
                         #return_type
                         #where_clause_tokens
                         {
                           Self::#variant_ident { #( #direct_construction_args ),* }
                         }
                     }
                 };
                 ctx.standalone_constructors.push( constructor );
             }
             // --- End Standalone Constructor ---

             // Associated method (direct constructor)
             let mut params = Vec::new();
             let mut args = Vec::new();
             // Iterate over ctx.variant_field_info directly (remove &)
             for field_info in ctx.variant_field_info // Corrected iteration
             {
                 let field_ident = &field_info.ident;
                 let param_name = ident::ident_maybe_raw( field_ident ); // Uses ident_maybe_raw
                 let field_type = &field_info.ty;
                 params.push( quote! { #param_name : impl Into< #field_type > } );
                 args.push( quote! { #field_ident : #param_name.into() } );
             }
             let static_method = {
                 let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 quote!
                 {
                     /// Constructor for the #variant_ident struct variant (scalar style).
                     #[ inline( always ) ]
                     #vis fn #method_name
                     (
                       #( #params ),*
                     )
                     -> Self
                     #where_clause_tokens
                     {
                       Self::#variant_ident { #( #args ),* }
                     }
                 }
             };
             ctx.methods.push( static_method );
         }
         else // Default: Subformer (Implicit Former)
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

             // --- Generate Storage ---
             let phantom_field_type = macro_tools::phantom::tuple( &generics.params ); // Use qualified path and correct generics
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
             // Push Storage struct definition
             let storage_struct_tokens = {
                 let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 quote!
                 {
                   #[ derive( Debug ) ] // Removed Default derive here
                   #vis struct #storage_struct_name < #enum_generics_impl >
                   #where_clause_tokens
                   {
                     #( #storage_fields, )*
                     _phantom : #phantom_field_type,
                   }
                 }
             };
             ctx.end_impls.push( storage_struct_tokens );
             // Push Default impl for Storage
             let storage_default_impl_tokens = {
                 let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 quote!
                 {
                   impl< #enum_generics_impl > ::core::default::Default
                   for #storage_struct_name < #enum_generics_ty_no_comma > // Use no_comma
                   #where_clause_tokens
                   {
                     #[ inline( always ) ]
                     fn default() -> Self
                     {
                       Self
                       {
                         #( #default_assignments, )*
                         _phantom : ::core::marker::PhantomData,
                       }
                     }
                   }
                 }
             };
             ctx.end_impls.push( storage_default_impl_tokens );
             // Push former::Storage impl
             let storage_trait_impl_tokens = {
                 let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 quote!
                 {
                   impl< #enum_generics_impl > former::Storage
                   for #storage_struct_name < #enum_generics_ty_no_comma > // Use no_comma
                   #where_clause_tokens
                   {
                     type Preformed = ( #( #field_types ),* ); // Preformed type is a tuple of field types
                   }
                 }
             };
             ctx.end_impls.push( storage_trait_impl_tokens );
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
             // Push former::StoragePreform impl
             let storage_preform_impl_tokens = {
                 let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 quote!
                 {
                   impl< #enum_generics_impl > former::StoragePreform
                   for #storage_struct_name < #enum_generics_ty_no_comma > // Use no_comma
                   #where_clause_tokens
                   {
                     fn preform( mut self ) -> Self::Preformed
                     {
                       #( let #preformed_tuple_elements_vec = #preform_field_assignments; )*
                       ( #( #preformed_tuple_elements_vec ),* )
                     }
                   }
                 }
             };
             ctx.end_impls.push( storage_preform_impl_tokens );

             // --- Generate DefinitionTypes ---
             // Construct generics list for DefinitionTypes
             let mut def_types_generics_impl_vec : Vec<GenericParam> = generics.params.iter().cloned().collect();
             let context_param : GenericParam = parse_quote!( Context2 = () );
             let formed_param : GenericParam = parse_quote!( Formed2 = #enum_name< #enum_generics_ty_no_comma > ); // Use no_comma
             def_types_generics_impl_vec.push( context_param.clone() );
             def_types_generics_impl_vec.push( formed_param.clone() );
             let def_types_generics_impl_punctuated = Punctuated::<_, Comma>::from_iter( def_types_generics_impl_vec );

             let ( _def_types_generics_with_defaults, def_types_generics_impl, def_types_generics_ty_with_comma, _def_types_generics_where ) = generic_params::decompose( &syn::Generics { params: def_types_generics_impl_punctuated.clone(), ..(*generics).clone() } );
             let def_types_generics_ty_no_comma = def_types_generics_ty_with_comma.pairs().map( | p | p.value().clone() ).collect::< Punctuated< _, Comma > >(); // Corrected: Added .clone()
             let def_types_phantom = macro_tools::phantom::tuple( &def_types_generics_impl ); // Use qualified path
             // Push DefinitionTypes struct definition
             let def_types_struct_tokens = {
                 let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 quote!
                 {
                   #[ derive( Debug ) ]
                   #vis struct #def_types_name < #def_types_generics_impl >
                   #where_clause_tokens
                   {
                     _phantom : #def_types_phantom,
                   }
                 }
             };
             ctx.end_impls.push( def_types_struct_tokens );
             // Push Default impl for DefinitionTypes
             let def_types_default_impl_tokens = {
                 let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 quote!
                 {
                   impl< #def_types_generics_impl > ::core::default::Default
                   for #def_types_name < #def_types_generics_ty_no_comma > // Use no_comma
                   #where_clause_tokens
                   {
                     fn default() -> Self
                     {
                       Self { _phantom : ::core::marker::PhantomData }
                     }
                   }
                 }
             };
             ctx.end_impls.push( def_types_default_impl_tokens );
             // Push former::FormerDefinitionTypes impl
             let former_definition_types_impl_tokens = {
                 let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 quote!
                 {
                   impl< #def_types_generics_impl > former::FormerDefinitionTypes
                   for #def_types_name < #def_types_generics_ty_no_comma > // Use no_comma
                   #where_clause_tokens
                   {
                     type Storage = #storage_struct_name< #enum_generics_ty_no_comma >; // Use no_comma
                     type Context = Context2;
                     type Formed = Formed2; // Note: Formed2 already uses #enum_name
                     // Removed End associated type as it's not part of FormerDefinitionTypes
                   }
                 }
             };
             ctx.end_impls.push( former_definition_types_impl_tokens );
             // Push former::FormerMutator impl
             let former_mutator_impl_tokens = {
                 let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 quote!
                 {
                   impl< #def_types_generics_impl > former::FormerMutator
                   for #def_types_name < #def_types_generics_ty_no_comma > // Use no_comma
                   #where_clause_tokens
                   {
                     // Default empty mutator
                   }
                 }
             };
             ctx.end_impls.push( former_mutator_impl_tokens );

             // --- Generate Definition ---
             // Construct generics list for Definition
             let mut def_generics_impl_vec : Vec<GenericParam> = generics.params.iter().cloned().collect();
             // let context_param : GenericParam = parse_quote!( Context2 = () ); // Already defined
             // let formed_param : GenericParam = parse_quote!( Formed2 = #enum_name< #enum_generics_ty_no_comma > ); // Already defined
             let end_param : GenericParam = parse_quote!( End2 = #end_struct_name< #enum_generics_ty_no_comma > ); // Use no_comma
             def_generics_impl_vec.push( context_param.clone() ); // Clone before moving
             def_generics_impl_vec.push( formed_param.clone() ); // Clone before moving
             def_generics_impl_vec.push( end_param.clone() ); // Clone before moving
             let def_generics_impl_punctuated = Punctuated::<_, Comma>::from_iter( def_generics_impl_vec );

             let def_generics_syn = syn::Generics { params: def_generics_impl_punctuated.clone(), ..(*generics).clone() };
             let ( _def_generics_with_defaults, def_generics_impl, def_generics_ty_with_comma, _def_generics_where ) = generic_params::decompose( &def_generics_syn );
             let def_generics_ty_no_comma = def_generics_ty_with_comma.pairs().map( | p | p.value().clone() ).collect::< Punctuated< _, Comma > >(); // Corrected: Added .clone()
             let def_phantom = macro_tools::phantom::tuple( &def_generics_impl ); // Use qualified path
             // Push Definition struct definition
             let def_struct_tokens = {
                 let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 quote!
                 {
                   #[ derive( Debug ) ]
                   #vis struct #def_name < #def_generics_impl >
                   #where_clause_tokens
                   {
                     _phantom : #def_phantom,
                   }
                 }
             };
             ctx.end_impls.push( def_struct_tokens );
             // Push Default impl for Definition
             let def_default_impl_tokens = {
                 let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 quote!
                 {
                   impl< #def_generics_impl > ::core::default::Default
                   for #def_name < #def_generics_ty_no_comma > // Use no_comma
                   #where_clause_tokens
                   {
                     fn default() -> Self
                     {
                       Self { _phantom : ::core::marker::PhantomData }
                     }
                   }
                 }
             };
             ctx.end_impls.push( def_default_impl_tokens );
             // Push former::FormerDefinition impl
             let former_definition_impl_tokens = {
                 let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 // Add the End2 bound here
                 let mut where_clause_with_end_bound = where_clause_tokens.clone();
                 if !where_clause_with_end_bound.to_string().contains("where") {
                     where_clause_with_end_bound = quote! { where };
                 } else if !where_clause_with_end_bound.to_string().ends_with(',') && !where_clause_with_end_bound.to_string().ends_with("where ") {
                     where_clause_with_end_bound = quote! { #where_clause_with_end_bound , };
                 }
                 // Construct DefinitionTypes generics list for the bound
                 let mut def_types_bound_generics_vec : Vec<GenericParam> = enum_generics_ty_no_comma.clone().into_iter().collect(); // Use clone().into_iter()
                 let context_param : GenericParam = parse_quote!( Context2 ); // Use generic parameter directly
                 let formed_param : GenericParam = parse_quote!( Formed2 ); // Use generic parameter directly
                 def_types_bound_generics_vec.push( context_param );
                 def_types_bound_generics_vec.push( formed_param );
                 let def_types_bound_generics = Punctuated::<_, Comma>::from_iter( def_types_bound_generics_vec );

                 where_clause_with_end_bound = quote! { #where_clause_with_end_bound End2 : former::FormingEnd< #def_types_name< #def_types_bound_generics > > }; // Use constructed list

                 quote!
                 {
                   impl< #def_generics_impl > former::FormerDefinition
                   for #def_name < #def_generics_ty_no_comma > // Use no_comma
                   #where_clause_with_end_bound // Use the clause with the End2 bound
                   {
                     type Storage = #storage_struct_name< #enum_generics_ty_no_comma >; // Use no_comma
                     type Context = Context2; // Use Context2 generic param
                     type Formed = Formed2; // Use Formed2 generic param
                     // Correctly reference DefinitionTypes with its generics
                     type Types = #def_types_name< #def_types_bound_generics >; // Use constructed list
                     type End = End2; // Use End2 generic param
                   }
                 }
             };
             ctx.end_impls.push( former_definition_impl_tokens );

             // --- Generate Former Struct ---
             // Construct the generics for the former struct directly
             let mut former_generics_params_vec : Vec<GenericParam> = generics.params.iter().cloned().collect();
             // Construct the Definition generic argument
             let mut def_arg_generics_vec : Vec<GenericParam> = enum_generics_ty_no_comma.clone().into_iter().collect(); // Use clone().into_iter()
             let context_arg_param : GenericParam = parse_quote!( Context = () );
             let formed_arg_param : GenericParam = parse_quote!( Formed = #enum_name<#enum_generics_ty_no_comma> );
             let end_arg_param : GenericParam = parse_quote!( End = #end_struct_name<#enum_generics_ty_no_comma> );
             def_arg_generics_vec.push( context_arg_param.clone() );
             def_arg_generics_vec.push( formed_arg_param.clone() );
             def_arg_generics_vec.push( end_arg_param.clone() );
             let def_arg_generics = Punctuated::<_, Comma>::from_iter( def_arg_generics_vec );
             let def_param : GenericParam = parse_quote!( Definition = #def_name< #def_arg_generics > );
             former_generics_params_vec.push( def_param.clone() );
             let former_generics_params = Punctuated::<_, Comma>::from_iter( former_generics_params_vec );


             // Define necessary bounds for the Definition generic in the Former's where clause
             let mut former_where_predicates : Punctuated< syn::WherePredicate, Comma > = Punctuated::new();
             former_where_predicates.push( parse_quote!{ Definition : former::FormerDefinition< Storage = #storage_struct_name< #enum_generics_ty_no_comma > > } ); // Use no_comma
             // Construct DefinitionTypes generics list for the bound
             let mut def_types_bound_generics_vec : Vec<GenericParam> = enum_generics_ty_no_comma.clone().into_iter().collect(); // Use clone().into_iter()
             // let context_param_bound : GenericParam = parse_quote!( Context = () ); // Already defined
             // let formed_param_bound : GenericParam = parse_quote!( Formed = #enum_name< #enum_generics_ty_no_comma > ); // Already defined
             def_types_bound_generics_vec.push( context_param.clone() );
             def_types_bound_generics_vec.push( formed_param.clone() );
             let def_types_bound_generics = Punctuated::<_, Comma>::from_iter( def_types_bound_generics_vec );
             former_where_predicates.push( parse_quote!{ Definition::Types : former::FormerDefinitionTypes< Storage = #storage_struct_name< #enum_generics_ty_no_comma >, Context = (), Formed = #enum_name< #enum_generics_ty_no_comma > > } ); // Use no_comma, () and EnumName for Context/Formed
             // Add FormerMutator bound
             former_where_predicates.push( parse_quote!{ Definition::Types : former::FormerMutator } );
             // Add enum's original where clause predicates
             let final_where_clause : Punctuated< WherePredicate, Comma > = generics.where_clause.as_ref().map( | wc | wc.predicates.clone() ).unwrap_or_default().into_iter().chain( former_where_predicates ).collect(); // Corrected: Collect into Punctuated

             let former_generics_syn = syn::Generics {
                 lt_token: generics.lt_token,
                 params: former_generics_params,
                 gt_token: generics.gt_token,
                 where_clause: Some(syn::WhereClause {
                     where_token: syn::token::Where::default(), // Use default token
                     predicates: final_where_clause, // Use the combined predicates
                 }),
             };

             let ( _former_generics_with_defaults, former_generics_impl, former_generics_ty_with_comma, _former_generics_where_clause ) = generic_params::decompose( &former_generics_syn ); // Use _former_generics_where
             let former_generics_ty_no_comma = former_generics_ty_with_comma.pairs().map( | p | p.value().clone() ).collect::< Punctuated< _, Comma > >(); // Corrected: Added .clone()
             // Push Former struct definition
             let former_struct_tokens = {
                 // Use the correctly constructed where clause from former_generics_syn
                 let where_clause_tokens = if let Some( where_clause ) = &former_generics_syn.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 quote!
                 {
                   #[ doc = "Former for the #variant_ident variant." ]
                   #vis struct #former_name < #former_generics_impl >
                   #where_clause_tokens // Use the constructed where clause
                   {
                     /// Temporary storage for all fields during the formation process.
                     pub storage : Definition::Storage,
                     /// Optional context.
                     pub context : ::core::option::Option< Definition::Context >,
                     /// Optional handler for the end of formation.
                     pub on_end : ::core::option::Option< Definition::End >,
                     // Add phantom data for Definition generic
                     _phantom_def : ::core::marker::PhantomData< Definition >,
                   }
                 }
             };
             ctx.end_impls.push( former_struct_tokens );
             // --- Generate Former Impl + Setters ---
             let setters = variant_field_info.iter().map( |f_info|
             {
               let field_ident = &f_info.ident; // Use f_info
               let field_type = &f_info.ty; // Use f_info
               let setter_name = ident::ident_maybe_raw( field_ident ); // Uses ident_maybe_raw
               quote!
               {
                 #[ doc = "Setter for the #field_ident field." ] // FIX: Add doc comment for setter
                 #[ inline ]
                 pub fn #setter_name< Src >( mut self, src : Src ) -> Self
                 where Src : ::core::convert::Into< #field_type >
                 {
                   debug_assert!( self.storage.#field_ident.is_none() );
                   self.storage.#field_ident = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
                   self
                 }
               }
             });
             // Push Former impl block
             let former_impl_tokens = {
                 // Use the correctly constructed where clause from former_generics_syn
                 let former_impl_where_clause = former_generics_syn.where_clause.as_ref().map( | wc | &wc.predicates ).cloned().unwrap_or_default();
                 // Add FormerMutator bound to the where clause - Already included in former_impl_where_clause
                 // let former_impl_where_clause = quote!{ where #former_impl_where_clause Definition::Types : former::FormerMutator };
                 quote!
                 {
                   #[ automatically_derived ]
                    impl< #former_generics_impl > #former_name < #former_generics_ty_no_comma > // Use no_comma
                    where #former_impl_where_clause // Use the constructed where clause with bounds
                    {
                        use former::FormingEnd; // Bring FormingEnd trait into scope
                        // Standard former methods (new, begin, form, end) - Adjusted to use Definition::Types
                        #[ inline( always ) ] pub fn new( on_end : Definition::End ) -> Self { Self::begin( None, None, on_end ) }
                        #[ inline( always ) ] pub fn new_coercing< IntoEnd >( end : IntoEnd ) -> Self where IntoEnd : Into< Definition::End > { Self::begin_coercing( None, None, end ) }
                        #[ inline( always ) ] pub fn begin ( mut storage : ::core::option::Option< Definition::Storage >, context : ::core::option::Option< Definition::Context >, on_end : Definition::End ) -> Self
                        {
                          if storage.is_none() { storage = Some( Default::default() ); }
                          Self { storage : storage.unwrap(), context, on_end : Some( on_end ), _phantom_def : ::core::marker::PhantomData }
                        }
                        #[ inline( always ) ] pub fn begin_coercing< IntoEnd > ( mut storage : ::core::option::Option< Definition::Storage >, context : ::core::option::Option< Definition::Context >, on_end : IntoEnd ) -> Self where IntoEnd : Into< Definition::End >
                        {
                          if storage.is_none() { storage = Some( Default::default() ); }
                          Self { storage : storage.unwrap(), context, on_end : Some( on_end.into() ), _phantom_def : ::core::marker::PhantomData }
                        }
                        #[ inline( always ) ] pub fn form( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed { self.end() }
                        #[ inline( always ) ] pub fn end( mut self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
                        {
                          let context = self.context.take();
                          let on_end = self.on_end.take().unwrap();
                          // Apply mutator
                          < Definition::Types as former::FormerMutator >::form_mutation( &mut self.storage, &mut context ); // Apply mutator here
                          on_end.call( self.storage, context )
                        }
                        // Field setters
                        #( #setters )*
                    }
                 }
             };
             ctx.end_impls.push( former_impl_tokens );
             // --- Generate End Struct ---
             let phantom_field_type = macro_tools::phantom::tuple( &generics.params ); // Use qualified path and correct generics
             // Push End struct definition
             let end_struct_tokens = {
                 let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 quote!
                 {
                   #[ derive( Default, Debug ) ]
                   #vis struct #end_struct_name < #enum_generics_impl >
                   #where_clause_tokens
                   {
                     _phantom : #phantom_field_type,
                   }
                 }
             };
             ctx.end_impls.push( end_struct_tokens );
             // --- Generate End Impl ---
             let tuple_indices = ( 0..ctx.variant_field_info.len() ).map( syn::Index::from ); // Needed for destructuring
             let field_idents_for_construction : Vec<_> = ctx.variant_field_info.iter().map( |f| &f.ident ).collect(); // Needed for struct literal
             // Generate token stream for the type within the angle brackets for FormingEnd impl - Use () and EnumName for Context/Formed
             let forming_end_impl_tokens = {
                 let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 // Construct DefinitionTypes generics list for FormingEnd impl
                 let mut forming_end_def_types_generics_vec : Vec<GenericParam> = enum_generics_ty_no_comma.clone().into_iter().collect(); // Use clone().into_iter()
                 let context_param : GenericParam = parse_quote!( Context2 = () );
                 let formed_param : GenericParam = parse_quote!( Formed2 = #enum_name< #enum_generics_ty_no_comma > );
                 forming_end_def_types_generics_vec.push( context_param );
                 forming_end_def_types_generics_vec.push( formed_param );
                 let forming_end_def_types_generics = Punctuated::<_, Comma>::from_iter( forming_end_def_types_generics_vec );

                 quote!
                 {
                   #[ automatically_derived ]
                   impl< #enum_generics_impl > former::FormingEnd
                   <
                     // Correct generics usage and add comma_if_enum_generics
                     #def_types_name< #forming_end_def_types_generics > // Use constructed list
                   >
                   for #end_struct_name < #enum_generics_ty_no_comma > // Use no_comma
                   #where_clause_tokens
                   {
                     #[ inline( always ) ]
                     fn call
                     (
                       &self,
                       sub_storage : #storage_struct_name< #enum_generics_ty_no_comma >, // Use no_comma
                       _context : Option< () >,
                     )
                     ->
                     #enum_name< #enum_generics_ty_no_comma > // Use no_comma
                     {
                       // Correctly destructure the tuple from preform and use field names
                        let preformed_tuple = former::StoragePreform::preform( sub_storage );
                        #enum_name::#variant_ident
                        {
                         #( #field_idents_for_construction : preformed_tuple.#tuple_indices ),* // Construct using field names
                        }
                     }
                   }
                 }
             };
             ctx.end_impls.push( forming_end_impl_tokens );
             // --- Generate Static Method ---
             // Push static method for the implicit Former - Use () and EnumName for Context/Formed
             let static_method_tokens = {
                 let _where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                     if where_clause.predicates.is_empty() {
                         quote! {}
                     } else {
                         let predicates = &where_clause.predicates;
                         quote! { where #predicates }
                     }
                 } else {
                     quote! {}
                 };
                 // Construct Definition generics list for return type
                 let mut static_method_def_generics_vec : Vec<GenericParam> = enum_generics_ty_no_comma.clone().into_iter().collect(); // Use clone().into_iter()
                 let context_param : GenericParam = parse_quote!( Context2 = () );
                 let formed_param : GenericParam = parse_quote!( Formed2 = #enum_name< #enum_generics_ty_no_comma > );
                 let end_param : GenericParam = parse_quote!( End2 = #end_struct_name< #enum_generics_ty_no_comma > );
                 static_method_def_generics_vec.push( context_param );
                 static_method_def_generics_vec.push( formed_param );
                 static_method_def_generics_vec.push( end_param );
                 let static_method_def_generics = Punctuated::<_, Comma>::from_iter( static_method_def_generics_vec );

                 quote!
                 {
                   /// Starts forming the #variant_ident variant using its implicit former.
                   #[ inline( always ) ]
                   #vis fn #method_name ()
                   ->
                   #former_name
                   <
                     #enum_generics_ty_no_comma, // Use no_comma // Enum generics
                     // Default definition for the implicit former
                     #def_name< #static_method_def_generics > // Use constructed list
                   >
                   {
                     #former_name::begin( None, None, #end_struct_name::< #enum_generics_ty_no_comma >::default() ) // Use no_comma
                   }
                 }
             };
             ctx.methods.push( static_method_tokens );
             // --- Generate Standalone Constructor (Subform Struct(N)) ---
             if struct_attrs.standalone_constructors.value( false )
             {
                 let constructor_params : Vec<_> = variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
                 let all_fields_are_args = !variant_field_info.is_empty() && variant_field_info.iter().all( |f| f.is_constructor_arg );
                 // Construct Definition generics list for return type
                 let mut standalone_def_generics_vec : Vec<GenericParam> = enum_generics_ty_no_comma.clone().into_iter().collect(); // Use clone().into_iter()
                 let context_param : GenericParam = parse_quote!( Context2 = () );
                 let formed_param : GenericParam = parse_quote!( Formed2 = #enum_name< #enum_generics_ty_no_comma > );
                 let end_param : GenericParam = parse_quote!( End2 = #end_struct_name< #enum_generics_ty_no_comma > );
                 standalone_def_generics_vec.push( context_param );
                 standalone_def_generics_vec.push( formed_param );
                 standalone_def_generics_vec.push( end_param );
                 let standalone_def_generics = Punctuated::<_, Comma>::from_iter( standalone_def_generics_vec );
                 // Construct Former generics list for return type
                 let mut standalone_former_generics_vec : Vec<GenericParam> = enum_generics_ty_no_comma.clone().into_iter().collect(); // Use clone().into_iter()
                 let def_param : GenericParam = parse_quote!( Definition = #def_name< #standalone_def_generics > );
                 standalone_former_generics_vec.push( def_param );
                 let standalone_former_generics = Punctuated::<_, Comma>::from_iter( standalone_former_generics_vec );

                 let return_type = if all_fields_are_args { quote! { #enum_name< #enum_generics_ty_no_comma > } } else { quote! { #former_name < #standalone_former_generics > } }; // Use constructed lists
                 let initial_storage_assignments = variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let fi = &f.ident; let pn = ident::ident_maybe_raw( fi ); quote! { #fi : ::core::option::Option::Some( #pn.into() ) } } ); // Filter only constructor args
                 let initial_storage_code = if constructor_params.is_empty() { quote! { ::core::option::Option::None } } else { quote! { ::core::option::Option::Some( #storage_struct_name :: < #enum_generics_ty_no_comma > { #( #initial_storage_assignments, )* ..Default::default() } ) } }; // Use no_comma
                 let constructor_body = if all_fields_are_args { let construction_args = variant_field_info.iter().map( |f| { let fi = &f.ident; let pn = ident::ident_maybe_raw( fi ); quote! { #fi : #pn.into() } } ); quote! { #enum_name::#variant_ident { #( #construction_args ),* } } } else { quote! { #former_name::begin( #initial_storage_code, None, #end_struct_name::< #enum_generics_ty_no_comma >::default() ) } }; // Use no_comma
                 let standalone_constructor_tokens = {
                     let where_clause_tokens = if let Some( where_clause ) = &ctx.generics.where_clause {
                         if where_clause.predicates.is_empty() {
                             quote! {}
                         } else {
                             let predicates = &where_clause.predicates;
                             quote! { where #predicates }
                         }
                     } else {
                         quote! {}
                     };
                     quote!
                     {
                         /// Standalone constructor for the #variant_ident subform variant.
                         #[ inline( always ) ]
                         #vis fn #method_name < #enum_generics_impl >
                         (
                           #( #constructor_params ),*
                         )
                         ->
                         #return_type
                         #where_clause_tokens
                         {
                           #constructor_body
                         }
                     }
                 };
                 ctx.standalone_constructors.push( standalone_constructor_tokens );
             }
             // --- End Standalone Constructor ---


         } // End Default: Subformer
       }
       _ => return Err( Error::new_spanned( variant, "Former derive macro only supports named fields for struct variants" ) ),
     }
     Ok( () )
   }