<!-- {{# generate.module_header{} #}} -->

# Module :: former
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_former_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_former_push.yml) [![docs.rs](https://img.shields.io/docsrs/former?color=e3e8f0&logo=docs.rs)](https://docs.rs/former) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fformer%2Fexamples%2Fformer_trivial.rs,RUN_POSTFIX=--example%20module%2Fcore%2Fformer%2Fexamples%2Fformer_trivial.rs/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

A flexible implementation of the Builder pattern supporting nested builders and collection-specific subformers.

## What is `Former`?

The `former` crate provides a powerful derive macro, `#[ derive( Former ) ]`, that automatically implements the **Builder pattern** for your Rust structs and enums.

Its primary goal is to **simplify the construction of complex objects**, especially those with numerous fields, optional values, default settings, collections, or nested structures, making your initialization code more readable and maintainable.

## Why Use `Former`?

Compared to manually implementing the Builder pattern or using other builder crates, `former` offers several advantages:

*   **Reduced Boilerplate:** `#[ derive( Former ) ]` automatically generates the builder struct, storage, and setters, saving you significant repetitive coding effort.
*   **Fluent & Readable API:** Construct objects step-by-step using clear, chainable methods (`.field_name( value )`).
*   **Effortless Defaults & Optionals:** Fields automatically use their `Default` implementation if not set. `Option< T >` fields are handled seamlessly – you only set them if you have a `Some( value )`. Custom defaults can be specified easily with `#[ former( default = ... ) ]`.
*   **Powerful Collection & Nested Struct Handling:** `former` truly shines with its **subformer** system. Easily build `Vec`, `HashMap`, `HashSet`, and other collections element-by-element, or configure nested structs using their own dedicated formers within the parent's builder chain. This is often more complex to achieve with other solutions.

## Installation

Add `former` to your `Cargo.toml`:

```sh
cargo add former
```

The default features enable the `Former` derive macro and support for standard collections, covering most common use cases.

## Basic Usage

Derive `Former` on your struct and use the generated `::former()` method to start building:

```rust
# #[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
# fn main() {}
# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# fn main()
# {
  use former::Former;

  #[ derive( Debug, PartialEq, Former ) ]
  pub struct UserProfile
  {
    age : i32, // Required field
    username : String, // Required field
    bio : Option< String >, // Optional field
  }

  let profile = UserProfile::former()
  .age( 30 )
  .username( "JohnDoe".to_string() )
  // .bio is optional, so we don't *have* to call its setter
  .form();

  let expected = UserProfile
  {
    age : 30,
    username : "JohnDoe".to_string(),
    bio : None, // Defaults to None if not set
  };
  assert_eq!( profile, expected );
  dbg!( &profile );
  // > &profile = UserProfile {
  // >     age: 30,
  // >     username: "JohnDoe",
  // >     bio: None,
  // > }

  // Example setting the optional field:
  let profile_with_bio = UserProfile::former()
  .age( 30 )
  .username( "JohnDoe".to_string() )
  .bio( "Software Developer".to_string() ) // Set the optional bio
  .form();

  let expected_with_bio = UserProfile
  {
    age : 30,
    username : "JohnDoe".to_string(),
    bio : Some( "Software Developer".to_string() ),
  };
  assert_eq!( profile_with_bio, expected_with_bio );
  dbg!( &profile_with_bio );
  // > &profile_with_bio = UserProfile {
  // >     age: 30,
  // >     username: "JohnDoe",
  // >     bio: Some( "Software Developer" ),
  // > }
# }
```

[Run this example locally](https://github.com/Wandalen/wTools/blob/master/module/core/former/examples/former_trivial.rs) | [Try it online](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fformer%2Fexamples%2Fformer_trivial.rs,RUN_POSTFIX=--example%20module%2Fcore%2Fformer%2Fexamples%2Fformer_trivial.rs/https://github.com/Wandalen/wTools)

## Handling Optionals and Defaults

`Former` makes working with optional fields and default values straightforward:

*   **`Option< T >` Fields:** As seen in the basic example, fields of type `Option< T >` automatically default to `None`. You only need to call the setter if you have a `Some( value )`.

*   **Custom Defaults:** For required fields that don't implement `Default`, or when you need a specific default value other than the type's default, use the `#[ former( default = ... ) ]` attribute:

```rust
# #[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
# fn main() {}
# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# fn main()
# {
  use former::Former;

  #[ derive( Debug, PartialEq, Former ) ]
  pub struct Config
  {
    #[ former( default = 1024 ) ] // Use 1024 if .buffer_size() is not called
    buffer_size : i32,
    timeout : Option< i32 >, // Defaults to None
    #[ former( default = true ) ] // Default for bool
    enabled : bool,
  }

  // Only set the optional timeout
  let config1 = Config::former()
  .timeout( 5000 )
  .form();

  assert_eq!( config1.buffer_size, 1024 ); // Got default
  assert_eq!( config1.timeout, Some( 5000 ) );
  assert_eq!( config1.enabled, true ); // Got default

  // Set everything, overriding defaults
  let config2 = Config::former()
  .buffer_size( 4096 )
  .timeout( 1000 )
  .enabled( false )
  .form();

  assert_eq!( config2.buffer_size, 4096 );
  assert_eq!( config2.timeout, Some( 1000 ) );
  assert_eq!( config2.enabled, false );
# }
```
[See full example code](https://github.com/Wandalen/wTools/blob/master/module/core/former/examples/former_custom_defaults.rs)

## Building Collections & Nested Structs (Subformers)

Where `former` significantly simplifies complex scenarios is in building collections (`Vec`, `HashMap`, etc.) or nested structs. It achieves this through **subformers**. Instead of setting the entire collection/struct at once, you get a dedicated builder for the field:

**Example: Building a `Vec`**

```rust
# #[ cfg( not( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ) ]
# fn main() {}
# #[ cfg( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ]
# fn main()
# {
  use former::Former;

  #[ derive( Debug, PartialEq, Former ) ]
  pub struct Report
  {
    title : String,
    #[ subform_collection ] // Enables the `.entries()` subformer
    entries : Vec< String >,
  }

  let report = Report::former()
  .title( "Log Report".to_string() )
  .entries() // Get the subformer for the Vec
    .add( "Entry 1".to_string() ) // Use subformer methods to modify the Vec
    .add( "Entry 2".to_string() )
    .end() // Return control to the parent former (ReportFormer)
  .form(); // Finalize the Report

  assert_eq!( report.title, "Log Report" );
  assert_eq!( report.entries, vec![ "Entry 1".to_string(), "Entry 2".to_string() ] );
  dbg!( &report );
  // > &report = Report {
  // >     title: "Log Report",
  // >     entries: [
  // >         "Entry 1",
  // >         "Entry 2",
  // >     ],
  // > }
# }
```
[See Vec example](https://github.com/Wandalen/wTools/blob/master/module/core/former/examples/former_collection_vector.rs) | [See HashMap example](https://github.com/Wandalen/wTools/blob/master/module/core/former/examples/former_collection_hashmap.rs)

`former` provides different subform attributes (`#[ subform_collection ]`, `#[ subform_entry ]`, `#[ subform_scalar ]`) for various collection and nesting patterns.

## Standalone Constructors

For scenarios where you want a direct constructor function instead of always starting with `YourType::former()`, `former` offers standalone constructors.

*   **Enable:** Add `#[ standalone_constructors ]` to your struct or enum definition.
*   **Function Name:** A function named after your type (in snake_case) will be generated (e.g., `my_struct()` for `struct MyStruct`). For enums, functions are named after variants (e.g., `my_variant()` for `enum E { MyVariant }`).
*   **Arguments:** By default, the constructor takes no arguments and returns the `Former` type.
*   **Specify Arguments:** Mark specific fields with `#[ arg_for_constructor ]` to make them required arguments for the standalone constructor.
*   **Return Type (Option 2 Logic):**
    *   If **all** fields of the struct/variant are marked with `#[ arg_for_constructor ]`, the standalone constructor returns the instance directly (`Self`).
    *   If **zero or some** fields are marked, the standalone constructor returns the `Former` type, pre-initialized with the provided arguments.

**Example: Struct Standalone Constructors**

```rust
# #[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
# fn main() {}
# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# fn main()
# {
  use former::Former;

  #[ derive( Debug, PartialEq, Former ) ]
  #[ standalone_constructors ] // Enable standalone constructors
  pub struct ServerConfig
  {
    #[ arg_for_constructor ] // This field is a constructor arg
    host : String,
    #[ arg_for_constructor ] // This field is also a constructor arg
    port : u16,
    timeout : Option< u32 >, // This field is NOT a constructor arg
  }

  // Not all fields are args, so `server_config` returns the Former
  let config_former = server_config( "localhost".to_string(), 8080u16 ); // Added u16 suffix

  // Set the remaining field and form
  let config = config_former
  .timeout( 5000u32 ) // Added u32 suffix
  .form();

  assert_eq!( config.host, "localhost" );
  assert_eq!( config.port, 8080u16 ); // Added u16 suffix
  assert_eq!( config.timeout, Some( 5000u32 ) ); // Added u32 suffix

  #[ derive( Debug, PartialEq, Former ) ]
  #[ standalone_constructors ]
  pub struct Point
  {
    #[ arg_for_constructor ]
    x : i32,
    #[ arg_for_constructor ]
    y : i32,
  }

  // ALL fields are args, so `point` returns Self directly
  let p = point( 10, 20 );
  assert_eq!( p.x, 10 );
  assert_eq!( p.y, 20 );
# }
```

**Example: Enum Standalone Constructors**

```rust
# #[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
# fn main() {}
# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# fn main()
# {
  use former::Former;

  #[ derive( Debug, PartialEq, Former ) ]
  #[ standalone_constructors ]
  pub enum Message
  {
    Quit, // Unit variant constructor `quit()` returns Self
    Write // Tuple variant constructor `write()` returns Former
    {
      #[ arg_for_constructor ] // Only this field is an arg
      text : String,
      urgent : bool, // Not an arg
    },
    Move // Struct variant constructor `move_point()` returns Self
    {
      #[ arg_for_constructor ]
      x : i32,
      #[ arg_for_constructor ]
      y : i32,
    }
  }

  // Unit variant - returns Self
  let m1 = quit();
  assert_eq!( m1, Message::Quit );

  // Tuple variant - not all fields are args, returns Former
  let m2_former = write( "hello".to_string() );
  let m2 = m2_former.urgent( true ).form();
  assert_eq!( m2, Message::Write { text: "hello".to_string(), urgent: true } );

  // Struct variant - all fields are args, returns Self
  let m3 = r#move( 1, 2 ); // Use raw identifier `r#move` as `move` is a keyword
  assert_eq!( m3, Message::Move { x: 1, y: 2 } );
# }
```

## Key Features Overview

*   **Automatic Builder Generation:** `#[ derive( Former ) ]` for structs and enums.
*   **Fluent API:** Chainable setter methods for a clean construction flow.
*   **Defaults & Optionals:** Seamless handling of `Default` values and `Option< T >` fields. Custom defaults via `#[ former( default = ... ) ]`.
*   **Subformers:** Powerful mechanism for building nested structures and collections:
    *   `#[ subform_scalar ]`: For fields whose type also derives `Former`.
    *   `#[ subform_collection ]`: For collections like `Vec`, `HashMap`, `HashSet`, etc., providing methods like `.add()` or `.insert()`.
    *   `#[ subform_entry ]`: For collections where each entry is built individually using its own former.
*   **Customization:**
    *   Rename setters: `#[ scalar( name = ... ) ]`, `#[ subform_... ( name = ... ) ]`.
    *   Disable default setters: `#[ scalar( setter = false ) ]`, `#[ subform_... ( setter = false ) ]`.
    *   Define custom setters directly in `impl Former`.
    *   Specify collection definitions: `#[ subform_collection( definition = ... ) ]`.
*   **Advanced Control:**
    *   Storage-only fields: `#[ storage_fields( ... ) ]`.
    *   Custom mutation logic: `#[ mutator( custom ) ]` + `impl FormerMutator`.
    *   Custom end-of-forming logic: Implement `FormingEnd`.
    *   Custom collection support: Implement `Collection` traits.
*   **Component Model:** Separate derives (`Assign`, `ComponentFrom`, `ComponentsAssign`, `FromComponents`) for type-based field access and conversion (See `former_types` documentation).

## Where to Go Next

*   **[Advanced Usage & Concepts](https://github.com/Wandalen/wTools/tree/master/module/core/former/advanced.md):** Dive deeper into subformers, customization options, storage, context, definitions, mutators, and custom collections.
*   **[Examples Directory](https://github.com/Wandalen/wTools/tree/master/module/core/former/examples):** Explore practical, runnable examples showcasing various features.
*   **[API Documentation (docs.rs)](https://docs.rs/former):** Get detailed information on all public types, traits, and functions.
*   **[Repository (GitHub)](https://github.com/Wandalen/wTools/tree/master/module/core/former):** View the source code, contribute, or report issues.
