<!-- {{# generate.module_header{} #}} -->

# Module :: former
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental)[![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_former_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_former_push.yml)[![docs.rs](https://img.shields.io/docsrs/former?color=e3e8f0&logo=docs.rs)](https://docs.rs/former)[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fformer_trivial%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20former_trivial/https://github.com/Wandalen/wTools)
<!--{ generate.module_header.end }-->

A flexible and extensible implementation of the builder pattern.

It offers specialized subformers for common Rust collections like `Vec`, `HashMap`, and `HashSet`, enabling the construction of complex data structures in a fluent and intuitive manner.

## How Former Works

- **Trait Derivation** : By deriving `Former` on a struct, you automatically generate builder methods for each field.
- **Fluent Interface** : Each field's builder method allows for setting the value of that field and returns a mutable reference to the builder,
  enabling method chaining.
- **Optional Fields** : Optional fields can be easily handled without needing to explicitly set them to `None`.
- **Finalization** : The `.form()` method finalizes the building process and returns the constructed struct instance.

This approach abstracts away the need for manually implementing a builder for each struct, making code more readable and maintainable.

## Basic use-case

The provided code snippet illustrates a basic use-case of the Former crate in Rust, which is used to apply the builder pattern for structured and flexible object creation. Below is a detailed explanation of each part of the markdown chapter, aimed at clarifying how the Former trait simplifies struct instantiation.

```rust
#[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
fn main()
{
  use former::Former;

  #[ derive( Debug, PartialEq, Former ) ]
  #[ perform( fn greet_user() ) ]
  pub struct UserProfile
  {
    #[default(1)]
    age : i32,

    username : String,

    #[alias(bio)]
    bio_optional : Option< String >, // Fields could be optional
  }

  impl UserProfile
  {
    fn greet_user(self) -> Self
    {
      println!("Hello, {}", self.username);
      self
    }
  }

  let profile = UserProfile::former()
  .age( 30 )
  .username( "JohnDoe".to_string() )
  .bio_optional( "Software Developer".to_string() ) // Optionally provide a bio
  .form();
  // .perform(); // same as `form()` but will execute method passed to `perform` attribute

  dbg!( &profile );
  // Expected output:
  // &profile = UserProfile {
  //   age: 30,
  //   username: "JohnDoe",
  //   bio_optional: Some("Software Developer"),
  // }

 }
 ```

<details>
<summary>The code above will be expanded to this</summary>

```rust

#[ derive( Debug, PartialEq ) ]
pub struct UserProfile
{
  age : i32,
  username : String,
  bio_optional : Option< String >, // Fields could be optional
}

impl UserProfile
{
  #[ inline( always ) ]
  pub fn former() -> UserProfileFormer< UserProfile, former::ReturnFormed >
  {
    UserProfileFormer::< UserProfile, former::ReturnFormed >::new()
  }
}

#[ derive( Debug, Default ) ]
pub struct UserProfileFormerStorage
{
  age : Option< i32 >,
  username : Option< String >,
  bio_optional : Option< String >,
}

pub struct UserProfileFormer
<
  FormerContext = UserProfile,
  FormerEnd = former::ReturnFormed,
>
where
  FormerEnd : former::ToSuperFormer< UserProfile, FormerContext >,
{
  storage : UserProfileFormerStorage,
  context : Option< FormerContext >,
  on_end : Option< FormerEnd >,
}

impl< FormerContext, FormerEnd > UserProfileFormer< FormerContext, FormerEnd >
where
  FormerEnd : former::ToSuperFormer< UserProfile, FormerContext >,
{
  #[ inline( always ) ]
  pub fn form( mut self ) -> UserProfile
  {
    let age = if self.storage.age.is_some()
    {
      self.storage.age.take().unwrap()
    }
    else
    {
      let val : i32 =
      {
        trait NotDefault< T >
        {
          fn maybe_default( self : &Self ) -> T { panic!( "Field 'age' isn't initialized" ) }
        }
        trait WithDefault< T >
        {
          fn maybe_default( self : &Self ) -> T;
        }
        impl< T > NotDefault< T > for &::core::marker::PhantomData< T > {}
        impl< T > WithDefault< T > for ::core::marker::PhantomData< T >
        where
          T : ::core::default::Default,
        {
          fn maybe_default( self : &Self ) -> T
          {
            T::default()
          }
        }
        ( &::core::marker::PhantomData::< i32 > ).maybe_default()
      };
      val
    };
    let username = if self.storage.username.is_some()
    {
      self.storage.username.take().unwrap()
    }
    else
    {
      let val : String =
      {
        trait NotDefault< T >
        {
          fn maybe_default( self : &Self ) -> T { panic!( "Field 'username' isn't initialized" ) }
        }
        trait WithDefault< T >
        {
          fn maybe_default( self : &Self ) -> T;
        }
        impl< T > NotDefault< T > for &::core::marker::PhantomData< T > {}
        impl< T > WithDefault< T > for ::core::marker::PhantomData< T >
        where
          T : ::core::default::Default,
        {
          fn maybe_default( self : &Self ) -> T
          {
            T::default()
          }
        }
        ( &::core::marker::PhantomData::< String > ).maybe_default()
      };
      val
    };
    let bio_optional = if self.storage.bio_optional.is_some()
    {
      Option::Some( self.storage.bio_optional.take().unwrap() )
    }
    else
    {
      Option::None
    };
    let result = UserProfile
    {
      age,
      username,
      bio_optional,
    };
    return result;
  }

  #[ inline( always ) ]
  pub fn perform( self ) -> UserProfile
  {
    let result = self.form();
    return result;
  }

  #[ inline( always ) ]
  pub fn new() -> UserProfileFormer< UserProfile, former::ReturnFormed >
  {
    UserProfileFormer::< UserProfile, former::ReturnFormed >::begin( None, former::ReturnFormed )
  }

  #[ inline( always ) ]
  pub fn begin
  (
    context : Option< FormerContext >,
    on_end : FormerEnd,
  ) -> Self
  {
    Self
    {
      storage : core::default::Default::default(),
      context : context,
      on_end : Option::Some( on_end ),
    }
  }

  #[ inline( always ) ]
  pub fn end( mut self ) -> FormerContext
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let formed = self.form();
    on_end.call( formed, context )
  }

  #[ inline ]
  pub fn age< Src >( mut self, src : Src ) -> Self
  where
    Src : Into< i32 >,
  {
    debug_assert!( self.storage.age.is_none() );
    self.storage.age = Option::Some( src.into() );
    self
  }

  #[ inline ]
  pub fn username< Src >( mut self, src : Src ) -> Self
  where
    Src : Into< String >,
  {
    debug_assert!( self.storage.username.is_none() );
    self.storage.username = Option::Some( src.into() );
    self
  }

  #[ inline ]
  pub fn bio_optional< Src >( mut self, src : Src ) -> Self
  where
    Src : Into< String >,
  {
    debug_assert!( self.storage.bio_optional.is_none() );
    self.storage.bio_optional = Option::Some( src.into() );
    self
  }
}

let profile = UserProfile::former()
.age( 30 )
.username( "JohnDoe".to_string() )
.bio_optional( "Software Developer".to_string() )
.form();

dbg!( &profile );
// Expected output:
// &profile = UserProfile {
//   age: 30,
//   username: "JohnDoe",
//   bio_optional: Some("Software Developer"),
// }

```

</details>

### Custom and Alternative Setters

With help of `Former`, it is possible to define multiple versions of a setter for a single field, providing the flexibility to include custom logic within the setter methods. This feature is particularly useful when you need to preprocess data or enforce specific constraints before assigning values to fields. Custom setters should have unique names to differentiate them from the default setters generated by `Former`, allowing for specialized behavior while maintaining clarity in your code.

```rust
# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# {

use former::Former;

/// Structure with a custom setter.
#[ derive( Debug, Former ) ]
pub struct StructWithCustomSetters
{
  word : String,
}

impl StructWithCustomSettersFormer
{

  // Custom alternative setter for `word`
  pub fn word_exclaimed( mut self, value : impl Into< String > ) -> Self
  {
    debug_assert!( self.storage.word.is_none() );
    self.storage.word = Some( format!( "{}!", value.into() ) );
    self
  }

}

let example = StructWithCustomSetters::former()
.word( "Hello" )
.form();
assert_eq!( example.word, "Hello".to_string() );

let example = StructWithCustomSetters::former()
.word_exclaimed( "Hello" )
.form();
assert_eq!( example.word, "Hello!".to_string() );

# }
```

In the example above showcases a custom alternative setter, `word_exclaimed`, which appends an exclamation mark to the input string before storing it. This approach allows for additional processing or validation of the input data without compromising the simplicity of the builder pattern.

### Custom Setter Overriding

But it's also possible to completely override setter and write its own from scratch. For that use attribe `[ setter( false ) ]` to disable setter.

```rust
# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# {

use former::Former;

/// Structure with a custom setter.
#[ derive( Debug, Former ) ]
pub struct StructWithCustomSetters
{
  #[ setter( false ) ]
  word : String,
}

impl StructWithCustomSettersFormer
{

  // Custom alternative setter for `word`
  pub fn word( mut self, value : impl Into< String > ) -> Self
  {
    debug_assert!( self.storage.word.is_none() );
    self.storage.word = Some( format!( "{}!", value.into() ) );
    self
  }

}

let example = StructWithCustomSetters::former()
.word( "Hello" )
.form();
assert_eq!( example.word, "Hello!".to_string() );
# }
```

In the example above, the default setter for `word` is disabled, and a custom setter is defined to automatically append an exclamation mark to the string. This method allows for complete control over the data assignment process, enabling the inclusion of any necessary logic or validation steps.

## Custom Default

The `Former` crate enhances struct initialization in Rust by allowing the specification of custom default values for fields through the `default` attribute. This feature not only provides a way to set initial values for struct fields without relying on the `Default` trait but also adds flexibility in handling cases where a field's type does not implement `Default`, or a non-standard default value is desired.

```rust
# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# {

use former::Former;

/// Structure with default attributes.
#[ derive(  Debug, PartialEq, Former ) ]
pub struct ExampleStruct
{
  #[ default( 5 ) ]
  number : i32,
  #[ default( "Hello, Former!".to_string() ) ]
  greeting : String,
  #[ default( vec![ 10, 20, 30 ] ) ]
  numbers : Vec< i32 >,
}

let instance = ExampleStruct::former().form();
let expected = ExampleStruct
{
  number : 5,
  greeting : "Hello, Former!".to_string(),
  numbers : vec![ 10, 20, 30 ],
};
assert_eq!( instance, expected );
dbg!( &instance );
// > &instance = ExampleStruct {
// >    number: 5,
// >    greeting: "Hello, Former!",
// >    numbers: [
// >        10,
// >        20,
// >        30,
// >    ],
// > }
# }
```

The above code snippet showcases the `Former` crate's ability to initialize struct fields with custom default values:
- The `number` field is initialized to `5`.
- The `greeting` field defaults to a greeting message, "Hello, Former!".
- The `numbers` field starts with a vector containing the integers `10`, `20`, and `30`.

This approach significantly simplifies struct construction, particularly for complex types or where defaults beyond the `Default` trait's capability are required. By utilizing the `default` attribute, developers can ensure their structs are initialized safely and predictably, enhancing code clarity and maintainability.

## Concept of subformer

Subformers are specialized builders used within the `Former` framework to construct nested or collection-based data structures like vectors, hash maps, and hash sets. They simplify the process of adding elements to these structures by providing a fluent interface that can be seamlessly integrated into the overall builder pattern of a parent struct. This approach allows for clean and intuitive initialization of complex data structures, enhancing code readability and maintainability.

### Subformer example: Building a Vector

The following example illustrates how to use a `VectorSubformer` to construct a `Vec` field within a struct. The subformer enables adding elements to the vector with a fluent interface, streamlining the process of populating collection fields within structs.

```rust
# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# #[ cfg( not( feature = "no_std" ) ) ]
# {

#[ derive( Debug, PartialEq, former::Former ) ]
pub struct StructWithVec
{
  #[ subformer( former::VectorSubformer ) ]
  vec : Vec< &'static str >,
}

let instance = StructWithVec::former()
.vec()
  .push( "apple" )
  .push( "banana" )
  .end()
.form();

assert_eq!( instance, StructWithVec { vec: vec![ "apple", "banana" ] } );
# }
```

### Subformer example: Building a Hashmap

This example demonstrates the use of a `HashMapSubformer` to build a hash map within a struct. The subformer provides a concise way to insert key-value pairs into the map, making it easier to manage and construct hash map fields.

```rust
# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# #[ cfg( not( feature = "no_std" ) ) ]
# {

use test_tools::exposed::*;

#[ derive( Debug, PartialEq, former::Former ) ]
pub struct StructWithMap
{
  #[ subformer( former::HashMapSubformer ) ]
  map : std::collections::HashMap< &'static str, &'static str >,
}

let struct1 = StructWithMap::former()
.map()
  .insert( "a", "b" )
  .insert( "c", "d" )
  .end()
.form()
;
assert_eq!( struct1, StructWithMap { map : hmap!{ "a" => "b", "c" => "d" } } );
# }
```

### Subformer example: Building a Hashset

In the following example, a `HashSetSubformer` is utilized to construct a hash set within a struct. This illustrates the convenience of adding elements to a set using the builder pattern facilitated by subformers.

```rust
# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# #[ cfg( not( feature = "no_std" ) ) ]
# {

use test_tools::exposed::*;

#[ derive( Debug, PartialEq, former::Former ) ]
pub struct StructWithSet
{
  #[ subformer( former::HashSetSubformer ) ]
  set : std::collections::HashSet< &'static str >,
}

let instance = StructWithSet::former()
.set()
  .insert("apple")
  .insert("banana")
  .end()
.form();

assert_eq!(instance, StructWithSet { set : hset![ "apple", "banana" ] });
# }
```

### Custom Subformer

It is possible to use former of one structure to construct field of another one and integrate it into former of the last one.

The example below illustrates how to incorporate the builder pattern of one structure as a subformer in another, enabling nested struct initialization within a single fluent interface.


Example of how to use former of another structure as subformer of former of current one
function `command` integrate `CommandFormer` into `AggregatorFormer`.

```rust
# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# {

fn main()
{
  use std::collections::HashMap;
  use former::Former;

  // Command struct with Former derived for builder pattern support
  #[ derive( Debug, PartialEq, Former ) ]
  pub struct Command
  {
    name : String,
    description : String,
  }

  // Aggregator struct to hold commands
  #[ derive( Debug, PartialEq, Former ) ]
  pub struct Aggregator
  {
    #[ setter( false ) ]
    command : HashMap< String, Command >,
  }

  // Use CommandFormer as custom subformer for AggregatorFormer to add commands by name.
  impl< Context, End > AggregatorFormer< Context, End >
  where
    End : former::ToSuperFormer< Aggregator, Context >,
  {
    pub fn command< IntoName >( self, name : IntoName ) -> CommandFormer< Self, impl former::ToSuperFormer< Command, Self > >
    where
      IntoName: core::convert::Into< String >,
    {
      let on_end = | command : Command, super_former : core::option::Option< Self > | -> Self
      {
        let mut super_former = super_former.unwrap();
        if let Some( ref mut commands ) = super_former.storage.command
        {
          commands.insert( command.name.clone(), command );
        }
        else
        {
          let mut commands: HashMap< String, Command > = Default::default();
          commands.insert( command.name.clone(), command );
          super_former.storage.command = Some( commands );
        }
        super_former
      };
      let former = CommandFormer::begin( None, Some( self ), on_end );
      former.name( name )
    }
  }

  let ca = Aggregator::former()
  .command( "echo" )
    .description( "prints all subjects and properties" ) // sets additional properties using custom subformer
    .end()
  .command( "exit" )
    .description( "just exit" ) // Sets additional properties using using custom subformer
    .end()
  .form();

  dbg!( &ca );
  // > &ca = Aggregator {
  // >     command: {
  // >          "echo": Command {
  // >              name: "echo",
  // >              description: "prints all subjects and properties",
  // >          },
  // >          "exit": Command {
  // >              name: "exit",
  // >              description: "just exit",
  // >          },
  // >     },
  // > }
}
# }
```

In this example, the `Aggregator` struct functions as a container for multiple `Command` structs, each identified by a unique command name. The `AggregatorFormer` implements a custom method `command`, which serves as a subformer for adding `Command` instances into the `Aggregator`.

- **Command Definition**: Each `Command` consists of a `name` and a `description`, and we derive `Former` to enable easy setting of these properties using a builder pattern.
- **Aggregator Definition**: It holds a collection of `Command` objects in a `HashMap`. The `#[setter(false)]` attribute is used to disable the default setter, and a custom method `command` is defined to facilitate the addition of commands with specific attributes.
- **Custom Subformer Integration**: The `command` method in the `AggregatorFormer` initializes a `CommandFormer` with a closure that integrates the `Command` into the `Aggregator`'s `command` map upon completion.

This pattern of using a structure's former as a subformer within another facilitates the creation of deeply nested or complex data structures through a coherent and fluent interface, showcasing the powerful capabilities of the `Former` framework for Rust applications.

### To add to your project

```sh
cargo add former
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/former_trivial
cargo run
```
