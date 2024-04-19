
use super::*;

//

#[ test ]
fn basic()
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
      _phantom : core::marker::PhantomData< ( &'a(), Context, Formed ) >,
    }
  };

  let got = the_module::item::phantom_add( &item );
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
  let got = the_module::item::phantom_add( &input );

  let exp : syn::ItemStruct = parse_quote!
  {
    struct TestStruct
    {
      _phantom : core::marker::PhantomData<()>,
      // xxx : ?
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
  let got = the_module::item::phantom_add( &input );

  let exp : syn::ItemStruct = parse_quote!
  {
    struct TestStruct< T, U >
    {
      _phantom : core::marker::PhantomData< ( T, U ) >,
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
  let got = the_module::item::phantom_add( &input );

  let exp : syn::ItemStruct = parse_quote!
  {
    struct TestStruct< 'a, 'b >
    {
      _phantom : core::marker::PhantomData< ( &'a (), &'b () ) >,
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
  let got = the_module::item::phantom_add( &input );

  let exp : syn::ItemStruct = parse_quote!
  {
    struct TestStruct< const N : usize >
    {
      _phantom : core::marker::PhantomData< ( N, ) >,
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
  let got = the_module::item::phantom_add( &input );

  let exp : syn::ItemStruct = parse_quote!
  {
    struct TestStruct< T, 'a, const N : usize >
    {
      _phantom : core::marker::PhantomData< ( T, &'a (), N ) >,
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
  let got = the_module::item::phantom_add( &input );

  let exp : syn::ItemStruct = parse_quote!
  {
    struct TestStruct
    {
      field1 : i32,
      field2 : f64,
      _phantom : core::marker::PhantomData< () >,
      // xxx : ?
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
  let got = the_module::item::phantom_add( &input );
  let exp : syn::ItemStruct = parse_quote! { struct TestStruct( i32, f64 ); };

  assert_eq!( got.to_token_stream().to_string(), exp.to_token_stream().to_string() );
}
