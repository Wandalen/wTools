
use super::*;

//

#[ test ]
fn assumptions()
{

  // let code : syn::ItemStruct = syn::parse_quote!
  // {
  //   pub struct Struct1Former
  //   <
  //     Definition = Struct1FormerDefinition< (), Struct1, former::ReturnPreformed >,
  //   >
  //   {}
  // };
  // tree_print!( code );

  // let mut a : syn::Generics = parse_quote!
  // {
  //   < 'a, T >
  // };
  // let mut b : syn::IntoGenericsArgs = parse_quote!
  // {
  //   < (), Struct1, former::ReturnPreformed >
  // };
  // let got = generic_params::merge( &a.into(), &b.into() );
  // // let got = definition_extra_generics;

  // let mut _got : syn::Generics = parse_quote!
  // {
  //   < Struct1, former::ReturnPreformed >
  // };

  // let mut _got : syn::Generics = parse_quote!
  // {
  //   < (), Struct1, former::ReturnPreformed >
  // };

}

//

#[ test ]
fn into_generics_args_empty_generics()
{
  use syn::{ Generics, AngleBracketedGenericArguments, token };
  use macro_tools::IntoGenericsArgs;
  use proc_macro2::Span;

  let generics = Generics::default();
  let exp = AngleBracketedGenericArguments
  {
    colon2_token: None,
    lt_token: token::Lt::default(),
    args: syn::punctuated::Punctuated::new(),
    gt_token: token::Gt::default(),
  };
  let got = generics.into_generics_args();
  assert_eq!( exp, got, "Failed into_generics_args_empty_generics: expected {:?}, got {:?}", exp, got );
}

//

// #[ test ]
// fn into_generics_args_single_type_parameter()
// {
//   use syn::{ Generics, GenericParam, TypeParam, AngleBracketedGenericArguments, GenericArgument, Type, TypePath, Ident };
//   use macro_tools::IntoGenericsArgs;
//
//   let generics = Generics
//   {
//     params : Some( GenericParam::Type( TypeParam { ident: Ident::new( "T", proc_macro2::Span::call_site() ) } ) ).into_iter().collect(),
//     ..Default::default()
//   };
//   let exp = AngleBracketedGenericArguments
//   {
//     args : vec![ GenericArgument::Type( Type::Path( TypePath { qself: None, path: Ident::new( "T", proc_macro2::Span::call_site() ).into() } ) ) ],
//     ..Default::default()
//   };
//   let got = generics.into_generics_args();
//   assert_eq!( exp, got, "Failed into_generics_args_single_type_parameter: expected {:?}, got {:?}", exp, got );
// }
//
// #[ test ]
// fn into_generics_args_single_lifetime_parameter()
// {
//   use syn::{ Generics, GenericParam, LifetimeDef, Lifetime, AngleBracketedGenericArguments, GenericArgument };
//   use macro_tools::IntoGenericsArgs;
//
//   let generics = Generics
//   {
//     params: Some( GenericParam::Lifetime( LifetimeDef { lifetime: Lifetime::new( "'a", proc_macro2::Span::call_site() ) } ) ).into_iter().collect(),
//     ..Default::default()
//   };
//   let exp = AngleBracketedGenericArguments
//   {
//     args: vec![ GenericArgument::Lifetime( Lifetime::new( "'a", proc_macro2::Span::call_site() ) ) ],
//     ..Default::default()
//   };
//   let got = generics.into_generics_args();
//   assert_eq!( exp, got, "Failed into_generics_args_single_lifetime_parameter: expected {:?}, got {:?}", exp, got );
// }
//
// #[ test ]
// fn into_generics_args_single_const_parameter()
// {
//   use syn::{ Generics, GenericParam, ConstParam, AngleBracketedGenericArguments, GenericArgument, Expr, ExprPath, Ident };
//   use macro_tools::IntoGenericsArgs;
//
//   let generics = Generics
//   {
//     params: Some( GenericParam::Const( ConstParam { ident: Ident::new( "N", proc_macro2::Span::call_site() ) } ) ).into_iter().collect(),
//     ..Default::default()
//   };
//   let exp = AngleBracketedGenericArguments
//   {
//     args: vec![ GenericArgument::Const( Expr::Path( ExprPath { attrs: vec![], qself: None, path: Ident::new( "N", proc_macro2::Span::call_site() ).into() } ) ) ],
//     ..Default::default()
//   };
//   let got = generics.into_generics_args();
//   assert_eq!( exp, got, "Failed into_generics_args_single_const_parameter: expected {:?}, got {:?}", exp, got );
// }
//
// #[ test ]
// fn into_generics_args_mixed_parameters()
// {
//   use syn::{ Generics, GenericParam, TypeParam, LifetimeDef, Lifetime, ConstParam, AngleBracketedGenericArguments, GenericArgument, Type, TypePath, Expr, ExprPath, Ident };
//   use macro_tools::IntoGenericsArgs;
//
//   let generics = Generics
//   {
//     params : vec!
//     [
//       GenericParam::Type( TypeParam { ident: Ident::new( "T", proc_macro2::Span::call_site() ) } ),
//       GenericParam::Lifetime( LifetimeDef { lifetime: Lifetime::new( "'a", proc_macro2::Span::call_site() ) } ),
//       GenericParam::Const( ConstParam { ident: Ident::new( "N", proc_macro2::Span::call_site() ) } )
//     ].into_iter().collect(),
//     ..Default::default()
//   };
//   let exp = AngleBracketedGenericArguments
//   {
//     args : vec!
//     [
//       GenericArgument::Type( Type::Path( TypePath { qself: None, path: Ident::new( "T", proc_macro2::Span::call_site() ).into() } ) ),
//       GenericArgument::Lifetime( Lifetime::new( "'a", proc_macro2::Span::call_site() ) ),
//       GenericArgument::Const( Expr::Path( ExprPath { attrs: vec![], qself: None, path: Ident::new( "N", proc_macro2::Span::call_site() ).into() } ) )
//     ],
//     ..Default::default()
//   };
//   let got = generics.into_generics_args();
//   assert_eq!( exp, got, "Failed into_generics_args_mixed_parameters: expected {:?}, got {:?}", exp, got );
// }
