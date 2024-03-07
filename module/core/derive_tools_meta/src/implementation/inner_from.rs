
use macro_tools::proc_macro2::TokenStream;

use super::*;

//

pub fn inner_from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< InputParsed >( input )?;
  let field_types = parsed.field_types;
  let field_names = parsed.field_names;
  let item_name = parsed.item_name;
  let result =
  match ( field_types.len(), field_names )
  {
    ( 0, _ ) => generate_unit( item_name ),
    ( 1, Some( field_names ) ) =>
    {
      let field_name = field_names.get( 0 ).unwrap();
      let field_type = field_types.get( 0 ).unwrap();
      generate_from_impl_named( item_name, field_type, field_name )
    }
    ( 1, None ) =>
    {
      let field_type = field_types.get( 0 ).unwrap();
      generate_from_impl( item_name, field_type )
    }
    ( _, Some( field_names ) ) =>
    {
      let params: Vec< TokenStream > = field_names.iter()
      .map( | field_name | qt! { src.#field_name } )
      .collect();
      generate_from_impl_multiple_fields( item_name, &field_types, &params )
    }
    ( _, None ) =>
    {
      let params: Vec< TokenStream > = ( 0..field_types.len() )
      .map( | index |
      {
        let index: TokenStream = index.to_string().parse().unwrap();
        qt! { src.#index }
      })
      .collect();
      generate_from_impl_multiple_fields( item_name, &field_types, &params )
    }
  };
  Ok( result )
}

fn generate_from_impl_named( item_name: syn::Ident, field_type: &syn::Type, field_name: &syn::Ident ) -> TokenStream
{
  qt!
  {
    #[ allow( non_local_definitions ) ]
    #[ automatically_derived ]
    // impl From< MyStruct > for i32
    impl From< #item_name > for #field_type
    {
      #[ inline( always ) ]
      // fm from( src: MyStruct ) -> Self
      fn from( src: #item_name ) -> Self
      {
        // src.a
        src.#field_name
      }
    }
  }
}

fn generate_from_impl( item_name: syn::Ident, field_type: &syn::Type ) -> TokenStream
{
  qt!
  {
    #[ allow( non_local_definitions ) ]
    #[ automatically_derived ]
    // impl From< IsTransparent> for bool
    impl From< #item_name > for #field_type
    {
      #[ inline( always ) ]
      // fn from( src: IsTransparent ) -> Self
      fn from( src: #item_name ) -> Self
      {
        src.0
      }
    }
  }
}

fn generate_from_impl_multiple_fields ( item_name: syn::Ident, field_types: &Vec< syn::Type >, params: &Vec< TokenStream > ) -> TokenStream
{
  qt!
  {
    #[ allow( non_local_definitions ) ]
    #[ automatically_derived ]
    // impl From< StructWithManyFields > for ( i32, bool )
    impl From< #item_name > for ( #(#field_types), *)
    {
      #[ inline( always ) ]
      // fn from( src: StructWithManyFields ) -> Self
      fn from( src: #item_name ) -> Self
      {
        //( src.0, src.1 )
        (#(#params), *)
      }
    }
  }
}

fn generate_unit( item_name: syn::Ident ) -> TokenStream
{
  qt!
  {
    #[ allow( non_local_definitions ) ]
    #[ automatically_derived ]
    // impl From< UnitStruct > for ()
    impl From< #item_name > for ()
    {
      #[ inline( always ) ]
      // fn from( src: UnitStruct ) -> ()
      fn from( src: #item_name ) -> ()
      {
        ()
      }
    }
  }
}
