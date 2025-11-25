<!-- {{# generate.module_header{} #}} -->

# Module :: former
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_former_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_former_push.yml) [![docs.rs](https://img.shields.io/docsrs/former?color=e3e8f0&logo=docs.rs)](https://docs.rs/former) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fformer%2Fexamples%2Fformer_trivial.rs,RUN_POSTFIX=--example%20module%2Fcore%2Fformer%2Fexamples%2Fformer_trivial.rs/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

A flexible implementation of the Builder pattern supporting nested builders and collection-specific subformers. Comprehensive struct support with enum support under active development.

## What is `Former`?

The `former` crate provides a powerful derive macro, `#[ derive( Former ) ]`, that automatically implements the **Builder pattern** for your Rust structs and enums.

Its primary goal is to **simplify the construction of complex objects**, especially those with numerous fields, optional values, default settings, collections, and nested structures, making your initialization code more readable and maintainable.

**Current Status**: Struct support is fully functional and production-ready. Enum support is actively developed with 227 total tests passing, including functional unit variants, tuple variants, and multi-field patterns. Some advanced features like `#[arg_for_constructor]` are still under development.

## Why Use `Former`?

Compared to manually implementing the Builder pattern or using other builder crates, `former` offers several advantages:

*   **Reduced Boilerplate:** `#[ derive( Former ) ]` automatically generates the builder struct, storage, and setters, saving you significant repetitive coding effort.
*   **Fluent & Readable API:** Construct objects step-by-step using clear, chainable methods (`.field_name( value )`).
*   **Comprehensive Struct Support:** Fully implemented builder pattern for structs with automatic generation of setters, defaults, and subformers
*   **Effortless Defaults & Optionals:** Fields automatically use their `Default` implementation if not set. `Option< T >` fields are handled seamlessly – you only set them if you have a `Some( value )`. Custom defaults can be specified easily with `#[ former( default = ... ) ]`.
*   **Powerful Collection & Nested Struct Handling:** `former` truly shines with its **subformer** system. Easily build `Vec`, `HashMap`, `HashSet`, and other collections element-by-element, or configure nested structs using their own dedicated formers within the parent's builder chain. This is often more complex to achieve with other solutions.

## Installation

Add `former` to your `Cargo.toml`:

```sh
cargo add former
```

The default features enable the `Former` derive macro and support for standard collections, covering most common use cases.

## When to Use Former

Former is designed for building complex, nested data structures with compile-time guarantees. Consider using Former when:

✅ **Use Former when you need:**
- Nested builders (`Parent::former().child().field(x).end()`)
- Collection building (adding items one-by-one with dedicated subformers)
- Complex validation logic with custom mutators
- Subform composition for hierarchical data
- Compile-time type-safe construction

❌ **Consider simpler alternatives when:**
- Building simple flat structs (< 5 fields, no nesting)
- No custom defaults or validation needed
- Just need a basic `.build()` method

### Decision Matrix

| Fields | Nesting | Collections | Recommendation |
|--------|---------|-------------|----------------|
| < 5    | No      | No          | `typed-builder` or manual |
| 5-10   | No      | No          | Either Former or `typed-builder` |
| Any    | Yes     | -           | **Former** |
| Any    | -       | Yes         | **Former** |

**The ROI**: Former's additional complexity pays off when you have ~5+ fields with nesting or need collection builders. For simple cases, the overhead may not be justified.

### Known Limitations

Former has some architectural limitations you should be aware of:

- **Lifetimes**: Cannot use borrowed data (`&'a T`) in Former structs. Use owned types (`String`, `Vec`) or smart pointers (`Arc`, `Cow`). [See workarounds](limitations.md#lifetime-constraints).
- **Generic Enums**: Currently not supported due to parser limitations. Use concrete types instead. [Details](limitations.md#generic-enum-parsing).
- **Multi-Variant Enums**: May encounter trait conflicts in complex scenarios. Single-variant enums work perfectly. [Details](limitations.md#trait-conflicts).

📖 **Full limitations documentation**: [limitations.md](limitations.md)

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

```rust,ignore
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
    #[ subform_collection( definition = former::VectorDefinition ) ] // Enables the `.entries()` subformer
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
*   **Function Name:** A function named after your type (in `snake_case`) will be generated (e.g., `my_struct()` for `struct MyStruct`). For enums, functions are named after variants (e.g., `my_variant()` for `enum E { MyVariant }`).
*   **Arguments:** By default, all fields become constructor arguments.
*   **Exclude Arguments:** Mark specific fields with `#[ former_ignore ]` to exclude them from constructor arguments.
*   **Return Type Logic:**
    *   If **no** fields are marked with `#[ former_ignore ]`, the standalone constructor takes all fields as arguments and returns the instance directly (`Self`).
    *   If **any** fields are marked with `#[ former_ignore ]`, the standalone constructor takes only non-ignored fields as arguments and returns the `Former` type.

**Example: Struct Standalone Constructors**

```rust,ignore
# #[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
# fn main() {}
# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# fn main()
# {
  use former::Former;

  #[ derive( Debug, PartialEq ) ] // Former not yet implemented for standalone_constructors
  // #[ standalone_constructors ] // Enable standalone constructors
  pub struct ServerConfig
  {
    host : String,     // Will be constructor arg
    port : u16,        // Will be constructor arg  
    #[ former_ignore ] // This field is NOT a constructor arg
    timeout : Option< u32 >,
  }

  // Some fields ignored, so `server_config` returns the Former
  let config_former = server_config( "localhost".to_string(), 8080u16 ); // Added u16 suffix

  // Set the ignored field and form
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
    x : i32,  // Will be constructor arg
    y : i32,  // Will be constructor arg
  }

  // NO fields ignored, so `point` returns Self directly
  let p = point( 10, 20 );
  assert_eq!( p.x, 10 );
  assert_eq!( p.y, 20 );
# }
```

**Example: Enum Standalone Constructors**

<!-- qqq : xxx : fix it -->
<!-- ```rust
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
``` -->

## Vocabulary & Terminology

Understanding the terminology used in `former` will help you leverage its full potential, especially when working with enums and variants:

### Core Concepts

*   **`Former`:** A builder object that accumulates field values and produces the final instance via `.form()`.
*   **`Storage`:** Internal structure that holds the building state, containing options for each field.
*   **`Subformer`:** A specialized former for building nested structures, collections, or complex field types.
*   **`FormingEnd`:** A mechanism that controls what happens when `.form()` is called on a (sub)former.

### Variant Types (for Enums)

*   **Unit Variant:** An enum variant with no associated data (e.g., `Status::Active`).
*   **Tuple Variant:** An enum variant with unnamed fields in parentheses (e.g., `Message::Error(String)`, `Point::Coords(i32, i32)`).
*   **Struct Variant:** An enum variant with named fields in braces (e.g., `Request::Get { url: String, headers: Vec<String> }`).

### Variant Field Categories

*   **Zero-Field Variant:** A variant with no fields - can be unit (`Status::Active`) or empty tuple (`Status::Active()`).
*   **Single-Field Variant:** A variant with exactly one field (e.g., `Message::Text(String)` or `User::Profile { name: String }`).
*   **Multi-Field Variant:** A variant with multiple fields (e.g., `Point::Coords(i32, i32)` or `Request::Post { url: String, body: String }`).

### Constructor Types

*   **Scalar Constructor:** A method that takes direct values and immediately returns the enum instance (e.g., `Message::text("hello")` → `Message::Text("hello")`).
*   **Subform Constructor:** A method that returns a former/builder for constructing the variant step-by-step, useful for complex variants.
*   **Direct Constructor:** Simple constructor for variants with no fields (e.g., `Status::active()` → `Status::Active`).

### Enum Constructor Patterns

*   **Method-style Constructor:** Instance methods on the enum type (e.g., `MyEnum::variant_name(...)`).
*   **Standalone Constructor:** Top-level functions generated when `#[standalone_constructors]` is used (e.g., `variant_name(...)`).

### Variant Attributes

*   **`#[scalar]`:** Forces generation of a scalar constructor that takes field values directly and returns the enum instance.
*   **`#[subform_scalar]`:** For single-field variants where the field type implements `Former` - generates a method returning the field's former.
*   **`#[standalone_constructors]`:** Applied to the enum itself, generates top-level constructor functions for each variant.
*   **`#[former_ignore]`:** Applied to individual fields, excludes them from being parameters in standalone constructors.

### Advanced Concepts

*   **Implicit Variant Former:** An automatically generated former for variants with multiple fields, providing individual field setters.
*   **End-of-forming Logic:** Custom behavior when a former completes, enabling advanced patterns like validation or transformation.
*   **Context Propagation:** Mechanism for passing data through nested formers in complex builder hierarchies.

## Key Features Overview

*   **Automatic Builder Generation:** `#[ derive( Former ) ]` for structs (enums under development).
*   **Fluent API:** Chainable setter methods for a clean construction flow.
*   **Production-Ready Struct Support:** Complete implementation with all features working:
    *   **Field setters:** Individual setter methods for each field
    *   **Default handling:** Automatic use of `Default` trait or custom defaults
    *   **Optional fields:** Seamless `Option<T>` support
    *   **Subformers:** Nested builders for complex field types
*   **Defaults & Optionals:** Seamless handling of `Default` values and `Option< T >` fields. Custom defaults via `#[ former( default = ... ) ]`.
*   **Collection & Nested Struct Support:** Powerful subformer system for building complex structures:
    *   `#[ subform_scalar ]`: For fields whose type also derives `Former`
    *   `#[ subform_collection ]`: For collections like `Vec`, `HashMap`, `HashSet`, etc., providing methods like `.add()` or `.insert()`
    *   `#[ subform_entry ]`: For collections where each entry is built individually using its own former
*   **Enum Support (Active Development):** Comprehensive implementation with working functionality:
    *   **Unit variants:** Direct constructors (e.g., `MyEnum::variant()`) - Fully functional
    *   **Tuple variants:** Scalar constructors and subformers based on field count and attributes - Core patterns working
    *   **Struct variants:** Subformers with individual field setters or scalar constructors - Core patterns working
    *   **Flexible attributes:** `#[scalar]`, `#[subform_scalar]`, `#[standalone_constructors]` for fine-grained control
    *   **Known limitations:** Single-field tuple variants with primitives require explicit `#[scalar]` attribute, `#[former_ignore]` not yet implemented
*   **Customization:**
    *   Rename setters: `#[ scalar( name = ... ) ]`, `#[ subform_... ( name = ... ) ]`
    *   Disable default setters: `#[ scalar( setter = false ) ]`, `#[ subform_... ( setter = false ) ]`
    *   Define custom setters directly in `impl Former`
    *   Specify collection definitions: `#[ subform_collection( definition = ... ) ]`
*   **Advanced Control:**
    *   Storage-only fields: `#[ storage_fields( ... ) ]`.
    *   Custom mutation logic: `#[ mutator( custom ) ]` + `impl FormerMutator`.
    *   Custom end-of-forming logic: Implement `FormingEnd`.
    *   Custom collection support: Implement `Collection` traits.

## Troubleshooting

### Common Issues

**"Missing Former types" Error**
- **Symptom**: Errors like `BreakFormer not found` or `RunFormerDefinition not found`
- **Cause**: Required struct types don't have `#[derive(Former)]` enabled
- **Solution**: Check for commented-out `// #[derive(Debug, Clone, PartialEq, former::Former)]` and uncomment them
- **Note**: Historical "trailing comma issue" has been resolved - Former derive works correctly now

**Raw Identifier Compilation Errors**
- **Symptom**: Panic with error like `"KeywordVariantEnumr#breakFormerStorage" is not a valid identifier`
- **Cause**: Bug in enum variant handling with raw identifiers (e.g., `r#break`, `r#move`)
- **Workaround**: Use explicit `#[scalar]` attribute on variants with keyword identifiers
- **Status**: Known issue with utility functions available but not fully integrated

**Inner Doc Comment Errors (E0753)**
- **Symptom**: `inner doc comments are not permitted here` when compiling tests
- **Cause**: Files with `//!` comments included via `include!()` macro 
- **Solution**: Replace `//!` with regular `//` comments in included test files

**Test Import/Scope Issues**
- **Symptom**: `TestEnum not found` or similar import errors in test files
- **Solution**: Update import paths to use full crate paths (e.g., `use crate::inc::module::TestEnum`)
- **Architecture**: `*_only_test.rs` files are included by `derive.rs`/`manual.rs`, not standalone modules

**Enum Field Method Not Found**
- **Symptom**: Method like `.field_name()` not found on enum variant former
- **Cause**: Current enum Former implementation uses positional setters, not field delegation
- **Workaround**: Use positional setters like `._0(value)` instead of `.field_name(value)`
- **Alternative**: Mark complex variants as `#[scalar]` for direct construction

**Standalone Constructor Conflicts**
- **Symptom**: "Old behavior conflicts" in manual implementations
- **Cause**: Manual implementations following outdated patterns
- **Solution**: Update standalone constructors to return `Self` directly when no fields are marked with `#[former_ignore]`

## Where to Go Next

*   **[Technical Specification](spec.md):** Complete behavioral specification defining the Former macro's rules and expected behavior.
*   **[Advanced Usage & Concepts](https://github.com/Wandalen/wTools/tree/master/module/core/former/advanced.md):** Dive deeper into subformers, customization options, storage, context, definitions, mutators, and custom collections.
*   **[Examples Directory](https://github.com/Wandalen/wTools/tree/master/module/core/former/examples):** Explore practical, runnable examples showcasing various features.
*   **[API Documentation (docs.rs)](https://docs.rs/former):** Get detailed information on all public types, traits, and functions.
*   **[Repository (GitHub)](https://github.com/Wandalen/wTools/tree/master/module/core/former):** View the source code, contribute, or report issues.

