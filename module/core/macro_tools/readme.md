<!-- {{# generate.module_header{} #}} -->

# Module :: `proc_macro_tools`
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_macro_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_macro_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/macro_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/macro_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fmacro_tools%2Fexamples%2Fmacro_tools_trivial.rs,RUN_POSTFIX=--example%20module%2Fcore%2Fmacro_tools%2Fexamples%2Fmacro_tools_trivial.rs/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

**A comprehensive toolkit for writing robust and maintainable procedural macros in Rust.**

## Why macro_tools?

Writing procedural macros can be challenging due to:

- **Complex token stream manipulation** - Manually handling token streams is error-prone and verbose
- **Boilerplate-heavy code** - Common patterns require significant repetitive code
- **Poor error handling** - Difficult to generate helpful error messages for macro users
- **Limited type introspection** - Extracting type information from parsed syntax trees is complex

`macro_tools` solves these problems by providing:

- 🛠️ **High-level utilities** for token stream manipulation
- 🔍 **Advanced parsers** for attributes, generics, and types
- 🎯 **Precise error reporting** with span-aware messages
- 📦 **Zero-dependency core** - Only depends on `syn`, `quote`, and `proc-macro2`
- 🚀 **Proven in production** - Battle-tested in real-world macro systems

## Quick Start

Add `macro_tools` to your `Cargo.toml`:

```toml
[dependencies]
macro_tools = "0.24.0"
```

### Example: Extract Type Parameters

```rust
use macro_tools::{ typ, qt };

// Parse a type and extract its parameters
let code = qt!( Option< i32 > );
let tree_type = syn::parse2::< syn::Type >( code ).unwrap();

// Extract type parameters
let params = typ::type_parameters( &tree_type, 0..=0 );
params.iter().for_each( |param| println!( "{}", qt!( #param ) ) );
// Output: i32
```

Try out `cargo run --example macro_tools_extract_type_parameters`.
<br/>
[See code](./examples/macro_tools_extract_type_parameters.rs).

### Example: Parse Attributes with Properties

This example shows the structure of attribute parsing. For a complete working example with all trait implementations, see the full example file.

```rust
use macro_tools::exposed::*;

// Define a custom attribute with properties
#[ derive( Debug ) ]
pub struct CustomAttribute
{
  pub enabled : AttributePropertyBoolean,
  pub name : AttributePropertyOptionalSyn< syn::LitStr >,
}

// After implementing required traits (AttributeComponent, Parse, etc.)
// you can parse attributes like this:
// let attr : syn::Attribute = syn::parse_quote!( #[ custom( enabled = true, name = "example" ) ] );
// let parsed = CustomAttribute::from_meta( &attr )?;
// assert!( parsed.enabled.value() );
```

Try out `cargo run --example macro_tools_parse_attributes`.
<br/>
[See code](./examples/macro_tools_parse_attributes.rs).

## Features

### 🎯 Type Analysis Tools

Extract and analyze type information:

- **`typ`** - Type parsing and parameter extraction utilities
- Extract nested generic parameters
- Parse complex type expressions
- Handle path types, arrays, tuples, and more

### 🔧 Generic Parameter Utilities

Advanced generic parameter manipulation:

- **`generic_params`** - Tools for working with `syn::Generics`
  - Decompose generics for different contexts
  - Merge generic parameters from multiple sources
  - Filter and transform generic parameters
  - Generate appropriate tokens for impl blocks

### 📝 Attribute Parsing Framework

Powerful attribute parsing with derive-macro-like experience:

- **`attr`** - Attribute parsing utilities
  - Parse structured attributes with properties
  - Support for optional, boolean, and custom property types
  - Generate helpful error messages
  - Composable attribute parsing with the `Assign` trait

### 🔍 Syntax Tree Helpers

Work with Rust syntax trees effectively:

- **`struct_like`** - Parse and manipulate struct-like items
- **`item_struct`** - Struct-specific utilities
- **`quantifier`** - Extract quantifiers from type expressions
- **`name`** - Name and path manipulation
- **`punctuated`** - Work with punctuated sequences

### 🛠️ Token Stream Utilities

Core utilities for procedural macros:

- **`tokens`** - Token stream manipulation
- **`equation`** - Parse and generate equations
- **`diag`** - Enhanced diagnostics with custom error formatting

## Advanced Example: Generic Function Implementation

<!-- {{# generate.module{} #}} -->

The purpose of `typ::type_parameters` is to extract type parameters from a given Rust type.
In this example, we generate a type `core::option::Option<i8, i16, i32, i64>` and extract its type parameters.

```rust
#[ cfg( not( all( feature = "enabled", feature = "typ" ) ) ) ]
fn main(){}
#[ cfg( all( feature = "enabled", feature = "typ" ) ) ]
fn main()
{
  // Import necessary macros and modules from the `macro_tools` crate.
  use macro_tools::{ typ, qt };

  // Generate a token stream representing the type `core::option::Option<i8, i16, i32, i64>`.
  let code = qt!( core::option::Option< i8, i16, i32, i64 > );

  // Parse the generated token stream into a `syn::Type` object.
  // `syn::Type` is a syntax tree node representing a Rust type.
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();

  // Extract type parameters from the parsed type.
  // `typ::type_parameters` takes a reference to a `syn::Type` and a range.
  // It returns a vector of type parameters within the specified range.
  // Here, `0..=2` specifies that we are interested in the first three type parameters.
  let got = typ::type_parameters( &tree_type, 0..=2 );

  // Iterate over the extracted type parameters and print each one.
  // The `qt!` macro is used to convert the type parameter back to a token stream for printing.
  got.iter().for_each( | e | println!( "{}", qt!( #e ) ) );

  /* Expected output:
     i8
     i16
     i32
  */
}
```

Try out `cargo run --example macro_tools_trivial`.
<br/>
[See code](./examples/macro_tools_trivial.rs).

### Example: Attribute Properties

This example demonstrates an approach to parsing attributes and their properties.
The attributes are collected into a struct that aggregates them, and attribute properties
are parsed using reusable components from a library. The example shows how to use
`AttributePropertyBoolean` for parsing boolean properties and the roles of the traits
`AttributePropertyComponent` and `AttributeComponent`. The `Assign` trait is
also used to simplify the logic of assigning fields.

Attributes are collected into a `ItemAttributes` struct, and attribute properties are parsed
using reusable components like `AttributePropertyBoolean`.

- `AttributeComponent`: A trait that defines how an attribute should be parsed from a `syn::Attribute`.
- `AttributePropertyComponent`: A trait that defines a marker for attribute properties.
- `Assign`: A trait that simplifies the logic of assigning fields to a struct. Using a
  component-based approach requires each field to have a unique type, which aligns with the
  strengths of strongly-typed languages. This method ensures that the logic of
  assigning values to fields is encapsulated within the fields themselves, promoting modularity
  and reusability.

The reusable property components from the library come with parameters that distinguish
different properties of the same type. This is useful when an attribute has multiple boolean
properties, for instance. Such an approach helps to avoid limitations where it is
always possible to define traits for custom types, while it may not be possible for types
defined in other crates.

```rust

#[ cfg( not( all( feature = "enabled", feature = "attr_prop", debug_assertions ) )  ) ]
fn main(){}
#[ cfg( all( feature = "enabled", feature = "attr_prop", debug_assertions )  ) ]
fn main()
{

  use macro_tools::
  {
    attr,
    ct,
    syn_err,
    return_syn_err,
    qt,
    Result,
    AttributeComponent,
    AttributePropertyComponent,
    AttributePropertyBoolean,
    AttributePropertySingletone,
    Assign,
  };

  /// Represents the attributes of a struct. Aggregates all its attributes.
  #[ derive( Debug, Default ) ]
  pub struct ItemAttributes
  {
    /// Attribute for customizing the mutation process.
    pub mutator : AttributeMutator,
  }

  impl ItemAttributes
  {
    /// Constructs a `ItemAttributes` instance from an iterator of attributes.
    ///
    /// This function parses the provided attributes and assigns them to the
    /// appropriate fields in the `ItemAttributes` struct.
    pub fn from_attrs< 'a >( attrs : impl Iterator< Item = & 'a syn::Attribute > ) -> Result< Self >
    {
      let mut result = Self::default();

      // Closure to generate an error message for unknown attributes.
      let error = | attr : & syn::Attribute | -> syn::Error
      {
        let known_attributes = ct::str::format!
        (
          "Known attributes are: {}, {}.",
          "debug",
          AttributeMutator::KEYWORD,
        );
        syn_err!
        (
          attr,
          "Expects an attribute of format '#[ attribute( key1 = val1, key2 = val2 ) ]'\n  {known_attributes}\n  But got: '{}'",
          qt! { #attr }
        )
      };

      for attr in attrs
      {
        let key_ident = attr.path().get_ident().ok_or_else( || error( attr ) )?;
        let key_str = format!( "{}", key_ident );
        match key_str.as_ref()
        {
          AttributeMutator::KEYWORD => result.assign( AttributeMutator::from_meta( attr )? ),
          "debug" => {},
          _ => {},
        }
      }

      Ok( result )
    }
  }

  /// Represents attributes for customizing the mutation process in a forming operation.
  ///
  /// ## Example of code
  ///
  /// ```ignore
  /// #[ mutator( custom = true, debug = true ) ]
  /// ```
  #[ derive( Debug, Default ) ]
  pub struct AttributeMutator
  {
    /// Indicates whether a custom mutator should be generated.
    /// Defaults to `false`, meaning no custom mutator is generated unless explicitly requested.
    pub custom : AttributePropertyCustom,
    /// Specifies whether to print code generated for the field.
    /// Defaults to `false`, which means no hint is provided unless explicitly requested.
    pub debug : AttributePropertyDebug,
  }

  impl AttributeComponent for AttributeMutator
  {
    const KEYWORD : & 'static str = "mutator";

    /// Parses a `syn::Attribute` into an `AttributeMutator`.
    fn from_meta( attr : & syn::Attribute ) -> Result< Self >
    {
      match attr.meta
      {
        syn::Meta::List( ref meta_list ) =>
        {
          return syn::parse2::< AttributeMutator >( meta_list.tokens.clone() );
        },
        syn::Meta::Path( ref _path ) =>
        {
          return Ok( Default::default() )
        },
        _ => return_syn_err!
        (
          attr,
          "Expects an attribute of format `#[ mutator( custom = true ) ]`. \nGot: {}",
          qt! { #attr }
        ),
      }
    }
  }

  // Implement `Assign` trait to allow assigning `AttributeMutator` to `ItemAttributes`.
  impl< IntoT > Assign< AttributeMutator, IntoT > for ItemAttributes
  where
    IntoT : Into< AttributeMutator >,
  {
    #[ inline( always ) ]
    fn assign( & mut self, component : IntoT )
    {
      self.mutator = component.into();
    }
  }

  // Implement `Assign` trait to allow assigning `AttributePropertyDebug` to `AttributeMutator`.
  impl< IntoT > Assign< AttributePropertyDebug, IntoT > for AttributeMutator
  where
    IntoT : Into< AttributePropertyDebug >,
  {
    #[ inline( always ) ]
    fn assign( & mut self, component : IntoT )
    {
      self.debug = component.into();
    }
  }

  // Implement `Assign` trait to allow assigning `AttributePropertyCustom` to `AttributeMutator`.
  impl< IntoT > Assign< AttributePropertyCustom, IntoT > for AttributeMutator
  where
    IntoT : Into< AttributePropertyCustom >,
  {
    #[ inline( always ) ]
    fn assign( & mut self, component : IntoT )
    {
      self.custom = component.into();
    }
  }

  impl syn::parse::Parse for AttributeMutator
  {
    fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
    {
      let mut result = Self::default();

      let error = | ident : & syn::Ident | -> syn::Error
      {
        let known = ct::str::format!
        (
          "Known entries of attribute {} are: {}, {}.",
          AttributeMutator::KEYWORD,
          AttributePropertyCustom::KEYWORD,
          AttributePropertyDebug::KEYWORD,
        );
        syn_err!
        (
          ident,
          r#"Expects an attribute of format '#[ mutator( custom = false ) ]'
    {known}
    But got: '{}'
  "#,
          qt! { #ident }
        )
      };

      while !input.is_empty()
      {
        let lookahead = input.lookahead1();
        if lookahead.peek( syn::Ident )
        {
          let ident : syn::Ident = input.parse()?;

          match ident.to_string().as_str()
          {
            AttributePropertyCustom::KEYWORD => result.assign( AttributePropertyCustom::parse( input )? ),
            AttributePropertyDebug::KEYWORD => result.assign( AttributePropertyDebug::from( true ) ),
            _ => return Err( error( & ident ) ),
          }
        }
        else
        {
          return Err( lookahead.error() );
        }

        // Optional comma handling
        if input.peek( syn::Token![,] )
        {
          input.parse::< syn::Token![,] >()?;
        }
      }

      Ok( result )
    }
  }

  // == Attribute properties

  /// Marker type for attribute property to specify whether to provide a sketch as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct AttributePropertyDebugMarker;

  impl AttributePropertyComponent for AttributePropertyDebugMarker
  {
    const KEYWORD : & 'static str = "debug";
  }

  /// Specifies whether to provide a sketch as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub type AttributePropertyDebug = AttributePropertySingletone< AttributePropertyDebugMarker >;

  // ==

  /// Marker type for attribute property to indicate whether a custom code should be generated.
  /// Defaults to `false`, meaning no custom code is generated unless explicitly requested.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct AttributePropertyCustomMarker;

  impl AttributePropertyComponent for AttributePropertyCustomMarker
  {
    const KEYWORD : & 'static str = "custom";
  }

  /// Indicates whether a custom code should be generated.
  /// Defaults to `false`, meaning no custom code is generated unless explicitly requested.
  pub type AttributePropertyCustom = AttributePropertyBoolean< AttributePropertyCustomMarker >;

  // == test code

  // Parse an attribute and construct a `ItemAttributes` instance.
  let input : syn::Attribute = syn::parse_quote!( #[ mutator( custom = true ) ] );
  let attrs : ItemAttributes = ItemAttributes::from_attrs( std::iter::once( & input ) ).unwrap();
  println!( "{:?}", attrs );

  // Test `AttributePropertyBoolean` functionality.
  let attr : AttributePropertyBoolean< AttributePropertyDebugMarker > = AttributePropertyBoolean::default();
  assert_eq!( attr.internal(), false );
  let attr : AttributePropertyBoolean< AttributePropertyDebugMarker > = true.into();
  assert_eq!( attr.internal(), true );
  let attr : AttributePropertyBoolean< AttributePropertyDebugMarker > = false.into();
  assert_eq!( attr.internal(), false );

}

```

Try out `cargo run --example macro_tools_attr_prop`.
<br/>
[See code](./examples/macro_tools_attr_prop.rs).

## Real-World Use Cases

`macro_tools` is ideal for:

- **Derive Macros** - Building derive macros with proper error handling and type analysis
- **Attribute Macros** - Parsing complex attributes with multiple properties
- **Code Generation** - Generating boilerplate code based on type structure
- **DSL Implementation** - Creating domain-specific languages with procedural macros

## Documentation

For detailed documentation, visit:
- [API Documentation](https://docs.rs/macro_tools)
- [Examples](./examples)

## Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/Wandalen/wTools/blob/master/CONTRIBUTING.md).

## License

Licensed under the MIT License. See [LICENSE](https://github.com/Wandalen/wTools/blob/master/LICENSE) for details.

## Repository

[GitHub Repository](https://github.com/Wandalen/wTools/tree/master/module/core/macro_tools)

### To add to your project

```sh
cargo add proc_macro_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/macro_tools_trivial
cargo run
```
