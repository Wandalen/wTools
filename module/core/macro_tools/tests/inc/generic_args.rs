//
// use super::*;
//
// //
//
// #[ test ]
// fn assumptions()
// {
//
//   // let code : syn::ItemStruct = syn::parse_quote!
//   // {
//   //   pub struct Struct1Former
//   //   <
//   //     Definition = Struct1FormerDefinition< (), Struct1, former::ReturnPreformed >,
//   //   >
//   //   {}
//   // };
//   // tree_print!( code );
//
//   // let mut a : syn::Generics = parse_quote!
//   // {
//   //   < 'a, T >
//   // };
//   // let mut b : syn::IntoGenericsArgs = parse_quote!
//   // {
//   //   < (), Struct1, former::ReturnPreformed >
//   // };
//   // let got = generics::merge( &a.into(), &b.into() );
//   // // let got = definition_extra_generics;
//
//   // let mut _got : syn::Generics = parse_quote!
//   // {
//   //   < Struct1, former::ReturnPreformed >
//   // };
//
//   // let mut _got : syn::Generics = parse_quote!
//   // {
//   //   < (), Struct1, former::ReturnPreformed >
//   // };
//
// }
//
// //
//
// #[ test ]
// fn into_generics_args_empty_generics()
// {
//   use syn::{ Generics, IntoGenericsArgs };
//
//   let generics = Generics::default();
//   let exp = IntoGenericsArgs::default();
//   let got = generics.into_generics_args();
//   assert_eq!( exp, got, "Failed into_generics_args_empty_generics: expected {:?}, got {:?}", exp, got );
// }
//
// //
//
// #[ test ]
// fn into_generics_args_single_type_parameter()
// {
//   use syn::{ Generics, GenericParam, IntoGenericsArgs, GenericArgument, Type, TypePath, Ident };
//
//   let generics = Generics
//   {
//     params: vec![ GenericParam::Type( syn::TypeParam { ident: Ident::new( "T", proc_macro2::Span::call_site() ), ..Default::default() } ) ].into(),
//     ..Default::default()
//   };
//   let exp = IntoGenericsArgs
//   {
//     args: vec![ GenericArgument::Type( Type::Path( TypePath { qself: None, path: "T".into() } ) ) ],
//     ..Default::default()
//   };
//   let got = generics.into_generics_args();
//   assert_eq!( exp, got, "Failed into_generics_args_single_type_parameter: expected {:?}, got {:?}", exp, got );
// }
//
// //
//
// #[ test ]
// fn into_generics_args_single_lifetime_parameter()
// {
//   use syn::{ Generics, GenericParam, IntoGenericsArgs, GenericArgument, Lifetime };
//
//   let generics = Generics
//   {
//     params: vec![ GenericParam::Lifetime( syn::LifetimeDef { lifetime: Lifetime::new( "'a", proc_macro2::Span::call_site() ), ..Default::default() } ) ].into(),
//     ..Default::default()
//   };
//   let exp = IntoGenericsArgs
//   {
//     args: vec![ GenericArgument::Lifetime( Lifetime::new( "'a", proc_macro2::Span::call_site() ) ) ],
//     ..Default::default()
//   };
//   let got = generics.into_generics_args();
//   assert_eq!( exp, got, "Failed into_generics_args_single_lifetime_parameter: expected {:?}, got {:?}", exp, got );
// }
//
// //
//
// #[ test ]
// fn into_generics_args_single_const_parameter()
// {
//   use syn::{ Generics, GenericParam, IntoGenericsArgs, GenericArgument, Expr, ExprPath, Ident };
//
//   let generics = Generics
//   {
//     params: vec![ GenericParam::Const( syn::ConstParam { ident: Ident::new( "N", proc_macro2::Span::call_site() ), ..Default::default() } ) ].into(),
//     ..Default::default()
//   };
//   let exp = IntoGenericsArgs
//   {
//     args: vec![ GenericArgument::Const( Expr::Path( ExprPath { attrs: vec![], qself: None, path: "N".into() } ) ) ],
//     ..Default::default()
//   };
//   let got = generics.into_generics_args();
//   assert_eq!( exp, got, "Failed into_generics_args_single_const_parameter: expected {:?}, got {:?}", exp, got );
// }
//
// //
//
// #[ test ]
// fn into_generics_args_mixed_parameters()
// {
//   use syn::{ Generics, GenericParam, IntoGenericsArgs, GenericArgument, Type, TypePath, Lifetime, Expr, ExprPath, Ident };
//
//   let generics = Generics
//   {
//     params : vec!
//     [
//       GenericParam::Type( syn::TypeParam { ident: Ident::new( "T", proc_macro2::Span::call_site() ), ..Default::default() } ),
//       GenericParam::Lifetime( syn::LifetimeDef { lifetime: Lifetime::new( "'a", proc_macro2::Span::call_site() ), ..Default::default() } ),
//       GenericParam::Const( syn::ConstParam { ident: Ident::new( "N", proc_macro2::Span::call_site() ), ..Default::default() } )
//     ].into(),
//     ..Default::default()
//   };
//   let exp = IntoGenericsArgs
//   {
//     args : vec!
//     [
//       GenericArgument::Type( Type::Path( TypePath { qself: None, path: "T".into() } ) ),
//       GenericArgument::Lifetime( Lifetime::new( "'a", proc_macro2::Span::call_site() ) ),
//       GenericArgument::Const( Expr::Path( ExprPath { attrs: vec![], qself: None, path: "N".into() } ) )
//     ],
//     ..Default::default()
//   };
//   let got = generics.into_generics_args();
//   assert_eq!( exp, got, "Failed into_generics_args_mixed_parameters: expected {:?}, got {:?}", exp, got );
// }
