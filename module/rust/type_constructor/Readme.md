# Module :: type_constructor
[![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleFundamentalDataTypePush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleFundamentalDataTypePush.yml) [![docs.rs](https://img.shields.io/docsrs/type_constructor?color=e3e8f0&logo=docs.rs)](https://docs.rs/type_constructor) [![discord](https://img.shields.io/discord/872391416519737405?color=e3e8f0&logo=discord&logoColor=e3e8f0)](https://discord.gg/JwTG6d2b)

Fundamental data types and type constructors, like Single, Pair, Many.

## Single

Type constructor to define tuple wrapping a given type.

Quite often you need to wrap a given type into new one.
For example if orphan rule became and obstacle one should introduce a new type wrapping foreing one.
Type constructr `types!` does exaclty that and auto-implement traits From, Into and Deref for the constructed type.

## Example :: single line single.

To define your own single use macro `types!`. Single-line definition looks like that.

```rust
use type_constructor::prelude::*;
types!( single MySingle : i32 );
let x = MySingle( 13 );
println!( "x : {}", x.0 );
```

It gererates code:

```rust
use type_constructor::prelude::*;

pub struct MySingle( pub i32 );

impl core::ops::Deref for MySingle
{
  type Target = i32;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}
impl From< i32 > for MySingle
{
  fn from( src : i32 ) -> Self
  {
    Self( src )
  }
}
impl From< MySingle > for i32
{
  fn from( src : MySingle ) -> Self
  {
    src.0
  }
}

let x = MySingle( 13 );
println!( "x : {}", x.0 );
```

### Example :: single with derives and attributes.

It's possible to define attributes as well as derives.

```rust
use type_constructor::prelude::*;
types!
{
  /// This is also attribute and macro understands it.
  #[ derive( Debug ) ]
  single MySingle : i32;
}
let x = MySingle( 13 );
dbg!( x );
```

It gererates code:

```rust
use type_constructor::prelude::*;

/// This is also attribute and macro understands it.
#[ derive( Debug ) ]
pub struct MySingle( pub i32 );

impl core::ops::Deref for MySingle
{
  type Target = i32;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}
impl From< i32 > for MySingle
{
  fn from( src : i32 ) -> Self
  {
    Self( src )
  }
}
impl From< MySingle > for i32
{
  fn from( src : MySingle ) -> Self
  {
    src.0
  }
}

let x = MySingle( 13 );
dbg!( x );
```

### Example :: single with struct instead of macro.

Sometimes it's sufficient to use common type instead of defining a brand new.
You may use paramtetrized struct `Single< T >` instead of macro `types!` if that is the case.

```rust
use type_constructor::prelude::*;
let x = Single::< i32 >( 13 );
dbg!( x );
```

### Example :: single with parametrized element.

Element of tuple could be parametrized.

```rust
use type_constructor::prelude::*;
types!
{
  #[ derive( Debug ) ]
  single MySingle : std::sync::Arc< T : Copy >;
}
let x = MySingle( std::sync::Arc::new( 13 ) );
dbg!( x );
```

It gererates code:

```rust
use type_constructor::*;

#[ derive( Debug ) ]
pub struct MySingle< T : Copy >( pub std::sync::Arc< T > );

impl<T: Copy> core::ops::Deref for MySingle< T >
{
  type Target = std::sync::Arc< T >;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}
impl< T : Copy > From< std::sync::Arc< T > > for MySingle< T >
{
  fn from( src : std::sync::Arc<T>) -> Self {
    Self( src )
  }
}
impl< T : Copy > From< MySingle< T > > for std::sync::Arc< T >
{
  fn from(src: MySingle<T>) -> Self
  {
    src.0
  }
}

let x = MySingle( std::sync::Arc::new( 13 ) );
```

### Example :: single with parametrized tuple.

Instead of parametrizing the element it's possible to define a parametrized tuple.


```rust
use type_constructor::prelude::*;
types!
{
  #[ derive( Debug ) ]
  single MySingle : < T : Copy >;
}
let x = MySingle( 13 );
dbg!( x );
```

It gererates code:

```rust
#[ derive( Debug ) ]
pub struct MySingle< T : Copy >( pub T );

impl< T : Copy > core::ops::Deref
for MySingle< T >
{
  type Target = T;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl< T : Copy > From< T >
for MySingle< T >
{
  fn from( src : T ) -> Self
  {
    Self( src )
  }
}

let x = MySingle( 13 );
dbg!( 13 );
```

### To add to your project

``` shell
cargo add type_constructor
```

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/type_constructor_trivial_sample
cargo run
```