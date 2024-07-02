
use super::*;
use the_module::parse_quote;

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
  // let got = generic_params::generic_args::merge( &a.into(), &b.into() );
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
  a_id!( exp, got, "Failed into_generic_args_empty_generics: exp {:?}, got {:?}", exp, got );
}

//
#[ test ]
fn into_generic_args_single_type_parameter()
{
  use syn::
  {
    Generics,
    AngleBracketedGenericArguments,
    parse_quote
  };
  use macro_tools::IntoGenericArgs;

  // Generate the generics with a single type parameter using parse_quote
  let generics : Generics = parse_quote!
  {
    < T >
  };

  // Create the exp AngleBracketedGenericArguments using parse_quote
  let exp : AngleBracketedGenericArguments = parse_quote!
  {
    < T >
  };

  let got = generics.into_generic_args();
  a_id!( exp, got, "Failed into_generic_args_single_type_parameter: exp {:?}, got {:?}", exp, got );
}

///

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

  // Create the exp AngleBracketedGenericArguments using parse_quote
  let exp : AngleBracketedGenericArguments = parse_quote!
  {
    < 'a >
  };

  // Use the implementation to generate the actual output
  let got = generics.into_generic_args();

  // Debug prints for better traceability in case of failure
  println!( "Expected: {:?}", exp );
  println!( "Got: {:?}", got );

  // Assert to check if the exp matches the got
  a_id!( exp, got, "Failed into_generic_args_single_lifetime_parameter: exp {:?}, got {:?}", exp, got );
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

  // Manually construct the exp value
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

  a_id!( exp, got, "Failed into_generic_args_single_const_parameter: exp {:?}, got {:?}", exp, got );
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

  // Manually construct the exp value
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
  a_id!( exp, got, "Failed into_generic_args_mixed_parameters: exp {:?}, got {:?}", exp, got );
}

// = generic_args::merge

#[ test ]
fn merge_empty_arguments()
{
  use syn::AngleBracketedGenericArguments;
  use macro_tools::generic_args;

  let a : AngleBracketedGenericArguments = parse_quote! { <> };
  let b : AngleBracketedGenericArguments = parse_quote! { <> };
  let exp : AngleBracketedGenericArguments = parse_quote! { <> };

  let got = generic_args::merge( &a, &b );
  a_id!( got, exp, "Merging two empty arguments should got in empty arguments" );
}

//

#[ test ]
fn merge_one_empty_one_non_empty()
{
  use syn::AngleBracketedGenericArguments;
  use macro_tools::generic_args;

  let a : AngleBracketedGenericArguments = parse_quote! { < T, U > };
  let b : AngleBracketedGenericArguments = parse_quote! { <> };
  let exp : AngleBracketedGenericArguments = parse_quote! { < T, U > };

  let got = generic_args::merge( &a, &b );
  a_id!( got, exp, "Merging non-empty with empty should got in the non-empty" );
}

//

#[ test ]
fn merge_duplicate_arguments()
{
  use syn::AngleBracketedGenericArguments;
  use macro_tools::generic_args;

  let a : AngleBracketedGenericArguments = parse_quote! { < T > };
  let b : AngleBracketedGenericArguments = parse_quote! { < T > };
  let exp : AngleBracketedGenericArguments = parse_quote! { < T, T > };

  let got = generic_args::merge( &a, &b );
  a_id!( got, exp, "Duplicates should be preserved in the output" );
}

//

#[ test ]
fn merge_large_number_of_arguments()
{
  use syn::AngleBracketedGenericArguments;
  use macro_tools::generic_args;

  let a : AngleBracketedGenericArguments = parse_quote! { <A, B, C, D, E, F, G, H, I, J> };
  let b : AngleBracketedGenericArguments = parse_quote! { <K, L, M, N, O, P, Q, R, S, T> };
  let exp : AngleBracketedGenericArguments = parse_quote! { <A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T> };

  let got = generic_args::merge( &a, &b );
  a_id!( got, exp, "Merging large number of arguments should succeed without altering order or count" );
}

//

#[ test ]
fn merge_complex_generic_constraints()
{
  use syn::AngleBracketedGenericArguments;
  use macro_tools::generic_args;

  let a : AngleBracketedGenericArguments = parse_quote! { < T : Clone + Send, U: Default > };
  let b : AngleBracketedGenericArguments = parse_quote! { < V : core::fmt::Debug + Sync > };
  let exp : AngleBracketedGenericArguments = parse_quote! { < T: Clone + Send, U: Default, V: core::fmt::Debug + Sync > };

  let got = generic_args::merge( &a, &b );
  a_id!( got, exp, "Complex constraints should be merged correctly" );
}

//

#[ test ]
fn merge_different_orders_of_arguments()
{
  use syn::AngleBracketedGenericArguments;
  use macro_tools::generic_args;

  let a : AngleBracketedGenericArguments = parse_quote! { < T, U > };
  let b : AngleBracketedGenericArguments = parse_quote! { < V, W > };
  let exp : AngleBracketedGenericArguments = parse_quote! { < T, U, V, W > };

  let got = generic_args::merge( &a, &b );
  a_id!( got, exp, "Order of arguments should be preserved as per the inputs" );
}

//

#[ test ]
fn merge_interaction_with_lifetimes_and_constants()
{
  use syn::AngleBracketedGenericArguments;
  use macro_tools::generic_args;

  let a : AngleBracketedGenericArguments = parse_quote! { < 'a, M : T > };
  let b : AngleBracketedGenericArguments = parse_quote! { < 'b, N > };
  let exp : AngleBracketedGenericArguments = parse_quote! { <'a, 'b, M : T, N > };

  let got = generic_args::merge( &a, &b );
  // a_id!(tree_diagnostics_str!( exp ), tree_diagnostics_str!( got ) );
  a_id!( got, exp, "Lifetimes and constants should be interleaved correctly" );

}
