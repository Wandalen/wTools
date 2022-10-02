use proc_macro2::Ident;
use quote::quote;
use syn::{ parse_macro_input, Fields, ItemStruct, Field, TypeParam };

struct DerivePair
{
  ident : Ident,
  generics : ( Option< TypeParam >, Option< TypeParam > ),
  fields : ( Field, Field ),
}

impl DerivePair
{
  fn parse( input: ItemStruct ) -> Self
  {
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
      ident : ident.to_owned(),
      generics : ( generics.next(), generics.next() ),
      fields : ( fields.next().unwrap(), fields.next().unwrap() ),
    }
  }

  fn impl_from_slice( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generic1 = &self.generics.0.as_ref().map( | g | quote!( #g, ) );
    let generic2 = &self.generics.1.as_ref().map( | g | quote!( #g, ) );
    let gtype1 = &self.generics.0.as_ref().map( | g |
    { let gi = g.ident.to_owned(); quote!( #gi, ) } );
    let gtype2 = &self.generics.1.as_ref().map( | g |
    { let gi = g.ident.to_owned(); quote!( #gi, ) } );
    let param1 = &self.fields.0.ty;
    let param2 = &self.fields.1.ty;
    quote!
    (
      impl
      <
        #generic1 #generic2
        Into1 : Into< #param1 >, Into2 : Into< #param2 >
      >
      From
      <(
       Into1, Into2
      )>
      for #struct_name< #gtype1 #gtype2 >
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
    let generic1 = &self.generics.0.as_ref().map( | g | quote!( #g, ) );
    let generic2 = &self.generics.1.as_ref().map( | g | quote!( #g, ) );
    let gtype1 = &self.generics.0.as_ref().map( | g |
    { let gi = g.ident.to_owned(); quote!( #gi, ) } );
    let gtype2 = &self.generics.1.as_ref().map( | g |
    { let gi = g.ident.to_owned(); quote!( #gi, ) } );
    let param1 = &self.fields.0.ty;
    let param2 = &self.fields.1.ty;
    quote!
    (
      impl< #generic1 #generic2 >
      From  < #struct_name< #gtype1 #gtype2 > >
      for ( #param1, #param2 )
      {
        #[ inline ]
        fn from( src : #struct_name< #gtype1 #gtype2 > ) -> Self
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
  