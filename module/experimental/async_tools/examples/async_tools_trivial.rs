//! Demonstrates basic usage of async conversion traits.
//!
//! This example shows how to use `AsyncFrom`, `AsyncInto`, `AsyncTryFrom`, and `AsyncTryInto`
//! traits for asynchronous type conversions. These traits enable conversions that involve
//! async operations like network I/O, database queries, or other async contexts where
//! standard synchronous From/Into traits cannot be used.

#[ tokio ::main ]
async fn main()
{
  #[ cfg( feature = "enabled" ) ]
  {
    use async_tools :: { async_trait, AsyncFrom, AsyncInto, AsyncTryFrom, AsyncTryInto };

    // =================================================================
    // Example 1: Basic AsyncFrom implementation
    // =================================================================

    /// Custom type wrapping a u32
    struct MyNumber( u32 );

    #[ async_trait ]
    impl AsyncFrom< String > for MyNumber
    {
      async fn async_from( value : String ) -> Self
      {
        // Simulate asynchronous work (e.g., database lookup, validation)
        tokio ::time ::sleep( tokio ::time ::Duration ::from_millis( 10 ) ).await;
        let num = value.parse :: < u32 >().unwrap_or( 0 );
        MyNumber( num )
      }
    }

    // Using AsyncFrom directly
    let my_num : MyNumber = MyNumber ::async_from( "42".to_string() ).await;
    println!( "AsyncFrom: Converted '42' to {}", my_num.0 );
    assert_eq!( my_num.0, 42 );

    // Using AsyncInto (automatically implemented via blanket impl)
    let my_num : MyNumber = "99".to_string().async_into().await;
    println!( "AsyncInto: Converted '99' to {}", my_num.0 );
    assert_eq!( my_num.0, 99 );

    // =================================================================
    // Example 2: Fallible async conversion with AsyncTryFrom
    // =================================================================

    /// Validated number that requires successful parsing
    struct ValidatedNumber( u32 );

    #[ async_trait ]
    impl AsyncTryFrom< String > for ValidatedNumber
    {
      type Error = core ::num ::ParseIntError;

      async fn async_try_from( value : String ) -> Result< Self, Self ::Error >
      {
        // Simulate async validation (e.g., checking against external service)
        tokio ::time ::sleep( tokio ::time ::Duration ::from_millis( 5 ) ).await;
        let num = value.parse :: < u32 >()?;
        Ok( ValidatedNumber( num ) )
      }
    }

    // Using AsyncTryFrom with valid input
    let result = ValidatedNumber ::async_try_from( "42".to_string() ).await;
    match result
    {
      Ok( num ) =>
      {
        println!( "AsyncTryFrom: Successfully converted '42' to {}", num.0 );
        assert_eq!( num.0, 42 );
      }
      Err( e ) => panic!( "Unexpected error: {e:?}" ),
    }

    // Using AsyncTryFrom with invalid input
    let result = ValidatedNumber ::async_try_from( "invalid".to_string() ).await;
    match result
    {
      Ok( _num ) => panic!( "Should have failed for invalid input" ),
      Err( _e ) =>
      {
        println!( "AsyncTryFrom: Correctly rejected invalid input 'invalid'" );
      }
    }

    // Using AsyncTryInto (automatically implemented via blanket impl)
    let result : Result< ValidatedNumber, _ > = "123".to_string().async_try_into().await;
    match result
    {
      Ok( num ) =>
      {
        println!( "AsyncTryInto: Successfully converted '123' to {}", num.0 );
        assert_eq!( num.0, 123 );
      }
      Err( e ) => panic!( "Unexpected error: {e:?}" ),
    }

    // Using AsyncTryInto with invalid input
    let result : Result< ValidatedNumber, _ > = "not_a_number".to_string().async_try_into().await;
    match result
    {
      Ok( _num ) => panic!( "Should have failed for invalid input" ),
      Err( _e ) =>
      {
        println!( "AsyncTryInto: Correctly rejected invalid input 'not_a_number'" );
      }
    }

    println!( "\nAll async conversion examples completed successfully!" );
  }
}
