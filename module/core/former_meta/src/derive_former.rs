
use super::*;
use iter_tools::{ Itertools, process_results };
use macro_tools::{ attr, diag, generic_params, generic_args, typ, derive, Result };
use proc_macro2::TokenStream;

// qqq : implement interfaces for other collections


mod field;
use field::*;
mod field_attrs;
use field_attrs::*;
mod struct_attrs;
use struct_attrs::*;

/// Generates the code for implementing the `FormerMutator` trait for a specified former definition type.
///
/// This function generate code that implements the `FormerMutator` trait based on the given
/// former definition types and their associated generics. The `FormerMutator` trait provides the
/// functionality to mutate the storage and context of an entity just before its formation process
/// completes. This is particularly useful for performing final adjustments or validations on the data
/// before the entity is fully constructed.
///
/// # Example
///
/// Below is an example of how the generated code might look:
///
/// ```rust, ignore
/// impl< Context, Formed > former::FormerMutator
/// for Struct1FormerDefinitionTypes< Context, Formed >
/// {
///   /// Mutates the context and storage of the entity just before the formation process completes.
///   #[ inline ]
///   fn form_mutation( storage : &mut Self::Storage, _context : &mut ::core::option::Option< Self::Context > )
///   {
///     storage.a.get_or_insert_with( Default::default );
///     storage.b.get_or_insert_with( Default::default );
///     storage.c = Some( format!( "{:?} - {}", storage.a.unwrap(), storage.b.as_ref().unwrap() ) );
///   }
/// }
/// ```
///

pub fn mutator
(
  stru : &syn::Ident,
  original_input : &proc_macro::TokenStream,
  mutator : &AttributeMutator,
  former_definition_types : &syn::Ident,
  former_definition_types_generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  former_definition_types_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  former_definition_types_generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
)
-> Result< TokenStream >
{
  let former_mutator_code = if mutator.custom
  {
    qt!{}
  }
  else
  {
    qt!
    {
      impl< #former_definition_types_generics_impl > former::FormerMutator
      for #former_definition_types < #former_definition_types_generics_ty >
      where
        #former_definition_types_generics_where
      {
      }
    }
  };

  if mutator.hint
  {
    let hint = format!
    (
      r#"
= Example of custom mutator

impl< {} > former::FormerMutator
for {former_definition_types} < {} >
where
  {}
{{
  /// Mutates the context and storage of the entity just before the formation process completes.
  #[ inline ]
  fn form_mutation( storage : &mut Self::Storage, context : &mut Option< Self::Context > )
  {{
  }}
}}
      "#,
      format!( "{}", qt!{ #former_definition_types_generics_impl } ),
      format!( "{}", qt!{ #former_definition_types_generics_ty } ),
      format!( "{}", qt!{ #former_definition_types_generics_where } ),
    );
    // println!( "{hint}" );
    let about = format!
    (
r#"derive : Former
structure : {stru}"#,
    );
    diag::report_print( about, original_input, hint );
  };

  Ok( former_mutator_code )
}

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

  let doc_former_struct = format!
  (
r#"
Structure to form [{}]. Represents a forming entity designed to construct objects through a builder pattern.

This structure holds temporary storage and context during the formation process and
utilizes a defined end strategy to finalize the object creation.
"#,
    stru
  );

  ( doc_former_mod, doc_former_struct )
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

  /* names */

  let vis = &ast.vis;
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

  let as_subformer_end_doc = format!
  (
    r#"
Represents an end condition for former of [`${stru}`], tying the lifecycle of forming processes to a broader context.

This trait is intended for use with subformer alias, ensuring that end conditions are met according to the
specific needs of the broader forming context. It mandates the implementation of `former::FormingEnd`.
    "#
  );

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
  let former_definition_types_generics = generic_params::merge( &generics, &extra.into() );
  let ( former_definition_types_generics_with_defaults, former_definition_types_generics_impl, former_definition_types_generics_ty, former_definition_types_generics_where )
  = generic_params::decompose( &former_definition_types_generics );

  let former_definition_types_phantom = macro_tools::phantom::tuple( &former_definition_types_generics_impl );

  /* parameters for definition */

  let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < __Context = (), __Formed = #stru < #struct_generics_ty >, __End = former::ReturnPreformed >
  };
  let generics_of_definition = generic_params::merge( &generics, &extra.into() );
  let ( former_definition_generics_with_defaults, former_definition_generics_impl, former_definition_generics_ty, former_definition_generics_where )
  = generic_params::decompose( &generics_of_definition );

  let former_definition_phantom = macro_tools::phantom::tuple( &former_definition_generics_impl );

  /* struct attributes */

  let ( _doc_former_mod, doc_former_struct ) = doc_generate( stru );
  let ( perform, perform_output, perform_generics ) = struct_attrs.performer()?;

  /* fields */

  let fields = derive::named_fields( &ast )?;

  let formed_fields : Vec< Result< FormerField< '_ > > > = fields
  .into_iter()
  .map( | field |
  {
    FormerField::from_syn( field, true, true )
  })
  .collect();
  let formed_fields : Vec< _ > = process_results( formed_fields, | iter | iter.collect() )?;

  let storage_fields : Vec< Result< FormerField< '_ > > > = struct_attrs
  .storage_fields()
  .iter()
  .map( | field |
  {
    FormerField::from_syn( &field, true, false )
  })
  .collect();
  let storage_fields : Vec< _ > = process_results( storage_fields, | iter | iter.collect() )?;

  let
  (
    storage_field_none,
    storage_field_optional,
    storage_field_name,
    storage_field_preform,
    former_field_setter,
  )
  :
  ( Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ > )
  = formed_fields
  .iter()
  .chain( storage_fields.iter() )
  .map( | field |
  {(
    field.storage_fields_none(),
    field.storage_field_optional(),
    field.storage_field_name(),
    field.storage_field_preform(),
    field.former_field_setter
    (
      &stru,
      &original_input,
      &struct_generics_impl,
      &struct_generics_ty,
      &struct_generics_where,
      &former,
      &former_generics_impl,
      &former_generics_ty,
      &former_generics_where,
      &former_storage,
    ),
  )}).multiunzip();

  let results : Result< Vec< _ > > = former_field_setter.into_iter().collect();
  let ( former_field_setter, namespace_code ) : ( Vec< _ >, Vec< _ > ) = results?.into_iter().unzip();

  let storage_field_preform : Vec< _ > = process_results( storage_field_preform, | iter | iter.collect() )?;

  let former_mutator_code = mutator
  (
    &stru,
    &original_input,
    &struct_attrs.mutator,
    &former_definition_types,
    &former_definition_types_generics_impl,
    &former_definition_types_generics_ty,
    &former_definition_types_generics_where,
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
      /// Provides a mechanism to initiate the formation process with a default completion behavior.
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
      type Types = #former_definition_types < #struct_generics_ty __Context, __Formed >;
    }

    impl< #struct_generics_impl __Context, __Formed > former::EntityToDefinitionTypes< __Context, __Formed >
    for #stru < #struct_generics_ty >
    where
      #struct_generics_where
    {
      type Types = #former_definition_types < #struct_generics_ty __Context, __Formed >;
    }

    // = definition types

    /// Defines the generic parameters for formation behavior including context, form, and end conditions.
    #[ derive( Debug ) ]
    #vis struct #former_definition_types < #former_definition_types_generics_with_defaults >
    where
      #former_definition_types_generics_where
    {
      // _phantom : ::core::marker::PhantomData< ( __Context, __Formed ) >,
      _phantom : #former_definition_types_phantom,
    }

    impl < #former_definition_types_generics_impl > ::core::default::Default
    for #former_definition_types < #former_definition_types_generics_ty >
    where
      #former_definition_types_generics_where
    {
      fn default() -> Self
      {
        Self
        {
          _phantom : ::core::marker::PhantomData,
        }
      }
    }

    impl < #former_definition_types_generics_impl > former::FormerDefinitionTypes
    for #former_definition_types < #former_definition_types_generics_ty >
    where
      #former_definition_types_generics_where
    {
      type Storage = #former_storage < #struct_generics_ty >;
      type Formed = __Formed;
      type Context = __Context;
    }

    // = definition

    /// Holds the definition types used during the formation process.
    #[ derive( Debug ) ]
    #vis struct #former_definition < #former_definition_generics_with_defaults >
    where
      #former_definition_generics_where
    {
      // _phantom : ::core::marker::PhantomData< ( __Context, __Formed, __End ) >,
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
          _phantom : ::core::marker::PhantomData,
        }
      }
    }

    impl < #former_definition_generics_impl > former::FormerDefinition
    for #former_definition < #former_definition_generics_ty >
    where
      __End : former::FormingEnd< #former_definition_types < #former_definition_types_generics_ty > >,
      #former_definition_generics_where
    {
      type Types = #former_definition_types < #former_definition_types_generics_ty >;
      type End = __End;
      type Storage = #former_storage < #struct_generics_ty >;
      type Formed = __Formed;
      type Context = __Context;
    }

    // = former mutator

    #former_mutator_code

    // = storage

    #[ doc = "Stores potential values for fields during the formation process." ]
    #[ allow( explicit_outlives_requirements ) ]
    #vis struct #former_storage < #struct_generics_with_defaults >
    where
      #struct_generics_where
    {
      #(
        /// A field
        #storage_field_optional,
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
          #( #storage_field_none, )*
        }
      }

    }

    impl < #struct_generics_impl > former::Storage
    for #former_storage < #struct_generics_ty >
    where
      #struct_generics_where
    {
      type Preformed = #stru < #struct_generics_ty >;
    }

    impl < #struct_generics_impl > former::StoragePreform
    for #former_storage < #struct_generics_ty >
    where
      #struct_generics_where
    {
      // type Preformed = #stru < #struct_generics_ty >;

      fn preform( mut self ) -> Self::Preformed
      {
        #( #storage_field_preform )*
        // Rust does not support that, yet
        // let result = < Definition::Types as former::FormerDefinitionTypes >::Formed
        let result = #stru :: < #struct_generics_ty >
        {
          #( #storage_field_name )*
          // #( #storage_field_name, )*
        };
        return result;
      }

    }

    // = former

    #[ doc = #doc_former_struct ]
    #vis struct #former < #former_generics_with_defaults >
    where
      #former_generics_where
    {
      /// Temporary storage for all fields during the formation process. It contains
      ///   partial data that progressively builds up to the final object.
      pub storage : Definition::Storage,
      /// An optional context providing additional data or state necessary for custom
      ///   formation logic or to facilitate this former's role as a subformer within another former.
      pub context : ::core::option::Option< Definition::Context >,
      /// An optional closure or handler that is invoked to transform the accumulated
      ///   temporary storage into the final object structure once formation is complete.
      pub on_end : ::core::option::Option< Definition::End >,
    }

    #[ automatically_derived ]
    impl < #former_generics_impl > #former < #former_generics_ty >
    where
      #former_generics_where
    {

      ///
      /// Initializes a former with an end condition and default storage.
      ///
      #[ inline( always ) ]
      pub fn new( on_end : Definition::End ) -> Self
      {
        Self::begin_coercing( None, None, on_end )
      }

      ///
      /// Initializes a former with a coercible end condition.
      ///
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
      /// Begins the formation process with specified context and termination logic.
      ///
      #[ inline( always ) ]
      pub fn begin
      (
        mut storage : ::core::option::Option< Definition::Storage >,
        context : ::core::option::Option< Definition::Context >,
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
      /// Starts the formation process with coercible end condition and optional initial values.
      ///
      #[ inline( always ) ]
      pub fn begin_coercing< IntoEnd >
      (
        mut storage : ::core::option::Option< Definition::Storage >,
        context : ::core::option::Option< Definition::Context >,
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
      /// Wrapper for `end` to align with common builder pattern terminologies.
      ///
      #[ inline( always ) ]
      pub fn form( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      {
        self.end()
      }

      ///
      /// Completes the formation and returns the formed object.
      ///
      #[ inline( always ) ]
      pub fn end( mut self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      {
        let on_end = self.on_end.take().unwrap();
        let mut context = self.context.take();
        < Definition::Types as former::FormerMutator >::form_mutation( &mut self.storage, &mut context );
        former::FormingEnd::< Definition::Types >::call( &on_end, self.storage, context )
      }

      #(
        #former_field_setter
      )*

    }

    // = former :: preform

    impl< #former_generics_impl > #former< #former_generics_ty >
    where
      Definition : former::FormerDefinition< Storage = #former_storage < #struct_generics_ty >, Formed = #stru < #struct_generics_ty > >,
      Definition::Types : former::FormerDefinitionTypes< Storage = #former_storage < #struct_generics_ty >, Formed = #stru < #struct_generics_ty > >,
      #former_generics_where
    {

      /// Executes the transformation from the former's storage state to the preformed object as specified by the definition.
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
        storage : ::core::option::Option< Definition::Storage >,
        context : ::core::option::Option< Definition::Context >,
        on_end : Definition::End,
      )
      -> Self
      {
        debug_assert!( storage.is_none() );
        Self::begin( None, context, on_end )
      }

    }

    // = subformer

    /// Provides a specialized former for structure using predefined settings for superformer and end conditions.
    ///
    /// This type alias configures former of the structure with a specific definition to streamline its usage in broader contexts,
    /// especially where structure needs to be integrated into larger structures with a clear termination condition.
    #vis type #as_subformer < #struct_generics_ty __Superformer, __End > = #former
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

    // = etc

    #(
      #namespace_code
    )*

  };

  if has_debug
  {
    let about = format!( "derive : Former\nstructure : {stru}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}
