// File: module/core/former_meta/src/derive_former/former_enum/tuple_non_zero.rs
use super::*; // Use items from parent module (former_enum)

use macro_tools::
{
  generic_params, Result,
  quote::{ format_ident, quote }, // Removed unused TokenStream
  ident,
  parse_quote,
  syn::punctuated::Punctuated,
  syn::token::Comma,
};
use syn::
{
  self,
  Fields,
  GenericParam,
  TypeParam,
  ConstParam,
  LifetimeParam,
  GenericArgument,
  Expr,
};
use convert_case::{ Case, Casing };

/// Handles the generation of code for tuple variants with non-zero fields.
// #[ allow( unused_variables, clippy::too_many_lines ) ] // Removed unused_variables, too_many_lines might still apply
#[ allow( clippy::too_many_lines ) ] // Keep this one for now
pub( super ) fn handle_tuple_non_zero_variant< 'a >
(
  ctx : &mut EnumVariantHandlerContext< 'a >, // Changed signature to use context struct
) -> Result< () >
{
  let variant_ident = &ctx.variant.ident;
  let method_name = format_ident!( "{}", variant_ident.to_string().to_case( Case::Snake ) );
  let method_name = ident::ident_maybe_raw( &method_name );
  let ( _enum_generics_with_defaults, enum_generics_impl, enum_generics_ty, _enum_generics_where_punctuated )
  = generic_params::decompose( ctx.generics );
  let enum_generics_where = ctx.merged_where_clause;

  let wants_scalar = ctx.variant_attrs.scalar.is_some() && ctx.variant_attrs.scalar.as_ref().unwrap().setter();
  let wants_subform_scalar = ctx.variant_attrs.subform_scalar.is_some();

  let fields = match &ctx.variant.fields { Fields::Unnamed( f ) => f, _ => unreachable!() };
  let field_count = fields.unnamed.len();

  // FIX: Reinstate match statement
  match field_count
  {
    1 =>
    {
      // --- Single-Field Tuple Variant ---
      let field_info = &ctx.variant_field_info[0];
      let inner_type = &field_info.ty;

      if wants_scalar
      {
        // --- Scalar Tuple(1) ---
        if ctx.struct_attrs.standalone_constructors.value( false )
        {
          let constructor_params : Vec<_> = ctx.variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
          let all_fields_are_args = !ctx.variant_field_info.is_empty() && ctx.variant_field_info.iter().all( |f| f.is_constructor_arg );
          // Access enum_name and enum_generics_ty from ctx
          let return_type = if all_fields_are_args { quote! { #&ctx.enum_name< #enum_generics_ty > } } else { return Err( syn::Error::new_spanned( ctx.variant, "#[scalar] on single-field variant implies all fields are constructor args, but #[arg_for_constructor] is missing." ) ); };
          let param_name = format_ident!( "_0" );
          let constructor = quote!
          {
            /// Standalone constructor for the #variant_ident variant (scalar style).
            #[ inline( always ) ]
            #ctx.vis fn #method_name < #enum_generics_impl > ( #( #constructor_params ),* ) -> #return_type where #enum_generics_where { Self::#variant_ident( #param_name.into() ) } // FIX: Corrected interpolation of ctx.vis
          };
          ctx.standalone_constructors.push( constructor );
        }
        let param_name = format_ident!( "_0" );
        let static_method = quote!
        {
          /// Constructor for the #variant_ident variant (scalar style).
          #[ inline( always ) ]
          #ctx.vis fn #method_name( #param_name : impl Into< #inner_type > ) -> Self { Self::#variant_ident( #param_name.into() ) } // FIX: Corrected interpolation of ctx.vis
        };
        ctx.methods.push( static_method );
      }
      else // Default or explicit subform_scalar -> Subformer
      {
        if wants_subform_scalar
        {
          if !matches!( inner_type, syn::Type::Path( _ ) ) { return Err( syn::Error::new_spanned( inner_type, "#[subform_scalar] can only be applied to variants holding a path type (e.g., MyStruct, Option<T>), not tuples, references, etc." ) ); }
        }
        else // Default case
        if !matches!( inner_type, syn::Type::Path( _ ) ) { return Err( syn::Error::new_spanned( inner_type, "Default subforming requires the single field of a tuple variant to be a path type (e.g., MyStruct, Option<T>)." ) ); }

        // Access enum_name from ctx
        let end_struct_name = format_ident!( "{}{}End", ctx.enum_name, variant_ident );
        let ( inner_type_name, inner_generics ) = match inner_type { syn::Type::Path( tp ) => { let s = tp.path.segments.last().unwrap(); ( s.ident.clone(), s.arguments.clone() ) }, _ => unreachable!() };
        let inner_former_name = format_ident!( "{}Former", inner_type_name );
        let inner_storage_name = format_ident!( "{}FormerStorage", inner_type_name );
        let inner_def_name = format_ident!( "{}FormerDefinition", inner_type_name );
        let inner_def_types_name = format_ident!( "{}FormerDefinitionTypes", inner_type_name );
        let inner_generics_params : Punctuated<GenericParam, Comma> = match &inner_generics { syn::PathArguments::AngleBracketed( args ) => args.args.iter().map( |arg| match arg { GenericArgument::Type( ty ) => match ty { syn::Type::Path( p ) => GenericParam::Type( TypeParam { ident: p.path.get_ident().unwrap().clone(), attrs: vec![], colon_token: None, bounds: Punctuated::new(), eq_token: None, default: None } ), _ => panic!("Unsupported generic argument type for TypeParam ident extraction"), }, GenericArgument::Lifetime( lt ) => GenericParam::Lifetime( LifetimeParam::new( lt.clone() ) ), GenericArgument::Const( c ) => match c { Expr::Path( p ) => GenericParam::Const( ConstParam { ident: p.path.get_ident().unwrap().clone(), attrs: vec![], const_token: Default::default(), colon_token: Default::default(), ty: parse_quote!(_), eq_token: None, default: None } ), _ => panic!("Unsupported const expression for ConstParam ident extraction"), }, _ => panic!("Unsupported generic argument type"), }).collect(), _ => Punctuated::new(), };
        let mut inner_generics_ty_punctuated = inner_generics_params.clone();
        if !inner_generics_ty_punctuated.empty_or_trailing() { inner_generics_ty_punctuated.push_punct( Default::default() ); }
        let comma_if_enum_generics = if enum_generics_ty.is_empty() { quote!{} } else { quote!{ , } };

        if ctx.struct_attrs.standalone_constructors.value( false )
        {
          let constructor_params : Vec<_> = ctx.variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
          let all_fields_are_args = !ctx.variant_field_info.is_empty() && ctx.variant_field_info.iter().all( |f| f.is_constructor_arg );
          // Access enum_name and enum_generics_ty from ctx
          let return_type = if all_fields_are_args { quote! { #&ctx.enum_name< #enum_generics_ty > } } else { quote! { #inner_former_name < #inner_generics_ty_punctuated #inner_def_name < #inner_generics_ty_punctuated (), #comma_if_enum_generics #&ctx.enum_name< #enum_generics_ty >, #end_struct_name < #enum_generics_ty > > > } }; // FIX: Added comma
          let initial_storage_code = if field_info.is_constructor_arg { let param_name = format_ident!( "_0" ); quote! { ::core::option::Option::Some( #inner_storage_name :: < #inner_generics_ty_punctuated > { _0 : ::core::option::Option::Some( #param_name.into() ) } ) } } else { quote! { ::core::option::Option::None } };
          // Access vis from ctx
          let constructor = quote!
          {
            /// Standalone constructor for the #variant_ident variant (scalar style).
            #[ inline( always ) ]
            #ctx.vis fn #method_name < #enum_generics_impl > ( #( #constructor_params ),* ) -> #return_type where #enum_generics_where { #inner_former_name::begin( #initial_storage_code, None, #end_struct_name::< #enum_generics_ty >::default() ) } // FIX: Corrected interpolation of ctx.vis
          };
          ctx.standalone_constructors.push( constructor );
        }

        // Access generics from ctx
        let phantom_field_type = macro_tools::phantom::tuple( &ctx.generics.params );
        // Access vis from ctx
        let end_struct_def = quote! { #[ derive( Default, Debug ) ] #ctx.vis struct #end_struct_name < #enum_generics_impl > where #enum_generics_where { _phantom : #phantom_field_type, } }; // FIX: Corrected interpolation of ctx.vis
        let end_impl = quote!
        {
          #[ automatically_derived ]
          impl< #enum_generics_impl > former::FormingEnd
          // FIX: Added comma after ()
          // Access enum_name and enum_generics_ty from ctx
          < #inner_def_types_name< #inner_generics_ty_punctuated (), #comma_if_enum_generics #&ctx.enum_name< #enum_generics_ty > > >
          for #end_struct_name < #enum_generics_ty >
          where #enum_generics_where
          {
            #[ inline( always ) ]
            fn call
            (
              &self,
              sub_storage : #inner_storage_name< #inner_generics_ty_punctuated >,
              _context : Option< () > // FIX: Removed trailing comma
            )
            // Access enum_name and enum_generics_ty from ctx
            -> #&ctx.enum_name< #enum_generics_ty >
            {
              let data = former::StoragePreform::preform( sub_storage );
              // Access enum_name from ctx
              #&ctx.enum_name::#variant_ident( data )
            }
          }
        };
        let static_method = quote!
        {
          /// Starts forming the #variant_ident variant using its implicit former.
          #[ inline( always ) ]
          #ctx.vis fn #method_name () // FIX: Corrected interpolation of ctx.vis
          -> #inner_former_name
          <
            #inner_generics_ty_punctuated
            #inner_def_name
            // FIX: Added comma after ()
            // Access enum_name and enum_generics_ty from ctx
            < #inner_generics_ty_punctuated (), #comma_if_enum_generics #&ctx.enum_name< #enum_generics_ty >, #end_struct_name < #enum_generics_ty > >
          >
          { #inner_former_name::begin( None, None, #end_struct_name::< #enum_generics_ty >::default() ) }
        };
        ctx.methods.push( static_method );
        ctx.end_impls.push( quote!{ #end_struct_def #end_impl } );
      }
    }
    _ => // len > 1
    {
      // --- Multi-Field Tuple Variant ---
      if wants_subform_scalar
      {
        return Err( syn::Error::new_spanned( ctx.variant, "#[subform_scalar] cannot be used on tuple variants with multiple fields." ) );
      }
      // Default is scalar for multi-field tuple
      else // Default or explicit scalar
      {
        if ctx.struct_attrs.standalone_constructors.value( false )
        {
          let constructor_params : Vec<_> = ctx.variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
          // Access enum_name and enum_generics_ty from ctx
          let return_type = quote! { #&ctx.enum_name< #enum_generics_ty > };
          let mut direct_construction_args = Vec::new();
          // FIX: Iterate over ctx.variant_field_info directly (remove &)
          for field_info_inner in ctx.variant_field_info { let param_name = &field_info_inner.ident; direct_construction_args.push( quote! { #param_name.into() } ); }
          // Access vis from ctx
          let constructor = quote! { #[ inline( always ) ] #ctx.vis fn #method_name < #enum_generics_impl > ( #( #constructor_params ),* ) -> #return_type where #enum_generics_where { Self::#variant_ident( #( #direct_construction_args ),* ) } }; // FIX: Corrected interpolation of ctx.vis
          ctx.standalone_constructors.push( constructor );
        }
        let mut params = Vec::new();
        let mut args = Vec::new();
        // FIX: Iterate over ctx.variant_field_info directly (remove &)
        for field_info in ctx.variant_field_info { let param_name = &field_info.ident; let field_type = &field_info.ty; params.push( quote! { #param_name : impl Into< #field_type > } ); args.push( quote! { #param_name.into() } ); }
        // Access vis from ctx
        let static_method = quote! { #[ inline( always ) ] #ctx.vis fn #method_name ( #( #params ),* ) -> Self { Self::#variant_ident( #( #args ),* ) } }; // FIX: Corrected interpolation of ctx.vis
        ctx.methods.push( static_method );
      }
    }
  }
  Ok( () )
}