//! Send bounds validation tests
//!
//! This file validates that async conversion traits work correctly with Send bounds
//! in multi-threaded async runtimes. Per `docs/invariant/001_send_bounds.md`, Send bounds are
//! required for compatibility with tokio and other multi-threaded runtimes.
//!
//! ## Root Cause
//! Send bounds were required in blanket implementations but never explicitly tested.
//! Without Send, futures cannot move between threads in multi-threaded runtimes.
//!
//! ## Why Not Caught Initially
//! Original test suite focused on functional correctness but didn't verify
//! multi-threaded runtime compatibility explicitly.
//!
//! ## Prevention
//! These tests ensure Send bounds work correctly by spawning tasks on multi-threaded
//! runtime and moving conversions between threads.

use async_from::{ async_trait, AsyncFrom, AsyncInto, AsyncTryFrom, AsyncTryInto };

// =========================================================================
// Send Bounds: AsyncFrom/AsyncInto
// =========================================================================

/// Test `AsyncFrom` with Send bounds in multi-threaded runtime
///
/// **What This Tests**: `AsyncFrom` futures can be sent between threads
///
/// **Why Critical**: Per `docs/invariant/001_send_bounds.md`, most async runtimes are multi-threaded.
/// Futures must implement Send to move between threads.
#[ tokio::test( flavor = "multi_thread", worker_threads = 2 ) ]
async fn test_async_from_send_bounds()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncFrom< String > for MyNumber
  {
    async fn async_from( value : String ) -> Self
    {
      // Simulate async work
      tokio::time::sleep( tokio::time::Duration::from_millis( 1 ) ).await;
      let num = value.parse::< u32 >().unwrap_or( 0 );
      MyNumber( num )
    }
  }

  // Spawn on multi-threaded runtime - requires Send
  let handle = tokio::spawn( async move
  {
    let num = MyNumber::async_from( "42".to_string() ).await;
    num.0
  } );

  let result = handle.await.expect( "Task should complete successfully" );
  assert_eq!( result, 42, "`AsyncFrom` should work with Send bounds across threads" );
}

/// Test `AsyncInto` with Send bounds in multi-threaded runtime
///
/// **What This Tests**: `AsyncInto` futures (via blanket impl) can be sent between threads
///
/// **Why Critical**: Blanket implementations require T: Send + U: Send.
/// Must verify this works in practice.
#[ tokio::test( flavor = "multi_thread", worker_threads = 2 ) ]
async fn test_async_into_send_bounds()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncFrom< String > for MyNumber
  {
    async fn async_from( value : String ) -> Self
    {
      tokio::time::sleep( tokio::time::Duration::from_millis( 1 ) ).await;
      let num = value.parse::< u32 >().unwrap_or( 0 );
      MyNumber( num )
    }
  }

  // Spawn on multi-threaded runtime - requires Send
  let handle = tokio::spawn( async move
  {
    let num : MyNumber = "42".to_string().async_into().await;
    num.0
  } );

  let result = handle.await.expect( "Task should complete successfully" );
  assert_eq!( result, 42, "`AsyncInto` should work with Send bounds across threads" );
}

// =========================================================================
// Send Bounds: AsyncTryFrom/AsyncTryInto
// =========================================================================

/// Test `AsyncTryFrom` with Send bounds in multi-threaded runtime
///
/// **What This Tests**: `AsyncTryFrom` futures can be sent between threads
///
/// **Why Critical**: Fallible conversions must also work in multi-threaded contexts.
#[ tokio::test( flavor = "multi_thread", worker_threads = 2 ) ]
async fn test_async_try_from_send_bounds()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncTryFrom< String > for MyNumber
  {
    type Error = core::num::ParseIntError;

    async fn async_try_from( value : String ) -> Result< Self, Self::Error >
    {
      tokio::time::sleep( tokio::time::Duration::from_millis( 1 ) ).await;
      let num = value.parse::< u32 >()?;
      Ok( MyNumber( num ) )
    }
  }

  // Spawn on multi-threaded runtime - requires Send
  let handle = tokio::spawn( async move
  {
    MyNumber::async_try_from( "42".to_string() ).await
  } );

  let result = handle.await.expect( "Task should complete successfully" );
  assert!( result.is_ok(), "`AsyncTryFrom` should work with Send bounds across threads" );
  assert_eq!( result.unwrap().0, 42, "Result should be 42" );
}

/// Test `AsyncTryInto` with Send bounds in multi-threaded runtime
///
/// **What This Tests**: `AsyncTryInto` futures (via blanket impl) can be sent between threads
///
/// **Why Critical**: Blanket implementations require T: Send + U: Send.
/// Must verify fallible conversions work in multi-threaded contexts.
#[ tokio::test( flavor = "multi_thread", worker_threads = 2 ) ]
async fn test_async_try_into_send_bounds()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncTryFrom< String > for MyNumber
  {
    type Error = core::num::ParseIntError;

    async fn async_try_from( value : String ) -> Result< Self, Self::Error >
    {
      tokio::time::sleep( tokio::time::Duration::from_millis( 1 ) ).await;
      let num = value.parse::< u32 >()?;
      Ok( MyNumber( num ) )
    }
  }

  // Spawn on multi-threaded runtime - requires Send
  let handle = tokio::spawn( async move
  {
    let result : Result< MyNumber, _ > = "42".to_string().async_try_into().await;
    result
  } );

  let result = handle.await.expect( "Task should complete successfully" );
  assert!( result.is_ok(), "`AsyncTryInto` should work with Send bounds across threads" );
  assert_eq!( result.unwrap().0, 42, "Result should be 42" );
}

// =========================================================================
// Concurrent Multi-Threaded Conversions
// =========================================================================

/// Test multiple concurrent conversions across threads
///
/// **What This Tests**: Multiple async conversions can run concurrently
/// across multiple threads in multi-threaded runtime.
///
/// **Why Critical**: Real-world applications spawn many concurrent tasks.
/// Must verify no race conditions or deadlocks.
#[ tokio::test( flavor = "multi_thread", worker_threads = 4 ) ]
async fn test_concurrent_multi_threaded_conversions()
{
  struct MyNumber( u32 );

  #[ async_trait ]
  impl AsyncFrom< String > for MyNumber
  {
    async fn async_from( value : String ) -> Self
    {
      tokio::time::sleep( tokio::time::Duration::from_millis( 5 ) ).await;
      let num = value.parse::< u32 >().unwrap_or( 0 );
      MyNumber( num )
    }
  }

  // Spawn multiple tasks on different threads
  let handles : Vec< _ > = ( 0..10 )
    .map( | i |
    {
      tokio::spawn( async move
      {
        let num = MyNumber::async_from( format!( "{}", i * 10 ) ).await;
        num.0
      } )
    } )
    .collect();

  // Wait for all tasks to complete
  let results : Vec< u32 > = futures::future::join_all( handles )
    .await
    .into_iter()
    .map( | r | r.expect( "Task should complete" ) )
    .collect();

  // Verify all conversions succeeded
  assert_eq!( results.len(), 10, "All 10 tasks should complete" );
  for ( i, &result ) in results.iter().enumerate()
  {
    let expected = u32::try_from( i ).unwrap() * 10;
    assert_eq!( result, expected, "Task {} should produce {}", i, i * 10 );
  }
}
