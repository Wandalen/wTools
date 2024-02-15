//!
//! Variadic constructor. Constructor with n arguments. Like Default, but with arguments.
//!

/// Internal namespace.
pub( crate ) mod private
{

  ///
  /// Constructor without arguments. Alias of Default.
  ///

  #[ allow( non_camel_case_types ) ]
  pub trait From_0
  where
    Self : Sized,
  {
    // /// Constructor without arguments.
    // fn from() -> Self
    // {
    //   Self::from_0()
    // }
    /// Constructor without arguments.
    fn from_0() -> Self;
  }

  impl< All > From_0 for All
  where
    All : Default,
  {
    /// Constructor without arguments.
    fn from_0() -> Self
    {
      Self::default()
    }
  }

  ///
  /// Constructor with single argument.
  ///

  #[ allow( non_camel_case_types ) ]
  pub trait From_1< Arg >
  where
    Self : Sized,
  {
    // /// Constructor without arguments.
    // fn from( arg : Arg ) -> Self
    // {
    //   Self::from_1( arg )
    // }
    /// Constructor without arguments.
    fn from_1( arg : Arg ) -> Self;
  }

  impl< T, All > From_1< ( T, ) > for All
  where
    All : From_1< T >,
  {
    fn from_1( arg : ( T, ) ) -> Self
    {
      From_1::< T >::from_1( arg.0 )
    }
  }

  // impl< T, All > From_1< T > for All
  // where
  //   All : core::convert::From< T >,
  // {
  //   fn from_1( arg : T ) -> Self
  //   {
  //     core::convert::From::< T >::from( arg )
  //   }
  // }

  // impl< T1, T2, All > From_1< ( T1, T2 ) > for All
  // where
  //   All : core::convert::From< ( T1, T2 ) >,
  // {
  //   fn from_1( arg : ( T1, T2 ) ) -> Self
  //   {
  //     core::convert::From::< ( T1, T2 ) >::from( arg )
  //   }
  // }

  ///  value-to-value conversion that consumes the input value. Change left and rught, but keep semantic of `From_1``.
  #[ allow( non_camel_case_types ) ]
  pub trait Into1< T > : Sized
  {
    /// Converts this type into the (usually inferred) input type.
    fn to( self ) -> T;
  }

  impl< All, F > Into1< F > for All
  where
    F : From_1< All >,
  {
    #[inline]
    fn to( self ) -> F
    {
      F::from_1( self )
    }
  }

  // impl< All, F > Into1< F > for All
  // where
  //   F : From_1< F >,
  //   F : From< All >,
  // {
  //   #[inline]
  //   fn to( self ) -> F
  //   {
  //     F::from_1( From::from( self ) )
  //   }
  // }

  // impl< T, All > From< ( T, ) > for All
  // where
  //   All : From_1< T >,
  // {
  // }

  ///
  /// Constructor with two arguments.
  ///

  #[ allow( non_camel_case_types ) ]
  pub trait From_2< Arg1, Arg2 >
  where
    Self : Sized,
  {
    // /// Constructor with two arguments.
    // fn from( arg1 : Arg1, arg2 : Arg2 ) -> Self
    // {
    //   Self::from_2( arg1, arg2 )
    // }
    /// Constructor with two arguments.
    fn from_2( arg1 : Arg1, arg2 : Arg2 ) -> Self;
  }

  impl< T1, T2, All > From_1< ( T1, T2 ) > for All
  where
    All : From_2< T1, T2 >,
  {
    fn from_1( arg : ( T1, T2 ) ) -> Self
    {
      From_2::< T1, T2 >::from_2( arg.0, arg.1 )
    }
  }

  ///
  /// Constructor with three arguments.
  ///

  #[ allow( non_camel_case_types ) ]
  pub trait From_3< Arg1, Arg2, Arg3 >
  where
    Self : Sized,
  {
    // /// Constructor with three arguments.
    // fn from( arg1 : Arg1, arg2 : Arg2, arg3 : Arg3 ) -> Self
    // {
    //   Self::from_3( arg1, arg2, arg3 )
    // }
    /// Constructor with three arguments.
    fn from_3( arg1 : Arg1, arg2 : Arg2, arg3 : Arg3 ) -> Self;
  }

  impl< T1, T2, T3, All > From_1< ( T1, T2, T3 ) > for All
  where
    All : From_3< T1, T2, T3 >,
  {
    fn from_1( arg : ( T1, T2, T3 ) ) -> Self
    {
      From_3::< T1, T2, T3 >::from_3( arg.0, arg.1, arg.2 )
    }
  }

//   ///
//   /// Constructor with four arguments.
//   ///
//
//   #[ allow( non_camel_case_types ) ]
//   pub trait From_4< Arg1, Arg2, Arg3, Arg4 >
//   where
//     Self : Sized,
//   {
//     /// Constructor with four arguments.
//     fn from( arg1 : Arg1, arg2 : Arg2, arg3 : Arg3, arg4 : Arg4 ) -> Self
//     {
//       Self::from_4( arg1, arg2, arg3, arg4 )
//     }
//     /// Constructor with four arguments.
//     fn from_4( arg1 : Arg1, arg2 : Arg2, arg3 : Arg3, arg4 : Arg4 ) -> Self;
//   }

  ///
  /// Variadic constructor.
  ///
  /// Implement traits [From_0], [From_1] up to MakeN to provide the interface to construct your structure with a different set of arguments.
  /// In this example structure, Struct1 could be constructed either without arguments, with a single argument, or with two arguments.
  /// - Constructor without arguments fills fields with zero.
  /// - Constructor with a single argument sets both fields to the value of the argument.
  /// - Constructor with 2 arguments set individual values of each field.
  ///
  /// ```rust
  /// # #[ cfg( feature = "from" ) ]
  /// # {
  ///   use type_constructor::prelude::*;
  ///
  ///   #[ derive( Debug, PartialEq ) ]
  ///   struct Struct1
  ///   {
  ///     a : i32,
  ///     b : i32,
  ///   }
  ///
  ///   impl From_0 for Struct1
  ///   {
  ///     fn from_0() -> Self
  ///     {
  ///       Self { a : 0, b : 0 }
  ///     }
  ///   }
  ///
  ///   impl From_1< i32 > for Struct1
  ///   {
  ///     fn from_1( val : i32 ) -> Self
  ///     {
  ///       Self { a : val, b : val }
  ///     }
  ///   }
  ///
  ///   impl From_2< i32, i32 > for Struct1
  ///   {
  ///     fn from_2( val1 : i32, val2 : i32 ) -> Self
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
  /// cd examples/type_constructor_trivial_sample
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
      $crate::wtools::From_0::from_0();
    };

    (
      $Arg1 : expr $(,)?
    )
    =>
    {
      $crate::wtools::From_1::from_1( $Arg1 );
    };

    (
      $Arg1 : expr, $Arg2 : expr $(,)?
    )
    =>
    {
      $crate::wtools::From_2::from_2( $Arg1, $Arg2 );
    };

    (
      $Arg1 : expr, $Arg2 : expr, $Arg3 : expr $(,)?
    )
    =>
    {
      $crate::wtools::From_3::from_3( $Arg1, $Arg2, $Arg3 );
    };

    // (
    //   $Arg1 : expr, $Arg2 : expr, $Arg3 : expr, $Arg4 : expr $(,)?
    // )
    // =>
    // {
    //   $crate::wtools::From_4::from_4( $Arg1, $Arg2, $Arg3, $Arg4 );
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
          "Variadic constructor supports up to 1 arguments.\n",
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

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };

}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

// #[ doc( inline ) ]
#[ allow( unused_imports ) ]
// pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {

    From_0,
    From_1,
    Into1,
    From_2,
    From_3,

    from,

  };

  // pub use type_constructor_from_meta::VariadicFrom;
}
