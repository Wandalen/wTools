// File: module/core/former_meta/src/derive_former/former_enum/struct_non_zero.rs
use super::*; // Use items from parent module (former_enum)

use macro_tools::
{
  generic_params, Result,
  quote::{ format_ident, quote },
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
  ctx : &mut EnumVariantHandlerContext< 'a >,
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
  let method_name_ident_temp = format_ident!( "{}", method_name_snake_str, span = variant_ident.span() );
  let method_name = ident::ident_maybe_raw( &method_name_ident_temp );

  let ( _enum_generics_with_defaults, enum_generics_impl, enum_generics_ty, _enum_generics_where )
  = generic_params::decompose( generics );

  // Check if the attribute is present using .is_some()
  let wants_subform_scalar = variant_attrs.subform_scalar.is_some();
  let wants_scalar = variant_attrs.scalar.is_some();

  // FIX: Helper for conditional comma
  let comma_if_enum_generics = if enum_generics_ty.is_empty() { quote!{} } else { quote!{ , } };


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
                  Expr::Path( p ) => GenericParam::Const( ConstParam { ident: p.path.get_ident().unwrap().clone(), attrs: vec![], const_token: Default::default(), colon_token: Default::default(), ty: parse_quote!(_), eq_token: None, default: None } ),
                  &_ => panic!("Unsupported const expression for ConstParam ident extraction"),
                },
              _ => panic!("Unsupported generic argument type"), // Or return error
            }).collect(),
            _ => Punctuated::new(),
          };
          let mut inner_generics_ty_punctuated = inner_generics_params.clone();
          if !inner_generics_ty_punctuated.empty_or_trailing() { inner_generics_ty_punctuated.push_punct( Default::default() ); }


          // --- Standalone Constructor (Subform Struct(1)) ---
          if struct_attrs.standalone_constructors.value( false )
          {
              let constructor_params : Vec<_> = variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
              let all_fields_are_args = !variant_field_info.is_empty() && variant_field_info.iter().all( |f| f.is_constructor_arg );
              let return_type = if all_fields_are_args
              {
                 quote! { #enum_name< #enum_generics_ty > }
              }
              else
              {
                quote! { #inner_former_name < #inner_generics_ty_punctuated #inner_def_name < #inner_generics_ty_punctuated (), #comma_if_enum_generics #enum_name< #enum_generics_ty >, #end_struct_name < #enum_generics_ty > > > }
              };
              // FIX: Use inner_generics_ty_punctuated in storage init
              let initial_storage_code = if field_info.is_constructor_arg
              {
                let fi = &field_info.ident;
                let pn = ident::ident_maybe_raw( fi );
                quote!
                {
                  ::core::option::Option::Some
                  (
                    #inner_storage_name :: < #inner_generics_ty_punctuated >
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
                          #end_struct_name::< #enum_generics_ty >::default() // End
                        )
                      }
                  }
              };
              ctx.standalone_constructors.push( constructor.into() );
             }
             // --- End Standalone Constructor ---

             // Associated method logic
             let phantom_field_type = macro_tools::phantom::tuple( &generics.params ); // FIX: Use qualified path and correct generics
             let _field_ident = &field_info.ident; // Get the single field's ident
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
             ctx.end_impls.push( end_struct_tokens.into() );
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
             let forming_end_type_tokens = quote! {
                 #inner_def_types_name< #inner_generics_ty_punctuated (), #comma_if_enum_generics #enum_name< #enum_generics_ty > >
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
                     // FIX: Correct generics usage and add comma_if_enum_generics
                     #forming_end_type_tokens
                   >
                   for #end_struct_name < #enum_generics_ty >
                   #where_clause_tokens
                   {
                     #[ inline( always ) ]
                     fn call
                     (
                       &self,
                       sub_storage : #inner_storage_name< #inner_generics_ty_punctuated >,
                       _context : Option< () >,
                     )
                     ->
                     #enum_name< #enum_generics_ty >
                     {
                       // FIX: Handle single vs multi-field preformed type
                       let preformed_tuple = former::StoragePreform::preform( sub_storage );
                       #enum_name::#variant_ident
                       {
                         #field_assignments_tokens
                       }
                     }
                   }
                 }
             };
             ctx.end_impls.push( forming_end_impl_tokens.into() );
             let static_method = quote!
             {
               /// Starts forming the #variant_ident variant using its implicit former.
               #[ inline( always ) ]
               #vis fn #method_name ()
               ->
               #inner_former_name
               <
                 #inner_generics_ty_punctuated
                 #inner_def_name
                 <
                   #inner_generics_ty_punctuated (), #comma_if_enum_generics #enum_name< #enum_generics_ty >, #end_struct_name < #enum_generics_ty > >
                 >
               >
               {
                 #inner_former_name::begin( None, None, #end_struct_name::< #enum_generics_ty >::default() )
               }
             };
             ctx.methods.push( static_method.into() );

         }
         else if wants_scalar
         {
             // --- Scalar Struct(N) Variant ---
             // --- Standalone Constructor (Scalar Struct(N)) ---
             if struct_attrs.standalone_constructors.value( false )
             {
                 let constructor_params : Vec<_> = variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
                 let return_type = {
                   quote! { #enum_name< #enum_generics_ty > }
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
                 ctx.standalone_constructors.push( constructor.into() );
             }
             // --- End Standalone Constructor ---

             // Associated method (direct constructor)
             let mut params = Vec::new();
             let mut args = Vec::new();
             // FIX: Iterate over ctx.variant_field_info directly (remove &)
             for field_info in ctx.variant_field_info.iter()
             {
                 let field_ident = &field_info.ident;
                 let param_name = ident::ident_maybe_raw( field_ident );
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
             ctx.methods.push( static_method.into() );
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
             let phantom_field_type = macro_tools::phantom::tuple( &generics.params ); // FIX: Use qualified path and correct generics
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
             ctx.end_impls.push( storage_struct_tokens.into() );
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
                   for #storage_struct_name < #enum_generics_ty >
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
             ctx.end_impls.push( storage_default_impl_tokens.into() );
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
                   for #storage_struct_name < #enum_generics_ty >
                   #where_clause_tokens
                   {
                     type Preformed = ( #( #field_types ),* ); // Preformed type is a tuple of field types
                   }
                 }
             };
             ctx.end_impls.push( storage_trait_impl_tokens.into() );
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
                   for #storage_struct_name < #enum_generics_ty >
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
             ctx.end_impls.push( storage_preform_impl_tokens.into() );

             // --- Generate DefinitionTypes ---
             // FIX: Correctly merge generics and handle commas
             let mut def_types_generics_impl_punctuated : Punctuated<GenericParam, Comma> = generics.params.clone();
             if !def_types_generics_impl_punctuated.is_empty() && !def_types_generics_impl_punctuated.trailing_punct() { def_types_generics_impl_punctuated.push_punct( Default::default() ); }
             def_types_generics_impl_punctuated.push( parse_quote!( Context2 = () ) );
             def_types_generics_impl_punctuated.push( parse_quote!( Formed2 = #enum_name< #enum_generics_ty > ) );
             let ( _def_types_generics_with_defaults, def_types_generics_impl, def_types_generics_ty, _def_types_generics_where ) = generic_params::decompose( &syn::Generics { params: def_types_generics_impl_punctuated, ..(*generics).clone() } );
             let def_types_phantom = macro_tools::phantom::tuple( &def_types_generics_impl ); // FIX: Use qualified path
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
             ctx.end_impls.push( def_types_struct_tokens.into() );
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
                   for #def_types_name < #def_types_generics_ty >
                   #where_clause_tokens
                   {
                     fn default() -> Self
                     {
                       Self { _phantom : ::core::marker::PhantomData }
                     }
                   }
                 }
             };
             ctx.end_impls.push( def_types_default_impl_tokens.into() );
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
                   for #def_types_name < #def_types_generics_ty >
                   #where_clause_tokens
                   {
                     type Storage = #storage_struct_name< #enum_generics_ty >;
                     type Context = Context2;
                     type Formed = Formed2; // Note: Formed2 already uses #enum_name
                     // FIX: Correctly reference DefinitionTypes with its generics
                     type Types = #def_types_name< #enum_generics_ty #comma_if_enum_generics Context2, Formed2 >;
                     type End = End2;
                   }
                 }
             };
             ctx.end_impls.push( former_definition_types_impl_tokens.into() );
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
                   for #def_types_name < #def_types_generics_ty >
                   #where_clause_tokens
                   {
                     // Default empty mutator
                   }
                 }
             };
             ctx.end_impls.push( former_mutator_impl_tokens.into() );

             // --- Generate Definition ---
             // FIX: Correctly merge generics and handle commas
             let mut def_generics_impl_punctuated : Punctuated<GenericParam, Comma> = generics.params.clone();
             if !def_generics_impl_punctuated.is_empty() && !def_generics_impl_punctuated.trailing_punct() { def_generics_impl_punctuated.push_punct( Default::default() ); }
             def_generics_impl_punctuated.push( parse_quote!( Context2 = () ) );
             def_generics_impl_punctuated.push( parse_quote!( Formed2 = #enum_name< #enum_generics_ty > ) );
             def_generics_impl_punctuated.push( parse_quote!( End2 = #end_struct_name< #enum_generics_ty > ) );
             let def_generics_syn = syn::Generics { params: def_generics_impl_punctuated, ..(*generics).clone() };
             let ( _def_generics_with_defaults, def_generics_impl, def_generics_ty, _def_generics_where ) = generic_params::decompose( &def_generics_syn );
             let def_phantom = macro_tools::phantom::tuple( &def_generics_impl ); // FIX: Use qualified path
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
             ctx.end_impls.push( def_struct_tokens.into() );
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
                   for #def_name < #def_generics_ty >
                   #where_clause_tokens
                   {
                     fn default() -> Self
                     {
                       Self { _phantom : ::core::marker::PhantomData }
                     }
                   }
                 }
             };
             ctx.end_impls.push( def_default_impl_tokens.into() );
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
                 quote!
                 {
                   impl< #def_generics_impl > former::FormerDefinition
                   for #def_name < #def_generics_ty >
                   #where_clause_tokens
                   {
                     type Storage = #storage_struct_name< #enum_generics_ty >;
                     type Context = Context2;
                     type Formed = Formed2; // Note: Formed2 already uses #enum_name
                     // FIX: Correctly reference DefinitionTypes with its generics
                     type Types = #def_types_name< #enum_generics_ty #comma_if_enum_generics Context2, Formed2 >;
                     type End = End2;
                   }
                 }
             };
             ctx.end_impls.push( former_definition_impl_tokens.into() );

             // --- Generate Former Struct ---
             // Construct the generics for the former struct directly
             let mut former_generics_params = generics.params.clone();
             if !former_generics_params.is_empty() && !former_generics_params.trailing_punct() { former_generics_params.push_punct( Default::default() ); }
             former_generics_params.push( parse_quote!( Definition = #def_name< #enum_generics_ty #comma_if_enum_generics (), #enum_name<#enum_generics_ty>, #end_struct_name<#enum_generics_ty> > ) );

             let mut former_where_predicates = Punctuated::new();
             former_where_predicates.push( parse_quote!{ Definition : former::FormerDefinition< Storage = #storage_struct_name< #enum_generics_ty > > } );
             former_where_predicates.push( parse_quote!{ Definition::Types : former::FormerDefinitionTypes< Storage = #storage_struct_name< #enum_generics_ty > > } );
             if let Some( enum_where ) = &generics.where_clause
             {
               for predicate in &enum_where.predicates
               {
                 let _ = predicate.clone() ; // Add let _ = to fix unused must use warning
               }
             }

             let former_generics_syn = syn::Generics {
                 lt_token: generics.lt_token,
                 params: former_generics_params,
                 gt_token: generics.gt_token,
                 where_clause: Some(syn::WhereClause {
                     where_token: Default::default(),
                     predicates: former_where_predicates,
                 }),
             };

             let ( _former_generics_with_defaults, former_generics_impl, former_generics_ty, _former_generics_where ) = generic_params::decompose( &former_generics_syn );
             // Push Former struct definition
             let former_struct_tokens = {
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
                   #[ doc = "Former for the #variant_ident variant." ]
                   #vis struct #former_name < #former_generics_impl >
                   #where_clause_tokens
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
             ctx.end_impls.push( former_struct_tokens.into() );
             // --- Generate Former Impl + Setters ---
             let setters = variant_field_info.iter().map( |f_info|
             {
               let field_ident = &f_info.ident;
               let field_type = &f_info.ty;
               let setter_name = ident::ident_maybe_raw( field_ident );
               quote!
               {
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
                   impl< #former_generics_impl > former::FormerName < #former_generics_ty >
                   #where_clause_tokens
                   {
                     // Standard former methods (new, begin, form, end)
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
                       // Apply mutator if needed (assuming default empty mutator for now)
                       // < Definition::Types as former::FormerMutator >::form_mutation( &mut self.storage, &mut self.context );
                       on_end.call( self.storage, context )
                     }
                     // Field setters
                     #( #setters )*
                   }
                 }
             };
             ctx.end_impls.push( former_impl_tokens.into() );
             // --- Generate End Struct ---
             let phantom_field_type = macro_tools::phantom::tuple( &generics.params ); // FIX: Use qualified path and correct generics
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
             ctx.end_impls.push( end_struct_tokens.into() );
             // --- Generate End Impl ---
             let _tuple_indices = ( 0..ctx.variant_field_info.len() ).map( syn::Index::from );
             let _field_idents_for_construction : Vec<_> = ctx.variant_field_info.iter().map( |f| &f.ident ).collect();
             // Generate token stream for struct field assignments in call function
             let field_assignments_tokens = {
                 let mut tokens = quote! {};
                 let tuple_indices = ( 0..ctx.variant_field_info.len() ).map( syn::Index::from );
                 let field_idents_for_construction : Vec<_> = ctx.variant_field_info.iter().map( |f| &f.ident ).collect();
                 for (field_ident, tuple_index) in field_idents_for_construction.iter().zip(tuple_indices) {
                     tokens.extend(quote! { #field_ident : preformed_tuple.#tuple_index, });
                 }
                 tokens
             };
             // Generate token stream for the type within the angle brackets for FormingEnd
             let forming_end_type_tokens = quote! {
                 #def_types_name< #enum_generics_ty #comma_if_enum_generics (), #enum_name< #enum_generics_ty > >
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
                     // FIX: Correct generics usage and add comma_if_enum_generics
                     #forming_end_type_tokens
                   >
                   for #end_struct_name < #enum_generics_ty >
                   #where_clause_tokens
                   {
                     #[ inline( always ) ]
                     fn call
                     (
                       &self,
                       sub_storage : #storage_struct_name< #enum_generics_ty >,
                       _context : Option< () >,
                     )
                     ->
                     #enum_name< #enum_generics_ty >
                     {
                       // FIX: Handle single vs multi-field preformed type
                       let preformed_tuple = former::StoragePreform::preform( sub_storage );
                       #enum_name::#variant_ident
                       {
                         #field_assignments_tokens
                       }
                     }
                   }
                 }
             };
             ctx.end_impls.push( forming_end_impl_tokens.into() );
             // --- Generate Static Method ---
             // Push static method for Former
             let static_method_tokens = {
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
                   /// Starts forming the #variant_ident variant using its implicit former.
                   #[ inline( always ) ]
                   #vis fn #method_name ()
                   ->
                   #former_name
                   <
                     #enum_generics_ty, // Enum generics
                     // Default definition
                     #def_name< #enum_generics_ty #comma_if_enum_generics (), #enum_name< #enum_generics_ty >, #end_struct_name< #enum_generics_ty > >
                   >
                   {
                     #former_name::begin( None, None, #end_struct_name::< #enum_generics_ty >::default() )
                   }
                 }
             };
             ctx.methods.push( static_method_tokens.into() );
             // --- Generate Standalone Constructor (Subform Struct(N)) ---
             if struct_attrs.standalone_constructors.value( false )
             {
                 let constructor_params : Vec<_> = variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
                 let all_fields_are_args = !variant_field_info.is_empty() && variant_field_info.iter().all( |f| f.is_constructor_arg );
                 // FIX: Added comma in return type generics
                 let return_type = if all_fields_are_args { quote! { #enum_name< #enum_generics_ty > } } else { quote! { #former_name < #enum_generics_ty, #def_name< #enum_generics_ty #comma_if_enum_generics (), #enum_name< #enum_generics_ty >, #end_struct_name< #enum_generics_ty > > > } };
                 let initial_storage_assignments = variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let fi = &f.ident; let pn = ident::ident_maybe_raw( fi ); quote! { #fi : ::core::option::Option::Some( #pn.into() ) } } ); // Filter only constructor args
                 let initial_storage_code = if constructor_params.is_empty() { quote! { ::core::option::Option::None } } else { quote! { ::core::option::Option::Some( #storage_struct_name :: < #enum_generics_ty > { #( #initial_storage_assignments, )* ..Default::default() } ) } };
                 let constructor_body = if all_fields_are_args { let construction_args = variant_field_info.iter().map( |f| { let fi = &f.ident; let pn = ident::ident_maybe_raw( fi ); quote! { #fi : #pn.into() } } ); quote! { #enum_name::#variant_ident { #( #construction_args ),* } } } else { quote! { #former_name::begin( #initial_storage_code, None, #end_struct_name::< #enum_generics_ty >::default() ) } };
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
                 ctx.standalone_constructors.push( standalone_constructor_tokens.into() );
             }
             // --- End Standalone Constructor ---


         } // End Default: Subformer
       }
       _ => return Err( Error::new_spanned( variant, "Former derive macro only supports named fields for struct variants" ) ),
     }
     Ok( () )
   }
