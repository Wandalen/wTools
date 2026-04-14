//! Integration tests for multiple derives working together.
//!
//! Tests combinations of derives on the same struct to ensure they:
//! - Don't conflict with each other
//! - Work correctly when combined
//! - Integrate with standard library derives (`Clone`, `Debug`, `PartialEq`, etc.)
//!
//! ## Test Matrix
//!
//! | Test Case | Derives | Expected | Status |
//! |-----------|---------|----------|--------|
//! | `test_deref_derefmut_together` | Deref + DerefMut | Both work | ✅ |
//! | `test_asref_asmut_together` | AsRef + AsMut | Both work | ✅ |
//! | `test_index_indexmut_together` | IndexMut (provides both) | Both work | ✅ |
//! | `test_deref_from_new` | Deref + From + New | All work | ✅ |
//! | `test_with_std_derives` | Derive + Clone/Debug/PartialEq | All work | ✅ |
//! | `test_nested_derived_types` | Wrapper of wrapper | Works | ✅ |

use derive_tools_meta::*;

/// Test 1: `Deref` + `DerefMut` together (most common combination)
///
/// Should implement both immutable and mutable dereferencing.
#[ test ]
fn test_deref_derefmut_together()
{
  #[ derive( Deref, DerefMut ) ]
  struct Wrapper( String );

  let mut w = Wrapper( "hello".to_string() );

  // Deref (immutable)
  assert_eq!( &*w, "hello" );
  assert_eq!( w.len(), 5 ); // Method call through Deref

  // DerefMut (mutable)
  w.push_str( " world" );
  assert_eq!( w.0, "hello world" );
}

/// Test 2: `AsRef` + `AsMut` together
///
/// Should implement both immutable and mutable reference conversion.
#[ test ]
fn test_asref_asmut_together()
{
  #[ derive( AsRef, AsMut ) ]
  struct Container( Vec< i32 > );

  let mut c = Container( vec![ 1, 2, 3 ] );

  // AsRef (immutable)
  let v_ref: &Vec< i32 > = c.as_ref();
  assert_eq!( v_ref.len(), 3 );

  // AsMut (mutable)
  let v_mut: &mut Vec< i32 > = c.as_mut();
  v_mut.push( 4 );
  assert_eq!( c.0.len(), 4 );
}

/// Test 3: `IndexMut` provides both `Index` and `IndexMut`
///
/// `IndexMut` derive automatically generates BOTH `Index` and `IndexMut` implementations.
#[ test ]
fn test_index_indexmut_together()
{
  #[ derive( IndexMut ) ]
  struct Wrapper( Vec< i32 > );

  let mut w = Wrapper( vec![ 10, 20, 30 ] );

  // Index (immutable) - provided by IndexMut derive
  let vec_ref: &Vec< i32 > = &w[ 0 ];
  assert_eq!( vec_ref[ 0 ], 10 );

  // IndexMut (mutable)
  let vec_mut: &mut Vec< i32 > = &mut w[ 0 ];
  vec_mut[ 1 ] = 99;
  assert_eq!( w.0[ 1 ], 99 );
}

/// Test 4: Deref + From + New together (common newtype pattern)
///
/// Tests three derives that don't conflict: transparent access + conversion + constructor.
#[ test ]
fn test_deref_from_new()
{
  #[ derive( Deref, From, New ) ]
  struct UserId
  {
    id: String
  }

  // New constructor
  let user1 = UserId::new( "user123".to_string() );
  assert_eq!( user1.id, "user123" );

  // From conversion
  let user2 = UserId::from( "user456".to_string() );
  assert_eq!( user2.id, "user456" );

  // Deref access
  assert_eq!( user2.len(), 7 ); // String::len() through Deref
}

/// Test 5: Derives with standard library derives
///
/// Ensures `derive_tools` macros work alongside `Clone`, `Debug`, `PartialEq`, etc.
#[ test ]
fn test_with_std_derives()
{
  #[ derive( Clone, Debug, PartialEq, Deref, DerefMut, From, New ) ]
  struct Wrapper
  {
    value: i32
  }

  let w1 = Wrapper::new( 42 );
  let w2 = Wrapper::from( 42 );

  // Standard derives work
  assert_eq!( w1, w2 ); // PartialEq
  let w3 = w1.clone(); // Clone
  assert_eq!( format!( "{w3:?}" ), "Wrapper { value: 42 }" ); // Debug

  // derive_tools derives work
  assert_eq!( *w3, 42 ); // Deref
}

/// Test 6: Nested derived types
///
/// Tests wrapper of wrapper with derives on both levels.
#[ test ]
fn test_nested_derived_types()
{
  #[ derive( Deref, From ) ]
  struct Inner( String );

  #[ derive( Deref, From ) ]
  struct Outer( Inner );

  // Double From conversion
  let inner = Inner::from( "test".to_string() );
  let outer = Outer::from( inner );

  // Double Deref (outer -> inner -> String)
  assert_eq!( &**outer, "test" );

  // Can also use methods through double deref
  assert_eq!( outer.len(), 4 );
}

/// Test 7: Multi-field struct with multiple marker attributes
///
/// Tests that different derives can target different fields with markers.
#[ test ]
fn test_multi_field_multiple_markers()
{
  #[ derive( Deref, DerefMut, AsRef, AsMut ) ]
  struct MultiField
  {
    #[ deref ]
    #[ deref_mut ]
    #[ as_ref ]
    #[ as_mut ]
    primary: String,
    secondary: i32,
  }

  let mut m = MultiField
  {
    primary: "hello".to_string(),
    secondary: 42,
  };

  // All derives target the same marked field
  assert_eq!( &*m, "hello" ); // Deref

  let s: &String = m.as_ref(); // AsRef
  assert_eq!( s, "hello" );

  // Mutable operations
  m.push_str( " world" ); // DerefMut
  assert_eq!( m.primary, "hello world" );

  let s_mut: &mut String = m.as_mut(); // AsMut
  s_mut.push( '!' );
  assert_eq!( m.primary, "hello world!" );

  // Other field unchanged
  assert_eq!( m.secondary, 42 );
}

/// Test 8: Generic types with multiple derives
///
/// Tests that multiple derives work together on generic structs.
#[ test ]
fn test_generic_multiple_derives()
{
  #[ derive( Clone, Debug, PartialEq, Deref, From, New, AsRef, Index ) ]
  struct GenericWrapper< T >
  {
    value: T
  }

  let w1: GenericWrapper< Vec< i32 >> = GenericWrapper::new( vec![ 1, 2, 3 ] );
  let w2 = GenericWrapper::from( vec![ 1, 2, 3 ] );

  assert_eq!( w1, w2 );
  assert_eq!( w1.len(), 3 ); // Deref to Vec

  // Index returns reference to the field
  let v_from_index: &Vec< i32 > = &w1[ 0 ];
  assert_eq!( v_from_index[ 0 ], 1 );

  let v_ref: &Vec< i32 > = w1.as_ref(); // AsRef
  assert_eq!( v_ref.len(), 3 );
}
