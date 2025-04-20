# Crate Documentation

**Version:** 0.52.0

**Format Version:** 43

# Module `macro_tools`

<!-- {{# generate.module_header{} #}} -->

# Module :: `proc_macro_tools`
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_macro_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_macro_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/macro_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/macro_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fmacro_tools%2Fexamples%2Fmacro_tools_trivial.rs,RUN_POSTFIX=--example%20module%2Fcore%2Fmacro_tools%2Fexamples%2Fmacro_tools_trivial.rs/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Tools for writing procedural macros.

### Example: Trivial One

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

## Modules

## Module `attr`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "attr"))]`


Attributes analyzys and manipulation.


```rust
pub mod attr { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `has_debug`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::has_debug;
```

#### Re-export `is_standard`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::is_standard;
```

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `attr`

```rust
pub use super::super::attr;
```

#### Re-export `AttributesInner`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::AttributesInner;
```

#### Re-export `AttributesOuter`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::AttributesOuter;
```

#### Re-export `AttributeComponent`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::AttributeComponent;
```

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `attr_prop`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "attr_prop"))]`


Attribute's properties. Reuse them to define how to parse properties of an attribute.

# Example

```rust
use macro_tools::AttributePropertyBoolean;

#[ derive( Debug, Default, Clone, Copy ) ]
pub struct DebugMarker;

#[ derive( Debug, Default, Clone, Copy ) ]
pub struct EnabledMarker;

pub trait AttributePropertyComponent
{
  const KEYWORD : &'static str;
}

impl AttributePropertyComponent for DebugMarker
{
  const KEYWORD : &'static str = "debug";
}

impl AttributePropertyComponent for EnabledMarker
{
  const KEYWORD : &'static str = "enabled";
}

#[ derive( Debug, Default ) ]
struct MyAttributes
{
  pub debug : AttributePropertyBoolean< DebugMarker >,
  pub enabled : AttributePropertyBoolean< EnabledMarker >,
}

impl syn::parse::Parse for MyAttributes
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut debug = AttributePropertyBoolean::< DebugMarker >::default();
    let mut enabled = AttributePropertyBoolean::< EnabledMarker >::default();

    while !input.is_empty()
    {
      let lookahead = input.lookahead1();
      if lookahead.peek( syn::Ident )
      {
        let ident : syn::Ident = input.parse()?;
        match ident.to_string().as_str()
        {
          DebugMarker::KEYWORD => debug = input.parse()?,
          EnabledMarker::KEYWORD => enabled = input.parse()?,
          _ => return Err( lookahead.error() ),
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

    Ok( MyAttributes { debug, enabled } )
  }
}

let input : syn::Attribute = syn::parse_quote!( #[ attribute( enabled = true ) ] );
let meta = match input.meta
{
  syn::Meta::List( meta_list ) => meta_list,
  _ => panic!( "Expected a Meta::List" ),
};

let nested_meta_stream : proc_macro2::TokenStream = meta.tokens;
let attrs : MyAttributes = syn::parse2( nested_meta_stream ).unwrap();
println!( "{:?}", attrs );
```

In this example, the `AttributePropertyBoolean` struct is used to define attributes with boolean properties.
The `DebugMarker` and `EnabledMarker` structs act as markers to distinguish between different boolean attributes.
The `MyAttributes` struct aggregates these boolean attributes.

The `Parse` implementation for `MyAttributes` iterates through the attribute's key-value pairs,
identifying each by its marker's keyword and parsing the boolean value.
It uses the `ParseStream` to parse identifiers and their associated values,
matching them to the appropriate marker's keyword.
If an unrecognized identifier is encountered, it returns an error.

The `parse_quote!` macro is used to create a `syn::Attribute` instance with the attribute syntax,
which is then parsed into the `MyAttributes` struct. The resulting `MyAttributes` instance is printed to the console.

```rust
pub mod attr_prop { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `attr_prop`

```rust
pub use super::super::attr_prop;
```

#### Re-export `AttributePropertyComponent`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::private::AttributePropertyComponent;
```

#### Re-export `AttributePropertySingletone`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::singletone::AttributePropertySingletone;
```

#### Re-export `AttributePropertySingletoneMarker`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::singletone::AttributePropertySingletoneMarker;
```

#### Re-export `AttributePropertyOptionalSingletone`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::singletone_optional::AttributePropertyOptionalSingletone;
```

#### Re-export `AttributePropertyOptionalSingletoneMarker`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::singletone_optional::AttributePropertyOptionalSingletoneMarker;
```

#### Re-export `AttributePropertyBoolean`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::boolean::AttributePropertyBoolean;
```

#### Re-export `AttributePropertyBooleanMarker`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::boolean::AttributePropertyBooleanMarker;
```

#### Re-export `AttributePropertyOptionalBoolean`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::boolean_optional::AttributePropertyOptionalBoolean;
```

#### Re-export `AttributePropertyOptionalBooleanMarker`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::boolean_optional::AttributePropertyOptionalBooleanMarker;
```

#### Re-export `AttributePropertySyn`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::syn::AttributePropertySyn;
```

#### Re-export `AttributePropertySynMarker`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::syn::AttributePropertySynMarker;
```

#### Re-export `AttributePropertyOptionalSyn`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::syn_optional::AttributePropertyOptionalSyn;
```

#### Re-export `AttributePropertyOptionalSynMarker`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::syn_optional::AttributePropertyOptionalSynMarker;
```

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `components`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "components"))]`


Type-based assigning.


```rust
pub mod components { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

#### Re-export `::former_types::own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use ::former_types::own::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `components`

```rust
pub use super::super::components;
```

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

#### Re-export `::former_types::exposed::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use ::former_types::exposed::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `::former_types::prelude::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use ::former_types::prelude::*;
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `ct`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "ct"))]`


Compile-time tools.


```rust
pub mod ct { /* ... */ }
```

### Modules

## Module `str`

Compile-time const expressions for strings.

```rust
pub mod str { /* ... */ }
```

### Re-exports

#### Re-export `concatcp`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use ::const_format::concatcp as concat;
```

#### Re-export `formatcp`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use ::const_format::formatcp as format;
```

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

#### Re-export `::const_format::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use ::const_format::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `ct`

```rust
pub use super::super::ct;
```

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

Compile-time tools.

```rust
pub use own::*;
```

## Module `container_kind`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "container_kind"))]`


Determine kind of a container.


```rust
pub mod container_kind { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `ContainerKind`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::ContainerKind;
```

#### Re-export `of_type`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::of_type;
```

#### Re-export `of_optional`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::of_optional;
```

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `container_kind`

```rust
pub use super::super::container_kind;
```

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `derive`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "derive"))]`


Macro helpers around derive macro and structure [`syn::DeriveInput`].


```rust
pub mod derive { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `named_fields`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::named_fields;
```

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Parented namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `derive`

```rust
pub use super::super::derive;
```

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `diag`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "diag"))]`


Macro helpers.


```rust
pub mod diag { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Parented namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `diag`

```rust
pub use super::super::diag;
```

#### Re-export `indentation`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::indentation;
```

#### Re-export `report_format`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::report_format;
```

#### Re-export `report_print`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::report_print;
```

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `tree_print`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::tree_print;
```

#### Re-export `code_print`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::code_print;
```

#### Re-export `tree_diagnostics_str`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::tree_diagnostics_str;
```

#### Re-export `code_diagnostics_str`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::code_diagnostics_str;
```

#### Re-export `code_to_str`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::code_to_str;
```

#### Re-export `syn_err`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::syn_err;
```

#### Re-export `return_syn_err`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::return_syn_err;
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `equation`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "equation"))]`


Attributes analyzys and manipulation.


```rust
pub mod equation { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `from_meta`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::from_meta;
```

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `equation`

```rust
pub use super::super::equation;
```

#### Re-export `Equation`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::Equation;
```

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `generic_args`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "generic_args"))]`


This module provides utilities to handle and manipulate generic arguments using the `syn` crate. It includes traits and functions for transforming, merging, and managing generic parameters within procedural macros, enabling seamless syntactic analysis and code generation.


```rust
pub mod generic_args { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `merge`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::merge;
```

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `IntoGenericArgs`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::IntoGenericArgs;
```

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `generic_args`

```rust
pub use super::super::generic_args;
```

#### Re-export `super::prelude::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `generic_params`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "generic_params"))]`


Functions and structures to handle and manipulate generic parameters using the `syn` crate. It's designed to support macro-driven code generation by simplifying, merging, extracting, and decomposing `syn::Generics`.


```rust
pub mod generic_params { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `merge`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::merge;
```

#### Re-export `only_names`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::only_names;
```

#### Re-export `names`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::names;
```

#### Re-export `decompose`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::decompose;
```

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `GenericsWithWhere`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::GenericsWithWhere;
```

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `generic_params`

```rust
pub use super::super::generic_params;
```

#### Re-export `super::prelude::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `ident`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "ident"))]`


Utilities for manipulating identifiers, including keyword handling.


```rust
pub mod ident { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `ident_maybe_raw`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::ident_maybe_raw;
```

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `ident`

```rust
pub use super::super::ident;
```

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `item`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "item"))]`

This module provides various utilities and namespaces for working with `syn::Item`, specifically focusing on
ensuring syntactical correctness and managing different visibility levels within the code. It includes functions
to manipulate the structure of items, handle different kinds of fields, and provide a structured approach to
organizing the codebase into different access levels.

```rust
pub mod item { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `ensure_comma`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::ensure_comma;
```

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `item`

```rust
pub use super::super::item;
```

#### Re-export `super::prelude::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `item_struct`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "item_struct"))]`


Parse structures, like `struct { a : i32 }`.


```rust
pub mod item_struct { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `field_types`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::field_types;
```

#### Re-export `field_names`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::field_names;
```

#### Re-export `first_field_type`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::first_field_type;
```

#### Re-export `first_field_name`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::first_field_name;
```

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `item_struct`

```rust
pub use super::super::item_struct;
```

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `name`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "name"))]`


Tait to getn name of an Item.


```rust
pub mod name { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `name`

```rust
pub use super::super::name;
```

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `Name`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use private::Name;
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `kw`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "kw"))]`


Keywords


```rust
pub mod kw { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `kw`

```rust
pub use super::super::kw;
```

#### Re-export `is`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::is;
```

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `phantom`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "phantom"))]`


Responsible for generating marker `PhantomData` fields to avoid the rule requiring the usage of all generic parameters in a struct. This is often necessary to ensure that Rust's type system correctly tracks the ownership and lifetimes of these parameters without needing them to be explicitly used in the struct's fields.

Functions and structures to handle and manipulate `PhantomData` fields in structs using the `syn` crate. These utilities ensure that generic parameters are correctly accounted for in type checking, even if they are not directly used in the struct's fields.


```rust
pub mod phantom { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `add_to_item`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::add_to_item;
```

#### Re-export `tuple`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::tuple;
```

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `phantom`

```rust
pub use super::super::phantom;
```

#### Re-export `super::prelude::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `punctuated`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "punctuated"))]`


Structures and functions for handling `syn::punctuated::Punctuated` collections.

This module provides functionality to manipulate and ensure correct punctuation in `syn::punctuated::Punctuated` collections, commonly used in procedural macros to represent sequences of elements separated by punctuation marks, such as commas.


```rust
pub mod punctuated { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `ensure_trailing_comma`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::ensure_trailing_comma;
```

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `punctuated`

```rust
pub use super::super::punctuated;
```

#### Re-export `super::prelude::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use super::prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `quantifier`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "quantifier"))]`


Quantifiers like Pair and Many.


```rust
pub mod quantifier { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `quantifier`

```rust
pub use super::super::quantifier;
```

#### Re-export `AsMuchAsPossibleNoDelimiter`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::AsMuchAsPossibleNoDelimiter;
```

#### Re-export `Pair`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::Pair;
```

#### Re-export `Many`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::Many;
```

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `struct_like`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "struct_like"))]`


Parse structures, like `struct { a : i32 }`.


```rust
pub mod struct_like { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `StructLike`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::StructLike;
```

#### Re-export `FieldOrVariant`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::FieldOrVariant;
```

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `struct_like`

```rust
pub use super::super::struct_like;
```

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `tokens`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "tokens"))]`


Attributes analyzys and manipulation.


```rust
pub mod tokens { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `tokens`

```rust
pub use super::super::tokens;
```

#### Re-export `Tokens`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::Tokens;
```

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `typ`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "typ"))]`


Advanced syntax elements.


```rust
pub mod typ { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `type_rightmost`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::type_rightmost;
```

#### Re-export `type_parameters`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::type_parameters;
```

#### Re-export `is_optional`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::is_optional;
```

#### Re-export `parameter_first`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use private::parameter_first;
```

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `typ`

```rust
pub use super::super::typ;
```

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `typed`

**Attributes:**

- `#[<cfg>(all(feature = "enabled", feature = "typed"))]`


Typed parsing.


```rust
pub mod typed { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `parse_quote`

```rust
pub use syn::parse_quote;
```

#### Re-export `parse_quote`

```rust
pub use syn::parse_quote as qt;
```

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `typed`

```rust
pub use super::super::typed;
```

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `iter`

**Attributes:**

- `#[<cfg>(feature = "enabled")]`


Tailored iterator.


```rust
pub mod iter { /* ... */ }
```

### Modules

## Module `own`

**Attributes:**

- `#[allow(unused_imports)]`

Tailoted iterator.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `orphan::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use orphan::*;
```

#### Re-export `iter_tools::own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use iter_tools::own::*;
```

## Module `orphan`

**Attributes:**

- `#[allow(unused_imports)]`

Orphan namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `exposed::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use exposed::*;
```

## Module `exposed`

**Attributes:**

- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use prelude::*;
```

#### Re-export `iter_tools::exposed::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use iter_tools::exposed::*;
```

## Module `prelude`

**Attributes:**

- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `iter_tools::prelude::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use iter_tools::prelude::*;
```

### Re-exports

#### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use own::*;
```

## Module `dependency`

**Attributes:**

- `#[<cfg>(feature = "enabled")]`
- `#[allow(unused_imports)]`


Dependencies of the module.


```rust
pub mod dependency { /* ... */ }
```

### Re-exports

#### Re-export `syn`

```rust
pub use ::syn;
```

#### Re-export `quote`

```rust
pub use ::quote;
```

#### Re-export `proc_macro2`

```rust
pub use ::proc_macro2;
```

#### Re-export `interval_adapter`

```rust
pub use ::interval_adapter;
```

#### Re-export `clone_dyn_types`

```rust
pub use ::clone_dyn_types;
```

#### Re-export `former_types`

```rust
pub use ::former_types;
```

## Module `own`

**Attributes:**

- `#[<cfg>(feature = "enabled")]`
- `#[allow(unused_imports)]`

Own namespace of the module.

```rust
pub mod own { /* ... */ }
```

### Re-exports

#### Re-export `_all::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use _all::*;
```

## Module `orphan`

**Attributes:**

- `#[<cfg>(feature = "enabled")]`
- `#[allow(unused_imports)]`

Parented namespace of the module.

```rust
pub mod orphan { /* ... */ }
```

### Re-exports

#### Re-export `_all::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use _all::*;
```

## Module `exposed`

**Attributes:**

- `#[<cfg>(feature = "enabled")]`
- `#[allow(unused_imports)]`

Exposed namespace of the module.

```rust
pub mod exposed { /* ... */ }
```

### Re-exports

#### Re-export `_all::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use _all::*;
```

## Module `prelude`

**Attributes:**

- `#[<cfg>(feature = "enabled")]`
- `#[allow(unused_imports)]`

Prelude to use essentials: `use my_module::prelude::*`.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `syn`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use ::syn;
```

#### Re-export `proc_macro2`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use ::proc_macro2;
```

#### Re-export `quote`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use ::quote;
```

#### Re-export `quote`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use ::quote::quote;
```

#### Re-export `quote`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use ::quote::quote as qt;
```

#### Re-export `quote_spanned`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use ::quote::quote_spanned;
```

#### Re-export `format_ident`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use ::quote::format_ident;
```

#### Re-export `ParseStream`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use syn::parse::ParseStream;
```

#### Re-export `Token`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use syn::Token;
```

#### Re-export `Spanned`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use syn::spanned::Spanned;
```

#### Re-export `braced`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use syn::braced;
```

#### Re-export `bracketed`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use syn::bracketed;
```

#### Re-export `custom_keyword`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use syn::custom_keyword;
```

#### Re-export `custom_punctuation`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use syn::custom_punctuation;
```

#### Re-export `parenthesized`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use syn::parenthesized;
```

#### Re-export `parse_macro_input`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use syn::parse_macro_input;
```

#### Re-export `parse_quote`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use syn::parse_quote;
```

#### Re-export `parse_quote`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use syn::parse_quote as parse_qt;
```

#### Re-export `parse_quote_spanned`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use syn::parse_quote_spanned;
```

#### Re-export `parse_quote_spanned`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`

```rust
pub use syn::parse_quote_spanned as parse_qt_spanned;
```

#### Re-export `_all::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use _all::*;
```

#### Re-export `::interval_adapter::prelude::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use ::interval_adapter::prelude::*;
```

## Macros

### Macro `tree_print`

**Attributes:**

- `#[macro_export]`


Macro for diagnostics purpose to print both syntax tree and source code behind it with syntax tree.

### Basic use-case.
```
use macro_tools::prelude::*;

let code = qt!( std::collections::HashMap< i32, i32 > );
let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
tree_print!( tree_type );
```


```rust
pub macro_rules! tree_print {
    /* macro_rules! tree_print {
    ( $src :expr ) => { ... };
    ( $( $src :expr ),+ $(,)? ) => { ... };
} */
}
```

### Macro `code_print`

**Attributes:**

- `#[macro_export]`


Macro for diagnostics purpose to print both syntax tree and source code behind it without syntax tree.

### Basic use-case.
```
use macro_tools::prelude::*;

let code = qt!( std::collections::HashMap< i32, i32 > );
let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
tree_print!( tree_type );
```


```rust
pub macro_rules! code_print {
    /* macro_rules! code_print {
    ( $src :expr ) => { ... };
    ( $( $src :expr ),+ $(,)? ) => { ... };
} */
}
```

### Macro `tree_diagnostics_str`

**Attributes:**

- `#[macro_export]`


Macro for diagnostics purpose to export both syntax tree and source code behind it into a string.


```rust
pub macro_rules! tree_diagnostics_str {
    /* macro_rules! tree_diagnostics_str {
    ( $src :expr ) => { ... };
} */
}
```

### Macro `code_diagnostics_str`

**Attributes:**

- `#[macro_export]`


Macro for diagnostics purpose to diagnose source code behind it and export it into a string.


```rust
pub macro_rules! code_diagnostics_str {
    /* macro_rules! code_diagnostics_str {
    ( $src :expr ) => { ... };
} */
}
```

### Macro `code_to_str`

**Attributes:**

- `#[macro_export]`


Macro to export source code behind a syntax tree into a string.


```rust
pub macro_rules! code_to_str {
    /* macro_rules! code_to_str {
    ( $src :expr ) => { ... };
} */
}
```

### Macro `syn_err`

**Attributes:**

- `#[macro_export]`


Macro to generate syn error either with span of a syntax tree element or with default one `proc_macro2::Span::call_site()`.

### Basic use-case.
```
# use macro_tools::exposed::*;
syn_err!( "No attr" );
# ()
```


```rust
pub macro_rules! syn_err {
    /* macro_rules! syn_err {
    ( $msg:expr $(,)? ) => { ... };
    ( _, $msg:expr $(,)? ) => { ... };
    ( $span:expr, $msg:expr $(,)? ) => { ... };
    ( $span:expr, $msg:expr, $( $arg:expr ),+ $(,)? ) => { ... };
    ( _, $msg:expr, $( $arg:expr ),+ $(,)? ) => { ... };
} */
}
```

### Macro `return_syn_err`

**Attributes:**

- `#[macro_export]`


Macro to generate syn error either with span of a syntax tree element or with default one `proc_macro2::Span::call_site()`.

### Basic use-case.
```
# use macro_tools::exposed::*;
syn_err!( "No attr" );
# ()
```


```rust
pub macro_rules! return_syn_err {
    /* macro_rules! return_syn_err {
    ( $( $Arg : tt )* ) => { ... };
} */
}
```

## Re-exports

### Re-export `own::*`

**Attributes:**

- `#[doc(inline)]`
- `#[allow(unused_imports)]`
- `#[<cfg>(feature = "enabled")]`

```rust
pub use own::*;
```

