//! Manual validation test for `async_from` crate
//!
//! This file tests all examples from readme.md and validates missing corner cases
//! identified in the comprehensive corner case matrix.

#![ cfg( all( feature = "async_from", feature = "async_try_from" ) ) ]

use async_from::{ async_trait, AsyncFrom, AsyncInto, AsyncTryFrom, AsyncTryInto };

// =========================================================================
// Readme Example 1: AsyncFrom/AsyncInto
// =========================================================================

/// Test Example 1 from readme.md (`AsyncFrom`)
#[ tokio::test ]
async fn readme_example_async_from()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncFrom< String > for MyNumber
  {
    async fn async_from( value : String ) -> Self
    {
      let num = value.parse::< u32 >().unwrap_or( 0 );
      MyNumber( num )
    }
  }

  // Test AsyncFrom
  let num = MyNumber::async_from( "42".to_string() ).await;
  assert_eq!( num.0, 42, "AsyncFrom should convert '42' to 42" );

  // Test AsyncInto
  let num : MyNumber = "42".to_string().async_into().await;
  assert_eq!( num.0, 42, "AsyncInto should convert '42' to 42" );
}

// =========================================================================
// Readme Example 2: AsyncTryFrom/AsyncTryInto
// =========================================================================

/// Test Example 2 from readme.md (`AsyncTryFrom`)
#[ tokio::test ]
async fn readme_example_async_try_from()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncTryFrom< String > for MyNumber
  {
    type Error = core::num::ParseIntError;

    async fn async_try_from( value : String ) -> Result< Self, Self::Error >
    {
      let num = value.parse::< u32 >()?;
      Ok( MyNumber( num ) )
    }
  }

  // Test AsyncTryFrom success
  let result = MyNumber::async_try_from( "42".to_string() ).await;
  assert!( result.is_ok(), "AsyncTryFrom should succeed for valid input" );
  assert_eq!( result.unwrap().0, 42, "AsyncTryFrom should convert '42' to 42" );

  // Test AsyncTryInto success
  let result : Result< MyNumber, _ > = "42".to_string().async_try_into().await;
  assert!( result.is_ok(), "AsyncTryInto should succeed for valid input" );
  assert_eq!( result.unwrap().0, 42, "AsyncTryInto should convert '42' to 42" );
}

// =========================================================================
// Missing Corner Cases: AsyncTryFrom
// =========================================================================

/// Test ATF-9: Leading zeros should parse correctly
#[ tokio::test ]
async fn async_try_from_leading_zeros()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncTryFrom< String > for MyNumber
  {
    type Error = core::num::ParseIntError;

    async fn async_try_from( value : String ) -> Result< Self, Self::Error >
    {
      let num = value.parse::< u32 >()?;
      Ok( MyNumber( num ) )
    }
  }

  let result = MyNumber::async_try_from( "0042".to_string() ).await;
  assert!( result.is_ok(), "Leading zeros should parse successfully" );
  assert_eq!( result.unwrap().0, 42, "0042 should convert to 42" );
}

/// Test ATF-10: Leading/trailing spaces should be rejected
#[ tokio::test ]
async fn async_try_from_spaces()
{
  #[ allow( dead_code ) ]
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncTryFrom< String > for MyNumber
  {
    type Error = core::num::ParseIntError;

    async fn async_try_from( value : String ) -> Result< Self, Self::Error >
    {
      let num = value.parse::< u32 >()?;
      Ok( MyNumber( num ) )
    }
  }

  // Rust's parse() does NOT accept leading/trailing spaces
  let result = MyNumber::async_try_from( " 42 ".to_string() ).await;
  assert!( result.is_err(), "Leading/trailing spaces should be rejected" );

  let result = MyNumber::async_try_from( " 42".to_string() ).await;
  assert!( result.is_err(), "Leading space should be rejected" );

  let result = MyNumber::async_try_from( "42 ".to_string() ).await;
  assert!( result.is_err(), "Trailing space should be rejected" );
}

/// Test ATF-14: Float format should be rejected
#[ tokio::test ]
async fn async_try_from_float_format()
{
  #[ allow( dead_code ) ]
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncTryFrom< String > for MyNumber
  {
    type Error = core::num::ParseIntError;

    async fn async_try_from( value : String ) -> Result< Self, Self::Error >
    {
      let num = value.parse::< u32 >()?;
      Ok( MyNumber( num ) )
    }
  }

  let result = MyNumber::async_try_from( "42.5".to_string() ).await;
  assert!( result.is_err(), "Float format should be rejected for u32" );

  let result = MyNumber::async_try_from( "42.0".to_string() ).await;
  assert!( result.is_err(), "Float format with .0 should be rejected for u32" );
}

/// Test ATF-15: Scientific notation should be rejected
#[ tokio::test ]
async fn async_try_from_scientific_notation()
{
  #[ allow( dead_code ) ]
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncTryFrom< String > for MyNumber
  {
    type Error = core::num::ParseIntError;

    async fn async_try_from( value : String ) -> Result< Self, Self::Error >
    {
      let num = value.parse::< u32 >()?;
      Ok( MyNumber( num ) )
    }
  }

  let result = MyNumber::async_try_from( "4.2e1".to_string() ).await;
  assert!( result.is_err(), "Scientific notation should be rejected" );

  let result = MyNumber::async_try_from( "1e2".to_string() ).await;
  assert!( result.is_err(), "Scientific notation should be rejected" );
}

/// Test ATF-16: Plus sign should be accepted by Rust's `parse()`
#[ tokio::test ]
async fn async_try_from_plus_sign()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncTryFrom< String > for MyNumber
  {
    type Error = core::num::ParseIntError;

    async fn async_try_from( value : String ) -> Result< Self, Self::Error >
    {
      let num = value.parse::< u32 >()?;
      Ok( MyNumber( num ) )
    }
  }

  // Rust's parse() DOES accept leading '+' for u32
  let result = MyNumber::async_try_from( "+42".to_string() ).await;
  assert!( result.is_ok(), "Plus sign should be accepted for u32" );
  assert_eq!( result.unwrap().0, 42, "+42 should parse to 42" );
}

/// Test ATF-12: Unicode digits should be rejected
#[ tokio::test ]
async fn async_try_from_unicode_digits()
{
  #[ allow( dead_code ) ]
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncTryFrom< String > for MyNumber
  {
    type Error = core::num::ParseIntError;

    async fn async_try_from( value : String ) -> Result< Self, Self::Error >
    {
      let num = value.parse::< u32 >()?;
      Ok( MyNumber( num ) )
    }
  }

  // Arabic-Indic digits
  let result = MyNumber::async_try_from( "٤٢".to_string() ).await;
  assert!( result.is_err(), "Unicode digits should be rejected" );
}

/// Test ATF-13: Hex format should be rejected
#[ tokio::test ]
async fn async_try_from_hex_format()
{
  #[ allow( dead_code ) ]
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncTryFrom< String > for MyNumber
  {
    type Error = core::num::ParseIntError;

    async fn async_try_from( value : String ) -> Result< Self, Self::Error >
    {
      let num = value.parse::< u32 >()?;
      Ok( MyNumber( num ) )
    }
  }

  let result = MyNumber::async_try_from( "0x2A".to_string() ).await;
  assert!( result.is_err(), "Hex format should be rejected by default parse()" );
}

// =========================================================================
// Missing Corner Cases: AsyncFrom
// =========================================================================

/// Test AF-9: Leading zeros should parse correctly (infallible)
#[ tokio::test ]
async fn async_from_leading_zeros()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncFrom< String > for MyNumber
  {
    async fn async_from( value : String ) -> Self
    {
      let num = value.parse::< u32 >().unwrap_or( 0 );
      MyNumber( num )
    }
  }

  let num = MyNumber::async_from( "0042".to_string() ).await;
  assert_eq!( num.0, 42, "0042 should convert to 42" );
}

/// Test AF-10: Leading/trailing spaces should fallback to 0
#[ tokio::test ]
async fn async_from_spaces()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncFrom< String > for MyNumber
  {
    async fn async_from( value : String ) -> Self
    {
      let num = value.parse::< u32 >().unwrap_or( 0 );
      MyNumber( num )
    }
  }

  let num = MyNumber::async_from( " 42 ".to_string() ).await;
  assert_eq!( num.0, 0, "Spaces should fallback to 0" );

  let num = MyNumber::async_from( " 42".to_string() ).await;
  assert_eq!( num.0, 0, "Leading space should fallback to 0" );

  let num = MyNumber::async_from( "42 ".to_string() ).await;
  assert_eq!( num.0, 0, "Trailing space should fallback to 0" );
}

/// Test AF-7: Negative value should fallback to 0
#[ tokio::test ]
async fn async_from_negative()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncFrom< String > for MyNumber
  {
    async fn async_from( value : String ) -> Self
    {
      let num = value.parse::< u32 >().unwrap_or( 0 );
      MyNumber( num )
    }
  }

  let num = MyNumber::async_from( "-1".to_string() ).await;
  assert_eq!( num.0, 0, "Negative value should fallback to 0" );

  let num = MyNumber::async_from( "-42".to_string() ).await;
  assert_eq!( num.0, 0, "Negative value should fallback to 0" );
}

/// Test AF-8: Overflow should fallback to 0
#[ tokio::test ]
async fn async_from_overflow()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncFrom< String > for MyNumber
  {
    async fn async_from( value : String ) -> Self
    {
      let num = value.parse::< u32 >().unwrap_or( 0 );
      MyNumber( num )
    }
  }

  let num = MyNumber::async_from( "99999999999999".to_string() ).await;
  assert_eq!( num.0, 0, "Overflow should fallback to 0" );
}

/// Test AF-6: Maximum value should parse correctly
#[ tokio::test ]
async fn async_from_max_value()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncFrom< String > for MyNumber
  {
    async fn async_from( value : String ) -> Self
    {
      let num = value.parse::< u32 >().unwrap_or( 0 );
      MyNumber( num )
    }
  }

  let max_str = u32::MAX.to_string();
  let num = MyNumber::async_from( max_str ).await;
  assert_eq!( num.0, u32::MAX, "u32::MAX should parse correctly" );
}

/// Test AF-5: Zero value should parse correctly
#[ tokio::test ]
async fn async_from_zero()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncFrom< String > for MyNumber
  {
    async fn async_from( value : String ) -> Self
    {
      let num = value.parse::< u32 >().unwrap_or( 999 ); // Different fallback to distinguish
      MyNumber( num )
    }
  }

  let num = MyNumber::async_from( "0".to_string() ).await;
  assert_eq!( num.0, 0, "Zero should parse to 0, not fallback" );
}
