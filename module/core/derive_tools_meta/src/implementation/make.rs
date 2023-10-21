
use super::*;
// use iter_tools::{ Itertools, process_results };
use iter::{ process_results, Itertools };

/// Extension of iterator

pub trait IterExt
{
  // fn map_result_unzip< F, R, Item, T1, T2, T3 >( &self, f : F ) -> ( T1, T2 ,T3 )
  // where
  //   Self: Sized,
  //   F : FnMut( Item ) -> R
  // ;
}

impl< Iterator > IterExt for Iterator
where
  Iterator : core::iter::Iterator,
{
  // type Item = It;
//   fn map_result_unzip< F, R, Item, T1, T2, T3 >( &self, f : F ) -> ( T1, T2 ,T3 )
//   where
//     Self: Sized,
//     F : FnMut( Item ) -> R,
//     // F : FnMut( < Self as core::iter::Iterator >::Item ) -> R,
//   {
//     let vars_maybe = self.map( f );
//     let vars : Vec< _ > = process_results( vars_maybe, | iter | iter.collect() )?;
//
//     let result : ( Vec< _ >, Vec< _ >, Vec< _ > )
//     = vars.into_iter().multiunzip();
//     result
//   }
}

//

pub fn make( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let parsed = syn::parse::< InputParsed >( input )?;
  let fields = &parsed.fields;
  let item_name = parsed.item_name;

  let vars_maybe = fields.iter().map( | field |
  {
    let ident = field.ident.clone().ok_or_else( || syn_err!( parsed.item.span(), "Fields should be named" ) )?;
    Result::Ok
    ((
      qt!{ #ident = core::default::Default::default(); },
      qt!{ let #ident = src.into(); },
      qt!{ #ident, },
    ))
  });
  let vars : Vec< _ > = process_results( vars_maybe, | iter | iter.collect() )?;

  let
  (
    vars_assing_default,
    vars_from_src,
    vars
  ) : ( Vec< _ >, Vec< _ >, Vec< _ > )
  = vars.into_iter().multiunzip();

  let result = qt!
  {
    #[ automatically_derived ]
    impl wtools::Make0 for #item_name
    {
      fn make_0() -> Self
      {
        #( #vars_assing_default )*
        // let a = Default::default();
        // let b = Default::default();
        // let c = Default::default();
        // let d = Default::default();
        Self
        {
          #( #vars )*
          // a,
          // b,
          // c,
          // d,
        }
      }
    }

    #[ automatically_derived ]
    impl wtools::Make1< i32 > for #item_name
    {
      fn make_1( src : i32 ) -> Self
      {
        let a = src.into();
        let b = src.into();
        let c = src.into();
        let d = src.into();
        Self
        {
          a,
          b,
          c,
          d,
        }
      }
    }

//     #[ automatically_derived ]
//     impl wtools::Make2< i32, i32 > for #item_name
//     {
//       fn make_2( a : i32, b : i32 ) -> Self { Self{ a, b, c : b, d : b } }
//     }
//
//     #[ automatically_derived ]
//     impl wtools::Make3< i32, i32, i32 > for #item_name
//     {
//       fn make_3( a : i32, b : i32, c : i32 ) -> Self { Self{ a, b, c, d : c } }
//     }

  };

  Ok( result )
}

// use proc_macro2::{ TokenStream, Ident };
// use quote::{ quote, ToTokens };
// use syn::{ /*parse_macro_input,*/ Fields, ItemStruct };
//
// // * Change this if more traits already defined
// const MAX_MAKE_TRAIT_NUMBER : usize = 4;
//
//
// pub struct DeriveMake
// {
//   struct_name : Ident,
//   is_named : bool,
//   types : Vec< TokenStream >,
//   names : Vec< Option< TokenStream > >,
// }
//
// impl DeriveMake
// {
//   fn parse_fields( &mut self, fields : Fields )
//   {
//     match fields
//     {
//       Fields::Named( named ) =>
//       {
//         self.is_named = true;
//         let fields = &named.named;
//         fields.iter().for_each( | field |
//         {
//           self.names.push( Some( field.ident.as_ref().unwrap().to_token_stream() ) );
//           self.types.push( field.ty.clone().into_token_stream() );
//         })
//       },
//       Fields::Unnamed( unnamed ) =>
//       {
//         self.is_named = false;
//         let fields = &unnamed.unnamed;
//         fields.iter().for_each( | field |
//         {
//           self.names.push( None );
//           self.types.push( field.into_token_stream() );
//         })
//       },
//       _ => panic!( "Can'not implement \"Make\" for struct without fields" )
//     };
//   }
//
//   pub( crate ) fn parse( input : ItemStruct ) -> Self
//   {
//     let mut obj = Self
//     {
//       struct_name : input.ident,
//       is_named : false,
//       types : Vec::new(),
//       names : Vec::new()
//     };
//
//     obj.parse_fields( input.fields );
//
//     obj
//   }
//
//   fn impl_make0( &self ) -> TokenStream
//   {
//     let types = &self.types;
//     let struct_name = &self.struct_name;
//     let creation = if self.is_named
//     {
//       let names = &self.names;
//       quote!( Self{ #( #names : #types::default() ),* } )
//     }
//     else
//     {
//       quote!( Self( #( #types::default() ),* ) )
//     };
//     quote!
//     (
//       impl Make0 for #struct_name
//       {
//         fn make_0() -> Self
//         {
//           #creation
//         }
//       }
//     )
//   }
//
//   fn impl_make_n( &self, n : usize ) -> TokenStream
//   {
//     // ? If all defined traits already implemented - skips implementation next one
//     if n > MAX_MAKE_TRAIT_NUMBER { return quote!() }
//
//     let trait_name = format!( "{}{}", quote!( Make ), n ).parse::< TokenStream >().unwrap();
//     let trait_fn_name = format!( "{}{}", quote!( make_ ), n ).parse::< TokenStream >().unwrap();
//     let types = &self.types;
//     let struct_name = &self.struct_name;
//     let generic_type = types[ 0 .. n ].to_owned();
//     let mut vals = Vec::< TokenStream >::with_capacity( n );
//     let mut i = 1;
//     for _ in 0 .. types.len()
//     {
//       vals.push( format!( "{}{}", quote!( val_ ), i ).parse().unwrap() );
//       if n > i
//       {
//         i += 1
//       }
//     }
//
//     // make constructor
//     let creation = if self.is_named
//     {
//       let names = &self.names;
//       quote!( Self{ #( #names : #vals as #types ),* } )
//     }
//     else
//     {
//       quote!( Self( #( #vals as #types ),* ) )
//     };
//     // make implementation
//     quote!
//     (
//       impl #trait_name< #( #generic_type ),* > for #struct_name
//       {
//         fn #trait_fn_name( #( #vals : #generic_type ),* ) -> Self
//         {
//           #creation
//         }
//       }
//     )
//   }
//
//   // implements make for all defined "Make" traits
//   pub( crate ) fn impl_makes( &self ) -> TokenStream
//   {
//     let mut result = self.impl_make0();
//     for i in 1 .. self.types.len() + 1
//     {
//       let implementation = self.impl_make_n( i );
//       result = quote!
//       (
//         #result
//         #implementation
//       )
//     }
//     result
//   }
// }

// #[ proc_macro_derive( Make ) ]
// pub fn derive_make( input: proc_macro::TokenStream ) -> proc_macro::TokenStream
// {
//   let input = parse_macro_input!( input as syn::ItemStruct );
//   let dm = DeriveMake::parse( input );
//
//   proc_macro::TokenStream::from( dm.impl_makes() )
// }

// pub fn make( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
// {
//   let parsed = syn::parse::< ItemStruct >( input )?;
//
// //   let field_type = parsed.first_field_type()?;
// //   let item_name = parsed.item_name;
// //
// //   let result = qt!
// //   {
// //     impl AsMut< #field_type > for #item_name
// //     {
// //       fn as_mut( &mut self ) -> &mut #field_type
// //       {
// //         &mut self.0
// //       }
// //     }
// //   };
//   let dm = DeriveMake::parse( parsed );
//   Ok( dm.impl_makes() )
//   // Ok( parsed.impl_makes() )
// }
