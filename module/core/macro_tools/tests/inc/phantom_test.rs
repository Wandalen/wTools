
use super::*;
use the_module::{ tree_print };

#[ test ]
fn phantom_add_basic()
{

  let item : syn::ItemStruct = syn::parse_quote!
  {
    pub struct Struct1< 'a, Context, Formed >
    {
      f1 : int32,
    }
  };

  let exp : syn::ItemStruct = syn::parse_quote!
  {
    pub struct Struct1< 'a, Context, Formed >
    {
      f1 : int32,
      _phantom : ::core::marker::PhantomData< ( &'a(), *const Context, *const Formed ) >,
    }
  };

  let got = the_module::phantom::add_to_item( &item );
  // a_id!( tree_print!( got ), tree_print!( exp ) );
  a_id!( got, exp );

}

//

#[ test ]
fn phantom_add_no_generics()
{
  use syn::parse_quote;
  use quote::ToTokens;

  let input : syn::ItemStruct = parse_quote! { struct TestStruct {} };
  let got = the_module::phantom::add_to_item( &input );

  let exp : syn::ItemStruct = parse_quote!
  {
    struct TestStruct
    {
    }
  };

  assert_eq!( got.to_token_stream().to_string(), exp.to_token_stream().to_string() );
}

//

#[ test ]
fn phantom_add_type_generics()
{
  use syn::parse_quote;
  use quote::ToTokens;

  let input : syn::ItemStruct = parse_quote! { struct TestStruct< T, U > {} };
  let got = the_module::phantom::add_to_item( &input );

  let exp : syn::ItemStruct = parse_quote!
  {
    struct TestStruct< T, U >
    {
      _phantom : ::core::marker::PhantomData< ( *const T, *const U ) >,
    }
  };

  assert_eq!( got.to_token_stream().to_string(), exp.to_token_stream().to_string() );
}

//

#[ test ]
fn phantom_add_lifetime_generics()
{
  use syn::parse_quote;
  use quote::ToTokens;

  let input : syn::ItemStruct = parse_quote! { struct TestStruct< 'a, 'b > {} };
  let got = the_module::phantom::add_to_item( &input );

  let exp : syn::ItemStruct = parse_quote!
  {
    struct TestStruct< 'a, 'b >
    {
      _phantom : ::core::marker::PhantomData< ( &'a (), &'b () ) >,
    }
  };

  assert_eq!( got.to_token_stream().to_string(), exp.to_token_stream().to_string() );
}

//

#[ test ]
fn phantom_add_const_generics()
{
  use syn::parse_quote;
  use quote::ToTokens;

  let input : syn::ItemStruct = parse_quote! { struct TestStruct< const N : usize > {} };
  let got = the_module::phantom::add_to_item( &input );

  let exp : syn::ItemStruct = parse_quote!
  {
    struct TestStruct< const N : usize >
    {
      _phantom : ::core::marker::PhantomData< ( N, ) >,
    }
  };

  assert_eq!( got.to_token_stream().to_string(), exp.to_token_stream().to_string() );
}

//

#[ test ]
fn phantom_add_mixed_generics()
{
  use syn::parse_quote;
  use quote::ToTokens;

  let input : syn::ItemStruct = parse_quote! { struct TestStruct< T, 'a, const N : usize > {} };
  let got = the_module::phantom::add_to_item( &input );

  let exp : syn::ItemStruct = parse_quote!
  {
    struct TestStruct< T, 'a, const N : usize >
    {
      _phantom : ::core::marker::PhantomData< ( *const T, &'a (), N ) >,
    }
  };

  assert_eq!( got.to_token_stream().to_string(), exp.to_token_stream().to_string() );
}

//

#[ test ]
fn phantom_add_named_fields()
{
  use syn::parse_quote;
  use quote::ToTokens;

  let input : syn::ItemStruct = parse_quote! { struct TestStruct { field1 : i32, field2 : f64 } };
  let got = the_module::phantom::add_to_item( &input );

  let exp : syn::ItemStruct = parse_quote!
  {
    struct TestStruct
    {
      field1 : i32,
      field2 : f64,
    }
  };

  assert_eq!( got.to_token_stream().to_string(), exp.to_token_stream().to_string() );
}

//

#[ test ]
fn phantom_add_unnamed_fields()
{
  use syn::parse_quote;
  use quote::ToTokens;

  let input : syn::ItemStruct = parse_quote! { struct TestStruct( i32, f64 ); };
  let got = the_module::phantom::add_to_item( &input );
  let exp : syn::ItemStruct = parse_quote! { struct TestStruct( i32, f64, ); };

  assert_eq!( got.to_token_stream().to_string(), exp.to_token_stream().to_string() );
}

//

#[ test ]
fn phantom_add_unnamed_fields_with_generics()
{
  use syn::parse_quote;
  use quote::ToTokens;

  let input : syn::ItemStruct = parse_quote! { struct TestStruct< T, U >( T, U ); };
  let got = the_module::phantom::add_to_item( &input );

  let exp : syn::ItemStruct = parse_quote!
  {
    struct TestStruct< T, U >
    (
      T, U,
      ::core::marker::PhantomData< ( *const T, *const U ) >,
    );
  };

  assert_eq!( got.to_token_stream().to_string(), exp.to_token_stream().to_string() );
}

//

#[ test ]
fn phantom_add_unnamed_fields_lifetime_generics()
{
  use syn::parse_quote;
  use quote::ToTokens;

  let input : syn::ItemStruct = parse_quote! { struct TestStruct< 'a, 'b >( &'a i32, &'b f64 ); };
  let got = the_module::phantom::add_to_item( &input );

  let exp : syn::ItemStruct = parse_quote!
  {
    struct TestStruct< 'a, 'b >
    (
      &'a i32,
      &'b f64,
      ::core::marker::PhantomData< ( &'a (), &'b () ) >,
    );
  };

  assert_eq!( got.to_token_stream().to_string(), exp.to_token_stream().to_string() );
}

//

#[ test ]
fn phantom_add_unnamed_fields_const_generics()
{
  use syn::parse_quote;
  use quote::ToTokens;

  let input : syn::ItemStruct = parse_quote! { struct TestStruct< const N : usize >( [ i32 ; N ] ); };
  let got = the_module::phantom::add_to_item( &input );

  let exp : syn::ItemStruct = parse_quote!
  {
    struct TestStruct< const N : usize >
    (
      [ i32 ; N ],
      ::core::marker::PhantomData< ( N, ) >,
    );
  };

  assert_eq!( got.to_token_stream().to_string(), exp.to_token_stream().to_string() );
}

//

//
#[ test ]
fn phantom_tuple_empty_generics()
{
  use syn::{ punctuated::Punctuated, GenericParam, token::Comma, parse_quote };
  use macro_tools::phantom::tuple;

  let input : Punctuated< GenericParam, Comma > = Punctuated::new();
  let result = tuple( &input );

  let exp : syn::Type = parse_quote! { ::core::marker::PhantomData<()> };
  let got = result;

  assert_eq!( format!( "{:?}", exp ), format!( "{:?}", got ), "Expected empty PhantomData, got: {:?}", got );
}

//

#[ test ]
fn phantom_tuple_only_type_parameters()
{
  use syn::{ parse_quote, punctuated::Punctuated, GenericParam, token::Comma };
  use macro_tools::phantom::tuple;

  let input : Punctuated< GenericParam, Comma > = parse_quote! { T, U };
  let result = tuple( &input );

  let exp : syn::Type = parse_quote! { ::core::marker::PhantomData< ( *const T, *const U ) > };
  let got = result;

  assert_eq!( format!( "{:?}", exp ), format!( "{:?}", got ), "Expected PhantomData with type parameters, got: {:?}", got );
}

//

#[ test ]
fn phantom_tuple_mixed_generics()
{
  use syn::{ parse_quote, punctuated::Punctuated, GenericParam, token::Comma };
  use macro_tools::phantom::tuple;

  let input : Punctuated< GenericParam, Comma > = parse_quote! { T, 'a, const N: usize };
  let result = tuple( &input );

  let exp : syn::Type = parse_quote! { ::core::marker::PhantomData< ( *const T, &'a (), N ) > };
  let got = result;

  assert_eq!( format!( "{:?}", exp ), format!( "{:?}", got ), "Expected PhantomData with mixed generics, got: {:?}", got );
}
