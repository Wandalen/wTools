use super :: *;

#[ tokio ::test ]
async fn async_try_from_test()
{
  // Example implementation of AsyncTryFrom for a custom type
  struct MyNumber(u32);

  #[ the_module ::async_trait ]
  impl the_module ::AsyncTryFrom< String > for MyNumber 
  {
  type Error = core ::num ::ParseIntError;

  async fn async_try_from(value: String) -> Result< Self, Self ::Error >
  {
   // Simulate asynchronous work
   tokio ::time ::sleep(tokio ::time ::Duration ::from_millis(10)).await;
   let num = value.parse :: < u32 >()?;
   Ok(MyNumber(num))
 }
 }

  use the_module :: { AsyncTryFrom, AsyncTryInto };

  // Using AsyncTryFrom directly
  let result = MyNumber ::async_try_from("42".to_string()).await;
  assert!( result.is_ok(), "AsyncTryFrom should succeed for valid input" );
  let my_num = result.unwrap();
  assert_eq!( my_num.0, 42, "AsyncTryFrom should convert '42' to 42" );

  // Using AsyncTryInto, which is automatically implemented
  let result: Result< MyNumber, _ > = "42".to_string().async_try_into().await;
  assert!( result.is_ok(), "AsyncTryInto should succeed for valid input" );
  let my_num = result.unwrap();
  assert_eq!( my_num.0, 42, "AsyncTryInto should convert '42' to 42" );

  // Test error case - invalid input should fail
  let result = MyNumber ::async_try_from("invalid".to_string()).await;
  assert!( result.is_err(), "AsyncTryFrom should fail for invalid input" );

  let result: Result< MyNumber, _ > = "invalid".to_string().async_try_into().await;
  assert!( result.is_err(), "AsyncTryInto should fail for invalid input" );
}

#[ tokio ::test ]
async fn async_from_test()
{
  // Example implementation of AsyncFrom for a custom type
  struct MyNumber(u32);

  #[ the_module ::async_trait ]
  impl the_module ::AsyncFrom< String > for MyNumber 
  {
  async fn async_from(value: String) -> Self
  {
   // Simulate asynchronous work
   tokio ::time ::sleep(tokio ::time ::Duration ::from_millis(10)).await;
   let num = value.parse :: < u32 >().unwrap_or(0);
   MyNumber(num)
 }
 }

  use the_module :: { AsyncFrom, AsyncInto };

  // Using AsyncFrom directly
  let my_num: MyNumber = MyNumber ::async_from("42".to_string()).await;
  assert_eq!( my_num.0, 42, "AsyncFrom should convert '42' to 42" );

  // Using AsyncInto, which is automatically implemented
  let my_num: MyNumber = "42".to_string().async_into().await;
  assert_eq!( my_num.0, 42, "AsyncInto should convert '42' to 42" );

  // Test with invalid input - should fall back to 0 due to unwrap_or(0)
  let my_num: MyNumber = MyNumber ::async_from("invalid".to_string()).await;
  assert_eq!( my_num.0, 0, "AsyncFrom should fall back to 0 for invalid input" );

  let my_num: MyNumber = "invalid".to_string().async_into().await;
  assert_eq!( my_num.0, 0, "AsyncInto should fall back to 0 for invalid input" );
}
