// File: module/core/former_meta/src/derive_former.rs
#[ allow( clippy::wildcard_imports ) ]
use super::*;
use iter_tools::Itertools;
use macro_tools::
{
  attr, diag, generic_params, generic_args, typ, derive, Result,
  proc_macro2::TokenStream, quote::{ format_ident, quote }, syn::spanned::Spanned,
  ident,
};
#[ cfg( feature = "derive_former" ) ]
use convert_case::{ Case, Casing };


mod field_attrs;
#[ allow( clippy::wildcard_imports ) ]
use field_attrs::*;
mod field;
#[ allow( clippy::wildcard_imports ) ]
use field::*;
mod struct_attrs;
#[ allow( clippy::wildcard_imports ) ]
use struct_attrs::*;

/// Generates the code for implementing the `FormerMutator` trait for a specified former definition type.
/// If the `custom` attribute is not specified, a default empty implementation is generated.
/// If the `debug` attribute is specified, it prints an example of a custom mutator implementation.
#[ allow( clippy::format_in_format_args, clippy::unnecessary_wraps ) ]
pub fn mutator
(
  item : &syn::Ident,
  original_input : &proc_macro::TokenStream,
  mutator : &AttributeMutator,
  former_definition_types : &syn::Ident,
  former_definition_types_generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  former_definition_types_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  former_definition_types_generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
)
-> Result< TokenStream >
{
  let former_mutator_code = if mutator.custom.value( false )
  {
    // If custom mutator is requested via #[ mutator( custom ) ], generate nothing, assuming user provides the impl.
    quote!{}
  }
  else
  {
    // Otherwise, generate a default empty impl.
    quote!
    {
      impl< #former_definition_types_generics_impl > former::FormerMutator
      for #former_definition_types < #former_definition_types_generics_ty >
      where
        #former_definition_types_generics_where
      {
      }
    }
  };

  // If debug is enabled for the mutator attribute, print a helpful example.
  if mutator.debug.value( false )
  {
    let debug = format!
    (
      r"
= Example of custom mutator

impl< {} > former::FormerMutator
for {former_definition_types} < {} >
where
  {}
{{
  /// Mutates the context and storage of the entity just before the formation process completes.
  #[ inline ]
  fn form_mutation
  (
    storage : &mut Self::Storage,
    context : &mut Option< Self::Context >,
  )
  {{
    // Example: Set a default value if field 'a' wasn't provided
    // storage.a.get_or_insert_with( Default::default );
  }}
}}
      ",
      format!( "{}", quote!{ #former_definition_types_generics_impl } ),
      format!( "{}", quote!{ #former_definition_types_generics_ty } ),
      format!( "{}", quote!{ #former_definition_types_generics_where } ),
    );
    let about = format!
    (
r"derive : Former
item : {item}",
    );
    diag::report_print( about, original_input, debug );
  };

  Ok( former_mutator_code )
}


/// Generate documentation strings for the former struct and its module.
fn doc_generate( item : &syn::Ident ) -> ( String, String )
{

  let doc_former_mod = format!
  (
r" Implementation of former for [{item}].
"
  );

  let doc_former_struct = format!
  (
r"
Structure to form [{item}]. Represents a forming entity designed to construct objects through a builder pattern.

This structure holds temporary storage and context during the formation process and
utilizes a defined end strategy to finalize the object creation.
"
  );

  ( doc_former_mod, doc_former_struct )
}


/// Generate the whole Former ecosystem for either a struct or an enum.
/// This is the main entry point for the `#[derive(Former)]` macro.
#[ allow( clippy::too_many_lines ) ]
pub fn former( input : proc_macro::TokenStream ) -> Result< TokenStream >
{
  let original_input = input.clone();
  let ast = syn::parse::< syn::DeriveInput >( input )?;
  let has_debug = attr::has_debug( ast.attrs.iter() )?;

  // Dispatch based on whether the input is a struct, enum, or union.
  let result = match ast.data
  {
      syn::Data::Struct( ref data_struct ) =>
      {
          former_for_struct( &ast, data_struct, &original_input, has_debug )
      },
      syn::Data::Enum( ref data_enum ) =>
      {
          former_for_enum( &ast, data_enum, &original_input, has_debug )
      },
      syn::Data::Union( _ ) =>
      {
          // Unions are not supported.
          Err( syn::Error::new( ast.span(), "Former derive does not support unions" ) )
      }
  }?;

  // If the top-level `#[debug]` attribute was found, print the final generated code.
  if has_debug
  {
    let about = format!( "derive : Former\nstructure : {}", ast.ident );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}


/// Generate the Former ecosystem for a struct.
#[ allow( clippy::too_many_lines ) ]
fn former_for_struct
(
  ast : &syn::DeriveInput,
  _data_struct : &syn::DataStruct,
  original_input : &proc_macro::TokenStream,
  _has_debug : bool,
) -> Result< TokenStream >
{
  use macro_tools::IntoGenericArgs;

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
  let ( _doc_former_mod, doc_former_struct ) = doc_generate( item );
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
  let former_mutator_code = mutator( item, &original_input, &struct_attrs.mutator, &former_definition_types, &former_definition_types_generics_impl, &former_definition_types_generics_ty, &former_definition_types_generics_where )?;

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
    // #vis type #as_subformer < #struct_generics_impl __Superformer, __End > = #former
    // xxx
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

    // = etc: Insert any namespace code generated by field setters (e.g., End structs for subformers).
    #( #namespace_code )*

  };
  Ok( result )
}


/// Generate the Former ecosystem for an enum.
#[ allow( clippy::too_many_lines ) ]
fn former_for_enum
(
  ast : &syn::DeriveInput,
  data_enum : &syn::DataEnum,
  _original_input : &proc_macro::TokenStream,
  _has_debug : bool, // qqq : xxx : make sure debug works
) -> Result< TokenStream >
{
  let enum_name = &ast.ident;
  let vis = &ast.vis;
  let generics = &ast.generics;
  let ( _enum_generics_with_defaults, enum_generics_impl, enum_generics_ty, enum_generics_where )
  = generic_params::decompose( generics );

  let mut methods = Vec::new();
  let mut end_impls = Vec::new();

  for variant in &data_enum.variants
  {
    let variant_ident = &variant.ident;

    // Only handle tuple variants with exactly one field.
    let inner_type = match &variant.fields
    {
        syn::Fields::Unnamed( fields ) if fields.unnamed.len() == 1 =>
        {
            &fields.unnamed.first().unwrap().ty
        },
        _ =>
        {
          return Err
          (
            syn::Error::new_spanned
            (
              &variant.fields,
              "Former derive on enums currently only supports tuple variants with exactly one field, like `VariantName(DataType)`"
            )
          );
        }
    };

    // Generate names for methods and types.
    let variant_name_str = variant_ident.to_string();
    let method_name_snake_str = variant_name_str.to_case( Case::Snake );
    let method_name_ident_temp = format_ident!( "{}", method_name_snake_str, span = variant_ident.span() );
    // Ensure raw identifier `r#` is used if the snake_case name is a keyword.
    let method_name = ident::ident_maybe_raw( &method_name_ident_temp );

    // Generate name for the specialized End struct for this variant.
    let end_struct_name = format_ident!( "{}{}End", enum_name, variant_ident );
    // Extract the base name of the inner type (assuming it's a path).
    let inner_type_name = match inner_type
    {
        syn::Type::Path( type_path ) => type_path.path.segments.last().map( | s | s.ident.clone() )
            .ok_or_else( || syn::Error::new_spanned( inner_type, "Cannot derive name from type path") )?,
        _ => return Err( syn::Error::new_spanned( inner_type, "Inner variant type must be a path type (like MyStruct or MyStruct<T>) to derive Former" ) ),
    };

    // Generate names for the inner type's Former components.
    let inner_former_name = format_ident!( "{}Former", inner_type_name );
    let inner_storage_name = format_ident!( "{}FormerStorage", inner_type_name );
    let inner_def_name = format_ident!( "{}FormerDefinition", inner_type_name );
    let inner_def_types_name = format_ident!( "{}FormerDefinitionTypes", inner_type_name );

    // Placeholders for handling generics of the inner type (TODO).
    let _inner_generics_impl = quote! {};
    let inner_generics_ty = quote! {};
    let inner_generics_where = quote! {};

    // Generate the definition for the specialized End struct.
    let end_struct_def = quote!
    {
      #[ derive( Default, Debug ) ]
      #vis struct #end_struct_name;
    };

    // Generate the `impl FormingEnd` block for the specialized End struct.
    let end_impl = quote!
    {
      #[ automatically_derived ]
      impl< #enum_generics_impl > former::FormingEnd
      <
          // Define the types for the inner former: Context=(), Formed=TheEnum<...>
          #inner_def_types_name< #inner_generics_ty (), #enum_name< #enum_generics_ty > >
      >
      for #end_struct_name
      where // Include where clauses from the enum and potentially the inner type.
        #enum_generics_where #inner_generics_where
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

    // Generate the static method on the enum (e.g., `pub fn run() -> RunFormer<...>` ).
    let static_method = quote!
    {
      #[ inline( always ) ]
      #vis fn #method_name() // Use the generated (potentially raw) method name.
      -> #inner_former_name // Return type is the Former for the inner data type.
         <
           // Configure the inner former's definition:
           #inner_def_name
           <
               (),                             // Context is ().
               #enum_name< #enum_generics_ty >, // The final type to be Formed is the enum itself.
               #end_struct_name                // Use the specialized End struct.
           >
         >
      {
          // Start the inner former using its `begin` associated function.
          #inner_former_name::begin( None, None, #end_struct_name::default() )
      }
    };

    methods.push( static_method );
    end_impls.push( quote!{ #end_struct_def #end_impl } );

  } // End variant loop

  // Combine generated code for the enum.
  let result = quote!
  {
      // Implement the static methods on the enum.
      #[ automatically_derived ]
      impl< #enum_generics_impl > #enum_name< #enum_generics_ty >
      where
        #enum_generics_where
      {
          #( #methods )*
      }

      // Define the End structs and their implementations outside the enum impl block.
      #( #end_impls )*
  };

  Ok( result )
}