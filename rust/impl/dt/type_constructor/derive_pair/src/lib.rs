use proc_macro2::Ident;
use quote::{ quote, ToTokens };
use syn::{ parse_macro_input, Fields, ItemStruct, Field, TypeParam };

struct DerivePair
{
  ident : Ident,
  generics : Vec< TypeParam >,
  fields : ( Field, Field ),
}

impl DerivePair
{
  fn parse( input: ItemStruct ) -> Self
  {
    let ident = &input.ident;
    let generics =  input.generics.type_params().cloned().collect::< Vec< _ > >() ;
    let mut fields  = match &input.fields
    {
      Fields::Unnamed( fields ) =>
      {
        fields.unnamed.iter().cloned()
      },
      _ => unimplemented!()
    };
    Self
    {
      ident : ident.to_owned(),
      generics,
      fields : ( fields.next().unwrap(), fields.next().unwrap() ),
    }
  }

  fn impl_from_tuple( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.generics.iter()
    .map( | g | g.ident.to_owned() ).collect::< Vec< _ > >();
    let param1 = &self.fields.0.ty;
    let param2 = &self.fields.1.ty;
    quote!
    (
      impl
      <
        #( #generics, )*
        Into1 : Into< #param1 >, Into2 : Into< #param2 >
      >
      From
      <(
       Into1, Into2
      )>
      for #struct_name< #( #gtypes ),* >
      {
        #[ inline ]
        fn from( src : ( Into1, Into2 ) ) -> Self
        {
          Self( src.0.into(), src.1.into() )
        }
      }
    )
  }

  fn impl_to_tuple( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.generics.iter()
    .map( | g | g.ident.to_owned() ).collect::< Vec< _ > >();
    let param1 = &self.fields.0.ty;
    let param2 = &self.fields.1.ty;
    quote!
    (
      impl< #( #generics ),* >
      From  < #struct_name< #( #gtypes ),* > >
      for ( #param1, #param2 )
      {
        #[ inline ]
        fn from( src : #struct_name< #( #gtypes ),* > ) -> Self
        {
          ( src.0, src.1 )
        }
      }
    )
  }

  fn impl_from_slice( &self ) -> proc_macro2::TokenStream
  {
    quote!()
  }

  fn impl_to_slice( &self ) -> proc_macro2::TokenStream
  {
    quote!()
  }

  fn impl_as_tuple( &self ) -> proc_macro2::TokenStream
  {
    quote!()
  }

  fn impl_as_array( &self ) -> proc_macro2::TokenStream
  {
    quote!()
  }

  fn impl_as_slice( &self ) -> proc_macro2::TokenStream
  {
    quote!()
  }

  fn impl_clone_as_tuple( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.generics.iter()
    .map( | g | g.ident.to_owned() ).collect::< Vec< _ > >();
    let param1 = &self.fields.0.ty;
    let param2 = &self.fields.1.ty;

    let where_clauses = &gtypes.iter().map( | gt |
    {
      quote!( #gt : Clone )
    })
    .collect::< Vec< _ > >();
    quote!
    (
      impl< #( #generics ),* >
      CloneAsTuple<( #param1, #param2 )>
      for #struct_name< #( #gtypes ),* >
      where
        #( #where_clauses ),*
      {
        #[ inline ]
        fn clone_as_tuple( &self ) -> ( #param1, #param2 )
        {
          ( self.0.clone(), self.1.clone() )
        }
      }
    )
  }

  fn impl_clone_as_array( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.generics.iter()
    .map( | g | g.ident.to_owned() ).collect::< Vec< _ > >();
    let param1 = &self.fields.0.ty;
    let where_clauses = &gtypes.iter().map( | gt |
    {
      quote!( #gt : Clone )
    })
    .collect::< Vec< _ > >();
    quote!
    (
      impl< #( #generics ),* >
      CloneAsArray< #param1, 2 >
      for #struct_name< #( #gtypes ),* >
      where
        #( #where_clauses ),*
      {
        #[ inline ]
        fn clone_as_array( &self ) -> [ #param1; 2 ]
        {
          [ self.0.clone(), self.1.clone() ]
        }
      }
    )
  }
}

#[ proc_macro_derive( Pair ) ]
pub fn derive_make( input: proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let input = parse_macro_input!( input as syn::ItemStruct );
  let dp = DerivePair::parse( input );

  let mut impls =
  vec!
  [
    dp.impl_from_tuple(),
    dp.impl_to_tuple(),
    dp.impl_clone_as_tuple(),
  ];

  // if two fields has the same types => it can be stored into array/slice/...
  let type1_as_string = &dp.fields.0.ty.clone().into_token_stream().to_string();
  let type2_as_string = &dp.fields.1.ty.clone().into_token_stream().to_string();
  if type1_as_string == type2_as_string
  {
    impls.push( dp.impl_from_slice() );
    impls.push( dp.impl_to_slice() );
    impls.push( dp.impl_clone_as_array() );
  }
  let result = impls.iter().fold( quote!(), | mut result, i |
  {
    result = quote!( #result #i );
    result
  });

  proc_macro::TokenStream::from( result )
}
  