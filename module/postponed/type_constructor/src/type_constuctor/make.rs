// /// Internal namespace.
// #[ cfg( feature = "make" ) ]
// pub( crate ) mod private
// {
//
//   ///
//   /// Constructor without arguments.
//   ///
//
//   pub trait From_0
//   where
//     Self : Sized,
//   {
//     /// Constructor without arguments.
//     fn make() -> Self
//     {
//       Self::from_0()
//     }
//     /// Constructor without arguments.
//     fn from_0() -> Self;
//   }
//
//   // xxx : auto impl from Default, please
//
//   ///
//   /// Constructor with single argument.
//   ///
//
//   pub trait From_1< Arg >
//   where
//     Self : Sized,
//   {
//     /// Constructor without arguments.
//     fn make( arg : Arg ) -> Self
//     {
//       Self::from_1( arg )
//     }
//     /// Constructor without arguments.
//     fn from_1( arg : Arg ) -> Self;
//   }
//
//   ///
//   /// Constructor with two arguments.
//   ///
//
//   pub trait From_2< Arg1, Arg2 >
//   where
//     Self : Sized,
//   {
//     /// Constructor with two arguments.
//     fn make( arg1 : Arg1, arg2 : Arg2 ) -> Self
//     {
//       Self::from_2( arg1, arg2 )
//     }
//     /// Constructor with two arguments.
//     fn from_2( arg1 : Arg1, arg2 : Arg2 ) -> Self;
//   }
//
//   ///
//   /// Constructor with three arguments.
//   ///
//
//   pub trait From_3< Arg1, Arg2, Arg3 >
//   where
//     Self : Sized,
//   {
//     /// Constructor with three arguments.
//     fn make( arg1 : Arg1, arg2 : Arg2, arg3 : Arg3 ) -> Self
//     {
//       Self::from_3( arg1, arg2, arg3 )
//     }
//     /// Constructor with three arguments.
//     fn from_3( arg1 : Arg1, arg2 : Arg2, arg3 : Arg3 ) -> Self;
//   }
//
// //   ///
// //   /// Constructor with four arguments.
// //   ///
// //
// //   pub trait From_4< Arg1, Arg2, Arg3, Arg4 >
// //   where
// //     Self : Sized,
// //   {
// //     /// Constructor with four arguments.
// //     fn make( arg1 : Arg1, arg2 : Arg2, arg3 : Arg3, arg4 : Arg4 ) -> Self
// //     {
// //       Self::from_4( arg1, arg2, arg3, arg4 )
// //     }
// //     /// Constructor with four arguments.
// //     fn from_4( arg1 : Arg1, arg2 : Arg2, arg3 : Arg3, arg4 : Arg4 ) -> Self;
// //   }
//
//   ///
//   /// Variadic constructor.
//   ///
//   /// Implement traits [From_0], [From_1] up to MakeN to provide the interface to construct your structure with a different set of arguments.
//   /// In this example structure, Struct1 could be constructed either without arguments, with a single argument, or with two arguments.
//   /// - Constructor without arguments fills fields with zero.
//   /// - Constructor with a single argument sets both fields to the value of the argument.
//   /// - Constructor with 2 arguments set individual values of each field.
//   ///
//   /// ```rust
//   /// #[ cfg( feature = "make" ) ]
//   /// {
//   ///   use type_constructor::prelude::*;
//   ///
//   ///   #[ derive( Debug, PartialEq ) ]
//   ///   struct Struct1
//   ///   {
//   ///     a : i32,
//   ///     b : i32,
//   ///   }
//   ///
//   ///   impl From_0 for Struct1
//   ///   {
//   ///     fn from_0() -> Self
//   ///     {
//   ///       Self { a : 0, b : 0 }
//   ///     }
//   ///   }
//   ///
//   ///   impl From_1< i32 > for Struct1
//   ///   {
//   ///     fn from_1( val : i32 ) -> Self
//   ///     {
//   ///       Self { a : val, b : val }
//   ///     }
//   ///   }
//   ///
//   ///   impl From_2< i32, i32 > for Struct1
//   ///   {
//   ///     fn from_2( val1 : i32, val2 : i32 ) -> Self
//   ///     {
//   ///       Self { a : val1, b : val2 }
//   ///     }
//   ///   }
//   ///
//   ///   let got : Struct1 = from!();
//   ///   let exp = Struct1{ a : 0, b : 0 };
//   ///   assert_eq!( got, exp );
//   ///
//   ///   let got : Struct1 = from!( 13 );
//   ///   let exp = Struct1{ a : 13, b : 13 };
//   ///   assert_eq!( got, exp );
//   ///
//   ///   let got : Struct1 = from!( 1, 3 );
//   ///   let exp = Struct1{ a : 1, b : 3 };
//   ///   assert_eq!( got, exp );
//   /// }
//   ///
//   /// ```
//   ///
//   /// ### To add to your project
//   ///
//   /// ``` shell
//   /// cargo add type_constructor
//   /// ```
//   ///
//   /// ## Try out from the repository
//   ///
//   /// ``` shell test
//   /// git clone https://github.com/Wandalen/wTools
//   /// cd wTools
//   /// cd examples/type_constructor_trivial
//   /// cargo run
//   /// ```
//
//   #[ macro_export ]
//   macro_rules! make
//   {
//
//     (
//       $(,)?
//     )
//     =>
//     {
//       $crate::From_0::from_0();
//     };
//
//     (
//       $Arg1 : expr $(,)?
//     )
//     =>
//     {
//       $crate::From_1::from_1( $Arg1 );
//     };
//
//     (
//       $Arg1 : expr, $Arg2 : expr $(,)?
//     )
//     =>
//     {
//       $crate::From_2::from_2( $Arg1, $Arg2 );
//     };
//
//     (
//       $Arg1 : expr, $Arg2 : expr, $Arg3 : expr $(,)?
//     )
//     =>
//     {
//       $crate::From_3::from_3( $Arg1, $Arg2, $Arg3 );
//     };
//
//     // (
//     //   $Arg1 : expr, $Arg2 : expr, $Arg3 : expr, $Arg4 : expr $(,)?
//     // )
//     // =>
//     // {
//     //   $crate::From_4::from_4( $Arg1, $Arg2, $Arg3, $Arg4 );
//     // };
//
//     (
//       $( $Rest : tt )+
//     )
//     =>
//     {
//       compile_error!
//       (
//         concat!
//         (
//           "Variadic constructor supports up to 3 arguments.\n",
//           "Open an issue if you need more.\n",
//           "You passed:\n",
//           stringify!
//           (
//             from!( $( $Rest )+ )
//           )
//         )
//       );
//     };
//
//   }
//
//   pub use make;
// }
//
// /// Own namespace of the module.
// pub mod own
// {
//   #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
//   pub use orphan::*;
// }
//
// #[ doc( inline ) ]
// #[ allow( unused_imports ) ]
// pub use own::*;
//
// /// Orphan namespace of the module.
// pub mod orphan
// {
//   #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
//   pub use exposed::*;
// }
//
// /// Exposed namespace of the module.
// pub mod exposed
// {
//   #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
//   pub use prelude::*;
// }
//
// #[ doc( inline ) ]
// #[ allow( unused_imports ) ]
// pub use exposed::*;
//
// /// Prelude to use essentials: `use my_module::prelude::*`.
// pub mod prelude
// {
//   #[ cfg( feature = "make" ) ]
//   #[ doc( inline ) ]
  // // #[ allow( unused_imports ) ]
//   pub use private::
//   {
//
//     From_0,
//     From_1,
//     From_2,
//     From_3,
//     // From_4,
//
//     make,
//
//   };
//
//   #[ cfg( feature = "make" ) ]
//   pub use type_constructor_make_meta::VariadicFrom;
// }
