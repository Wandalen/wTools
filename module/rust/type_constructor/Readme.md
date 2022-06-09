# Module :: type_constructor
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleTypeConstructorPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleTypeConstructorPush.yml) [![docs.rs](https://img.shields.io/docsrs/type_constructor?color=e3e8f0&logo=docs.rs)](https://docs.rs/type_constructor) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Fundamental data types and type constructors, like Single, Pair, Homopair, Many.

In Rust, you often need to wrap a given type into a new one.
The role of the orphan rules in particular is basically to prevent you from implementing external traits for external types.
To overcome the restriction developer usually wrap the external type into a tuple introducing a new type.
Type constructor does exactly that and auto-implement traits From, Into, Deref and few more for the constructed type.

Besides type constructor for single element there are type constructors for `pair`, `homopair` and `many`:

- `Single` to wrap single element.
- `Pair` to wrap pair of distinct elements.
- `HomoPair` to wrap pair of elements with the same type.
- `Many` to wrap `Vec` of elements.

### Macro `types` for type constructing

Macro `types` is responsible for generating code for Single, Pair, Homopair, Many. Each type constructor has its own keyword for that, but Pair and Homopair use the same keyword difference in a number of constituent types. It is possible to define all types at once.

```rust ignore
{
  use type_constructor::prelude::*;

  types!
  {

    pub single MySingle : f32;
    pub single SingleWithParametrized : std::sync::Arc< T : Copy >;
    pub single SingleWithParameter : < T >;

    pub pair MyPair : f32;
    pub pair PairWithParametrized : std::sync::Arc< T1 : Copy >, std::sync::Arc< T2 : Copy >;
    pub pair PairWithParameter : < T1, T2 >;

    pub pair MyHomoPair : f32;
    pub pair HomoPairWithParametrized : std::sync::Arc< T : Copy >;
    pub pair HomoPairWithParameter : < T >;

    pub many MyMany : f32;
    pub many ManyWithParametrized : std::sync::Arc< T : Copy >;
    pub many ManyWithParameter : < T >;

  }
}
```

It generates more than 1000 lines of code, which otherwise you would have to write manually.

### Without macro

Macro `types` is exposed to generate new types, but in some cases, it is enough to reuse already generated types of such kind. The library ships such types: Single, Pair, Homopair, Many. Note: If you avoid generating new types you will get in a position to be not able to define your own implementation of foreign traits because of orphan rule.

```rust ignore

let i32_in_tuple = type_constructor::Single::< i32 >::from( 13 );
dbg!( i32_in_tuple );
// i32_in_tuple = Single( 13 )
let i32_and_f32_in_tuple = type_constructor::Pair::< i32, f32 >::from( ( 13, 13.0 ) );
dbg!( i32_and_f32_in_tuple );
// vec_of_i32_in_tuple = Pair( 13, 13.0 )
let two_i32_in_tuple = type_constructor::HomoPair::< i32 >::from( ( 13, 31 ) );
dbg!( two_i32_in_tuple );
// vec_of_i32_in_tuple = HomoPair( 13, 31 )
let vec_of_i32_in_tuple = type_constructor::Many::< i32 >::from( [ 1, 2, 3 ] );
dbg!( vec_of_i32_in_tuple );
// vec_of_i32_in_tuple = Many([ 1, 2, 3 ])

```

### Make.

Make is the variadic constructor. It's the unified interface of the arbitrary-length constructor.
After implementing several traits `Make0`, `Make1` up to `MakeN` one can use make `make!` to construct instances.

```rust ignore
#[ cfg( feature = "make" ) ]
{
  use type_constructor::prelude::*;

  let instance1 : Struct1 = make!();
  let instance2 : Struct1 = make!( 13 );
  let instance3 : Struct1 = make!( 1, 3 );

}
```

### VectorizedFrom

Standard `From` unfortunately is not autoimplemented for tuples and arrays and cant be implemented for them because of orphans restrictions.
That how pair of traits `VectorizedFrom`/`VectorizedInto` could be useful. They are implemented for tuples and arrays.
Their implementation is based on standard `From`, if `From` is implemented for elements of a tuple then `VectorizedFrom`/`VectorizedInto` implemented for collection containing them.

```rust
#[ cfg( feature = "vectorized_from" ) ]
{
  use type_constructor::prelude::*;
  types!( single Single1 : i32 );
  let src = ( 1, 3 );
  let got = <( Single1, Single1 )>::vectorized_from( src );
}
```

### Sample :: single-line single.

To define your own single-use macro `types!`. The single-line definition looks like that.

```rust
use type_constructor::prelude::*;

types!( pub single MySingle : i32 );
let x = MySingle( 13 );
println!( "x : {}", x.0 );
```

It generates code:

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

/* ... */

let x = MySingle( 13 );
println!( "x : {}", x.0 );
```

### Sample :: single with derives and attributes.

It's possible to define attributes as well as derives.

```rust
use type_constructor::prelude::*;
types!
{
  /// This is also attribute and macro understands it.
  #[ derive( Debug ) ]
  pub single MySingle : i32;
}
let x = MySingle( 13 );
dbg!( x );
```

It generates code:

```rust
use type_constructor::prelude::*;

/// This is also an attribute and macro understands it.
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

/* ... */

let x = MySingle( 13 );
dbg!( x );
```

### Sample :: single with struct instead of macro.

Sometimes it's sufficient to use a common type instead of defining a brand new one.
You may use parameterized struct `Single< T >` instead of macro `types!` if that is the case.

```rust
use type_constructor::prelude::*;
let x = Single::< i32 >( 13 );
dbg!( x );
```

### Sample :: single with a parametrized element.

Element of tuple could be parametrized.

```rust
use type_constructor::prelude::*;
types!
{
  #[ derive( Debug ) ]
  pub single MySingle : std::sync::Arc< T : Copy >;
}
let x = MySingle( std::sync::Arc::new( 13 ) );
dbg!( x );
```

It generates code:

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

/* ... */

let x = MySingle( std::sync::Arc::new( 13 ) );
```

### Sample :: single with parametrized tuple.

Instead of parametrizing the element, it's possible to define a parametrized tuple.


```rust
use type_constructor::prelude::*;
types!
{
  #[ derive( Debug ) ]
  pub single MySingle : < T : Copy >;
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

### Sample :: single-line pair

Sometimes you need to wrap more than a single element into a tupдe. If types of elements are different use `pair`. The same macro `types` is responsible for generating code for both `single`, `pair` and also `many`.

```rust
use type_constructor::prelude::*;

types!( pub pair MyPair : i32, i64 );
let x = MyPair( 13, 31 );
println!( "x : ( {}, {} )", x.0, x.1 );
// prints : x : ( 13, 31 )
```

It generates code:

```rust
use type_constructor::prelude::*;

pub struct MyPair( pub i32, pub i64 );

impl From< ( i32, i64 ) > for MyPair
{
  fn from( src : ( i32, i64 ) ) -> Self { Self( src.0, src.1 ) }
}

impl From< MyPair > for ( i32, i64 )
{
  fn from( src : MyPair ) -> Self { ( src.0, src.1 ) }
}

#[cfg( feature = "make" )]
impl Make2< i32, i64 > for MyPair
{
  fn make_2( _0 : i32, _1 : i64 ) -> Self { Self( _0, _1 ) }
}

/* ... */

let x = MyPair( 13, 31 );
println!( "x : ( {}, {} )", x.0, x.1 );
```

### Sample :: pair with parameters

Just like `single` `pair` may have parameters.

```rust
use type_constructor::prelude::*;

use core::fmt;
types!
{
  #[ derive( Debug ) ]
  pub pair MyPair : < T1 : fmt::Debug, T2 : fmt::Debug >;
}
let x = MyPair( 13, 13.0 );
dbg!( x );
// prints : x = MyPair( 13, 13.0 )
```

It generates code:

```rust
use type_constructor::prelude::*;
use core::fmt;

#[ derive( Debug ) ]
pub struct MyPair< T1, T2 >( pub T1, pub T2 );

impl< T1, T2 > From<( T1, T2 )> for MyPair< T1, T2 >
{
  fn from( src : ( T1, T2 ) ) -> Self { Self( src.0, src.1 ) }
}

impl< T1, T2 > From< MyPair< T1, T2 > > for ( T1, T2 )
{
  fn from( src : MyPair< T1, T2 > ) -> Self { ( src.0, src.1 ) }
}

#[ cfg( feature = "make" ) ]
impl< T1, T2 > Make0 for MyPair< T1, T2 >
where
  T1 : Default,
  T2 : Default,
{
  fn make_0() -> Self { Self( Default::default(), Default::default() ) }
}

#[ cfg( feature = "make" ) ]
impl< T1, T2 > Make2< T1, T2 > for MyPair< T1, T2 >
{
  fn make_2( _0 : T1, _1 : T2 ) -> Self { Self( _0, _1 ) }
}

/* ... */

let x = MyPair( 13, 13.0 );
dbg!( x );
// prints : x = MyPair( 13, 13.0 )
```

### Sample :: single-line homopair

If you need to wrap pair of elements with the same type use the type constructor `pair`. The same type constructor `pair` for both `pair` and `homopair`, difference in number of types in definition, `homopair` has only one, because both its element has the same type. The same macro `types` is responsible for generating code for both `single`, `pair` and also `many`.

```rust
use type_constructor::prelude::*;

types!( pub pair MyPair : i32, i64 );
let x = MyPair( 13, 31 );
println!( "x : ( {}, {} )", x.0, x.1 );
// prints : x : ( 13, 31 )
```

It gererates code:

```rust
use type_constructor::prelude::*;

pub struct MyPair( pub i32, pub i64 );

impl From< ( i32, i64 ) > for MyPair
{
  fn from( src : ( i32, i64 ) ) -> Self { Self( src.0, src.1 ) }
}

impl From< MyPair > for ( i32, i64 )
{
  fn from( src : MyPair ) -> Self { ( src.0, src.1 ) }
}

#[ cfg( feature = "make" ) ]
impl Make2< i32, i64 > for MyPair
{
  fn make_2( _0 : i32, _1 : i64 ) -> Self { Self( _0, _1 ) }
}

/* ... */

let x = MyPair( 13, 31 );
println!( "x : ( {}, {} )", x.0, x.1 );
```

### Sample :: homopair with parameters

Unlike `heteropair` `homopair` has much more traits implemented for it. Among such are: `clone_as_tuple`, `clone_as_array` to clone it as either tuple or array, `as_tuple`, `as_array`, `as_slice` to reinterpret it as either tuple or array or slice, traits `From`/`Into` are implemented to convert it from/into tuple, array, slice, scalar.

```rust
use type_constructor::prelude::*;

use core::fmt;
types!
{
  #[ derive( Debug ) ]
  pub pair MyHomoPair : < T : fmt::Debug >;
}
let x = MyHomoPair( 13, 31 );
dbg!( &x );
// prints : &x = MyHomoPair( 13, 31 )
let clone_as_array : [ i32 ; 2 ] = x.clone_as_array();
dbg!( &clone_as_array );
// prints : &clone_as_array = [ 13, 31 ]
let clone_as_tuple : ( i32 , i32 ) = x.clone_as_tuple();
dbg!( &clone_as_tuple );
// prints : &clone_as_tuple = ( 13, 31 )
```

It gererates code:

```rust
use type_constructor::prelude::*;
use core::fmt;

#[ derive( Debug ) ]
pub struct MyHomoPair< T >( pub T, pub T );

impl< T > core::ops::Deref for MyHomoPair< T >
{
  type Target = ( T, T );

  fn deref( &self ) -> &Self::Target
  {
    #[ cfg( debug_assertions ) ]
    {
      let layout1 = std::alloc::Layout::new::< Self >();
      let layout2 = std::alloc::Layout::new::< Self::Target >();
      debug_assert_eq!( layout1, layout2 );
    }
    unsafe { std::mem::transmute::< _, _ >( self ) }
  }
}

impl< T > core::ops::DerefMut for MyHomoPair< T >
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    #[ cfg( debug_assertions ) ]
    {
      let layout1 = std::alloc::Layout::new::< Self >();
      let layout2 = std::alloc::Layout::new::< Self::Target >();
      debug_assert_eq!( layout1, layout2 );
    }
    unsafe { std::mem::transmute::< _, _ >( self ) }
  }
}

impl< T > From< ( T, T ) > for MyHomoPair< T >
{
  fn from( src : ( T, T ) ) -> Self { Self( src.0, src.1 ) }
}

impl< T > From< MyHomoPair< T >> for ( T, T )
{
  fn from( src : MyHomoPair< T > ) -> Self { ( src.0, src.1 ) }
}

impl< T > From< [ T; 2 ] > for MyHomoPair< T >
where
  T : Clone,
{
  fn from( src : [ T; 2 ] ) -> Self { Self( src[ 0 ].clone(), src[ 1 ].clone() ) }
}

impl< T > From< MyHomoPair< T >> for [ T; 2 ]
{
  fn from( src : MyHomoPair< T > ) -> Self { [ src.0, src.1 ] }
}

impl< T > From< &[ T ] > for MyHomoPair< T >
where
  T : Clone,
{
  fn from( src : &[ T ] ) -> Self
  {
    debug_assert_eq!( src.len(), 2 );
    Self( src[ 0 ].clone(), src[ 1 ].clone() )
  }
}

impl< T > From< T > for MyHomoPair< T >
where
  T : Clone,
{
  fn from( src : T ) -> Self { Self( src.clone(), src.clone() ) }
}

impl< T > CloneAsTuple< ( T, T ) > for MyHomoPair< T >
where
  T : Clone,
{
  fn clone_as_tuple( &self ) -> ( T, T ) { ( self.0.clone(), self.1.clone() ) }
}

impl< T > CloneAsArray< T, 2 > for MyHomoPair< T >
where
  T : Clone,
{
  fn clone_as_array( &self ) -> [ T; 2 ] { [ self.0.clone(), self.1.clone() ] }
}

impl< T > AsTuple< ( T, T ) > for MyHomoPair< T >
{
  fn as_tuple( &self ) -> &( T, T ) { unsafe { std::mem::transmute::< &_, &( T, T ) >( self ) } }
}

impl< T > AsArray< T, 2 > for MyHomoPair< T >
{
  fn as_array( &self ) -> &[ T; 2 ] { unsafe { std::mem::transmute::< &_, &[ T; 2 ] >( self ) } }
}

impl< T > AsSlice< T > for MyHomoPair< T >
{
  fn as_slice( &self ) -> &[ T ] { &self.as_array()[ .. ] }
}

#[ cfg( feature = "make" ) ]
impl< T > Make0 for MyHomoPair< T >
where
  T : Default,
{
  fn make_0() -> Self { Self( Default::default(), Default::default() ) }
}

#[ cfg( feature = "make" ) ]
impl< T > Make1< T > for MyHomoPair< T >
where
  T : Clone,
{
  fn make_1( _0 : T ) -> Self { Self( _0.clone(), _0.clone() ) }
}

#[ cfg( feature = "make" ) ]
impl< T > Make2< T, T > for MyHomoPair< T >
{
  fn make_2( _0 : T, _1 : T ) -> Self { Self( _0, _1 ) }
}

/* ... */

let x = MyHomoPair( 13, 31 );
dbg!( &x );
// prints : &x = MyHomoPair( 13, 31 )
let clone_as_array : [ i32 ; 2 ] = x.clone_as_array();
dbg!( &clone_as_array );
// prints : &clone_as_array = [ 13, 31 ]
let clone_as_tuple : ( i32 , i32 ) = x.clone_as_tuple();
dbg!( &clone_as_tuple );
// prints : &clone_as_tuple = ( 13, 31 )
```

### Sample :: single-line many

Use type constructor `many` to wrap `Vec` in a tuple. Similar to `single` it has essential traits implemented for it.

```rust
// #[ cfg
// (
//   all
//   (
//     feature = "many",
//     any( feature = "use_std", feature = "use_alloc" ),
//   )
// )]
// {
//   use type_constructor::prelude::*;
//
//   types!( pub many MyMany : i32 );
//   let x = MyMany::from( [ 1, 2, 3 ] );
//   println!( "x : {:?}", x.0 );
// }
```

It generates code:

```rust
use type_constructor::prelude::*;

pub struct MyMany( pub std::vec::Vec< i32 > );

impl core::ops::Deref for MyMany
{
  type Target = std::vec::Vec< i32 >;

  fn deref( &self ) -> &Self::Target { &self.0 }
}

impl core::ops::DerefMut for MyMany
{
  fn deref_mut( &mut self ) -> &mut Self::Target { &mut self.0 }
}

impl From< i32 > for MyMany
{
  fn from( src : i32 ) -> Self { Self( vec![ src ] ) }
}

impl From< ( i32, ) > for MyMany
{
  fn from( src : ( i32, ) ) -> Self { Self( vec![ src.0 ] ) }
}

impl< const N: usize > From< [ i32; N ] > for MyMany
where
  i32 : Clone,
{
  fn from( src : [ i32; N ] ) -> Self { Self( std::vec::Vec::from( src ) ) }
}

impl From< &[ i32 ] > for MyMany
where
  i32 : Clone,
{
  fn from( src : &[ i32 ] ) -> Self
  {
    debug_assert_eq!( src.len(), 1 );
    Self( std::vec::Vec::from( src ) )
  }
}

impl AsSlice< i32 > for MyMany
where
  i32 : Clone,
{
  fn as_slice( &self ) -> &[ i32 ] { &self[ .. ] }
}

#[ cfg( feature = "make" ) ]
impl Make0 for MyMany
{
  fn make_0() -> Self { Self( std::vec::Vec::< i32 >::new() ) }
}

#[ cfg( feature = "make" ) ]
impl Make1< i32 > for MyMany
{
  fn make_1( _0 : i32 ) -> Self { Self( vec![ _0 ] ) }
}

#[ cfg( feature = "make" ) ]
impl Make2< i32, i32 > for MyMany
{
  fn make_2( _0 : i32, _1 : i32 ) -> Self { Self( vec![ _0, _1 ] ) }
}

#[ cfg( feature = "make" ) ]
impl Make3< i32, i32, i32 > for MyMany
{
  fn make_3( _0 : i32, _1 : i32, _2 : i32 ) -> Self { Self( vec![ _0, _1, _2 ] ) }
}

/* ... */

let x = MyMany::from( [ 1, 2, 3 ] );
println!( "x : {:?}", x.0 );
```

### Sample :: make - variadic constructor

Implement traits [Make0], [Make1] up to MakeN to provide the interface to construct your structure with a different set of arguments.
In this example structure, Struct1 could be constructed either without arguments, with a single argument, or with two arguments.
- Constructor without arguments fills fields with zero.
- Constructor with a single argument sets both fields to the value of the argument.
- Constructor with 2 arguments set individual values of each field.

```rust
#[ cfg( feature = "make" ) ]
{
  use type_constructor::prelude::*;

  #[ derive( Debug, PartialEq ) ]
  struct Struct1
  {
    a : i32,
    b : i32,
  }

  impl Make0 for Struct1
  {
    fn make_0() -> Self
    {
      Self { a : 0, b : 0 }
    }
  }

  impl Make1< i32 > for Struct1
  {
    fn make_1( val : i32 ) -> Self
    {
      Self { a : val, b : val }
    }
  }

  impl Make2< i32, i32 > for Struct1
  {
    fn make_2( val1 : i32, val2 : i32 ) -> Self
    {
      Self { a : val1, b : val2 }
    }
  }

  let got : Struct1 = make!();
  let exp = Struct1{ a : 0, b : 0 };
  assert_eq!( got, exp );

  let got : Struct1 = make!( 13 );
  let exp = Struct1{ a : 13, b : 13 };
  assert_eq!( got, exp );

  let got : Struct1 = make!( 1, 3 );
  let exp = Struct1{ a : 1, b : 3 };
  assert_eq!( got, exp );
}
```

### To add to your project

``` shell
cargo add type_constructor
```

## Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/type_constructor_trivial_sample
cargo run
```
