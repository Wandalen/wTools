<!-- {{# generate.module_header{} #}} -->

# Module :: former_types

<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_former_types_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_former_types_push.yml) [![docs.rs](https://img.shields.io/docsrs/former_types?color=e3e8f0&logo=docs.rs)](https://docs.rs/former_types) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fformer_types%2Fexamples%2Fformer_types_trivial.rs,RUN_POSTFIX=--example%20former_types_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

A flexible and extensible implementation of the builder pattern. Its compile-time structures and traits that are not generated but reused.

## Example: Using Trait ComponentAssign

Demonstrates setting various components (fields) of a struct.

The `former_types` crate provides a generic interface for setting components on an object. This example defines a `Person` struct
and implements the `ComponentAssign` trait for its fields. It shows how to use these implementations to set the fields of a `Person`
instance using different types that can be converted into the required types.

```rust
#[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
fn main() {}

#[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
fn main()
{
  use former_types::ComponentAssign;

  #[ derive( Default, PartialEq, Debug ) ]
  struct Person
  {
    age : i32,
    name : String,
  }

  impl< IntoT > ComponentAssign< i32, IntoT > for Person
  where
    IntoT : Into< i32 >,
  {
    fn assign( &mut self, component : IntoT )
    {
      self.age = component.into();
    }
  }

  impl< IntoT > ComponentAssign< String, IntoT > for Person
  where
    IntoT : Into< String >,
  {
    fn assign( &mut self, component : IntoT )
    {
      self.name = component.into();
    }
  }

  let mut got : Person = Default::default();
  got.assign( 13 );
  got.assign( "John" );
  assert_eq!( got, Person { age : 13, name : "John".to_string() } );
  dbg!( got );
  // > Person {
  // >   age: 13,
  // >   name: "John",
  // > }

}
```

Try out `cargo run --example former_types_trivial`.
<br/>
[See code](./examples/former_types_trivial.rs).
