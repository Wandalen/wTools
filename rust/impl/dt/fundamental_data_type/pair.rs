// /// Internal namespace.
// mod internal
// {
//   use crate::exposed::*;
//
//   ///
//   /// Type constructor to define tuple wrapping a given type.
//   ///
//   /// Quite often you need to wrap a given type into new one.
//   /// For example if orphan rule became and obstacle one should introduce a new type wrapping foreing one.
//   /// Type constructr `pair!` does exaclty that and auto-implement traits From, Into and Deref for the constructed type.
//   ///
//   /// ### Sample :: pair line pair.
//   ///
//   /// To define your own pair use macro `pair!`. Pair-line definition looks like that.
//   ///
//   /// ```rust
//   /// use fundamental_data_type::prelude::*;
//   /// pair!( MyPair : i32 );
//   /// let x = MyPair( 13 );
//   /// println!( "x : {}", x.0 );
//   /// ```
//   ///
//   /// It gererates code:
//   ///
//   /// ```rust
//   /// use fundamental_data_type::prelude::*;
//   ///
//   /// pub struct MyPair( pub i32 );
//   ///
//   /// impl core::ops::Deref for MyPair
//   /// {
//   ///   type Target = i32;
//   ///   fn deref( &self ) -> &Self::Target
//   ///   {
//   ///     &self.0
//   ///   }
//   /// }
//   /// impl From< i32 > for MyPair
//   /// {
//   ///   fn from( src : i32 ) -> Self
//   ///   {
//   ///     Self( src )
//   ///   }
//   /// }
//   /// impl From< MyPair > for i32
//   /// {
//   ///   fn from( src : MyPair ) -> Self
//   ///   {
//   ///     src.0
//   ///   }
//   /// }
//   ///
//   /// let x = MyPair( 13 );
//   /// println!( "x : {}", x.0 );
//   /// ```
//   ///
//   /// ### Sample :: pair with derives and attributes.
//   ///
//   /// It's possible to define attributes as well as derives.
//   ///
//   /// ```rust
//   /// use fundamental_data_type::prelude::*;
//   /// pair!
//   /// {
//   ///   /// This is also attribute and macro understands it.
//   ///   #[ derive( Debug ) ]
//   ///   MyPair : i32;
//   /// }
//   /// let x = MyPair( 13 );
//   /// dbg!( x );
//   /// ```
//   ///
//   /// It gererates code:
//   ///
//   /// ```rust
//   /// use fundamental_data_type::prelude::*;
//   ///
//   /// /// This is also attribute and macro understands it.
//   /// #[ derive( Debug ) ]
//   /// pub struct MyPair( pub i32 );
//   ///
//   /// impl core::ops::Deref for MyPair
//   /// {
//   ///   type Target = i32;
//   ///   fn deref( &self ) -> &Self::Target
//   ///   {
//   ///     &self.0
//   ///   }
//   /// }
//   /// impl From< i32 > for MyPair
//   /// {
//   ///   fn from( src : i32 ) -> Self
//   ///   {
//   ///     Self( src )
//   ///   }
//   /// }
//   /// impl From< MyPair > for i32
//   /// {
//   ///   fn from( src : MyPair ) -> Self
//   ///   {
//   ///     src.0
//   ///   }
//   /// }
//   ///
//   /// let x = MyPair( 13 );
//   /// dbg!( x );
//   /// ```
//   ///
//   /// ### Sample :: pair with struct instead of macro.
//   ///
//   /// Sometimes it's sufficient to use common type instead of defining a brand new.
//   /// You may use paramtetrized struct `Pair< T >` instead of macro `pair!` if that is the case.
//   ///
//   /// ```rust
//   /// use fundamental_data_type::prelude::*;
//   /// let x = Pair::< i32 >( 13 );
//   /// dbg!( x );
//   /// ```
//   ///
//   /// ### Sample :: pair with parametrized element.
//   ///
//   /// Element of tuple could be parametrized.
//   ///
//   /// ```rust
//   /// use fundamental_data_type::prelude::*;
//   /// pair!
//   /// {
//   ///   #[ derive( Debug ) ]
//   ///   MyPair : std::sync::Arc< T : Copy >;
//   /// }
//   /// let x = MyPair( std::sync::Arc::new( 13 ) );
//   /// dbg!( x );
//   /// ```
//   ///
//   /// It gererates code:
//   ///
//   /// ```rust
//   /// use fundamental_data_type::*;
//   ///
//   /// #[ derive( Debug ) ]
//   /// pub struct MyPair< T : Copy >( pub std::sync::Arc< T > );
//   ///
//   /// impl<T: Copy> core::ops::Deref for MyPair< T >
//   /// {
//   ///   type Target = std::sync::Arc< T >;
//   ///   fn deref( &self ) -> &Self::Target
//   ///   {
//   ///     &self.0
//   ///   }
//   /// }
//   /// impl< T : Copy > From< std::sync::Arc< T > > for MyPair< T >
//   /// {
//   ///   fn from( src : std::sync::Arc<T>) -> Self {
//   ///     Self( src )
//   ///   }
//   /// }
//   /// impl< T : Copy > From< MyPair< T > > for std::sync::Arc< T >
//   /// {
//   ///   fn from(src: MyPair<T>) -> Self
//   ///   {
//   ///     src.0
//   ///   }
//   /// }
//   ///
//   /// let x = MyPair( std::sync::Arc::new( 13 ) );
//   /// ```
//   ///
//   /// ### Sample :: pair with parametrized tuple.
//   ///
//   /// Instead of parametrizing the element it's possible to define a parametrized tuple.
//   ///
//   ///
//   /// ```rust
//   /// use fundamental_data_type::prelude::*;
//   /// pair!
//   /// {
//   ///   #[ derive( Debug ) ]
//   ///   MyPair : < T : Copy >;
//   /// }
//   /// let x = MyPair( 13 );
//   /// dbg!( x );
//   /// ```
//   ///
//   /// It gererates code:
//   ///
//   /// ```rust
//   /// #[ derive( Debug ) ]
//   /// pub struct MyPair( pub i32 );
//   ///
//   /// impl core::ops::Deref for MyPair
//   /// {
//   ///   type Target = i32;
//   ///   fn deref( &self ) -> &Self::Target
//   ///   {
//   ///     &self.0
//   ///   }
//   /// }
//   /// impl From< i32 > for MyPair
//   /// {
//   ///   fn from( src : i32 ) -> Self
//   ///   {
//   ///     Self( src )
//   ///   }
//   /// }
//   /// impl From< MyPair > for i32
//   /// {
//   ///   fn from( src : MyPair ) -> Self
//   ///   {
//   ///     src.0
//   ///   }
//   /// }
//   /// let x = MyPair( 13 );
//   /// dbg!( 13 );
//   /// ```
//   ///
//
//   // #[ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]
//
//   #[ macro_export ]
//   macro_rules! pair
//   {
//
//     // Pair : < T >;
//
//     (
//       $( #[ $Meta : meta ] )*
//       $Name : ident :
//       < $ParamName : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy2 : path )* )? >
//       $(;)?
//     )
//     =>
//     {
//       $( #[ $Meta ] )*
//       pub struct $Name
//       < $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
//       ( pub $ParamName );
//
//       impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? > core::ops::Deref
//       for $Name
//       < $ParamName >
//       {
//         type Target = $ParamName;
//         fn deref( &self ) -> &Self::Target
//         {
//           &self.0
//         }
//       }
//
//       impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? > From< $ParamName >
//       for $Name
//       < $ParamName >
//       {
//         fn from( src : $ParamName ) -> Self
//         {
//           Self( src )
//         }
//       }
//
//       // From Pair Into Element cant be implemented because of Rust restructions.
//
//     };
//
//     // Pair : < T1, ... >;
//
//     (
//       $( #[ $Meta : meta ] )*
//       $Name : ident :
//       < $ParamName : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy2 : path )* )? ,
//       $( $Rest : tt )*
//     )
//     =>
//     {
//       compile_error!
//       (
//         concat!
//         (
//           "Parametrized element should be pair, because Pair has only one element\n",
//           stringify!
//           (
//             $( #[ $Meta ] )*
//             $Name :
//             < $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ,
//             $( $Rest )*
//           )
//         )
//       );
//     };
//
//     // Pair : Element< T1, T2, ... >;
//
//     (
//       $( #[ $Meta : meta ] )*
//       $Name : ident : $TypeSplit1 : ident $( :: $TypeSplitN : ident )*
//       < $( $ParamName : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy2 : path )* )? ),* >
//       $(;)?
//     )
//     =>
//     {
//       $( #[ $Meta ] )*
//       pub struct $Name
//       < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* >
//       ( pub $TypeSplit1 $( :: $TypeSplitN )* < $( $ParamName ),* > );
//
//       impl< $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > core::ops::Deref
//       for $Name
//       < $( $ParamName ),* >
//       {
//         type Target = $TypeSplit1 $( :: $TypeSplitN )* < $( $ParamName ),* >;
//         fn deref( &self ) -> &Self::Target
//         {
//           &self.0
//         }
//       }
//
//       impl< $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* >
//       From< $TypeSplit1 $( :: $TypeSplitN )* < $( $ParamName ),* > >
//       for $Name
//       < $( $ParamName ),* >
//       {
//         fn from( src : $TypeSplit1 $( :: $TypeSplitN )* < $( $ParamName ),* > ) -> Self
//         {
//           Self( src )
//         }
//       }
//
//       impl< $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* >
//       From< $Name< $( $ParamName ),* > >
//       for $TypeSplit1 $( :: $TypeSplitN )* < $( $ParamName ),* >
//       {
//         fn from( src : $Name< $( $ParamName ),* > ) -> Self
//         {
//           src.0
//         }
//       }
//
//     };
//
//     // Pair : Element;
//
//     (
//       $( #[ $Meta : meta ] )*
//       $Name : ident : $TypeSplit1 : ident $( :: $TypeSplitN : ident )* $(;)?
//       // $Name : ident : $Type : ty $(;)?
//     )
//     =>
//     {
//       $crate::pair!
//       (
//         $( #[ $Meta ] )*
//         $Name : $TypeSplit1 $( :: $TypeSplitN )* <>;
//       );
//
//     };
//
//   }
//
// //   pair!
// //   {
// //
// //     ///
// //     /// A type wrapping a another type into a tuple.
// //     ///
// //     ///
// //     /// ### Sample :: struct instead of macro.
// //     ///
// //     /// Sometimes it's sufficient to use common type instead of defining a brand new one.
// //     /// You may use paramtetrized struct `fundamental_data_type::Pair< T >` instead of macro `fundamental_data_type::pair!` if that is the case.
// //     ///
// //     /// ```rust
// //     /// use fundamental_data_type::prelude::*;
// //     /// let x = Pair::< i32 >( 13 );
// //     /// dbg!( x );
// //     /// ```
// //     ///
// //
// //     #[ derive( Debug, Clone, PartialEq, Eq ) ]
// //     Pair : < T >;
// //
// //   }
//
//   pub use pair;
// }

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

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // pub use super::internal::
  // {
  //   // pair,
  //   // Pair,
  // };
}
