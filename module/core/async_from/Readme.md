<!-- {{# generate.module_header{} #}} -->

# Module :: async_from
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/Moduleasync_fromPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/Moduleasync_fromPush.yml) [![docs.rs](https://img.shields.io/docsrs/async_from?color=e3e8f0&logo=docs.rs)](https://docs.rs/async_from) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Async version of From, Into, TryFrom, TryInto.

The `async_from` crate provides asynchronous versions of the well-known `From`, `Into`, `TryFrom`, and `TryInto` traits. These traits are essential for handling conversions in Rust, and their asynchronous counterparts, allowing for conversions that involve asynchronous operations.

## Why Asynchronous Conversion Traits?

In Rust, the `From`, `Into`, `TryFrom`, and `TryInto` traits provide a standardized way to handle type conversions. The `async_from` module extends this functionality to asynchronous contexts with `AsyncFrom`, `AsyncInto`, `AsyncTryFrom`, and `AsyncTryInto` traits, offering several key benefits:

- **Simplicity**: Allow straightforward conversions without boilerplate, even in asynchronous contexts.
- **Consistency**: Provide a uniform interface for conversions across different types, aiding in writing predictable and maintainable code.
- **Error Handling**: Enable safe and explicit handling of conversion failures, essential for robust error management in commercial applications.
- **Asynchronous Contexts**: Facilitate conversions involving asynchronous operations, such as network requests or database queries, which are common in modern applications.

The `async_from` provides developers with the tools needed to handle complex conversions in an async context efficiently, which is particularly important for commercial applications requiring reliable and efficient handling of asynchronous operations.

### `AsyncFrom` and `AsyncInto`

Trait for asynchronous conversions from a type T.

These traits are designed for infallible asynchronous conversions. They allow you to convert types asynchronously, returning the result directly.

```rust
use async_from::{ async_trait, AsyncFrom, AsyncInto };

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

#[ tokio::main ]
async fn main()
{
  let num = MyNumber::async_from( "42".to_string() ).await;
  println!( "Converted: {}", num.0 );
  let num : MyNumber = "42".to_string().async_into().await;
  println!( "Converted: {}", num.0 );
}
```

### `AsyncTryFrom` and `AsyncTryInto`

Trait for asynchronous fallible conversions from a type T.

These traits are for fallible asynchronous conversions, where the conversion might fail. They return a `Result` wrapped in a `Future`, allowing you to handle errors gracefully.

```rust
use async_from::{ async_trait, AsyncTryFrom, AsyncTryInto };
use std::num::ParseIntError;

struct MyNumber( u32 );

#[ async_trait ]
impl AsyncTryFrom< String > for MyNumber
{
  type Error = ParseIntError;

  async fn async_try_from( value : String ) -> Result< Self, Self::Error >
  {
    let num = value.parse::< u32 >()?;
    Ok( MyNumber( num ) )
  }
}

#[ tokio::main ]
async fn main()
{
  match MyNumber::async_try_from( "42".to_string() ).await
  {
    Ok( my_num ) => println!( "Converted successfully: {}", my_num.0 ),
    Err( e ) => println!( "Conversion failed: {:?}", e ),
  }
  let result : Result< MyNumber, _ > = "42".to_string().async_try_into().await;
  match result
  {
    Ok( my_num ) => println!( "Converted successfully using AsyncTryInto: {}", my_num.0 ),
    Err( e ) => println!( "Conversion failed using AsyncTryInto: {:?}", e ),
  }
}
```
