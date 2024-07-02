# Module :: variadic_from

<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_variadic_from_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_variadic_from_push.yml) [![docs.rs](https://img.shields.io/docsrs/variadic_from?color=e3e8f0&logo=docs.rs)](https://docs.rs/variadic_from) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fvariadic_from%2Fexamples%2Fvariadic_from_trivial.rs,RUN_POSTFIX=--example%20variadic_from_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

The variadic from is designed to provide a way to implement the From-like traits for structs with a variable number of fields, allowing them to be constructed from tuples of different lengths or from individual arguments. This functionality is particularly useful for creating flexible constructors that enable different methods of instantiation for a struct. By automating the implementation of traits crate reduces boilerplate code and enhances code readability and maintainability.

Currently it support up to 3 arguments. If your structure has more than 3 fields derive generates nothing. Also it supports tuple conversion, allowing structs to be instantiated from tuples by leveraging the `From` and `Into` traits for seamless conversion.

### Basic use-case.

<!-- qqq : for Petro : make this generator working -->
<!--{ example.use{ code : "variadic_from_trivial", hidden_code : "variadic_from_expanded", link : true, try_out : true } }-->
<!--{ example.use.end }-->

This example demonstrates the use of the `variadic_from` macro to implement flexible
constructors for a struct, allowing it to be instantiated from different numbers of
arguments or tuples. It also showcases how to derive common traits like `Debug`,
`PartialEq`, `Default`, and `VariadicFrom` for the struct.

```rust
#[ cfg( not( all(feature = "enabled", feature = "type_variadic_from", feature = "derive_variadic_from" ) ) ) ]
fn main(){}
#[ cfg( all( feature = "enabled", feature = "type_variadic_from", feature = "derive_variadic_from" ) )]
fn main()
{
  use variadic_from::exposed::*;

  // Define a struct `MyStruct` with fields `a` and `b`.
  // The struct derives common traits like `Debug`, `PartialEq`, `Default`, and `VariadicFrom`.
  #[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
  // Use `#[ debug ]` to expand and debug generate code.
  // #[ debug ]
  struct MyStruct
  {
    a : i32,
    b : i32,
  }

  // Implement the `From1` trait for `MyStruct`, which allows constructing a `MyStruct` instance
  // from a single `i32` value by assigning it to both `a` and `b` fields.

  impl From1< i32 > for MyStruct
  {
    fn from1( a : i32 ) -> Self { Self { a, b : a } }
  }

  let got : MyStruct = from!();
  let exp = MyStruct { a : 0, b : 0 };
  assert_eq!( got, exp );

  let got : MyStruct = from!( 13 );
  let exp = MyStruct { a : 13, b : 13 };
  assert_eq!( got, exp );

  let got : MyStruct = from!( 13, 14 );
  let exp = MyStruct { a : 13, b : 14 };
  assert_eq!( got, exp );

  dbg!( exp );
  //> MyStruct {
  //>   a : 13,
  //>   b : 14,
  //> }

}
```

<details>
<summary>The code above will be expanded to this</summary>

```rust
#[ cfg( not( all(feature = "enabled", feature = "type_variadic_from" ) ) ) ]
fn main(){}
#[ cfg( all( feature = "enabled", feature = "type_variadic_from" ) )]
fn main()
{
  use variadic_from::exposed::*;

  // Define a struct `MyStruct` with fields `a` and `b`.
  // The struct derives common traits like `Debug`, `PartialEq`, `Default`
  // `VariadicFrom` defined manually.
  #[ derive( Debug, PartialEq, Default ) ]
  struct MyStruct
  {
    a : i32,
    b : i32,
  }

  // Implement the `From1` trait for `MyStruct`, which allows constructing a `MyStruct` instance
  // from a single `i32` value by assigning it to both `a` and `b` fields.
  impl From1< i32 > for MyStruct
  {
    fn from1( a : i32 ) -> Self { Self { a, b : a } }
  }

  // == begin of generated

  impl From2< i32, i32 > for MyStruct
  {
    fn from2( a : i32, b : i32 ) -> Self { Self{ a : a, b : b } }
  }

  impl From< ( i32, i32 ) > for MyStruct
  {
    #[ inline( always ) ]
    fn from( ( a, b ) : ( i32, i32 ) ) -> Self
    {
      Self::from2( a, b )
    }
  }

  // == end of generated

  let got : MyStruct = from!();
  let exp = MyStruct { a : 0, b : 0 };
  assert_eq!( got, exp );

  let got : MyStruct = from!( 13 );
  let exp = MyStruct { a : 13, b : 13 };
  assert_eq!( got, exp );

  let got : MyStruct = from!( 13, 14 );
  let exp = MyStruct { a : 13, b : 14 };
  assert_eq!( got, exp );

  dbg!( exp );
  //> MyStruct {
  //>   a : 13,
  //>   b : 14,
  //> }

}
```

</details>

Try out `cargo run --example variadic_from_trivial`.
<br/>
[See code](./examples/variadic_from_trivial.rs).

### To add to your project

```sh
cargo add variadic_from
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cargo run --example variadic_from_trivial
```
