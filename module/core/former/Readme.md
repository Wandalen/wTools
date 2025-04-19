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
    buffer_size : usize,
    timeout : Option< u32 >, // Defaults to None
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

*   **[Advanced Usage & Concepts](./advanced.md):** Dive deeper into subformers, customization options, storage, context, definitions, mutators, and custom collections. *(Link will work once `advanced.md` is created)*
*   **[Examples Directory](https://github.com/Wandalen/wTools/tree/master/module/core/former/examples):** Explore practical, runnable examples showcasing various features.
*   **[API Documentation (docs.rs)](https://docs.rs/former):** Get detailed information on all public types, traits, and functions.
*   **[Repository (GitHub)](https://github.com/Wandalen/wTools/tree/master/module/core/former):** View the source code, contribute, or report issues.
