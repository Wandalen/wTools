use proc_macro2::TokenStream;
use quote::{ quote, ToTokens };
use syn::{ parse_macro_input, Fields };


fn impl_make0( is_named : bool, fields : &Vec< Field > ) -> TokenStream
{
  let types = fields.iter()
  .map( | f | f.f_type.to_owned() )
  .collect::< Vec< _ > >();
  if is_named
  {
    let names = fields.iter()
    .map( | f | f.f_name.clone() )
    .collect::< Vec< _ > >();
    quote!( Self{ #( #names : #types::default() ),* } )
  }
  else
  {
    quote!( Self( #( #types::default() ),* ) )
  }
}

fn impl_make1( is_named : bool, fields : &Vec< Field > ) -> TokenStream
{
  let types = fields.iter()
  .map( | f | f.f_type.to_owned() )
  .collect::< Vec< _ > >();
  if is_named
  {
    let names = fields.iter()
    .map( | f | f.f_name.clone() )
    .collect::< Vec< _ > >();
    quote!( Self{ #( #names : val as #types ),* } )
  }
  else
  {
    quote!( Self( #( val as #types ),* ) )
  }
}

#[ derive( Debug ) ]
struct Field
{
  f_name : Option< TokenStream >,
  f_type : TokenStream,
}

#[ proc_macro_derive( Make ) ]
pub fn derive_make( input: proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let input = parse_macro_input!( input as syn::ItemStruct );
  let struct_name = input.ident;
  let is_named;
  let fields_vec = match input.fields
  {
    Fields::Named( named ) =>
    {
      is_named = true;
      let fields = &named.named;
      fields.iter().map( | field |
      {
        Field
        {
          f_name : Some( field.ident.as_ref().unwrap().to_token_stream() ),
          f_type : field.ty.clone().into_token_stream(),
        }
      })
      .collect::< Vec< _ > >()
    },
    Fields::Unnamed( unnamed ) =>
    {
      is_named = false;
      let fields = &unnamed.unnamed;
      fields.iter().map( | field |
      {
        Field
        {
          f_name : None,
          f_type : field.into_token_stream(),
        }
      } )
      .collect::< Vec< _ > >()
    },
    _ => unimplemented!()
  };

  let f_type = fields_vec[ 0 ].f_type.clone(); 

  let make0 = impl_make0( is_named, &fields_vec );
  let make1 = impl_make1( is_named, &fields_vec );

  let expanded = quote! {
    impl Make0 for #struct_name
    {
      fn make_0() -> Self
      {
        #make0
      }
    }

    impl Make1< #f_type > for #struct_name
    {
      fn make_1( val : #f_type ) -> Self
      {
        #make1
      }
    }
  };

  proc_macro::TokenStream::from( expanded )
}
