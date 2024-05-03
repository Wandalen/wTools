
use super::*;
use iter_tools::{ Itertools, process_results };
use macro_tools::{ attr, diag, generic_params, generic_args, container_kind, typ, Result };
use proc_macro2::TokenStream;

// zzz : explain concept of Storage
// xxx : feature to have storage fields
// xxx : introduce namespaces
// zzz : qqq : implement interfaces for other containers

mod field;
use field::*;
mod field_attrs;
use field_attrs::*;
mod struct_attrs;
use struct_attrs::*;

///
/// Generate documentation for the former.
///

fn doc_generate( stru : &syn::Ident ) -> ( String, String )
{

  let doc_former_mod = format!
  (
r#" Implementation of former for [{}].
"#,
    stru
  );

  let doc_example1 =
r#"
use former::Former;
#[ derive( Former ) ]
pub struct Struct1
{
  #[default( 31 ) ]
  field1 : i32,
}
"#;

  let doc_former_struct = format!
  (
r#" Object to form [{}]. If field's values is not set then default value of the field is set.

For specifying custom default value use attribute `default`. For example:
```
{}
```
"#,
    stru, doc_example1
  );

  ( doc_former_mod, doc_former_struct )
}

//

///
/// Generate parts, used for generating `perform()`` method.
///
/// Similar to `form()`, but will also invoke function from `perform` attribute, if specified.
///
/// # Example of returned tokens :
///
/// ## perform :
/// return result;
///
/// ## perform_output :
/// < T : ::core::default::Default >
///
/// ## perform_generics :
/// Vec< T >

pub fn performer
(
  attrs : &StructAttributes,
)
-> Result< ( TokenStream, TokenStream, TokenStream ) >
{

  let mut perform = qt!
  {
    return result;
  };
  let mut perform_output = qt!{ Definition::Formed };
  let mut perform_generics = qt!{};

  if let Some( ref attr ) = attrs.perform
  {

    // let attr_perform = syn::parse2::< AttributePerform >( meta_list.tokens.clone() )?;
    let signature = &attr.signature;
    let generics = &signature.generics;
    perform_generics = qt!{ #generics };
    let perform_ident = &signature.ident;
    let output = &signature.output;
    if let syn::ReturnType::Type( _, boxed_type ) = output
    {
      perform_output = qt!{ #boxed_type };
    }
    perform = qt!
    {
      return result.#perform_ident();
    };

  }

  Ok( ( perform, perform_output, perform_generics ) )
}

/// xxx : write documentation. provide example of generated code

pub fn storage_fields
(
  attrs : &StructAttributes,
)
-> Result< TokenStream >
{

  let mut result = qt!
  {
  };

  if let Some( ref attr ) = attrs.storage_fields
  {
    let storage_fields = &attr.fields;
    result = qt! { #storage_fields }
  }

  Ok( result )
}

///
/// Generate the whole Former ecosystem
///
/// Output examples can be found in [docs to former crate](https://docs.rs/former/latest/former/)
///

pub fn former( input : proc_macro::TokenStream ) -> Result< TokenStream >
{
  use macro_tools::IntoGenericArgs;

  let original_input = input.clone();
  let ast = match syn::parse::< syn::DeriveInput >( input )
  {
    Ok( syntax_tree ) => syntax_tree,
    Err( err ) => return Err( err ),
  };
  let has_debug = attr::has_debug( ast.attrs.iter() )?;
  let struct_attrs = StructAttributes::from_attrs( ast.attrs.iter() )?;

  let example_of_custom_setter = false;

  /* names */

  let stru = &ast.ident;
  let former_name = format!( "{}Former", stru );
  let former = syn::Ident::new( &former_name, stru.span() );
  let former_storage_name = format!( "{}FormerStorage", stru );
  let former_storage = syn::Ident::new( &former_storage_name, stru.span() );
  let former_definition_name = format!( "{}FormerDefinition", stru );
  let former_definition = syn::Ident::new( &former_definition_name, stru.span() );
  let former_definition_types_name = format!( "{}FormerDefinitionTypes", stru );
  let former_definition_types = syn::Ident::new( &former_definition_types_name, stru.span() );
  let as_subformer_name = format!( "{}AsSubformer", stru );
  let as_subformer = syn::Ident::new( &as_subformer_name, stru.span() );
  let as_subformer_end_name = format!( "{}AsSubformerEnd", stru );
  let as_subformer_end = syn::Ident::new( &as_subformer_end_name, stru.span() );

  // zzz : improve
  let as_subformer_end_doc = format!( "Alias for trait former::FormingEnd with context and formed the same type and definition of structure [`$(stru)`]. Use as subformer end of a field during process of forming of super structure." );

  /* parameters for structure */

  let generics = &ast.generics;
  let ( struct_generics_with_defaults, struct_generics_impl, struct_generics_ty, struct_generics_where )
  = generic_params::decompose( generics );

  /* parameters for definition */

  let extra : macro_tools::syn::AngleBracketedGenericArguments = parse_quote!
  {
    < (), #stru < #struct_generics_ty >, former::ReturnPreformed >
  };
  let former_definition_args = generic_args::merge( &generics.into_generic_args(), &extra.into() ).args;

  /* parameters for former */

  let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < Definition = #former_definition < #former_definition_args > >
    where
      Definition : former::FormerDefinition< Storage = #former_storage < #struct_generics_ty > >,
      Definition::Types : former::FormerDefinitionTypes< Storage = #former_storage < #struct_generics_ty > >,
  };
  let extra = generic_params::merge( &generics, &extra.into() );

  let ( former_generics_with_defaults, former_generics_impl, former_generics_ty, former_generics_where )
  = generic_params::decompose( &extra );

  /* parameters for former perform */

  let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < Definition = #former_definition < #former_definition_args > >
    where
      Definition : former::FormerDefinition
      <
        Storage = #former_storage < #struct_generics_ty >,
        Formed = #stru < #struct_generics_ty >,
      >,
      Definition::Types : former::FormerDefinitionTypes
      <
        Storage = #former_storage < #struct_generics_ty >,
        Formed = #stru < #struct_generics_ty >,
      >,
  };
  let extra = generic_params::merge( &generics, &extra.into() );

  let ( _former_perform_generics_with_defaults, former_perform_generics_impl, former_perform_generics_ty, former_perform_generics_where )
  = generic_params::decompose( &extra );

  /* parameters for definition types */

  let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < __Context = (), __Formed = #stru < #struct_generics_ty > >
  };
  let former_definition_type_generics = generic_params::merge( &generics, &extra.into() );
  let ( former_definition_type_generics_with_defaults, former_definition_type_generics_impl, former_definition_type_generics_ty, former_definition_type_generics_where )
  = generic_params::decompose( &former_definition_type_generics );

  let former_definition_type_phantom = macro_tools::phantom::tuple( &former_definition_type_generics_impl );

  /* parameters for definition */

  let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < __Context = (), __Formed = #stru < #struct_generics_ty >, __End = former::ReturnPreformed >
  };
  let generics_of_definition = generic_params::merge( &generics, &extra.into() );
  let ( former_definition_generics_with_defaults, former_definition_generics_impl, former_definition_generics_ty, former_definition_generics_where )
  = generic_params::decompose( &generics_of_definition );

  let former_definition_phantom = macro_tools::phantom::tuple( &former_definition_generics_impl );

  /* */

  let fields = match ast.data
  {
    syn::Data::Struct( ref data_struct ) => match data_struct.fields
    {
      syn::Fields::Named( ref fields_named ) =>
      {
        &fields_named.named
      },
      _ => return Err( syn_err!( ast, "Unknown format of data, expected syn::Fields::Named( ref fields_named )\n  {}", qt!{ #ast } ) ),
    },
    _ => return Err( syn_err!( ast, "Unknown format of data, expected syn::Data::Struct( ref data_struct )\n  {}", qt!{ #ast } ) ),
  };

  let former_fields : Vec< Result< FormerField< '_ > > > = fields.iter().map( | field |
  {
    let attrs = FieldAttributes::from_attrs( field.attrs.iter() )?;
    let vis = &field.vis;
    let ident = field.ident.as_ref()
    .ok_or_else( || syn_err!( field, "Expected that each field has key, but some does not:\n  {}", qt!{ #field } ) )?;
    let colon_token = &field.colon_token;
    let ty = &field.ty;
    let is_optional = typ::is_optional( ty );
    let of_type = container_kind::of_optional( ty ).0;
    let non_optional_ty : &syn::Type = if is_optional { typ::parameter_first( ty )? } else { ty };
    let field = FormerField { attrs, vis, ident, colon_token, ty, non_optional_ty, is_optional, of_type };
    Ok( field )
  }).collect();

  let former_fields : Vec< _ > = process_results( former_fields, | iter | iter.collect() )?;

  let
  (
    fields_none,
    fields_optional,
    fields_form,
    fields_names,
    fields_setter,
    fields_former_assign,
    fields_former_add,
  )
  :
  ( Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ > )
  = former_fields.iter().map( | field |
  {(
    field.none_map(),
    field.optional_map(),
    field.form_map(),
    field.name_map(),
    field.setter_map( &stru ),
    field.former_assign_end_map
    (
      &stru,
      &former,
      &former_generics_impl,
      &former_generics_ty,
      &former_generics_where,
    ),
    field.former_add_end_map
    (
      &stru,
      &former,
      &former_generics_ty,
      &struct_generics_impl,
      &struct_generics_ty,
      &struct_generics_where,
    ),
  )}).multiunzip();

  let fields_setter : Vec< _ > = process_results( fields_setter, | iter | iter.collect() )?;
  let fields_form : Vec< _ > = process_results( fields_form, | iter | iter.collect() )?;
  let fields_former_assign : Vec< _ > = process_results( fields_former_assign, | iter | iter.collect() )?;
  let fields_former_add : Vec< _ > = process_results( fields_former_add, | iter | iter.collect() )?;

  let ( _doc_former_mod, doc_former_struct ) = doc_generate( stru );
  let ( perform, perform_output, perform_generics ) = performer
  (
    &struct_attrs
  )?;

  let storage_fields = storage_fields
  (
    &struct_attrs
  )?;

  let result = qt!
  {

    // = formed

    #[ automatically_derived ]
    impl < #struct_generics_impl > #stru < #struct_generics_ty >
    where
      #struct_generics_where
    {

      ///
      /// Make former, variation of builder pattern to form structure defining values of fields step by step.
      ///

      #[ inline( always ) ]
      pub fn former() -> #former < #struct_generics_ty #former_definition< #former_definition_args > >
      {
        #former :: < #struct_generics_ty #former_definition< #former_definition_args > > :: new_coercing( former::ReturnPreformed )
      }

    }

    // = entity to former

    impl< #struct_generics_impl Definition > former::EntityToFormer< Definition >
    for #stru < #struct_generics_ty >
    where
      Definition : former::FormerDefinition< Storage = #former_storage < #struct_generics_ty > >,
      #struct_generics_where
    {
      type Former = #former < #struct_generics_ty Definition > ;
    }

    impl< #struct_generics_impl > former::EntityToStorage
    for #stru < #struct_generics_ty >
    where
      #struct_generics_where
    {
      type Storage = #former_storage < #struct_generics_ty >;
    }

    impl< #struct_generics_impl __Context, __Formed, __End > former::EntityToDefinition< __Context, __Formed, __End >
    for #stru < #struct_generics_ty >
    where
      __End : former::FormingEnd< #former_definition_types < #struct_generics_ty __Context, __Formed > >,
      #struct_generics_where
    {
      type Definition = #former_definition < #struct_generics_ty __Context, __Formed, __End >;
    }

    // = definition types

    #[ derive( Debug ) ]
    pub struct #former_definition_types < #former_definition_type_generics_with_defaults >
    where
      #former_definition_type_generics_where
    {
      // _phantom : core::marker::PhantomData< ( __Context, __Formed ) >,
      _phantom : #former_definition_type_phantom,
    }

    impl < #former_definition_type_generics_impl > ::core::default::Default
    for #former_definition_types < #former_definition_type_generics_ty >
    where
      #former_definition_type_generics_where
    {
      fn default() -> Self
      {
        Self
        {
          _phantom : core::marker::PhantomData,
        }
      }
    }

    impl < #former_definition_type_generics_impl > former::FormerDefinitionTypes
    for #former_definition_types < #former_definition_type_generics_ty >
    where
      #former_definition_type_generics_where
    {
      type Storage = #former_storage < #struct_generics_ty >;
      type Formed = __Formed;
      type Context = __Context;
    }

    // = definition

    #[ derive( Debug ) ]
    pub struct #former_definition < #former_definition_generics_with_defaults >
    where
      #former_definition_generics_where
    {
      // _phantom : core::marker::PhantomData< ( __Context, __Formed, __End ) >,
      _phantom : #former_definition_phantom,
    }

    impl < #former_definition_generics_impl > ::core::default::Default
    for #former_definition < #former_definition_generics_ty >
    where
      #former_definition_generics_where
    {
      fn default() -> Self
      {
        Self
        {
          _phantom : core::marker::PhantomData,
        }
      }
    }

    impl < #former_definition_generics_impl > former::FormerDefinition
    for #former_definition < #former_definition_generics_ty >
    where
      __End : former::FormingEnd< #former_definition_types < #former_definition_type_generics_ty > >,
      #former_definition_generics_where
    {
      type Types = #former_definition_types < #former_definition_type_generics_ty >;
      type End = __End;
      type Storage = #former_storage < #struct_generics_ty >;
      type Formed = __Formed;
      type Context = __Context;
    }

    // = storage

    #[ doc = "Container of a corresponding former." ]
    #[ allow( explicit_outlives_requirements ) ]
    // pub struct #former_storage < #struct_generics_ty >
    pub struct #former_storage < #struct_generics_with_defaults >
    where
      #struct_generics_where
    {
      #(
        /// A field
        #fields_optional,
      )*
    }

    impl < #struct_generics_impl > ::core::default::Default
    for #former_storage < #struct_generics_ty >
    where
      #struct_generics_where
    {

      #[ inline( always ) ]
      fn default() -> Self
      {
        Self
        {
          #( #fields_none, )*
        }
      }

    }

    impl < #struct_generics_impl > former::Storage
    for #former_storage < #struct_generics_ty >
    where
      #struct_generics_where
    {
      type Formed = #stru < #struct_generics_ty >;
    }

    impl < #struct_generics_impl > former::StoragePreform
    for #former_storage < #struct_generics_ty >
    where
      #struct_generics_where
    {
      type Preformed = #stru < #struct_generics_ty >;

      fn preform( mut self ) -> Self::Preformed
      {
        #( #fields_form )*
        // Rust does not support that, yet
        // let result = < Definition::Types as former::FormerDefinitionTypes >::Formed
        let result = #stru :: < #struct_generics_ty >
        {
          #( #fields_names, )*
        };
        return result;
      }

    }

    // = former

    #[ doc = #doc_former_struct ]
    pub struct #former < #former_generics_with_defaults >
    where
      #former_generics_where
    {
      storage : Definition::Storage,
      context : core::option::Option< Definition::Context >,
      on_end : core::option::Option< Definition::End >,
      // zzz : should on_end be optional?
    }

    #[ automatically_derived ]
    impl < #former_generics_impl > #former < #former_generics_ty >
    where
      #former_generics_where
    {

      ///
      /// Construct new instance of former with default parameters.
      ///
      // zzz : improve description
      #[ inline( always ) ]
      pub fn new( on_end : Definition::End ) -> Self
      {
        Self::begin_coercing( None, None, on_end )
      }

      ///
      /// Construct new instance of former with default parameters.
      ///
      // zzz : improve description
      #[ inline( always ) ]
      pub fn new_coercing< IntoEnd >( end : IntoEnd ) -> Self
      where
        IntoEnd : Into< Definition::End >,
      {
        Self::begin_coercing
        (
          None,
          None,
          end,
        )
      }

      ///
      /// Begin the process of forming. Expects context of forming to return it after forming.
      ///
      // zzz : improve description
      #[ inline( always ) ]
      pub fn begin
      (
        mut storage : core::option::Option< Definition::Storage >,
        context : core::option::Option< Definition::Context >,
        on_end : < Definition as former::FormerDefinition >::End,
      )
      -> Self
      {
        if storage.is_none()
        {
          storage = Some( ::core::default::Default::default() );
        }
        Self
        {
          storage : storage.unwrap(),
          context : context,
          on_end : ::core::option::Option::Some( on_end ),
        }
      }

      ///
      /// Begin the process of forming. Expects context of forming to return it after forming.
      ///
      // zzz : improve description
      #[ inline( always ) ]
      pub fn begin_coercing< IntoEnd >
      (
        mut storage : core::option::Option< Definition::Storage >,
        context : core::option::Option< Definition::Context >,
        on_end : IntoEnd,
      ) -> Self
      where
        IntoEnd : ::core::convert::Into< < Definition as former::FormerDefinition >::End >,
      {
        if storage.is_none()
        {
          storage = Some( ::core::default::Default::default() );
        }
        Self
        {
          storage : storage.unwrap(),
          context : context,
          on_end : ::core::option::Option::Some( ::core::convert::Into::into( on_end ) ),
        }
      }

      ///
      /// End the process of forming returning original context of forming.
      ///
      #[ inline( always ) ]
      pub fn form( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      {
        self.end()
      }

      ///
      /// End the process of forming returning original context of forming.
      ///
      #[ inline( always ) ]
      pub fn end( mut self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      {
        let on_end = self.on_end.take().unwrap();
        let context = self.context.take();
        former::FormingEnd::< Definition::Types >::call( &on_end, self.storage, context )
      }

      #(
        #fields_setter
      )*

    }

    // = former :: preform

    impl< #former_generics_impl > #former< #former_generics_ty >
    where
      Definition : former::FormerDefinition< Storage = #former_storage < #struct_generics_ty >, Formed = #stru < #struct_generics_ty > >,
      Definition::Types : former::FormerDefinitionTypes< Storage = #former_storage < #struct_generics_ty >, Formed = #stru < #struct_generics_ty > >,
      #former_generics_where
    {

      pub fn preform( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      {
        former::StoragePreform::preform( self.storage )
      }

    }

    // = former :: perform

    #[ automatically_derived ]
    impl < #former_perform_generics_impl > #former < #former_perform_generics_ty >
    where
      #former_perform_generics_where
    {

      ///
      /// Finish setting options and call perform on formed entity.
      ///
      /// If `perform` defined then associated method is called and its result returned instead of entity.
      /// For example `perform()` of structure with : `#[ perform( fn after1() -> &str > )` returns `&str`.
      ///
      #[ inline( always ) ]
      pub fn perform #perform_generics ( self ) -> #perform_output
      {
        let result = self.form();
        #perform
      }

    }

    // = former begin

    impl< #struct_generics_impl Definition > former::FormerBegin< Definition >
    // for ChildFormer< Definition >
    for #former
    <
      #struct_generics_ty
      Definition,
    >
    where
      Definition : former::FormerDefinition< Storage = #former_storage < #struct_generics_ty > >,
      #struct_generics_where
    {

      #[ inline( always ) ]
      fn former_begin
      (
        storage : core::option::Option< Definition::Storage >,
        context : core::option::Option< Definition::Context >,
        on_end : Definition::End,
      )
      -> Self
      {
        debug_assert!( storage.is_none() );
        Self::begin( None, context, on_end )
      }

    }

    // = subformer

    // zzz : improve description
    /// Use as subformer of a field during process of forming of super structure.
    pub type #as_subformer < #struct_generics_ty __Superformer, __End > = #former
    <
      #struct_generics_ty
      #former_definition
      <
        #struct_generics_ty
        __Superformer,
        __Superformer,
        __End,
        // impl former::FormingEnd< CommandFormerDefinitionTypes< K, __Superformer, __Superformer > >,
      >,
    >;

    // = as subformer end

    // zzz : imporove documentation
    #[ doc = #as_subformer_end_doc ]
    pub trait #as_subformer_end < #struct_generics_impl SuperFormer >
    where
      #struct_generics_where
      Self : former::FormingEnd
      <
        #former_definition_types < #struct_generics_ty SuperFormer, SuperFormer >,
      >,
    {
    }

    impl< #struct_generics_impl SuperFormer, __T > #as_subformer_end < #struct_generics_ty SuperFormer >
    for __T
    where
      #struct_generics_where
      Self : former::FormingEnd
      <
        #former_definition_types < #struct_generics_ty SuperFormer, SuperFormer >,
      >,
    {
    }

    // = container assign callbacks

    #(
      #fields_former_assign
    )*

    // = container add callbacks

    #(
      #fields_former_add
    )*

  };

  if has_debug
  {
    diag::debug_report_print( "derive : Former", original_input, &result );
  }

  // zzz : implement hints, rewrite
  if example_of_custom_setter
  {
    let _example =
r#"
impl< Context, End > UserProfileFormer< Context, End >
where
  End : former::FormingEnd< UserProfile, Context >,
{
  pub fn age< Src >( mut self, src : Src ) -> Self
  where
    Src : Into< i32 >,
  {
    debug_assert!( self.age.is_none() );
    self.storage.age = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
    self
  }
}
"#;
  }

  Ok( result )
}
