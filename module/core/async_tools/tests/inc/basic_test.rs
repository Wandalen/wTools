use super::*;

#[ tokio::test ]
async fn async_try_from_test()
{

  // Example implementation of AsyncTryFrom for a custom type
  struct MyNumber( u32 );

  // xxx : qqq : broken
  // #[ the_module::async_trait ]
  // impl< 'a > the_module::AsyncTryFrom< &'a str > for MyNumber
  // {
  //   type Error = std::num::ParseIntError;
  //
  //   async fn async_try_from( value : &'a str ) -> Result< Self, Self::Error >
  //   {
  //     // Simulate asynchronous work
  //     tokio::time::sleep( tokio::time::Duration::from_millis( 1 ) ).await;
  //     let num = value.parse::< u32 >()?;
  //     Ok( MyNumber( num ) )
  //   }
  // }

  #[ the_module::async_trait ]
  impl the_module::AsyncTryFrom< String > for MyNumber
  {
    type Error = std::num::ParseIntError;

    async fn async_try_from( value : String ) -> Result< Self, Self::Error >
    {
      // Simulate asynchronous work
      tokio::time::sleep( tokio::time::Duration::from_millis( 10 ) ).await;
      let num = value.parse::< u32 >()?;
      Ok( MyNumber( num ) )
    }
  }

  use the_module::{ AsyncTryFrom, AsyncTryInto };

  // Using AsyncTryFrom directly
  match MyNumber::async_try_from( "42".to_string() ).await
  {
    Ok( my_num ) => println!( "Converted successfully: {}", my_num.0 ),
    Err( e ) => println!( "Conversion failed: {:?}", e ),
  }

  // Using AsyncTryInto, which is automatically implemented
  let result : Result< MyNumber, _ > = "42".to_string().async_try_into().await;
  match result
  {
    Ok( my_num ) => println!( "Converted successfully using AsyncTryInto: {}", my_num.0 ),
    Err( e ) => println!( "Conversion failed using AsyncTryInto: {:?}", e ),
  }
}

#[ tokio::test ]
async fn async_from_test()
{
  // Example implementation of AsyncFrom for a custom type
  struct MyNumber( u32 );

  #[ the_module::async_trait ]
  impl the_module::AsyncFrom< String > for MyNumber
  {
    async fn async_tools( value : String ) -> Self
    {
      // Simulate asynchronous work
      tokio::time::sleep( tokio::time::Duration::from_millis( 10 ) ).await;
      let num = value.parse::< u32 >().unwrap_or( 0 );
      MyNumber( num )
    }
  }

  use the_module::{ AsyncFrom, AsyncInto };

  // Using AsyncFrom directly
  let my_num : MyNumber = MyNumber::async_tools( "42".to_string() ).await;
  println!( "Converted successfully using AsyncFrom: {}", my_num.0 );

  // Using AsyncInto, which is automatically implemented
  let my_num : MyNumber = "42".to_string().async_into().await;
  println!( "Converted successfully using AsyncInto: {}", my_num.0 );
}
