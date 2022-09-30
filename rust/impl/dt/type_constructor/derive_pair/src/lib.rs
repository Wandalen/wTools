use proc_macro2::Ident;
use quote::quote;
use syn::{ parse_macro_input, Fields, ItemStruct, Visibility, Field, TypeParam };

struct DerivePair
{
  _vis : Visibility,
  ident : Ident,
  generics : ( TypeParam, Option< TypeParam > ),
  fields : ( Field, Option< Field > ),
}

impl DerivePair
{
  fn parse( input: ItemStruct ) -> Self
  {
    let vis = &input.vis; 
    let ident = &input.ident;
    let generics = &mut input.generics.type_params().cloned();
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
      _vis : vis.to_owned(),
      ident : ident.to_owned(),
      generics : ( generics.next().unwrap(), generics.next() ),
      fields : ( fields.next().unwrap(), fields.next() ),
    }
  }

  fn impl_from_slice( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generic1 = &self.generics.0;
    let generic2 = &self.generics.1.as_ref().map( | g | quote!( #g, ) );
    let param1 = &self.fields.0.ty;
    let param2 = &self.fields.1.as_ref().map( | f | f.ty.to_owned() );
    let gtype1 = &generic1.ident;
    let gtype2 = &self.generics.1.as_ref().map( | g | g.ident.to_owned() );
    quote!
    (
      impl
      <
        #generic1, #generic2
        Into1 : Into< #param1 >, Into2 : Into< #param2 >
      >
      From
      <(
       Into1, Into2
      )>
      for #struct_name< #gtype1, #gtype2 >
      {
        #[ inline ]
        fn from( src : ( Into1, Into2 ) ) -> Self
        {
          Self( src.0.into(), src.1.into() )
        }
      }
    )
  }

  fn impl_to_slice( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generic1 = &self.generics.0;
    let generic2 = &self.generics.1.as_ref().map( | g | quote!( #g, ) );
    let param1 = &self.fields.0.ty;
    let param2 = &self.fields.1.as_ref().map( | f | f.ty.to_owned() );
    let gtype1 = &generic1.ident;
    let gtype2 = &self.generics.1.as_ref().map( | g | g.ident.to_owned() );
    quote!
    (
      impl < #generic1, #generic2 >
      From < #struct_name< #gtype1, #gtype2 > >
      for ( #param1, #param2 )
      {
        #[ inline ]
        fn from( src : #struct_name< #gtype1, #gtype2 > ) -> Self
        {
          ( src.0, src.1 )
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

  let impls =
  [
    dp.impl_from_slice(),
    dp.impl_to_slice(),
  ];
  let result = impls.iter().fold( quote!(), | mut result, i |
  {
    result = quote!( #result #i );
    result
  });


  proc_macro::TokenStream::from( result )
}
  