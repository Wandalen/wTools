// File: module/core/former_meta/src/derive_former/former_struct.rs

#![ allow( clippy::wildcard_imports ) ]
use super::*; // Use items from parent module (derive_former.rs)
use iter_tools::Itertools;
use macro_tools::
{
  // Removed unused: attr, diag, typ, Spanned, ident
  generic_params, generic_args, derive, Result,
  proc_macro2::TokenStream, quote::{ format_ident, quote },
  IntoGenericArgs,
};
// Removed unused: Casing
#[ cfg( feature = "derive_former" ) ]
use convert_case::{ Casing };


/// Generate the Former ecosystem for a struct.
#[ allow( clippy::too_many_lines ) ]
pub(super) fn former_for_struct // Make it pub(super)
(
  ast : &syn::DeriveInput,
  _data_struct : &syn::DataStruct,
  original_input : &proc_macro::TokenStream,
  _has_debug : bool,
) -> Result< TokenStream >
{
  // Parse struct-level attributes like `storage_fields`, `mutator`, `perform`.
  let struct_attrs = ItemAttributes::from_attrs( ast.attrs.iter() )?;

  /* names: Generate identifiers for the Former components based on the struct name. */
  let vis = &ast.vis; // Visibility of the original struct.
  let item = &ast.ident; // Name of the original struct.
  let former = format_ident!( "{item}Former" ); // e.g., MyStructFormer
  let former_storage = format_ident!( "{item}FormerStorage" ); // e.g., MyStructFormerStorage
  let former_definition = format_ident!( "{item}FormerDefinition" ); // e.g., MyStructFormerDefinition
  let former_definition_types = format_ident!( "{item}FormerDefinitionTypes" ); // e.g., MyStructFormerDefinitionTypes
  let as_subformer = format_ident!( "{item}AsSubformer" ); // e.g., MyStructAsSubformer
  let as_subformer_end = format_ident!( "{item}AsSubformerEnd" ); // e.g., MyStructAsSubformerEnd

  // Generate documentation string for the AsSubformerEnd trait.
  let as_subformer_end_doc = format!
  (
    r"
Represents an end condition for former of [`${item}`], tying the lifecycle of forming processes to a broader context.

This trait is intended for use with subformer alias, ensuring that end conditions are met according to the
specific needs of the broader forming context. It mandates the implementation of `former::FormingEnd`.
    "
  );

  /* parameters for structure: Decompose the original struct's generics. */
  let generics = &ast.generics;
  let
  (
    struct_generics_with_defaults, // Generics with defaults (e.g., `<T = i32>`). Used for struct definition.
    struct_generics_impl,          // Generics for `impl` block (e.g., `<T: Clone>`). Bounds, no defaults.
    struct_generics_ty,            // Generics for type usage (e.g., `<T>`). Names only.
    struct_generics_where          // Where clause predicates (e.g., `T: Send`).
  ) = generic_params::decompose( generics );

  /* parameters for definition: Merge struct generics with default definition parameters. */
  let extra : macro_tools::syn::AngleBracketedGenericArguments = parse_quote!
  {
    < (), #item < #struct_generics_ty >, former::ReturnPreformed > // Default Context, Formed, End
  };
  let former_definition_args = generic_args::merge( &generics.into_generic_args(), &extra ).args;

  /* parameters for former: Merge struct generics with the Definition generic parameter. */
  let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < Definition = #former_definition < #former_definition_args > >
    where
      Definition : former::FormerDefinition< Storage = #former_storage < #struct_generics_ty > >,
      Definition::Types : former::FormerDefinitionTypes< Storage = #former_storage < #struct_generics_ty > >,
  };
  let extra = generic_params::merge( generics, &extra.into() );
  let ( former_generics_with_defaults, former_generics_impl, former_generics_ty, former_generics_where )
  = generic_params::decompose( &extra );

  /* parameters for former perform: Similar to former parameters, but specifically for the perform method. */
  let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < Definition = #former_definition < #former_definition_args > >
    where
      Definition : former::FormerDefinition
      <
        Storage = #former_storage < #struct_generics_ty >,
        Formed = #item < #struct_generics_ty >,
      >,
      Definition::Types : former::FormerDefinitionTypes
      <
        Storage = #former_storage < #struct_generics_ty >,
        Formed = #item < #struct_generics_ty >,
      >,
  };
  let extra = generic_params::merge( generics, &extra.into() );
  let ( _former_perform_generics_with_defaults, former_perform_generics_impl, former_perform_generics_ty, former_perform_generics_where )
  = generic_params::decompose( &extra );

  /* parameters for definition types: Merge struct generics with Context and Formed parameters. */
  let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < __Context = (), __Formed = #item < #struct_generics_ty > >
  };
  let former_definition_types_generics = generic_params::merge( generics, &extra.into() );
  let ( former_definition_types_generics_with_defaults, former_definition_types_generics_impl, former_definition_types_generics_ty, former_definition_types_generics_where )
  = generic_params::decompose( &former_definition_types_generics );
  // Generate PhantomData tuple type based on the impl generics.
  let former_definition_types_phantom = macro_tools::phantom::tuple( &former_definition_types_generics_impl );

  /* parameters for definition: Merge struct generics with Context, Formed, and End parameters. */
  let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < __Context = (), __Formed = #item < #struct_generics_ty >, __End = former::ReturnPreformed >
  };
  let generics_of_definition = generic_params::merge( generics, &extra.into() );
  let ( former_definition_generics_with_defaults, former_definition_generics_impl, former_definition_generics_ty, former_definition_generics_where )
  = generic_params::decompose( &generics_of_definition );
  // Generate PhantomData tuple type based on the impl generics.
  let former_definition_phantom = macro_tools::phantom::tuple( &former_definition_generics_impl );

  /* struct attributes: Generate documentation and extract perform method details. */
  let ( _doc_former_mod, doc_former_struct ) = super::doc_generate( item ); // Use super::
  let ( perform, perform_output, perform_generics ) = struct_attrs.performer()?;

  /* fields: Process struct fields and storage_fields attribute. */
  let fields = derive::named_fields( &ast )?;
  // Create FormerField representation for actual struct fields.
  let formed_fields : Vec< _ > = fields
  .iter()
  .map( | field | FormerField::from_syn( field, true, true ) )
  .collect::< Result< _ > >()?;
  // Create FormerField representation for storage-only fields.
  let storage_fields : Vec< _ > = struct_attrs
  .storage_fields()
  .iter()
  .map( | field | FormerField::from_syn( field, true, false ) )
  .collect::< Result< _ > >()?;

  // Generate code snippets for each field (storage init, storage field def, preform logic, setters).
  let
  (
    storage_field_none, // Code for initializing storage field to None.
    storage_field_optional, // Code for the storage field definition (e.g., `pub field: Option<Type>`).
    storage_field_name, // Code for the field name (e.g., `field,`). Used in final struct construction.
    storage_field_preform, // Code for unwrapping/defaulting the field in `preform`.
    former_field_setter, // Code for the setter method(s) for the field.
  )
  :
  ( Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ > )
  = formed_fields // Combine actual fields and storage-only fields for processing.
  .iter()
  .chain( storage_fields.iter() )
  .map( | field | {(
    field.storage_fields_none(),
    field.storage_field_optional(),
    field.storage_field_name(), // Only generated if field.for_formed is true.
    field.storage_field_preform(), // Only generated if field.for_formed is true.
    field.former_field_setter
    (
      item,
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

  // Collect results, separating setters and namespace code (like End structs).
  let results : Result< Vec< _ > > = former_field_setter.into_iter().collect();
  let ( former_field_setter, namespace_code ) : ( Vec< _ >, Vec< _ > ) = results?.into_iter().unzip();
  // Collect preform logic results.
  let storage_field_preform : Vec< _ > = storage_field_preform.into_iter().collect::< Result< _ > >()?;
  // Generate mutator implementation code.
  let former_mutator_code = super::mutator // Use super::
  (
    item,
    &original_input,
    &struct_attrs.mutator,
    &former_definition_types,
    &former_definition_types_generics_impl,
    &former_definition_types_generics_ty,
    &former_definition_types_generics_where
  )?;

  // Assemble the final generated code using quote!
  let result = quote!
  {

    // = formed: Implement the `::former()` static method on the original struct.
    #[ automatically_derived ]
    impl < #struct_generics_impl > #item < #struct_generics_ty >
    where
      #struct_generics_where
    {
      /// Provides a mechanism to initiate the formation process with a default completion behavior.
      #[ inline( always ) ]
      pub fn former() -> #former < #struct_generics_ty #former_definition< #former_definition_args > >
      {
        #former :: < #struct_generics_ty #former_definition< #former_definition_args > > :: new_coercing( former::ReturnPreformed )
      }
    }

    // = entity to former: Implement former traits linking the struct to its generated components.
    impl< #struct_generics_impl Definition > former::EntityToFormer< Definition >
    for #item < #struct_generics_ty >
    where
      Definition : former::FormerDefinition< Storage = #former_storage < #struct_generics_ty > >,
      #struct_generics_where
    {
      type Former = #former < #struct_generics_ty Definition > ;
    }

    impl< #struct_generics_impl > former::EntityToStorage
    for #item < #struct_generics_ty >
    where
      #struct_generics_where
    {
      type Storage = #former_storage < #struct_generics_ty >;
    }

    impl< #struct_generics_impl __Context, __Formed, __End > former::EntityToDefinition< __Context, __Formed, __End >
    for #item < #struct_generics_ty >
    where
      __End : former::FormingEnd< #former_definition_types < #struct_generics_ty __Context, __Formed > >,
      #struct_generics_where
    {
      type Definition = #former_definition < #struct_generics_ty __Context, __Formed, __End >;
      type Types = #former_definition_types < #struct_generics_ty __Context, __Formed >;
    }

    impl< #struct_generics_impl __Context, __Formed > former::EntityToDefinitionTypes< __Context, __Formed >
    for #item < #struct_generics_ty >
    where
      #struct_generics_where
    {
      type Types = #former_definition_types < #struct_generics_ty __Context, __Formed >;
    }

    // = definition types: Define the FormerDefinitionTypes struct.
    /// Defines the generic parameters for formation behavior including context, form, and end conditions.
    #[ derive( Debug ) ]
    #vis struct #former_definition_types < #former_definition_types_generics_with_defaults >
    where
      #former_definition_types_generics_where
    {
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

    // = definition: Define the FormerDefinition struct.
    /// Holds the definition types used during the formation process.
    #[ derive( Debug ) ]
    #vis struct #former_definition < #former_definition_generics_with_defaults >
    where
      #former_definition_generics_where
    {
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

    // = former mutator: Implement the FormerMutator trait.
    #former_mutator_code

    // = storage: Define the FormerStorage struct.
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
      type Preformed = #item < #struct_generics_ty >;
    }

    impl < #struct_generics_impl > former::StoragePreform
    for #former_storage < #struct_generics_ty >
    where
      #struct_generics_where
    {
      fn preform( mut self ) -> Self::Preformed
      {
        #( #storage_field_preform )*
        let result = #item :: < #struct_generics_ty >
        {
          #( #storage_field_name )*
        };
        return result;
      }
    }

    // = former: Define the Former struct itself.
    #[ doc = #doc_former_struct ]
    #vis struct #former < #former_generics_with_defaults >
    where
      #former_generics_where
    {
      /// Temporary storage for all fields during the formation process.
      pub storage : Definition::Storage,
      /// Optional context.
      pub context : ::core::option::Option< Definition::Context >,
      /// Optional handler for the end of formation.
      pub on_end : ::core::option::Option< Definition::End >,
    }

    #[ automatically_derived ]
    impl < #former_generics_impl > #former < #former_generics_ty >
    where
      #former_generics_where
    {
      /// Initializes a former with an end condition and default storage.
      #[ inline( always ) ]
      pub fn new
      (
        on_end : Definition::End
      ) -> Self
      {
        Self::begin_coercing( ::core::option::Option::None, ::core::option::Option::None, on_end )
      }

      /// Initializes a former with a coercible end condition.
      #[ inline( always ) ]
      pub fn new_coercing< IntoEnd >
      (
        end : IntoEnd
      ) -> Self
      where
        IntoEnd : ::core::convert::Into< Definition::End >,
      {
        Self::begin_coercing
        (
          ::core::option::Option::None,
          ::core::option::Option::None,
          end,
        )
      }

      /// Begins the formation process with specified context and termination logic.
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
          storage = ::core::option::Option::Some( ::core::default::Default::default() );
        }
        Self
        {
          storage : storage.unwrap(),
          context : context,
          on_end : ::core::option::Option::Some( on_end ),
        }
      }

      /// Starts the formation process with coercible end condition and optional initial values.
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
          storage = ::core::option::Option::Some( ::core::default::Default::default() );
        }
        Self
        {
          storage : storage.unwrap(),
          context : context,
          on_end : ::core::option::Option::Some( ::core::convert::Into::into( on_end ) ),
        }
      }

      /// Wrapper for `end` to align with common builder pattern terminologies.
      #[ inline( always ) ]
      pub fn form( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      {
        self.end()
      }

      /// Completes the formation and returns the formed object.
      #[ inline( always ) ]
      pub fn end( mut self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      {
        let on_end = self.on_end.take().unwrap();
        let mut context = self.context.take();
        < Definition::Types as former::FormerMutator >::form_mutation( &mut self.storage, &mut context );
        former::FormingEnd::< Definition::Types >::call( &on_end, self.storage, context )
      }

      // Insert generated setter methods for each field.
      #(
        #former_field_setter
      )*

    }

    // = former :: preform: Implement `preform` for direct storage transformation.
    impl< #former_generics_impl > #former< #former_generics_ty >
    where
      Definition : former::FormerDefinition< Storage = #former_storage < #struct_generics_ty >, Formed = #item < #struct_generics_ty > >,
      Definition::Types : former::FormerDefinitionTypes< Storage = #former_storage < #struct_generics_ty >, Formed = #item < #struct_generics_ty > >,
      #former_generics_where
    {
      /// Executes the transformation from the former's storage state to the preformed object.
      pub fn preform( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      {
        former::StoragePreform::preform( self.storage )
      }
    }

    // = former :: perform: Implement `perform` if specified by attributes.
    #[ automatically_derived ]
    impl < #former_perform_generics_impl > #former < #former_perform_generics_ty >
    where
      #former_perform_generics_where
    {
      /// Finish setting options and call perform on formed entity.
      #[ inline( always ) ]
      pub fn perform #perform_generics ( self ) -> #perform_output
      {
        let result = self.form();
        #perform
      }
    }

    // = former begin: Implement `FormerBegin` trait.
    impl< #struct_generics_impl Definition > former::FormerBegin< Definition >
    for #former < #struct_generics_ty Definition, >
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
        Self::begin( ::core::option::Option::None, context, on_end )
      }
    }

    // = subformer: Define the `AsSubformer` type alias.
    /// Provides a specialized former for structure using predefined settings for superformer and end conditions.
    #vis type #as_subformer < #struct_generics_ty __Superformer, __End > = #former
    <
      #struct_generics_ty
      #former_definition
      <
        #struct_generics_ty
        __Superformer,
        __Superformer,
        __End,
      >,
    >;


    // = as subformer end: Define the `AsSubformerEnd` trait.
    #[ doc = #as_subformer_end_doc ]
    // CORRECTED: Use _ty for trait generics, _impl for where clause
    pub trait #as_subformer_end < #struct_generics_ty SuperFormer >
    where
      #struct_generics_impl // Use _impl here for constraints
      #struct_generics_where
      Self : former::FormingEnd
      <
        #former_definition_types < #struct_generics_ty SuperFormer, SuperFormer >,
      >,
    {
    }

    // CORRECTED: Use _impl for impl generics, _ty for trait path generics
    impl< #struct_generics_impl SuperFormer, __T > #as_subformer_end < #struct_generics_ty SuperFormer >
    for __T
    where
      #struct_generics_impl // Use _impl here for constraints
      #struct_generics_where
      Self : former::FormingEnd
      <
        #former_definition_types < #struct_generics_ty SuperFormer, SuperFormer >,
      >,
    {
    }

    // = etc: Insert any namespace code generated by field setters (e.g., End structs for subformers).
    #( #namespace_code )*

  };
  Ok( result )
}