use proc_macro2::Ident;
use quote::{ quote, ToTokens };
use syn::{ parse_macro_input, Fields, ItemStruct, Field, TypeParam };

struct DerivePair
{
  ident : Ident,
  generics : Vec< TypeParam >,
  gtypes : Vec< proc_macro2::Ident >,
  fields : ( Field, Field ),
}

impl DerivePair
{
  fn parse( input: ItemStruct ) -> Self
  {
    let ident = input.ident;
    let generics =  input.generics.type_params().cloned().collect::< Vec< _ > >() ;
    let gtypes = generics.iter().map( | g | g.ident.to_owned() ).collect::< Vec< _ > >();
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
      gtypes,
      fields : ( fields.next().unwrap(), fields.next().unwrap() ),
    }
  }

  fn impl_from_tuple( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.gtypes;
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
    let gtypes = &self.gtypes;
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

  fn impl_as_tuple( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.gtypes;
    let param1 = &self.fields.0.ty;
    let param2 = &self.fields.1.ty;

    quote!
    (
      impl< #( #generics ),* > AsTuple<( #param1, #param2 )> for #struct_name< #( #gtypes ),* >
      {
        #[ inline ]
        fn as_tuple( &self ) -> &( #param1, #param2 )
        {
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            core::mem::transmute::< &_, &( #param1, #param2 ) >( self )
          }
        }
      }
    )
  }

  fn impl_clone_as_tuple( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.gtypes;
    let param1 = &self.fields.0.ty;
    let param2 = &self.fields.1.ty;

    quote!
    (
      impl< #( #generics ),* >
      CloneAsTuple<( #param1, #param2 )>
      for #struct_name< #( #gtypes ),* >
      where
        #param1 : Clone,
        #param2 : Clone,
      {
        #[ inline ]
        fn clone_as_tuple( &self ) -> ( #param1, #param2 )
        {
          ( self.0.clone(), self.1.clone() )
        }
      }
    )
  }
  
  fn impl_from_array( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.gtypes;
    let param = &self.fields.0.ty;
    quote!
    (
      impl< #( #generics ),* > From<[ #param ; 2 ]> for #struct_name< #( #gtypes ),* >
      where
        #param : Clone,
      {
        #[ inline ]
        fn from( src : [ #param ; 2 ] ) -> Self
        {
          Self( src[ 0 ].clone(), src[ 1 ].clone() )
        }
      }
    )
  }

  fn impl_from_slice( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.gtypes;
    let param = &self.fields.0.ty;
    quote!
    (
      impl< #( #generics ),* > From<&[ #param ]> for #struct_name< #( #gtypes ),* >
      where
        #param : Clone,
      {
        #[ inline ]
        fn from( src : &[ #param ] ) -> Self
        {
          debug_assert_eq!( src.len(), 2 );
          Self( src[ 0 ].clone(), src[ 1 ].clone() )
        }
      }
    )
  }

  fn impl_to_array( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.gtypes;
    let param = &self.fields.0.ty;

    quote!
    (
      impl< #( #generics ),* > From< #struct_name< #( #gtypes ),* > > for [ #param ; 2 ]
      {
        #[ inline ]
        fn from( src : #struct_name< #( #gtypes ),* > ) -> Self
        {
          [ src.0, src.1 ]
        }
      }
    )
  }

  fn impl_as_slice( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.gtypes;
    let param = &self.fields.0.ty;

    quote!
    (
      impl< #( #generics ),* > AsSlice< #param > for #struct_name< #( #gtypes ),* >
      {
        #[ inline ]
        fn as_slice( &self ) -> &[ #param ]
        {
          &AsArray::as_array( self )[ .. ]
        }
      }
    )
  }

  fn impl_as_array( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.gtypes;
    let param = &self.fields.0.ty;

    quote!
    (
      impl< #( #generics ),* > AsArray< #param, 2 > for #struct_name< #( #gtypes ),* >
      {
        #[ inline ]
        fn as_array( &self ) -> &[ #param ; 2 ]
        {
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            core::mem::transmute::< &_, &[ #param ; 2 ] >( self )
          }
        }
      }
    )
  }

  fn impl_clone_as_array( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.gtypes;
    let param = &self.fields.0.ty;

    quote!
    (
      impl< #( #generics ),* >
      CloneAsArray< #param, 2 >
      for #struct_name< #( #gtypes ),* >
      where
        #param : Clone,
      {
        #[ inline ]
        fn clone_as_array( &self ) -> [ #param; 2 ]
        {
          [ self.0.clone(), self.1.clone() ]
        }
      }
    )
  }

  fn impl_from_value( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.gtypes;
    let param = &self.fields.0.ty;

    quote!
    (
      impl< #( #generics ),* > From< #param > for #struct_name< #( #gtypes ),* >
      where
        #param : Clone,
      {
        #[ inline ]
        fn from( src : #param ) -> Self
        {
          Self( src.clone(), src.clone() )
        }
      }
    )
  }

  fn impl_deref( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.gtypes;
    let param1 = &self.fields.0.ty;
    let param2 = &self.fields.1.ty;

    quote!
    (
      impl< #( #generics ),* > core::ops::Deref for #struct_name< #( #gtypes ),* >
      {
        type Target = ( #param1, #param2 );

        #[ inline ]
        fn deref( &self ) -> &Self::Target
        {
          #[ cfg( debug_assertions ) ]
          {
            let layout1 = core::alloc::Layout::new::< Self >();
            let layout2 = core::alloc::Layout::new::< Self::Target >();
            debug_assert_eq!( layout1, layout2 );
          }
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            core::mem::transmute::< _, _ >( self )
          }
        }
      }
    )
  }
  
  fn impl_deref_mut( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.gtypes;

    quote!
    (
      impl< #( #generics ),* > core::ops::DerefMut for #struct_name< #( #gtypes ),* >
      {
        #[ inline ]
        fn deref_mut( &mut self ) -> &mut Self::Target
        {
          #[ cfg( debug_assertions ) ]
          {
            let layout1 = core::alloc::Layout::new::< Self >();
            let layout2 = core::alloc::Layout::new::< Self::Target >();
            debug_assert_eq!( layout1, layout2 );
          }
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            core::mem::transmute::< _, _ >( self )
          }
        }
      }
    )
  }

  fn impl_make0( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.gtypes;
    let param1 = &self.fields.0.ty;
    let param2 = &self.fields.1.ty;

    quote!
    (
      impl< #( #generics ),* > Make0 for #struct_name< #( #gtypes ),* >
      where
        #param1 : Default,
        #param2 : Default,
      {
        #[ inline ]
        fn make_0() -> Self
        {
          Self( Default::default(), Default::default() )
        }
      }
    )
  }

  fn impl_make1( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.gtypes;
    let param = &self.fields.0.ty;

    quote!
    (
      impl< #( #generics ),* > Make1< #param > for #struct_name< #( #gtypes ),* >
      where
        #param : Clone
      {
        #[ inline ]
        fn make_1( val : #param ) -> Self
        {
          Self( val.clone(), val.clone() )
        }
      }
    )
  }

  fn impl_make2( &self ) -> proc_macro2::TokenStream
  {
    let struct_name = &self.ident;
    let generics = &self.generics;
    let gtypes = &self.gtypes;
    let param1 = &self.fields.0.ty;
    let param2 = &self.fields.1.ty;
    
    quote!
    (
      impl< #( #generics ),* > Make2< #param1, #param2 > for #struct_name< #( #gtypes ),* >
      {
        #[ inline ]
        fn make_2( _1 : #param1, _2 : #param2 ) -> Self
        {
          Self( _1, _2 )
        }
      }
    )
  }
}


#[ proc_macro_derive( Pair ) ]
pub fn derive_pair( input: proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let input = parse_macro_input!( input as syn::ItemStruct );
  let dp = DerivePair::parse( input );

  let mut impls =
  vec!
  [
    dp.impl_from_tuple(),
    dp.impl_to_tuple(),
    dp.impl_clone_as_tuple(),
    // dp.impl_make0(),
    dp.impl_make2(),
  ];

  // if two fields has the same types => it can be stored into array/slice/...
  let type1_as_string = &dp.fields.0.ty.clone().into_token_stream().to_string();
  let type2_as_string = &dp.fields.1.ty.clone().into_token_stream().to_string();
  if type1_as_string == type2_as_string
  {
    impls.push( dp.impl_make1() );
    impls.push( dp.impl_as_tuple() );
    impls.push( dp.impl_from_array() );
    impls.push( dp.impl_from_slice() );
    impls.push( dp.impl_to_array() );
    impls.push( dp.impl_as_slice() );
    impls.push( dp.impl_as_array() );
    impls.push( dp.impl_clone_as_array() );
    // impls.push( dp.impl_from_value() ); //! conflicts with impl_from_tuple
    impls.push( dp.impl_deref() );
    impls.push( dp.impl_deref_mut() );
  }
  let result = impls.iter().fold( quote!(), | mut result, i |
  {
    result = quote!( #result #i );
    result
  });

  proc_macro::TokenStream::from( result )
}
  