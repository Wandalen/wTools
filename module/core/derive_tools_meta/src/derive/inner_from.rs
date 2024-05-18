
use super::*;
use macro_tools::{ attr, diagm, item_struct, Result };

//

pub fn inner_from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< syn::ItemStruct >( input )?;
  let has_debug = attr::has_debug( parsed.attrs.iter() )?;
  let item_name = &parsed.ident;

  let mut field_types = item_struct::field_types( &parsed );
  let field_names = item_struct::field_names( &parsed );
  let result =
  match ( field_types.len(), field_names )
  {
    ( 0, _ ) => unit( item_name ),
    ( 1, Some( mut field_names ) ) =>
    {
      let field_name = field_names.next().unwrap();
      let field_type = field_types.next().unwrap();
      from_impl_named( item_name, field_type, field_name )
    }
    ( 1, None ) =>
    {
      let field_type = field_types.next().unwrap();
      from_impl( item_name, field_type )
    }
    ( _, Some( field_names ) ) =>
    {
      let params : Vec< proc_macro2::TokenStream > = field_names
      .map( | field_name | qt! { src.#field_name } )
      .collect();
      from_impl_multiple_fields( item_name, field_types, &params )
    }
    ( _, None ) =>
    {
      let params : Vec< proc_macro2::TokenStream > = ( 0..field_types.len() )
      .map( | index |
      {
        let index : proc_macro2::TokenStream = index.to_string().parse().unwrap();
        qt! { src.#index }
      })
      .collect();
      from_impl_multiple_fields( item_name, field_types, &params )
    }
  };

  if has_debug
  {
    let about = format!( "derive : InnerFrom\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

// qqq  : document, add example of generated code
fn from_impl_named
(
  item_name : syn::Ident,
  field_type : &syn::Type,
  field_name : &syn::Ident,
) -> proc_macro2::TokenStream
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

// qqq  : document, add example of generated code
fn from_impl
(
  item_name : syn::Ident,
  field_type : &syn::Type,
) -> proc_macro2::TokenStream
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

// qqq  : document, add example of generated code
fn from_impl_multiple_fields< 'a >
(
  item_name : syn::Ident,
  field_types : impl macro_tools::IterTrait< 'a, macro_tools::syn::Type >,
  // field_types : &Vec< &syn::Type >,
  params : &Vec< proc_macro2::TokenStream >,
) -> proc_macro2::TokenStream
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

// qqq  : document, add example of generated code
fn unit( item_name : syn::Ident ) -> proc_macro2::TokenStream
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
