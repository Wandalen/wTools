/// Internal namespace.
mod internal
{
  use crate::exposed::*;

  // xxx : no std
  // qqq : introduce features for type_constructor
  // qqq : for Dima : paste generated code for each sample
  //
  // xxx : redo implements
  // xxx : add core::fmt to prelude
  // xxx : write article about the module

  // xxx : use
  // - [pretty_assertions](https://github.com/colin-kiegel/rust-pretty-assertions) by [Colin Kiegel](https://github.com/colin-kiegel) :
  //  pretty assert_eq
  // xxx : introduce a_is

  ///
  /// Type constructor to define tuple wrapping a given type.
  ///
  /// In Rust, you often need to wrap a given type into a new one.
  /// The role of the orphan rules in particular is basically to prevent you from implementing external traits for external types.
  /// To overcome the restriction developer usually wrap the external type into a tuple introducing a new type.
  /// Type constructor does exactly that and auto-implement traits From, Into, Deref and few more for the constructed type.
  ///
  /// Besides type constructor for single element there are type constructors for `pair`, `homopair` and `many`:
  ///
  /// - `Single` to wrap single element.
  /// - `Pair` to wrap pair of distinct elements.
  /// - `HomoPair` to wrap pair of elements with the same type.
  /// - `Many` to wrap `Vec` of elements.
  ///
  /// ## Macro `types` for type constructing
  ///
  /// The same macro `types` is responsible for generating code for Single, Pair, Homopair, Many. Each type constructor has its own keyword for that, but Pair and Homopair use the same keyword difference in a number of constituent types. It is possible to define all types at once.
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  ///
  /// types!
  /// {
  ///
  ///   single MySingle : f32;
  ///   single SingleWithParametrized : std::sync::Arc< T : Copy >;
  ///   single SingleWithParameter : < T >;
  ///
  ///   pair MyPair : f32;
  ///   pair PairWithParametrized : std::sync::Arc< T1 : Copy >, std::sync::Arc< T2 : Copy >;
  ///   pair PairWithParameter : < T1, T2 >;
  ///
  ///   pair MyHomoPair : f32;
  ///   pair HomoPairWithParametrized : std::sync::Arc< T : Copy >;
  ///   pair HomoPairWithParameter : < T >;
  ///
  ///   many MyMany : f32;
  ///   many ManyWithParametrized : std::sync::Arc< T : Copy >;
  ///   many ManyWithParameter : < T >;
  ///
  /// }
  /// ```
  ///
  /// It generates more than 1000 lines of code, which otherwise you would have to write manually.
  ///
  /// ## Without macro
  ///
  /// Macro `types` is exposed to generate new types, but in some cases, it is enough to reuse already generated types of such kind. The library ships such types: Single, Pair, Homopair, Many. Note: If you avoid generating new types you will get in a position to be not able to define your own implementation of foreign traits because of orphan rule.
  ///
  /// ```rust
  /// let i32_in_tuple = type_constructor::Single::< i32 >::from( 13 );
  /// dbg!( i32_in_tuple );
  /// // i32_in_tuple = Single( 13 )
  /// let i32_and_f32_in_tuple = type_constructor::Pair::< i32, f32 >::from( ( 13, 13.0 ) );
  /// dbg!( i32_and_f32_in_tuple );
  /// // vec_of_i32_in_tuple = Pair( 13, 13.0 )
  /// let two_i32_in_tuple = type_constructor::HomoPair::< i32 >::from( ( 13, 31 ) );
  /// dbg!( two_i32_in_tuple );
  /// // vec_of_i32_in_tuple = HomoPair( 13, 31 )
  /// let vec_of_i32_in_tuple = type_constructor::Many::< i32 >::from( [ 1, 2, 3 ] );
  /// dbg!( vec_of_i32_in_tuple );
  /// // vec_of_i32_in_tuple = Many([ 1, 2, 3 ])
  /// ```
  ///
  /// ### Sample :: homopair with parameters
  ///
  /// Unlike `heteropair` `homopair` has much more traits implemented for it. Among such are: `clone_as_tuple`, `clone_as_array` to clone it as either tuple or array, `as_tuple`, `as_array`, `as_slice` to reinterpret it as either tuple or array or slice, traits `From`/`Into` are implemented to convert it from/into tuple, array, slice, scalar.
  ///
  ///
  /// ## Make.
  ///
  /// Make is the variadic constructor. It's the unified interface of the arbitrary-length constructor.
  /// After implementing several traits `Make0`, `Make1` up to `MakeN` one can use make `make!` to construct instances.
  ///
  /// ```rust ignore
  /// let instance1 : Struct1 = make!();
  /// let instance2 : Struct1 = make!( 13 );
  /// let instance3 : Struct1 = make!( 1, 3 );
  /// ```
  ///
  /// ### Sample :: single line single.
  ///
  /// To define your own single-use macro `types!`. The single-line definition looks like that.
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  /// types!( single MySingle : i32 );
  /// let x = MySingle( 13 );
  /// println!( "x : {}", x.0 );
  /// ```
  ///
  /// It generates code:
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  ///
  /// pub struct MySingle( pub i32 );
  ///
  /// impl core::ops::Deref for MySingle
  /// {
  ///   type Target = i32;
  ///   fn deref( &self ) -> &Self::Target
  ///   {
  ///     &self.0
  ///   }
  /// }
  /// impl From< i32 > for MySingle
  /// {
  ///   fn from( src : i32 ) -> Self
  ///   {
  ///     Self( src )
  ///   }
  /// }
  /// impl From< MySingle > for i32
  /// {
  ///   fn from( src : MySingle ) -> Self
  ///   {
  ///     src.0
  ///   }
  /// }
  ///
  /// let x = MySingle( 13 );
  /// println!( "x : {}", x.0 );
  /// ```
  ///
  /// ### Sample :: single with derives and attributes.
  ///
  /// It's possible to define attributes as well as derives.
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  /// types!
  /// {
  ///   /// This is also attribute and macro understands it.
  ///   #[ derive( Debug ) ]
  ///   single MySingle : i32;
  /// }
  /// let x = MySingle( 13 );
  /// dbg!( x );
  /// ```
  ///
  /// It generates code:
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  ///
  /// /// This is also an attribute and macro understands it.
  /// #[ derive( Debug ) ]
  /// pub struct MySingle( pub i32 );
  ///
  /// impl core::ops::Deref for MySingle
  /// {
  ///   type Target = i32;
  ///   fn deref( &self ) -> &Self::Target
  ///   {
  ///     &self.0
  ///   }
  /// }
  /// impl From< i32 > for MySingle
  /// {
  ///   fn from( src : i32 ) -> Self
  ///   {
  ///     Self( src )
  ///   }
  /// }
  /// impl From< MySingle > for i32
  /// {
  ///   fn from( src : MySingle ) -> Self
  ///   {
  ///     src.0
  ///   }
  /// }
  ///
  /// let x = MySingle( 13 );
  /// dbg!( x );
  /// ```
  ///
  /// ### Sample :: single with struct instead of macro.
  ///
  /// Sometimes it's sufficient to use a common type instead of defining a brand new one.
  /// You may use parameterized struct `Single< T >` instead of macro `types!` if that is the case.
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  /// let x = Single::< i32 >( 13 );
  /// dbg!( x );
  /// ```
  ///
  /// ### Sample :: single with a parametrized element.
  ///
  /// Element of tuple could be parametrized.
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  /// types!
  /// {
  ///   #[ derive( Debug ) ]
  ///   single MySingle : std::sync::Arc< T : Copy >;
  /// }
  /// let x = MySingle( std::sync::Arc::new( 13 ) );
  /// dbg!( x );
  /// ```
  ///
  /// It generates code:
  ///
  /// ```rust
  /// use type_constructor::*;
  ///
  /// #[ derive( Debug ) ]
  /// pub struct MySingle< T : Copy >( pub std::sync::Arc< T > );
  ///
  /// impl<T: Copy> core::ops::Deref for MySingle< T >
  /// {
  ///   type Target = std::sync::Arc< T >;
  ///   fn deref( &self ) -> &Self::Target
  ///   {
  ///     &self.0
  ///   }
  /// }
  /// impl< T : Copy > From< std::sync::Arc< T > > for MySingle< T >
  /// {
  ///   fn from( src : std::sync::Arc<T>) -> Self {
  ///     Self( src )
  ///   }
  /// }
  /// impl< T : Copy > From< MySingle< T > > for std::sync::Arc< T >
  /// {
  ///   fn from(src: MySingle<T>) -> Self
  ///   {
  ///     src.0
  ///   }
  /// }
  ///
  /// let x = MySingle( std::sync::Arc::new( 13 ) );
  /// ```
  ///
  /// ### Sample :: single with parametrized tuple.
  ///
  /// Instead of parametrizing the element, it's possible to define a parametrized tuple.
  ///
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  /// types!
  /// {
  ///   #[ derive( Debug ) ]
  ///   single MySingle : < T : Copy >;
  /// }
  /// let x = MySingle( 13 );
  /// dbg!( x );
  /// ```
  ///
  /// It generates code:
  ///
  /// ```rust
  /// #[ derive( Debug ) ]
  /// pub struct MySingle< T : Copy >( pub T );
  ///
  /// impl< T : Copy > core::ops::Deref
  /// for MySingle< T >
  /// {
  ///   type Target = T;
  ///   fn deref( &self ) -> &Self::Target
  ///   {
  ///     &self.0
  ///   }
  /// }
  ///
  /// impl< T : Copy > From< T >
  /// for MySingle< T >
  /// {
  ///   fn from( src : T ) -> Self
  ///   {
  ///     Self( src )
  ///   }
  /// }
  ///
  /// let x = MySingle( 13 );
  /// dbg!( 13 );
  /// ```
  ///
  /// ### Sample :: single-line pair
  ///
  /// Sometimes you need to wrap more than a single element into a tupдe. If types of elements are different use `pair`. The same macro `types` is responsible for generating code for both `single`, `pair` and also `many`.
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  ///
  /// types!( pair MyPair : i32, i64 );
  /// let x = MyPair( 13, 31 );
  /// println!( "x : ( {}, {} )", x.0, x.1 );
  /// // prints : x : ( 13, 31 )
  /// ```
  ///
  /// It generates code:
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  ///
  /// pub struct MyPair( pub i32, pub i64 );
  ///
  /// impl From< ( i32, i64 ) > for MyPair
  /// {
  ///   fn from( src : ( i32, i64 ) ) -> Self
  ///   {
  ///     Self( src.0, src.1 )
  ///   }
  /// }
  ///
  /// impl From< MyPair > for ( i32, i64 )
  /// {
  ///   fn from( src : MyPair ) -> Self
  ///   {
  ///     ( src.0, src.1 )
  ///   }
  /// }
  ///
  /// #[ cfg( feature = "make" ) ]
  /// impl Make2< i32, i64 > for MyPair
  /// {
  ///   fn make_2( _0 : i32, _1 : i64 ) -> Self
  ///   {
  ///     Self( _0, _1 )
  ///   }
  /// }
  ///
  /// let x = MyPair( 13, 31 );
  /// println!( "x : ( {}, {} )", x.0, x.1 );
  ///```
  ///
  /// ### Sample :: pair with parameters
  ///
  /// Just like `single` `pair` may have parameters.
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  ///
  /// use core::fmt;
  /// types!
  /// {
  ///   #[ derive( Debug ) ]
  ///   pair MyPair : < T1 : fmt::Debug, T2 : fmt::Debug >;
  /// }
  /// let x = MyPair( 13, 13.0 );
  /// dbg!( x );
  /// // prints : x = MyPair( 13, 13.0 )
  /// ```
  ///
  /// It generates code:
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  /// use core::fmt;
  ///
  /// pub struct MyPair< T1 : fmt::Debug, T2 : fmt::Debug >( pub T1, pub T2 );
  ///
  /// #[ automatically_derived ]
  /// #[ allow( unused_qualifications ) ]
  /// impl< T1 : ::core::fmt::Debug + fmt::Debug, T2 : ::core::fmt::Debug + fmt::Debug > ::core::fmt::Debug for MyPair< T1, T2 >
  /// {
  ///   fn fmt( &self, f : &mut ::core::fmt::Formatter ) -> ::core::fmt::Result
  ///   {
  ///     match *self
  ///     {
  ///       MyPair( ref __self_0_0, ref __self_0_1 ) =>
  ///       {
  ///         let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple( f, "MyPair" );
  ///         let _ = ::core::fmt::DebugTuple::field( debug_trait_builder, &&( *__self_0_0 ) );
  ///         let _ = ::core::fmt::DebugTuple::field( debug_trait_builder, &&( *__self_0_1 ) );
  ///         ::core::fmt::DebugTuple::finish( debug_trait_builder )
  ///       }
  ///     }
  ///   }
  /// }
  ///
  /// impl< T1 : fmt::Debug, T2 : fmt::Debug > From<( T1, T2 )> for MyPair< T1, T2 >
  /// {
  ///   fn from( src : ( T1, T2 ) ) -> Self { Self( src.0, src.1 ) }
  /// }
  ///
  /// impl< T1 : fmt::Debug, T2 : fmt::Debug > From< MyPair< T1, T2 > > for ( T1, T2 )
  /// {
  ///   fn from( src : MyPair< T1, T2 > ) -> Self { ( src.0, src.1 ) }
  /// }
  ///
  /// impl< T1 : fmt::Debug, T2 : fmt::Debug > Make0 for MyPair< T1, T2 >
  /// where
  ///   T1 : Default,
  ///   T2 : Default,
  /// {
  ///   fn make_0() -> Self { Self( Default::default(), Default::default() ) }
  /// }
  ///
  /// impl< T1 : fmt::Debug, T2 : fmt::Debug > Make2< T1, T2 > for MyPair< T1, T2 >
  /// {
  ///   fn make_2( _0 : T1, _1 : T2 ) -> Self { Self( _0, _1 ) }
  /// }
  ///
  /// let x = MyPair( 13, 13.0 );
  /// dbg!( x );
  /// // prints : x = MyPair( 13, 13.0 )
  /// ```
  ///
  /// ### Sample :: single-line homopair
  ///
  /// If you need to wrap pair of elements with the same type use the type constructor `pair`. The same type constructor `pair` for both `pair` and `homopair`, difference in number of types in definition, `homopair` has only one, because both its element has the same type. The same macro `types` is responsible for generating code for both `single`, `pair` and also `many`.
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  ///
  /// types!( pair MyPair : i32, i64 );
  /// let x = MyPair( 13, 31 );
  /// println!( "x : ( {}, {} )", x.0, x.1 );
  /// ```
  ///
  /// It generates code:
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  ///
  /// pub struct MyPair( pub i32, pub i64 );
  ///
  /// impl From< ( i32, i64 ) > for MyPair
  /// {
  ///   fn from( src : ( i32, i64 ) ) -> Self { Self( src.0, src.1 ) }
  /// }
  ///
  /// impl From< MyPair > for ( i32, i64 )
  /// {
  ///   fn from( src : MyPair ) -> Self { ( src.0, src.1 ) }
  /// }
  ///
  /// #[ cfg( feature = "make" ) ]
  /// impl Make2< i32, i64 > for MyPair
  /// {
  ///   fn make_2( _0 : i32, _1 : i64 ) -> Self { Self( _0, _1 ) }
  /// }
  ///
  /// let x = MyPair( 13, 31 );
  /// println!( "x : ( {}, {} )", x.0, x.1 );
  /// ```
  ///
  /// ### Sample :: homopair with parameters
  ///
  /// Unlike `heteropair` `homopair` has much more traits implemented for it. Among such are: `clone_as_tuple`, `clone_as_array` to clone it as either tuple or array, `as_tuple`, `as_array`, `as_slice` to reinterpret it as either tuple or array or slice, traits `From`/`Into` are implemented to convert it from/into tuple, array, slice, scalar.
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  ///
  /// use core::fmt;
  /// types!
  /// {
  ///   #[ derive( Debug ) ]
  ///   pair MyHomoPair : < T : fmt::Debug >;
  /// }
  /// let x = MyHomoPair( 13, 31 );
  /// dbg!( &x );
  /// // prints : &x = MyHomoPair( 13, 31 )
  /// let clone_as_array : [ i32 ; 2 ] = x.clone_as_array();
  /// dbg!( &clone_as_array );
  /// // prints : &clone_as_array = [ 13, 31 ]
  /// let clone_as_tuple : ( i32 , i32 ) = x.clone_as_tuple();
  /// dbg!( &clone_as_tuple );
  /// // prints : &clone_as_tuple = ( 13, 31 )
  /// ```
  ///
  /// It generates code:
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  /// use core::fmt;
  ///
  /// pub struct MyHomoPair< T : fmt::Debug >( pub T, pub T );
  ///
  /// #[ automatically_derived ]
  /// #[ allow( unused_qualifications ) ]
  /// impl< T : ::core::fmt::Debug + fmt::Debug > ::core::fmt::Debug for MyHomoPair< T >
  /// {
  ///   fn fmt( &self, f : &mut ::core::fmt::Formatter ) -> ::core::fmt::Result
  ///   {
  ///     match *self
  ///     {
  ///       MyHomoPair( ref __self_0_0, ref __self_0_1 ) =>
  ///       {
  ///         let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple( f, "MyHomoPair" );
  ///         let _ = ::core::fmt::DebugTuple::field( debug_trait_builder, &&( *__self_0_0 ) );
  ///         let _ = ::core::fmt::DebugTuple::field( debug_trait_builder, &&( *__self_0_1 ) );
  ///         ::core::fmt::DebugTuple::finish( debug_trait_builder )
  ///       }
  ///     }
  ///   }
  /// }
  ///
  /// impl< T : fmt::Debug > core::ops::Deref for MyHomoPair< T >
  /// {
  ///   type Target = ( T, T );
  ///
  ///   fn deref( &self ) -> &Self::Target
  ///   {
  ///     #[ cfg( debug_assertions ) ]
  ///     {
  ///       let layout1 = std::alloc::Layout::new::< Self >();
  ///       let layout2 = std::alloc::Layout::new::< Self::Target >();
  ///       if true
  ///       {
  ///         match ( &layout1, &layout2 )
  ///         {
  ///           ( left_val, right_val ) =>
  ///           {
  ///             if !( *left_val == *right_val )
  ///             {
  ///               let kind = ::core::panicking::AssertKind::Eq;
  ///               ::core::panicking::assert_failed( kind, &*left_val, &*right_val, ::core::option::Option::None );
  ///             }
  ///           }
  ///         };
  ///       };
  ///     }
  ///     unsafe { std::mem::transmute::< _, _ >( self ) }
  ///   }
  /// }
  ///
  /// impl< T : fmt::Debug > core::ops::DerefMut for MyHomoPair< T >
  /// {
  ///   fn deref_mut( &mut self ) -> &mut Self::Target
  ///   {
  ///     #[ cfg( debug_assertions ) ]
  ///     {
  ///       let layout1 = std::alloc::Layout::new::< Self >();
  ///       let layout2 = std::alloc::Layout::new::< Self::Target >();
  ///       if true
  ///       {
  ///         match ( &layout1, &layout2 )
  ///         {
  ///           ( left_val, right_val ) =>
  ///           {
  ///             if !( *left_val == *right_val )
  ///             {
  ///               let kind = ::core::panicking::AssertKind::Eq;
  ///               ::core::panicking::assert_failed( kind, &*left_val, &*right_val, ::core::option::Option::None );
  ///             }
  ///           }
  ///         };
  ///       };
  ///     }
  ///     unsafe { std::mem::transmute::< _, _ >( self ) }
  ///   }
  /// }
  ///
  /// impl< T : fmt::Debug > From< ( T, T ) > for MyHomoPair< T >
  /// {
  ///   fn from( src : ( T, T ) ) -> Self { Self( src.0, src.1 ) }
  /// }
  ///
  /// impl< T : fmt::Debug > From< MyHomoPair< T > > for ( T, T )
  /// {
  ///   fn from( src : MyHomoPair< T > ) -> Self { ( src.0, src.1 ) }
  /// }
  ///
  /// impl< T : fmt::Debug > From< [ T; 2 ] > for MyHomoPair< T >
  /// where
  ///   T : Clone,
  /// {
  ///   fn from( src : [ T; 2] ) -> Self { Self( src[ 0 ].clone(), src[ 1 ].clone() ) }
  /// }
  ///
  /// impl< T : fmt::Debug > From< MyHomoPair< T > > for [ T; 2 ]
  /// {
  ///   fn from( src : MyHomoPair< T > ) -> Self { [ src.0, src.1 ] }
  /// }
  ///
  /// impl< T : fmt::Debug > From< &[ T ] > for MyHomoPair< T >
  /// where
  ///   T : Clone,
  /// {
  ///   fn from( src : &[ T ] ) -> Self
  ///   {
  ///     if true
  ///     {
  ///       match ( &src.len(), &2 )
  ///       {
  ///         ( left_val, right_val ) =>
  ///         {
  ///           if !( *left_val == *right_val )
  ///           {
  ///             let kind = ::core::panicking::AssertKind::Eq;
  ///             ::core::panicking::assert_failed( kind, &*left_val, &*right_val, ::core::option::Option::None );
  ///           }
  ///         }
  ///       };
  ///     };
  ///     Self( src[ 0 ].clone(), src[ 1 ].clone() )
  ///   }
  /// }
  ///
  /// impl< T : fmt::Debug > From< T > for MyHomoPair< T >
  /// where
  ///   T : Clone,
  /// {
  ///   fn from( src : T ) -> Self { Self( src.clone(), src.clone() ) }
  /// }
  ///
  /// impl< T : fmt::Debug > CloneAsTuple< ( T, T ) > for MyHomoPair< T >
  /// where
  ///   T : Clone,
  /// {
  ///   fn clone_as_tuple( &self ) -> ( T, T ) { ( self.0.clone(), self.1.clone() ) }
  /// }
  ///
  /// impl< T : fmt::Debug > CloneAsArray< T, 2 > for MyHomoPair< T >
  /// where
  ///   T : Clone,
  /// {
  ///   fn clone_as_array( &self ) -> [ T; 2 ] { [ self.0.clone(), self.1.clone() ] }
  /// }
  ///
  /// impl< T : fmt::Debug > AsTuple< ( T, T ) > for MyHomoPair< T >
  /// {
  ///   fn as_tuple( &self ) -> &( T, T ) { unsafe { std::mem::transmute::< &_, &( T, T ) >( self ) } }
  /// }
  ///
  /// impl< T : fmt::Debug > AsArray< T, 2 > for MyHomoPair< T >
  /// {
  ///   fn as_array( &self ) -> &[ T; 2 ] { unsafe { std::mem::transmute::< &_, &[ T; 2 ] >( self ) } }
  /// }
  ///
  /// impl< T : fmt::Debug > AsSlice< T > for MyHomoPair< T >
  /// {
  ///   fn as_slice( &self ) -> &[ T ] { &self.as_array()[ .. ] }
  /// }
  ///
  /// impl< T : fmt::Debug > Make0 for MyHomoPair< T >
  /// where
  ///   T : Default,
  /// {
  ///   fn make_0() -> Self { Self( Default::default(), Default::default() ) }
  /// }
  ///
  /// impl< T : fmt::Debug > Make1< T > for MyHomoPair< T >
  /// where
  ///   T : Clone,
  /// {
  ///   fn make_1( _0 : T ) -> Self { Self( _0.clone(), _0.clone() ) }
  /// }
  ///
  /// impl< T : fmt::Debug > Make2< T, T > for MyHomoPair< T >
  /// {
  ///   fn make_2( _0 : T, _1 : T ) -> Self { Self( _0, _1 ) }
  /// }
  ///
  /// let x = MyHomoPair( 13, 31 );
  /// dbg!( &x );
  /// let clone_as_array : [ i32; 2 ] = x.clone_as_array();
  /// dbg!( &clone_as_array );
  /// let clone_as_tuple : ( i32, i32 ) = x.clone_as_tuple();
  /// dbg!( &clone_as_tuple );
  /// ```
  ///
  /// ### Sample :: single-line many
  ///
  /// Use type constructor `many` to wrap `Vec` in a tuple. Similar to `single` it has essential traits implemented for it.
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  ///
  /// types!( many MyMany : i32 );
  /// let x = MyMany::from( [ 1, 2, 3 ] );
  /// println!( "x : {:?}", x.0 );
  /// ```
  ///
  /// It generates code:
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  ///
  /// pub struct MyMany( pub std::vec::Vec< i32 > );
  ///
  /// impl core::ops::Deref for MyMany
  /// {
  ///   type Target = std::vec::Vec< i32 >;
  ///
  ///   fn deref( &self ) -> &Self::Target { &self.0 }
  /// }
  ///
  /// impl core::ops::DerefMut for MyMany
  /// {
  ///   fn deref_mut( &mut self ) -> &mut Self::Target { &mut self.0 }
  /// }
  ///
  /// impl From< i32 > for MyMany
  /// {
  ///   fn from( src : i32 ) -> Self { Self( < [ _ ] >::into_vec( box [ src ] ) ) }
  /// }
  ///
  /// impl From< ( i32, ) > for MyMany
  /// {
  ///   fn from( src : ( i32, ) ) -> Self { Self( < [ _ ] >::into_vec( box [ src.0 ] ) ) }
  /// }
  ///
  /// impl< const N : usize > From< [ i32; N ] > for MyMany
  /// where
  ///   i32 : Clone,
  /// {
  ///   fn from( src : [ i32; N ] ) -> Self { Self( std::vec::Vec::from( src ) ) }
  /// }
  ///
  /// impl From< &[ i32 ] > for MyMany
  /// where
  ///   i32 : Clone,
  /// {
  ///   fn from( src : &[ i32 ] ) -> Self
  ///   {
  ///     if true
  ///     {
  ///       match ( &src.len(), &1 )
  ///       {
  ///         ( left_val, right_val ) =>
  ///         {
  ///           if !( *left_val == *right_val )
  ///           {
  ///             let kind = ::core::panicking::AssertKind::Eq;
  ///             ::core::panicking::assert_failed( kind, &*left_val, &*right_val, ::core::option::Option::None );
  ///           }
  ///         }
  ///       };
  ///     };
  ///     Self( std::vec::Vec::from( src ) )
  ///   }
  /// }
  ///
  /// impl AsSlice< i32 > for MyMany
  /// where
  ///   i32 : Clone,
  /// {
  ///   fn as_slice( &self ) -> &[ i32 ] { &self[ .. ] }
  /// }
  ///
  /// #[ cfg( feature = "make" ) ]
  /// impl Make0 for MyMany
  /// {
  ///   fn make_0( ) -> Self { Self( std::vec::Vec::< i32 >::new( ) ) }
  /// }
  ///
  /// #[ cfg( feature = "make" ) ]
  /// impl Make1< i32 > for MyMany
  /// {
  ///   fn make_1( _0 : i32 ) -> Self { Self( < [ _ ] >::into_vec( box [ _0 ] ) ) }
  /// }
  ///
  /// #[ cfg( feature = "make" ) ]
  /// impl Make2< i32, i32 > for MyMany
  /// {
  ///   fn make_2( _0 : i32, _1 : i32 ) -> Self { Self( < [ _ ] >::into_vec( box [ _0, _1 ] ) ) }
  /// }
  ///
  /// #[ cfg( feature = "make" ) ]
  /// impl Make3< i32, i32, i32 > for MyMany
  /// {
  ///   fn make_3( _0 : i32, _1 : i32, _2 : i32 ) -> Self { Self( < [ _ ] >::into_vec( box [ _0, _1, _2 ] ) ) }
  /// }
  ///
  /// let x = MyMany::from( [ 1, 2, 3 ] );
  /// println!( "x : {:?}", x.0 );
  /// ```

  // #[ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

  #[ macro_export ]
  macro_rules! types
  {

    // No more.

    (
    )
    =>
    {
    };

    // No more.

    (
      ;
    )
    =>
    {
    };

    // single

    (
      $( #[ $Meta : meta ] )*
      single
      $( $Rest : tt )*
    )
    =>
    {
      $crate::_single!
      {
        $( #[ $Meta ] )*
        single
        $( $Rest )*
      }
    };

    // pair

    (
      $( #[ $Meta : meta ] )*
      pair
      $( $Rest : tt )*
    )
    =>
    {
      $crate::_pair!
      {
        $( #[ $Meta ] )*
        pair
        $( $Rest )*
      }
    };

    // many

    (
      $( #[ $Meta : meta ] )*
      many
      $( $Rest : tt )*
    )
    =>
    {
      $crate::_many!
      {
        $( #[ $Meta ] )*
        many
        $( $Rest )*
      }
    };

    // bad syntax

    (
      $( $Rest : tt )*
    )
    =>
    {
      compile_error!
      (
        concat!
        (
          "Bad syntax.\n",
          "Expects : {kind} {name} : {type}.\n",
          "For example : `single MySingle : std::sync::Arc< T : Copy >`.\n",
          "But got:\n",
          stringify!
          (
            $( $Rest )*
          ),
        )
      );
    };

  }

  pub use types;
}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::internal::
  {
    types,
  };
}

