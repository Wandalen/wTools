use super::*;
use macro_tools::{attr, diag, generic_params, item_struct, Result, syn::ItemStruct};

pub fn not( input : proc_macro::TokenStream  ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< ItemStruct >( input )?;
  let has_debug = attr::has_debug( parsed.attrs.iter() )?;
  let item_name = &parsed.ident;

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
    = generic_params::decompose( &parsed.generics );

  let constructor = generate_struct_constructor( &parsed );

  let result = qt!
  {
    impl< #generics_impl > ::core::ops::Not for #item_name< #generics_ty >
    where
      #generics_where
    {
      type Output = Self;

      fn not( self ) -> Self::Output
      {
        #constructor
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

fn generate_struct_constructor( item_struct: &ItemStruct ) -> proc_macro2::TokenStream
{
  let field_types = item_struct::field_types( &item_struct );
  let field_names = item_struct::field_names( &item_struct );

  match ( field_types.len(), field_names )
  {
    ( 0, _ ) => generate_unit_constructor(),
    ( _, Some( field_names )) => generate_named_constructor( field_names ),
    ( _, None ) => generate_tuple_constructor( field_types.len() ),
  }
}

fn generate_unit_constructor() -> proc_macro2::TokenStream
{
  qt! { Self {} }
}

fn generate_named_constructor<'a>
(
  field_names : impl macro_tools::IterTrait< 'a, &'a syn::Ident >
)
-> proc_macro2::TokenStream
{
  let values = field_names
    .clone()
    .map( | field_name  |
    {
      qt! { #field_name: !self.#field_name }
    });

  qt! { Self { #(#values),* } }
}

fn generate_tuple_constructor(fields_len : usize) -> proc_macro2::TokenStream
{
  let values = (0..fields_len)
    .map(|i|
    {
      let index = syn::Index::from(i);
      qt! { !self.#index }
    });

  qt! { Self ( #(#values),* ) }
}