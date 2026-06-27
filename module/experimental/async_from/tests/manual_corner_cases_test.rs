//! Manual corner case testing for async conversion traits
//!
//! This test file validates all corner cases identified in the corner case matrix.
//! Tests are organized by category for systematic validation.

#![ cfg( all( feature = "async_from", feature = "async_try_from" ) ) ]

use async_from :: { async_trait, AsyncFrom, AsyncInto, AsyncTryFrom, AsyncTryInto };

// =========================================================================
// Category 1: Value Edge Cases
// =========================================================================

/// Test empty string conversion with `AsyncFrom`
#[ tokio ::test ]
async fn test_async_from_empty_string()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncFrom< String > for MyNumber
  {
    async fn async_from( value : String ) -> Self
    {
      let num = value.parse ::< u32 >().unwrap_or( 0 );
      MyNumber( num )
    }
  }

  let result = MyNumber ::async_from( String ::new() ).await;
  assert_eq!( result.0, 0, "Empty string should default to 0" );
}

/// Test whitespace-only conversion with `AsyncFrom`
#[ tokio ::test ]
async fn test_async_from_whitespace_only()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncFrom< String > for MyNumber
  {
    async fn async_from( value : String ) -> Self
    {
      let num = value.parse ::< u32 >().unwrap_or( 0 );
      MyNumber( num )
    }
  }

  let result = MyNumber ::async_from( "   ".to_string() ).await;
  assert_eq!( result.0, 0, "Whitespace-only string should default to 0" );
}

/// Test boundary value (0) with `AsyncTryFrom`
#[ tokio ::test ]
async fn test_async_try_from_zero()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncTryFrom< String > for MyNumber
  {
    type Error = core ::num ::ParseIntError;

    async fn async_try_from( value : String ) -> Result< Self, Self ::Error >
    {
      let num = value.parse ::< u32 >()?;
      Ok( MyNumber( num ) )
    }
  }

  let result = MyNumber ::async_try_from( "0".to_string() ).await;
  assert!( result.is_ok(), "Zero should parse successfully" );
  assert_eq!( result.unwrap().0, 0, "Value should be 0" );
}

/// Test maximum value (`u32::MAX`) with `AsyncTryFrom`
#[ tokio ::test ]
async fn test_async_try_from_max_value()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncTryFrom< String > for MyNumber
  {
    type Error = core ::num ::ParseIntError;

    async fn async_try_from( value : String ) -> Result< Self, Self ::Error >
    {
      let num = value.parse ::< u32 >()?;
      Ok( MyNumber( num ) )
    }
  }

  let max_str = u32 ::MAX.to_string();
  let result = MyNumber ::async_try_from( max_str ).await;
  assert!( result.is_ok(), "u32::MAX should parse successfully" );
  assert_eq!( result.unwrap().0, u32 ::MAX, "Value should be u32::MAX" );
}

/// Test negative value with `AsyncTryFrom` (should fail for `u32`)
#[ tokio ::test ]
async fn test_async_try_from_negative_value()
{
  #[ allow( dead_code ) ]
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncTryFrom< String > for MyNumber
  {
    type Error = core ::num ::ParseIntError;

    async fn async_try_from( value : String ) -> Result< Self, Self ::Error >
    {
      let num = value.parse ::< u32 >()?;
      Ok( MyNumber( num ) )
    }
  }

  let result = MyNumber ::async_try_from( "-1".to_string() ).await;
  assert!( result.is_err(), "Negative value should fail for u32" );
}

/// Test very large number (exceeds `u32::MAX`) with `AsyncTryFrom`
#[ tokio ::test ]
async fn test_async_try_from_overflow()
{
  #[ allow( dead_code ) ]
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncTryFrom< String > for MyNumber
  {
    type Error = core ::num ::ParseIntError;

    async fn async_try_from( value : String ) -> Result< Self, Self ::Error >
    {
      let num = value.parse ::< u32 >()?;
      Ok( MyNumber( num ) )
    }
  }

  let result = MyNumber ::async_try_from( "99999999999999".to_string() ).await;
  assert!( result.is_err(), "Number exceeding u32::MAX should fail" );
}

/// Test special characters with `AsyncTryFrom`
#[ tokio ::test ]
async fn test_async_try_from_special_chars()
{
  #[ allow( dead_code ) ]
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncTryFrom< String > for MyNumber
  {
    type Error = core ::num ::ParseIntError;

    async fn async_try_from( value : String ) -> Result< Self, Self ::Error >
    {
      let num = value.parse ::< u32 >()?;
      Ok( MyNumber( num ) )
    }
  }

  let result = MyNumber ::async_try_from( "@#$%".to_string() ).await;
  assert!( result.is_err(), "Special characters should fail to parse" );
}

// =========================================================================
// Category 2: Async Behavior
// =========================================================================

/// Test immediate return (no actual async work)
#[ tokio ::test ]
async fn test_async_from_immediate_return()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncFrom< u32 > for MyNumber
  {
    async fn async_from( value : u32 ) -> Self
    {
      // No actual async work - immediate return
      MyNumber( value )
    }
  }

  let result = MyNumber ::async_from( 42 ).await;
  assert_eq!( result.0, 42, "Immediate return should work correctly" );
}

/// Test multiple concurrent conversions
#[ tokio ::test ]
async fn test_concurrent_conversions()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncFrom< String > for MyNumber
  {
    async fn async_from( value : String ) -> Self
    {
      tokio ::time ::sleep( tokio ::time ::Duration ::from_millis( 10 ) ).await;
      let num = value.parse ::< u32 >().unwrap_or( 0 );
      MyNumber( num )
    }
  }

  // Run 3 conversions concurrently
  let ( result1, result2, result3 ) = tokio ::join!(
    MyNumber ::async_from( "10".to_string() ),
    MyNumber ::async_from( "20".to_string() ),
    MyNumber ::async_from( "30".to_string() )
  );

  assert_eq!( result1.0, 10, "First conversion should succeed" );
  assert_eq!( result2.0, 20, "Second conversion should succeed" );
  assert_eq!( result3.0, 30, "Third conversion should succeed" );
}

// =========================================================================
// Category 3: Blanket Implementation Testing
// =========================================================================

/// Test `AsyncInto` with immediate return
#[ tokio ::test ]
async fn test_async_into_immediate()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncFrom< u32 > for MyNumber
  {
    async fn async_from( value : u32 ) -> Self
    {
      MyNumber( value )
    }
  }

  let result : MyNumber = 42u32.async_into().await;
  assert_eq!( result.0, 42, "AsyncInto should work via blanket impl" );
}

/// Test `AsyncTryInto` with error case
#[ tokio ::test ]
async fn test_async_try_into_error()
{
  #[ allow( dead_code ) ]
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncTryFrom< String > for MyNumber
  {
    type Error = core ::num ::ParseIntError;

    async fn async_try_from( value : String ) -> Result< Self, Self ::Error >
    {
      let num = value.parse ::< u32 >()?;
      Ok( MyNumber( num ) )
    }
  }

  let result : Result< MyNumber, _ > = "invalid".to_string().async_try_into().await;
  assert!( result.is_err(), "AsyncTryInto should propagate errors correctly" );
}
