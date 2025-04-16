#[ allow( clippy::wildcard_imports ) ]
use super::*;
use iter_tools::Itertools;
use macro_tools::
{
  attr, diag, generic_params, generic_args, typ, derive, Result, ident,
  proc_macro2::TokenStream, quote::{ format_ident, quote }, syn::spanned::Spanned
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
  fn form_mutation( storage : &mut Self::Storage, context : &mut Option< Self::Context > )
  {{
  }}
}}
      ",
      format!( "{}", qt!{ #former_definition_types_generics_impl } ),
      format!( "{}", qt!{ #former_definition_types_generics_ty } ),
      format!( "{}", qt!{ #former_definition_types_generics_where } ),
    );
    // println!( "{debug}" );
    let about = format!
    (
r"derive : Former
item : {item}",
    );
    diag::report_print( about, original_input, debug );
  };

  Ok( former_mutator_code )
}


/// Generate documentation for the former.
// ... (doc_generate function remains the same) ...
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
#[ allow( clippy::too_many_lines ) ]
pub fn former( input : proc_macro::TokenStream ) -> Result< TokenStream >
{
  let original_input = input.clone();
  let ast = syn::parse::< syn::DeriveInput >( input )?;
  let has_debug = attr::has_debug( ast.attrs.iter() )?;

  let result = match ast.data
  {
      syn::Data::Struct( ref data_struct ) =>
      {
          former_for_struct( &ast, data_struct, &original_input, has_debug )
      },
      syn::Data::Enum( ref data_enum ) =>
      {
          former_for_enum( &ast, data_enum, &original_input, has_debug ) // New function for enums
      },
      syn::Data::Union( _ ) =>
      {
          Err( syn::Error::new( ast.span(), "Former derive does not support unions" ) )
      }
  }?;

  if has_debug
  {
    let about = format!( "derive : Former\nstructure : {}", ast.ident );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

#[ allow( clippy::too_many_lines ) ]
fn former_for_struct
(
  ast : &syn::DeriveInput,
  _data_struct : &syn::DataStruct, // Keep parameter for consistency, might be used later
  original_input : &proc_macro::TokenStream,
  _has_debug : bool, // qqq : xxx : make sure debug works
) -> Result< TokenStream >
{
  // --- Existing struct logic starts here ---
  use macro_tools::IntoGenericArgs; // Needed for struct logic

  let struct_attrs = ItemAttributes::from_attrs( ast.attrs.iter() )?;

  /* names */

  let vis = &ast.vis;
  let item = &ast.ident;
  let former = format_ident!( "{item}Former" );
  let former_storage = format_ident!( "{item}FormerStorage" );
  let former_definition = format_ident!( "{item}FormerDefinition" );
  let former_definition_types = format_ident!( "{item}FormerDefinitionTypes" );
  let as_subformer = format_ident!( "{item}AsSubformer" );
  let as_subformer_end = format_ident!( "{item}AsSubformerEnd" );

  let as_subformer_end_doc = format!
  (
    r"
Represents an end condition for former of [`${item}`], tying the lifecycle of forming processes to a broader context.

This trait is intended for use with subformer alias, ensuring that end conditions are met according to the
specific needs of the broader forming context. It mandates the implementation of `former::FormingEnd`.
    "
  );

  /* parameters for structure */

  let generics = &ast.generics;
  let ( struct_generics_with_defaults, struct_generics_impl, struct_generics_ty, struct_generics_where )
  = generic_params::decompose( generics );

  /* parameters for definition */

  let extra : macro_tools::syn::AngleBracketedGenericArguments = parse_quote!
  {
    < (), #item < #struct_generics_ty >, former::ReturnPreformed >
  };
  let former_definition_args = generic_args::merge( &generics.into_generic_args(), &extra ).args;

  /* parameters for former */

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

  /* parameters for former perform */

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

  // FIX: Remove underscore from _former_perform_generics_ty when destructuring
  let ( _former_perform_generics_with_defaults, former_perform_generics_impl, former_perform_generics_ty, former_perform_generics_where )
  = generic_params::decompose( &extra );

  /* parameters for definition types */

  let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < __Context = (), __Formed = #item < #struct_generics_ty > >
  };
  let former_definition_types_generics = generic_params::merge( generics, &extra.into() );
  let ( former_definition_types_generics_with_defaults, former_definition_types_generics_impl, former_definition_types_generics_ty, former_definition_types_generics_where )
  = generic_params::decompose( &former_definition_types_generics );

  let former_definition_types_phantom = macro_tools::phantom::tuple( &former_definition_types_generics_impl );

  /* parameters for definition */

  let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < __Context = (), __Formed = #item < #struct_generics_ty >, __End = former::ReturnPreformed >
  };
  let generics_of_definition = generic_params::merge( generics, &extra.into() );
  let ( former_definition_generics_with_defaults, former_definition_generics_impl, former_definition_generics_ty, former_definition_generics_where )
  = generic_params::decompose( &generics_of_definition );

  let former_definition_phantom = macro_tools::phantom::tuple( &former_definition_generics_impl );

  /* struct attributes */

  let ( _doc_former_mod, doc_former_struct ) = doc_generate( item );
  let ( perform, perform_output, perform_generics ) = struct_attrs.performer()?;

  /* fields */

  let fields = derive::named_fields( &ast )?;

  let formed_fields : Vec< _ > = fields
  .iter() // Use iter() instead of into_iter()
  .map( | field |
  {
    FormerField::from_syn( field, true, true )
  })
  .collect::< Result< _ > >()?;

  let storage_fields : Vec< _ > = struct_attrs
  .storage_fields()
  .iter()
  .map( | field |
  {
    FormerField::from_syn( field, true, false )
  })
  .collect::< Result< _ > >()?;

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

  let results : Result< Vec< _ > > = former_field_setter.into_iter().collect();
  let ( former_field_setter, namespace_code ) : ( Vec< _ >, Vec< _ > ) = results?.into_iter().unzip();

  let storage_field_preform : Vec< _ > = storage_field_preform
  .into_iter()
  .collect::< Result< _ > >()?;

  let former_mutator_code = mutator
  (
    item,
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

    // = entity to former

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

    // = definition types

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

    // = definition

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

    // = former

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
      pub fn new( on_end : Definition::End ) -> Self
      {
        Self::begin_coercing( ::core::option::Option::None, ::core::option::Option::None, on_end )
      }

      /// Initializes a former with a coercible end condition.
      #[ inline( always ) ]
      pub fn new_coercing< IntoEnd >( end : IntoEnd ) -> Self
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

      #(
        #former_field_setter
      )*

    }

    // = former :: preform

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

    // = former :: perform

    #[ automatically_derived ]
    impl < #former_perform_generics_impl > #former < #former_perform_generics_ty > // FIX: Use the correct generic type parameter here
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

    // = former begin

    impl< #struct_generics_impl Definition > former::FormerBegin< Definition >
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
        Self::begin( ::core::option::Option::None, context, on_end )
      }

    }

    // = subformer

    /// Provides a specialized former for structure using predefined settings for superformer and end conditions.
    #vis type #as_subformer < #struct_generics_impl __Superformer, __End > = #former
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
  // --- Existing struct logic ends here ---
  Ok( result )
}

/// Generate the Former ecosystem for an enum.
#[ allow( clippy::too_many_lines ) ]
fn former_for_enum
(
  ast : &syn::DeriveInput,
  data_enum : &syn::DataEnum,
  _original_input : &proc_macro::TokenStream,
  _has_debug : bool,
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

    // Generate names
    // FIX: Explicit snake_case conversion + use ident_maybe_raw
    let variant_name_str = variant_ident.to_string();
    let method_name_snake_str = variant_name_str.to_case( Case::Snake );
    // Create a temporary Ident with the correct span for keyword checking
    let method_name_ident_temp = format_ident!( "{}", method_name_snake_str, span = variant_ident.span() );
    // Use the utility function on the temporary snake_case ident
    let method_name = ident::ident_maybe_raw( &method_name_ident_temp );

    let end_struct_name = format_ident!( "{}{}End", enum_name, variant_ident );
    let inner_type_name = match inner_type
    {
        syn::Type::Path( type_path ) => type_path.path.segments.last().map( | s | s.ident.clone() )
            .ok_or_else( || syn::Error::new_spanned( inner_type, "Cannot derive name from type path") )?,
        _ => return Err( syn::Error::new_spanned( inner_type, "Inner variant type must be a path type (like MyStruct or MyStruct<T>) to derive Former" ) ),
    };

    let inner_former_name = format_ident!( "{}Former", inner_type_name );
    let inner_storage_name = format_ident!( "{}FormerStorage", inner_type_name );
    let inner_def_name = format_ident!( "{}FormerDefinition", inner_type_name );
    let inner_def_types_name = format_ident!( "{}FormerDefinitionTypes", inner_type_name );

    let _inner_generics_impl = quote! {};
    let inner_generics_ty = quote! {};
    let inner_generics_where = quote! {};

    // Generate End struct definition
    let end_struct_def = qt!
    {
      #[derive(Default, Debug)]
      #vis struct #end_struct_name;
    };

    // Generate impl FormingEnd for End struct
    let end_impl = qt!
    {
      #[automatically_derived]
      impl<#enum_generics_impl> former::FormingEnd
      <
          #inner_def_types_name<#inner_generics_ty (), #enum_name<#enum_generics_ty> >
      >
      for #end_struct_name
      where #enum_generics_where #inner_generics_where
      {
          #[ inline( always ) ]
          fn call
          (
            &self,
            sub_storage : #inner_storage_name<#inner_generics_ty>,
            _context : Option< () >,
          ) -> #enum_name<#enum_generics_ty>
          {
            let data = former::StoragePreform::preform( sub_storage );
            #enum_name::#variant_ident( data )
          }
      }
    };

    // Generate static method
    let static_method = qt!
    {
      #[ inline( always ) ]
      #vis fn #method_name() // Use corrected method name
      -> #inner_former_name
         <
           #inner_def_name
           <
               (),
               #enum_name<#enum_generics_ty>,
               #end_struct_name
           >
         >
      {
          #inner_former_name::begin( None, None, #end_struct_name::default() )
      }
    };

    methods.push( static_method );
    end_impls.push( quote!{ #end_struct_def #end_impl } );

  } // End variant loop

  // Combine generated code
  let result = qt!
  {
      #[automatically_derived]
      impl<#enum_generics_impl> #enum_name<#enum_generics_ty> #enum_generics_where
      {
          #( #methods )*
      }

      #( #end_impls )*
  };

  Ok( result )
}