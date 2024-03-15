use super::*;
use macro_tools::{ type_struct, Result };

//

pub fn from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< type_struct::TypeStructParsed >( input )?;
  let field_types = parsed.field_types();
  let field_names = parsed.field_names();
  let item_name = parsed.item_name;
  let result =
  match ( field_types.len(), field_names )
  {
    ( 0, _ ) => { generate_unit(item_name) },
    ( 1, Some( field_names ) ) => generate_from_single_field_named( &field_types[ 0 ], &field_names[ 0 ], item_name ),
    ( 1, None ) => generate_from_single_field( &field_types[ 0 ], item_name ),
    ( _, Some( field_names ) ) => generate_from_multiple_fields_named( &field_types, &field_names, item_name ),
    ( _, None ) => generate_from_multiple_fields( &field_types, item_name ),
  };

  Ok( result )
}

fn generate_from_single_field_named( field_type: &syn::Type, field_name: &syn::Ident, item_name: syn::Ident ) -> proc_macro2::TokenStream
{
  qt!
  {
    #[ automatically_derived ]
    // impl From < i32 > for MyStruct
    impl From< #field_type > for #item_name
    {
      #[ inline( always ) ]
      // fn from( src: i32 ) -> Self
      fn from( src: #field_type ) -> Self
      {
        // Self { a: src }
        Self { #field_name: src }
      }
    }
  }
}

fn generate_from_single_field( field_type: &syn::Type, item_name: syn::Ident ) -> proc_macro2::TokenStream
{
  qt!
  {
    #[automatically_derived]
    // impl From< bool > for IsTransparent
    impl From< #field_type > for #item_name
    {
      #[ inline( always ) ]
      // fn from( src: bool ) -> Self
      fn from( src: #field_type ) -> Self
      {
        // Self(src)
        Self(src)
      }
    }
  }
}

fn generate_from_multiple_fields_named( field_types: &Vec< syn::Type >, field_names: &Vec< syn::Ident >, item_name: syn::Ident) -> proc_macro2::TokenStream
{
  let params: Vec< proc_macro2::TokenStream > = field_names
    .iter()
    .enumerate()
    .map(| ( index, field_name ) |
      {
        let index = index.to_string().parse::< proc_macro2::TokenStream >().unwrap();
        qt! { #field_name : src.#index }
      })
      .collect();

  qt!
  {
    // impl From< (i32, bool) > for StructNamedFields
    impl From< (#(#field_types), *) > for #item_name
    {
      #[ inline( always ) ]
      // fn from( src: (i32, bool) ) -> Self
      fn from( src: (#(#field_types), *) ) -> Self
      {
        // StructNamedFields{ a: src.0, b: src.1 }
        #item_name { #(#params), * }
      }
    }
  }
}

fn generate_from_multiple_fields( field_types: &Vec< syn::Type >, item_name: syn::Ident ) -> proc_macro2::TokenStream
{
  let params: Vec< proc_macro2::TokenStream > = ( 0..field_types.len() )
  .map( | index |
    {
      let index = index.to_string().parse::< proc_macro2::TokenStream >().unwrap();
        qt!( src.#index )
    } )
  .collect();

  qt!
  {
    // impl From< (i32, bool) > for StructWithManyFields
    impl From< (#(#field_types), *) > for #item_name
    {
      #[ inline( always ) ]
      // fn from( src: (i32, bool) ) -> Self
      fn from( src: (#(#field_types), *) ) -> Self
      {
        // StructWithManyFields( src.0, src.1 )
        #item_name( #(#params), *)
      }
    }
  }
}

fn generate_unit( item_name: syn::Ident ) -> proc_macro2::TokenStream
{
  qt!
  {
    // impl From< () > for UnitStruct
    impl From< () > for #item_name
    {
      #[ inline( always ) ]
      fn from( src: () ) -> Self
      {
        Self
      }
    }
  }
}