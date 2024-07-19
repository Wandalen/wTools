use super::*;
use macro_tools::
{
  attr,
  diag,
  generic_params,
  item_struct,
  Result,
  syn::ItemStruct,
};

mod field_attributes;
use field_attributes::*;
mod item_attributes;
use item_attributes::*;
use iter_tools::IterTrait;

/// Generates [Not](core::ops::Not) trait implementation for input struct.
pub fn not( input : proc_macro::TokenStream  ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< ItemStruct >( input )?;
  let has_debug = attr::has_debug( parsed.attrs.iter() )?;
  let item_attrs = ItemAttributes::from_attrs( parsed.attrs.iter() )?;
  let item_name = &parsed.ident;

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
    = generic_params::decompose( &parsed.generics );

  let field_attrs = parsed.fields.iter().map( | field | &field.attrs );
  let field_types = item_struct::field_types( &parsed );
  let field_names = item_struct::field_names( &parsed );

  let body = match ( field_types.len(), field_names )
  {
    ( 0, _ ) => generate_for_unit(),
    ( _, Some( field_names ) ) => generate_for_named( field_attrs, field_types, field_names, &item_attrs )?,
    ( _, None ) => generate_for_tuple( field_attrs, field_types, &item_attrs )?,
  };

  let result = qt!
  {
    impl< #generics_impl > ::core::ops::Not for #item_name< #generics_ty >
    where
      #generics_where
    {
      type Output = Self;

      fn not( self ) -> Self::Output
      {
        #body
      }
    }
  };

  if has_debug
  {
    let about = format!( "derive : Not\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

fn generate_for_unit() -> proc_macro2::TokenStream
{
  qt! { Self {} }
}

fn generate_for_named< 'a >
(
  field_attributes: impl IterTrait< 'a, &'a Vec< syn::Attribute > >,
  field_types : impl macro_tools::IterTrait< 'a, &'a syn::Type >,
  field_names : impl macro_tools::IterTrait< 'a, &'a syn::Ident >,
  item_attrs : &ItemAttributes,
)
-> Result< proc_macro2::TokenStream >
{
  let fields_enabled = field_attributes
  .map( | attrs| FieldAttributes::from_attrs( attrs.iter() ) )
  .collect::< Result< Vec< _ > > >()?
  .into_iter()
  .map( | fa | fa.config.enabled.value( item_attrs.config.enabled.value( item_attrs.config.enabled.value( true ) ) ) );

  let ( mut_ref_transformations, values ): ( Vec< proc_macro2::TokenStream >, Vec< proc_macro2::TokenStream > ) =
  field_types
  .clone()
  .zip( field_names )
  .zip( fields_enabled )
  .map( | ( ( field_type, field_name ), is_enabled ) |
  {
    match field_type
    {
      syn::Type::Reference( reference ) =>
      {
        (
          // If the field is a mutable reference, then change it value by reference
          if reference.mutability.is_some()
          {
            qt! { *self.#field_name = !*self.#field_name; }
          }
          else
          {
            qt! {}
          },
          qt! { #field_name: self.#field_name }
        )
      }
      _ =>
      {
        (
          qt!{},
          if is_enabled
          {
            qt! { #field_name: !self.#field_name }
          }
          else
          {
            qt! { #field_name: self.#field_name }
          }
        )
      }
    }
  })
  .unzip();

  Ok(
    qt!
    {
      #(#mut_ref_transformations)*
      Self { #(#values),* }
    }
  )
}

fn generate_for_tuple< 'a >
(
  field_attributes: impl IterTrait< 'a, &'a Vec<syn::Attribute> >,
  field_types : impl macro_tools::IterTrait< 'a, &'a syn::Type >,
  item_attrs : &ItemAttributes,
)
-> Result<proc_macro2::TokenStream>
{
  let fields_enabled = field_attributes
    .map( | attrs| FieldAttributes::from_attrs( attrs.iter() ) )
    .collect::< Result< Vec< _ > > >()?
    .into_iter()
    .map( | fa | fa.config.enabled.value( item_attrs.config.enabled.value( item_attrs.config.enabled.value( true ) ) ) );

  let ( mut_ref_transformations, values ): (Vec< proc_macro2::TokenStream >, Vec< proc_macro2::TokenStream > ) =
  field_types
  .clone()
  .enumerate()
  .zip( fields_enabled )
  .map( | ( ( index, field_type ), is_enabled ) |
  {
    let index = syn::Index::from( index );
    match field_type
    {
      syn::Type::Reference( reference ) =>
      {
        (
          // If the field is a mutable reference, then change it value by reference
          if reference.mutability.is_some()
          {
            qt! { *self.#index = !*self.#index; }
          }
          else
          {
            qt! {}
          },
          qt! { self.#index }
        )
      }
      _ =>
      {
        (
          qt!{},
          if is_enabled
          {
            qt! { !self.#index }
          }
          else
          {
            qt! { self.#index }
          }
        )
      }
    }
  })
  .unzip();

  Ok(
    qt!
    {
      #(#mut_ref_transformations)*
      Self ( #(#values),* )
    }
  )
}
