use super::*;
use macro_tools::
{
  attr,
  diag,
  generic_params,
  item_struct,
  Result,
  syn::ItemStruct
};

/// Generates [Not](core::ops::Not) trait implementation for input struct.
pub fn not( input : proc_macro::TokenStream  ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< ItemStruct >( input )?;
  let has_debug = attr::has_debug( parsed.attrs.iter() )?;
  let item_name = &parsed.ident;

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
    = generic_params::decompose( &parsed.generics );

  let body = generate_method_body( &parsed );

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

/// Produces body for [not](core::ops::Not::not) method depending on type of input [ItemStruct](ItemStruct).
fn generate_method_body(item_struct: &ItemStruct ) -> proc_macro2::TokenStream
{
  let field_types = item_struct::field_types( &item_struct );
  let field_names = item_struct::field_names( &item_struct );

  match ( field_types.len(), field_names )
  {
    ( 0, _ ) => generate_for_unit(),
    ( _, Some( field_names ) ) => generate_for_named(field_types, field_names ),
    ( _, None ) => generate_for_tuple( field_types ),
  }
}

fn generate_for_unit() -> proc_macro2::TokenStream
{
  qt! { Self {} }
}

fn generate_for_named<'a>
(
  field_types : impl macro_tools::IterTrait< 'a, &'a syn::Type >,
  field_names : impl macro_tools::IterTrait< 'a, &'a syn::Ident >,
)
-> proc_macro2::TokenStream
{
  let ( mut_ref_transformations, values ): (Vec< proc_macro2::TokenStream >, Vec< proc_macro2::TokenStream > ) =
  field_types
  .clone()
  .zip( field_names )
  .map( | ( field_type, field_name ) |
  {
    match field_type
    {
      syn::Type::Reference( reference ) =>
      {
        if reference.mutability.is_some()
        {
          ( qt! { *self.#field_name = !*self.#field_name; }, qt! { #field_name: self.#field_name } )
        }
        else
        {
          ( qt! {}, qt! { #field_name: self.#field_name } )
        }
      }
      _ => { ( qt!{}, qt! { #field_name: !self.#field_name } ) }
    }
  })
  .unzip();

  qt!
  {
    #(#mut_ref_transformations)*
    Self { #(#values),* }
  }
}

fn generate_for_tuple<'a>
(
  field_types : impl macro_tools::IterTrait< 'a, &'a syn::Type >,
)
-> proc_macro2::TokenStream
{
  let ( mut_ref_transformations, values ): (Vec< proc_macro2::TokenStream >, Vec< proc_macro2::TokenStream > ) =
  field_types
  .clone()
  .enumerate()
  .map( | ( index, field_type ) |
  {
    let index = syn::Index::from( index );
    match field_type
    {
      syn::Type::Reference( reference ) =>
      {
        if reference.mutability.is_some()
        {
          ( qt! { *self.#index = !*self.#index; }, qt! { self.#index } )
        }
        else
        {
          ( qt! {}, qt! { self.#index } )
        }
      }
      _ => { ( qt!{}, qt! { !self.#index } ) }
    }
  })
  .unzip();

  qt!
  {
    #(#mut_ref_transformations)*
    Self ( #(#values),* )
  }
}