
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
  // let mut b : syn::IntoGenericArgs = parse_quote!
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
fn into_generic_args_empty_generics()
{
  use syn::{ Generics, AngleBracketedGenericArguments, token };
  use macro_tools::IntoGenericArgs;
  use proc_macro2::Span;

  let generics = Generics::default();
  let got = generics.into_generic_args();
  let exp = AngleBracketedGenericArguments
  {
    colon2_token: None,
    lt_token: token::Lt::default(),
    args: syn::punctuated::Punctuated::new(),
    gt_token: token::Gt::default(),
  };
  assert_eq!( exp, got, "Failed into_generic_args_empty_generics: expected {:?}, got {:?}", exp, got );
}

//

#[ test ]
fn into_generic_args_single_type_parameter()
{
  use macro_tools::IntoGenericArgs;
  use syn::
  {
    Generics,
    GenericParam,
    TypeParam,
    AngleBracketedGenericArguments,
    GenericArgument,
    Type,
    TypePath,
    Ident,
    token,
    punctuated::Punctuated,
  };

  let mut args = Punctuated::new();
  args.push(GenericArgument::Type(Type::Path(TypePath {
    qself: None,
    path: Ident::new("T", proc_macro2::Span::call_site()).into(),
  })));

  let generics = Generics
  {
    params: Some(GenericParam::Type(TypeParam {
      attrs: Vec::new(),
      ident: Ident::new("T", proc_macro2::Span::call_site()),
      colon_token: None,
      bounds: Punctuated::new(),
      eq_token: None,
      default: None,
    })).into_iter().collect(),
    ..Default::default()
  };

  let exp = AngleBracketedGenericArguments
  {
    colon2_token: None,
    lt_token: token::Lt::default(),
    args,
    gt_token: token::Gt::default(),
  };

  let got = generics.into_generic_args();
  assert_eq!(exp, got, "Failed into_generic_args_single_type_parameter: expected {:?}, got {:?}", exp, got);
}

#[ test ]
fn into_generic_args_single_lifetime_parameter()
{
  use syn::
  {
    Generics,
    AngleBracketedGenericArguments,
    GenericArgument,
    parse_quote,
    punctuated::Punctuated
  };
  use macro_tools::IntoGenericArgs;

  // Generate the generics using parse_quote to include a lifetime parameter
  let generics : Generics = parse_quote!
  {
    < 'a >
  };

  // Create the expected AngleBracketedGenericArguments using parse_quote
  let exp : AngleBracketedGenericArguments = parse_quote!
  {
    < 'a >
  };

  // Use the implementation to generate the actual output
  let got = generics.into_generic_args();

  // Debug prints for better traceability in case of failure
  println!( "Expected: {:?}", exp );
  println!( "Got: {:?}", got );

  // Assert to check if the expected matches the got
  assert_eq!( exp, got, "Failed into_generic_args_single_lifetime_parameter: expected {:?}, got {:?}", exp, got );
}

#[ test ]
fn into_generic_args_single_const_parameter()
{
  use syn::
  {
    Generics,
    AngleBracketedGenericArguments,
    GenericArgument,
    Expr,
    ExprPath,
    Ident,
    token::{ self, Lt, Gt },
    punctuated::Punctuated
  };
  use macro_tools::IntoGenericArgs;

  // Use parse_quote to create the generic parameters
  let generics : Generics = parse_quote!
  {
    < const N: usize >
  };

  let got = generics.into_generic_args();

  // Manually construct the expected value
  let mut args = Punctuated::new();
  args.push_value( GenericArgument::Const( Expr::Path( ExprPath
  {
    attrs: vec![],
    qself: None,
    path: syn::Path::from( Ident::new( "N", proc_macro2::Span::call_site() )),
  })));

  let exp = AngleBracketedGenericArguments
  {
    colon2_token: None,
    lt_token: Lt::default(),
    args,
    gt_token: Gt::default(),
  };

  // Debug prints for better traceability in case of failure
  println!( "Expected: {:?}", exp );
  println!( "Got: {:?}", got );

  assert_eq!( exp, got, "Failed into_generic_args_single_const_parameter: expected {:?}, got {:?}", exp, got );
}


//

#[ test ]
fn into_generic_args_mixed_parameters()
{
  use syn::
  {
    Generics,
    AngleBracketedGenericArguments,
    GenericArgument,
    Type,
    TypePath,
    Expr,
    ExprPath,
    Ident,
    Lifetime,
    token::{ self, Comma },
    punctuated::Punctuated,
    parse_quote
  };
  use macro_tools::IntoGenericArgs;

  // Generate the actual value using the implementation
  let generics : Generics = parse_quote!
  {
    <T, 'a, const N: usize>
  };
  let got = generics.into_generic_args();

  // Manually construct the expected value
  let mut args = Punctuated::new();
  let t_type : GenericArgument = GenericArgument::Type( Type::Path( TypePath
  {
    qself: None,
    path: Ident::new( "T", proc_macro2::Span::call_site() ).into(),
  }));
  args.push_value( t_type );
  args.push_punct( Comma::default() );

  let a_lifetime = GenericArgument::Lifetime( Lifetime::new( "'a", proc_macro2::Span::call_site() ));
  args.push_value( a_lifetime );
  args.push_punct( Comma::default() );

  let n_const : GenericArgument = GenericArgument::Const( Expr::Path( ExprPath
  {
    attrs: vec![],
    qself: None,
    path: Ident::new( "N", proc_macro2::Span::call_site() ).into(),
  }));
  args.push_value( n_const );

  let exp = AngleBracketedGenericArguments
  {
    colon2_token: None,
    lt_token: token::Lt::default(),
    args,
    gt_token: token::Gt::default(),
  };

  // tree_print!( got );
  // tree_print!( exp );
  // a_id!(tree_diagnostics_str!( exp ), tree_diagnostics_str!( got ) );
  a_id!( exp, got, "Failed into_generic_args_mixed_parameters: expected {:?}, got {:?}", exp, got );
}
