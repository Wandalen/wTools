//!
//! Variadic constructor. Constructor with n arguments. Like Default, but with arguments.
//!

/// Internal namespace.
pub( crate ) mod private
{

//   ///
//   /// Constructor without arguments. Alias of Default.
//   ///
//
//   #[ allow( non_camel_case_types ) ]
//   pub trait From_0
//   where
//     Self : Sized,
//   {
//     // /// Constructor without arguments.
//     // fn from() -> Self
//     // {
//     //   Self::from_0()
//     // }
//     /// Constructor without arguments.
//     fn from_0() -> Self;
//   }
//
//   impl< All > From_0 for All
//   where
//     All : Default,
//   {
//     /// Constructor without arguments.
//     fn from_0() -> Self
//     {
//       Self::default()
//     }
//   }

  ///
  /// Constructor with single argument.
  ///

  #[ allow( non_camel_case_types ) ]
  pub trait From1< Arg >
  where
    Self : Sized,
  {
    /// Constructor with a single arguments.
    fn from1( arg : Arg ) -> Self;
  }

  impl< T, All > From1< ( T, ) > for All
  where
    All : From1< T >,
  {
    fn from1( arg : ( T, ) ) -> Self
    {
      From1::< T >::from1( arg.0 )
    }
  }

  impl< All > From1< () > for All
  where
    All : Default,
  {
    fn from1( _a : () ) -> Self { Self::default() }
  }

  // impl< All > From< () > for All
  // where
  //   All : Default,
  // {
  //   fn from( _a : () ) -> Self { Self::default() }
  // }

  // impl< T, All > From1< T > for All
  // where
  //   All : core::convert::From< T >,
  // {
  //   fn from1( arg : T ) -> Self
  //   {
  //     core::convert::From::< T >::from( arg )
  //   }
  // }

  // impl< T1, T2, All > From1< ( T1, T2 ) > for All
  // where
  //   All : core::convert::From< ( T1, T2 ) >,
  // {
  //   fn from1( arg : ( T1, T2 ) ) -> Self
  //   {
  //     core::convert::From::< ( T1, T2 ) >::from( arg )
  //   }
  // }

  ///  value-to-value conversion that consumes the input value. Change left and rught, but keep semantic of `From1``.
  #[ allow( non_camel_case_types ) ]
  pub trait Into1< T > : Sized
  {
    /// Converts this type into the (usually inferred) input type.
    fn to( self ) -> T;
  }

  impl< All, F > Into1< F > for All
  where
    F : From1< All >,
  {
    #[ inline ]
    fn to( self ) -> F
    {
      F::from1( self )
    }
  }

  // impl< All, F > Into1< F > for All
  // where
  //   F : From1< F >,
  //   F : From< All >,
  // {
  //   #[ inline ]
  //   fn to( self ) -> F
  //   {
  //     F::from1( From::from( self ) )
  //   }
  // }

  // impl< T, All > From< ( T, ) > for All
  // where
  //   All : From1< T >,
  // {
  // }

  ///
  /// Constructor with two arguments.
  ///

  #[ allow( non_camel_case_types ) ]
  pub trait From2< Arg1, Arg2 >
  where
    Self : Sized,
  {
    // /// Constructor with two arguments.
    // fn from( arg1 : Arg1, arg2 : Arg2 ) -> Self
    // {
    //   Self::from2( arg1, arg2 )
    // }
    /// Constructor with two arguments.
    fn from2( arg1 : Arg1, arg2 : Arg2 ) -> Self;
  }

  impl< T1, T2, All > From1< ( T1, T2 ) > for All
  where
    All : From2< T1, T2 >,
  {
    fn from1( arg : ( T1, T2 ) ) -> Self
    {
      From2::< T1, T2 >::from2( arg.0, arg.1 )
    }
  }

  ///
  /// Constructor with three arguments.
  ///

  #[ allow( non_camel_case_types ) ]
  pub trait From3< Arg1, Arg2, Arg3 >
  where
    Self : Sized,
  {
    // /// Constructor with three arguments.
    // fn from( arg1 : Arg1, arg2 : Arg2, arg3 : Arg3 ) -> Self
    // {
    //   Self::from3( arg1, arg2, arg3 )
    // }
    /// Constructor with three arguments.
    fn from3( arg1 : Arg1, arg2 : Arg2, arg3 : Arg3 ) -> Self;
  }

  impl< T1, T2, T3, All > From1< ( T1, T2, T3 ) > for All
  where
    All : From3< T1, T2, T3 >,
  {
    fn from1( arg : ( T1, T2, T3 ) ) -> Self
    {
      From3::< T1, T2, T3 >::from3( arg.0, arg.1, arg.2 )
    }
  }

//   ///
//   /// Constructor with four arguments.
//   ///
//
//   #[ allow( non_camel_case_types ) ]
//   pub trait From4< Arg1, Arg2, Arg3, Arg4 >
//   where
//     Self : Sized,
//   {
//     /// Constructor with four arguments.
//     fn from( arg1 : Arg1, arg2 : Arg2, arg3 : Arg3, arg4 : Arg4 ) -> Self
//     {
//       Self::from4( arg1, arg2, arg3, arg4 )
//     }
//     /// Constructor with four arguments.
//     fn from4( arg1 : Arg1, arg2 : Arg2, arg3 : Arg3, arg4 : Arg4 ) -> Self;
//   }

  // impl< T, E > From< ( E, ) > for T
  // where
  //   T : From1< ( E, ) >,
  // {
  //   /// Returns the argument unchanged.
  //   #[ inline( always ) ]
  //   fn from( src : T ) -> Self
  //   {
  //     Self::from1( src )
  //   }
  // }

  // not possible
  //
  // impl< T, F > From< T > for F
  // where
  //   F : From1< T >,
  // {
  //   /// Returns the argument unchanged.
  //   #[ inline( always ) ]
  //   fn from( src : T ) -> Self
  //   {
  //     Self::from1( src )
  //   }
  // }

  ///
  /// Variadic constructor.
  ///
  /// Implement traits [`From1`] from tuple with fields and [std::convert::From] from tuple with fields to provide the interface to construct your structure with a different set of arguments.
  /// In this example structure, Struct1 could be constructed either without arguments, with a single argument, or with two arguments.
  /// - Constructor without arguments fills fields with zero.
  /// - Constructor with a single argument sets both fields to the value of the argument.
  /// - Constructor with 2 arguments set individual values of each field.
  ///
  /// ```rust
  /// # #[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
  /// # {
  ///   use variadic_from::prelude::*;
  ///
  ///   #[ derive( Debug, PartialEq ) ]
  ///   struct Struct1
  ///   {
  ///     a : i32,
  ///     b : i32,
  ///   }
  ///
  ///   impl Default for Struct1
  ///   {
  ///     fn default() -> Self
  ///     {
  ///       Self { a : 0, b : 0 }
  ///     }
  ///   }
  ///
  ///   impl From1< i32 > for Struct1
  ///   {
  ///     fn from1( val : i32 ) -> Self
  ///     {
  ///       Self { a : val, b : val }
  ///     }
  ///   }
  ///
  ///   impl From2< i32, i32 > for Struct1
  ///   {
  ///     fn from2( val1 : i32, val2 : i32 ) -> Self
  ///     {
  ///       Self { a : val1, b : val2 }
  ///     }
  ///   }
  ///
  ///   let got : Struct1 = from!();
  ///   let exp = Struct1{ a : 0, b : 0 };
  ///   assert_eq!( got, exp );
  ///
  ///   let got : Struct1 = from!( 13 );
  ///   let exp = Struct1{ a : 13, b : 13 };
  ///   assert_eq!( got, exp );
  ///
  ///   let got : Struct1 = from!( 1, 3 );
  ///   let exp = Struct1{ a : 1, b : 3 };
  ///   assert_eq!( got, exp );
  /// # }
  ///
  /// ```
  ///
  /// ### To add to your project
  ///
  /// ``` shell
  /// cargo add type_constructor
  /// ```
  ///
  /// ## Try out from the repository
  ///
  /// ``` shell test
  /// git clone https://github.com/Wandalen/wTools
  /// cd wTools
  /// cd examples/type_constructor_trivial
  /// cargo run
  /// ```

  #[ macro_export ]
  macro_rules! from
  {

    (
      $(,)?
    )
    =>
    {
      ::core::default::Default::default();
    };

    (
      $Arg1 : expr $(,)?
    )
    =>
    {
      $crate::From1::from1( $Arg1 );
    };

    (
      $Arg1 : expr, $Arg2 : expr $(,)?
    )
    =>
    {
      $crate::From2::from2( $Arg1, $Arg2 );
    };

    (
      $Arg1 : expr, $Arg2 : expr, $Arg3 : expr $(,)?
    )
    =>
    {
      $crate::From3::from3( $Arg1, $Arg2, $Arg3 );
    };

    // (
    //   $Arg1 : expr, $Arg2 : expr, $Arg3 : expr, $Arg4 : expr $(,)?
    // )
    // =>
    // {
    //   $crate::From4::from4( $Arg1, $Arg2, $Arg3, $Arg4 );
    // };

    (
      $( $Rest : tt )+
    )
    =>
    {
      compile_error!
      (
        concat!
        (
          "Variadic constructor supports up to 3 arguments.\n",
          "Open an issue if you need more.\n",
          "You passed:\n",
          stringify!
          (
            from!( $( $Rest )+ )
          )
        )
      );
    };

  }

  pub use from;
}

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;

  #[ doc( inline ) ]
  pub use private::
  {
  };

}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*;
}


/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  #[ doc( inline ) ]
  pub use private::
  {

    // From_0,
    From1,
    Into1,
    From2,
    From3,

    from,

  };

  // pub use type_constructor_from_meta::VariadicFrom;
}
