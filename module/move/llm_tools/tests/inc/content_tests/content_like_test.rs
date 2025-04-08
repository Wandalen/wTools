
use super::*;
use core::fmt;
use std::convert::TryFrom;

type CA = the_module::content::ContentAny< Vec< u8 > >;

/// Tests the ContentLike implementation for String.
#[ cfg( test ) ]
mod string_impl
{
  use super::*; // Include helpers and types from parent mod
  use the_module::content::{ ContentLike, ContentType, ContentAny }; // Local use
  use the_module::content; // Local use for builder functions
  use serde_json; // Local use

  /// Tests `String::content_type()`.
  #[ test ]
  fn content_type()
  {
    let s = "test".to_string();
    // Explicitly call the trait method
    assert_eq!( <String as ContentLike<Vec<u8>>>::content_type( &s ), ContentType::String );
  }

  /// Tests `String::content_to_bytes()`.
  #[ test ]
  fn content_to_bytes()
  {
    let s = "hello".to_string();
    let expected_bytes = b"hello".to_vec();
    // Explicitly call the trait method
    assert_eq!( <String as ContentLike<Vec<u8>>>::content_to_bytes( s ), expected_bytes );
  }

  /// Tests `String::content_to_json()`.
  #[ test ]
  fn content_to_json()
  {
    let s = "json test".to_string();
    let expected_json = serde_json::Value::String( "json test".to_string() );
    // Explicitly call the trait method
    assert_eq!( <String as ContentLike<Vec<u8>>>::content_to_json( s ), expected_json );
  }

  /// Tests `String::into_any()`.
  #[ test ]
  fn into_any()
  {
    let s = "into".to_string();
    let expected_content = ContentAny::String( "into".to_string() );
    // Explicitly call the trait method
    assert_eq!( <String as ContentLike<Vec<u8>>>::into_any( s ), expected_content );
  }

  /// Tests `String::push_to()` success case.
  #[ test ]
  fn push_to_success()
  {
    let s = "push me".to_string();
    let mut target_array = content::array::< Vec< u8 > >();
    // Explicitly call the trait method
    target_array = <String as ContentLike<Vec<u8>>>::push_to( s, target_array );

    match target_array
    {
      ContentAny::Array( arr ) =>
      {
        assert_eq!( arr.len(), 1 );
        assert_eq!( arr[ 0 ], ContentAny::String( "push me".to_string() ) );
      }
      _ => panic!( "push_to did not result in an array" ),
    }
  }

  /// Tests `String::push_to()` panic case.
  #[ test ]
  #[ should_panic ]
  fn push_to_panic()
  {
    let s = "panic push".to_string();
    let target_not_array = content::null::< Vec< u8 > >();
    // Explicitly call the trait method
    let _ = <String as ContentLike<Vec<u8>>>::push_to( s, target_not_array );
  }
}

/// Tests the ContentLike implementation for bool.
#[ cfg( test ) ]
mod bool_impl
{
  use super::*;
  use the_module::content::{ ContentLike, ContentType, ContentAny }; // Local use
  use the_module::content; // Local use for builder functions
  use serde_json; // Local use

  /// Tests `bool::content_type()`.
  #[ test ]
  fn content_type()
  {
    assert_eq!( <bool as ContentLike<Vec<u8>>>::content_type( &true ), ContentType::Bool );
    assert_eq!( <bool as ContentLike<Vec<u8>>>::content_type( &false ), ContentType::Bool );
  }

  /// Tests `bool::content_to_bytes()`.
  #[ test ]
  fn content_to_bytes()
  {
    assert_eq!( <bool as ContentLike<Vec<u8>>>::content_to_bytes( true ), b"true".to_vec() );
    assert_eq!( <bool as ContentLike<Vec<u8>>>::content_to_bytes( false ), b"false".to_vec() );
  }

  /// Tests `bool::content_to_json()`.
  #[ test ]
  fn content_to_json()
  {
    assert_eq!( <bool as ContentLike<Vec<u8>>>::content_to_json( true ), serde_json::Value::Bool( true ) );
    assert_eq!( <bool as ContentLike<Vec<u8>>>::content_to_json( false ), serde_json::Value::Bool( false ) );
  }

  /// Tests `bool::into_any()`.
  #[ test ]
  fn into_any()
  {
    assert_eq!( <bool as ContentLike<Vec<u8>>>::into_any( true ), ContentAny::Bool( true ) );
    assert_eq!( <bool as ContentLike<Vec<u8>>>::into_any( false ), ContentAny::Bool( false ) );
  }

  /// Tests `bool::push_to()` success case.
  #[ test ]
  fn push_to_success()
  {
    let b = true;
    let mut target_array = content::array::< Vec< u8 > >();
    target_array = <bool as ContentLike<Vec<u8>>>::push_to( b, target_array );

    match target_array
    {
      ContentAny::Array( arr ) =>
      {
        assert_eq!( arr.len(), 1 );
        assert_eq!( arr[ 0 ], ContentAny::Bool( true ) );
      }
      _ => panic!( "push_to did not result in an array" ),
    }
  }

  /// Tests `bool::push_to()` panic case.
  #[ test ]
  #[ should_panic ]
  fn push_to_panic()
  {
    let b = false;
    let target_not_array = content::null::< Vec< u8 > >();
    let _ = <bool as ContentLike<Vec<u8>>>::push_to( b, target_not_array );
  }
}

/// Tests the ContentLike implementation for serde_json::Number.
#[ cfg( test ) ]
mod number_impl
{
  use super::*;
  use the_module::content::{ ContentLike, ContentType, ContentAny }; // Local use
  use the_module::content; // Local use for builder functions
  use serde_json; // Local use

  /// Tests `Number::content_type()`.
  #[ test ]
  fn content_type()
  {
    let n = serde_json::Number::from( 123 );
    assert_eq!( <serde_json::Number as ContentLike<Vec<u8>>>::content_type( &n ), ContentType::Number );
  }

  /// Tests `Number::content_to_bytes()`.
  #[ test ]
  fn content_to_bytes()
  {
    let n = serde_json::Number::from( -456 );
    assert_eq!( <serde_json::Number as ContentLike<Vec<u8>>>::content_to_bytes( n ), b"-456".to_vec() );
    let n_float = serde_json::Number::from_f64( 12.34 ).unwrap();
    assert_eq!( <serde_json::Number as ContentLike<Vec<u8>>>::content_to_bytes( n_float ), b"12.34".to_vec() );
  }

  /// Tests `Number::content_to_json()`.
  #[ test ]
  fn content_to_json()
  {
    let n = serde_json::Number::from( 789 );
    assert_eq!( <serde_json::Number as ContentLike<Vec<u8>>>::content_to_json( n.clone() ), serde_json::Value::Number( n ) );
  }

  /// Tests `Number::into_any()`.
  #[ test ]
  fn into_any()
  {
    let n = serde_json::Number::from( 10 );
    assert_eq!( <serde_json::Number as ContentLike<Vec<u8>>>::into_any( n.clone() ), ContentAny::Number( n ) );
  }

  /// Tests `Number::push_to()` success case.
  #[ test ]
  fn push_to_success()
  {
    let n = serde_json::Number::from( 11 );
    let mut target_array = content::array::< Vec< u8 > >();
    target_array = <serde_json::Number as ContentLike<Vec<u8>>>::push_to( n.clone(), target_array );

    match target_array
    {
      ContentAny::Array( arr ) =>
      {
        assert_eq!( arr.len(), 1 );
        assert_eq!( arr[ 0 ], ContentAny::Number( n ) );
      }
      _ => panic!( "push_to did not result in an array" ),
    }
  }

  /// Tests `Number::push_to()` panic case.
  #[ test ]
  #[ should_panic ]
  fn push_to_panic()
  {
    let n = serde_json::Number::from( 12 );
    let target_not_array = content::null::< Vec< u8 > >();
    let _ = <serde_json::Number as ContentLike<Vec<u8>>>::push_to( n, target_not_array );
  }
}

/// Tests the ContentLike implementation for Vec<ContentAny<S>>.
#[ cfg( test ) ]
mod vec_impl
{
  use super::*;
  use the_module::content::{ ContentLike, ContentType, ContentAny }; // Local use
  use the_module::content; // Local use for builder functions
  use serde_json; // Local use

  /// Tests `Vec::content_type()`.
  #[ test ]
  fn content_type()
  {
    let v: Vec< CA > = vec![];
    assert_eq!( <Vec<CA> as ContentLike<Vec<u8>>>::content_type( &v ), ContentType::Array );
    let v_non_empty: Vec< CA > = vec![ content::null() ];
    assert_eq!( <Vec<CA> as ContentLike<Vec<u8>>>::content_type( &v_non_empty ), ContentType::Array );
  }

  /// Tests `Vec::content_to_bytes()`.
  #[ test ]
  fn content_to_bytes()
  {
    let v_empty: Vec< CA > = vec![];
    assert_eq!( <Vec<CA> as ContentLike<Vec<u8>>>::content_to_bytes( v_empty ), Vec::< u8 >::new() );

    let v: Vec< CA > = vec!
    [
      content::string( "ab".into() ),
      content::string( "cde".into() ),
      content::bool( true ), // This will add "true" bytes
    ];
    let expected = b"ab".to_vec().into_iter()
                   .chain( b"cde".to_vec().into_iter() )
                   .chain( b"true".to_vec().into_iter() )
                   .collect::< Vec< _ > >();
    assert_eq!( <Vec<CA> as ContentLike<Vec<u8>>>::content_to_bytes( v ), expected );
  }

  /// Tests `Vec::content_to_json()`.
  #[ test ]
  fn content_to_json()
  {
     let v: Vec< CA > = vec!
    [
      content::string( "a".into() ),
      content::bool( false ),
      content::null(),
    ];
    let expected_json = serde_json::json!( [ "a", false, null ] );
    assert_eq!( <Vec<CA> as ContentLike<Vec<u8>>>::content_to_json( v ), expected_json );
  }

  /// Tests `Vec::into_any()`.
  #[ test ]
  fn into_any()
  {
    let v: Vec< CA > = vec![ content::number( 1.into() ) ];
    let v_clone = v.clone();
    assert_eq!( <Vec<CA> as ContentLike<Vec<u8>>>::into_any( v ), ContentAny::Array( v_clone ) );
  }

  /// Tests `Vec::push_to()` success case.
  #[ test ]
  fn push_to_success()
  {
    let v: Vec< CA > = vec![ content::string( "inner".into() ) ];
    let mut target_array = content::array::< Vec< u8 > >();
    target_array = <Vec<CA> as ContentLike<Vec<u8>>>::push_to( v.clone(), target_array );

    match target_array
    {
      ContentAny::Array( arr ) =>
      {
        assert_eq!( arr.len(), 1 );
        assert_eq!( arr[ 0 ], ContentAny::Array( v ) ); // Pushes the vec as a nested array
      }
      _ => panic!( "push_to did not result in an array" ),
    }
  }

  /// Tests `Vec::push_to()` panic case.
  #[ test ]
  #[ should_panic ]
  fn push_to_panic()
  {
    let v: Vec< CA > = vec![];
    let target_not_array = content::null::< Vec< u8 > >();
    let _ = <Vec<CA> as ContentLike<Vec<u8>>>::push_to( v, target_not_array );
  }
}

/// Tests the ContentLike implementation for ().
#[ cfg( test ) ]
mod unit_impl
{
  use super::*;
  use the_module::content::{ ContentLike, ContentType, ContentAny }; // Local use
  use the_module::content; // Local use for builder functions
  use serde_json; // Local use

  /// Tests `()::content_type()`.
  #[ test ]
  fn content_type()
  {
    assert_eq!( <() as ContentLike<Vec<u8>>>::content_type( &() ), ContentType::Null );
  }

  /// Tests `()::content_to_bytes()`.
  #[ test ]
  fn content_to_bytes()
  {
    assert_eq!( <() as ContentLike<Vec<u8>>>::content_to_bytes( () ), b"null".to_vec() );
  }

  /// Tests `()::content_to_json()`.
  #[ test ]
  fn content_to_json()
  {
    assert_eq!( <() as ContentLike<Vec<u8>>>::content_to_json( () ), serde_json::Value::Null );
  }

  /// Tests `()::into_any()`.
  #[ test ]
  fn into_any()
  {
    assert_eq!( <() as ContentLike<Vec<u8>>>::into_any( () ), ContentAny::Null );
  }

  /// Tests `()::push_to()` success case.
  #[ test ]
  fn push_to_success()
  {
    let u = ();
    let mut target_array = content::array::< Vec< u8 > >();
    target_array = <() as ContentLike<Vec<u8>>>::push_to( u, target_array );

    match target_array
    {
      ContentAny::Array( arr ) =>
      {
        assert_eq!( arr.len(), 1 );
        assert_eq!( arr[ 0 ], ContentAny::Null );
      }
      _ => panic!( "push_to did not result in an array" ),
    }
  }

  /// Tests `()::push_to()` panic case.
  #[ test ]
  #[ should_panic ]
  fn push_to_panic()
  {
    let u = ();
    let target_not_array = content::null::< Vec< u8 > >();
    let _ = <() as ContentLike<Vec<u8>>>::push_to( u, target_not_array );
  }
}
