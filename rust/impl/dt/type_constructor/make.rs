/// Internal namespace.
mod internal
{

  ///
  /// Constructor without arguments.
  ///

  pub trait Make0
  where
    Self : Sized,
  {
    /// Constructor without arguments.
    fn make() -> Self
    {
      Self::make_0()
    }
    /// Constructor without arguments.
    fn make_0() -> Self;
  }

  ///
  /// Constructor with single argument.
  ///

  pub trait Make1< Arg >
  where
    Self : Sized,
  {
    /// Constructor without arguments.
    fn make( arg : Arg ) -> Self
    {
      Self::make_1( arg )
    }
    /// Constructor without arguments.
    fn make_1( arg : Arg ) -> Self;
  }

  ///
  /// Constructor with two arguments.
  ///

  pub trait Make2< Arg1, Arg2 >
  where
    Self : Sized,
  {
    /// Constructor with two arguments.
    fn make( arg1 : Arg1, arg2 : Arg2 ) -> Self
    {
      Self::make_2( arg1, arg2 )
    }
    /// Constructor with two arguments.
    fn make_2( arg1 : Arg1, arg2 : Arg2 ) -> Self;
  }

  ///
  /// Constructor with three arguments.
  ///

  pub trait Make3< Arg1, Arg2, Arg3 >
  where
    Self : Sized,
  {
    /// Constructor with three arguments.
    fn make( arg1 : Arg1, arg2 : Arg2, arg3 : Arg3 ) -> Self
    {
      Self::make_3( arg1, arg2, arg3 )
    }
    /// Constructor with three arguments.
    fn make_3( arg1 : Arg1, arg2 : Arg2, arg3 : Arg3 ) -> Self;
  }

  ///
  /// Variadic constructor.
  ///
  /// Implement traits [Make0], [Make1] up to MakeN to provide the interface to construct your structure with a different set of arguments.
  /// In this example structure, Struct1 could be constructed either without arguments, with a single argument, or with two arguments.
  /// - Constructor without arguments fills fields with zero.
  /// - Constructor with a single argument sets both fields to the value of the argument.
  /// - Constructor with 2 arguments set individual values of each field.
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  ///
  /// #[ derive( Debug, PartialEq ) ]
  /// struct Struct1
  /// {
  ///   a : i32,
  ///   b : i32,
  /// }
  ///
  /// impl Make0 for Struct1
  /// {
  ///   fn make_0() -> Self
  ///   {
  ///     Self { a : 0, b : 0 }
  ///   }
  /// }
  ///
  /// impl Make1< i32 > for Struct1
  /// {
  ///   fn make_1( val : i32 ) -> Self
  ///   {
  ///     Self { a : val, b : val }
  ///   }
  /// }
  ///
  /// impl Make2< i32, i32 > for Struct1
  /// {
  ///   fn make_2( val1 : i32, val2 : i32 ) -> Self
  ///   {
  ///     Self { a : val1, b : val2 }
  ///   }
  /// }
  ///
  /// let got : Struct1 = make!();
  /// let exp = Struct1{ a : 0, b : 0 };
  /// assert_eq!( got, exp );
  ///
  /// let got : Struct1 = make!( 13 );
  /// let exp = Struct1{ a : 13, b : 13 };
  /// assert_eq!( got, exp );
  ///
  /// let got : Struct1 = make!( 1, 3 );
  /// let exp = Struct1{ a : 1, b : 3 };
  /// assert_eq!( got, exp );
  /// ```
  ///
  /// ## To add to your project
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
  /// cd sample/rust/type_constructor_trivial_sample
  /// cargo run
  /// ```

  #[ macro_export ]
  macro_rules! make
  {

    (
      $(,)?
    )
    =>
    {
      $crate::Make0::make_0();
    };

    (
      $Arg1 : expr $(,)?
    )
    =>
    {
      $crate::Make1::make_1( $Arg1 );
    };

    (
      $Arg1 : expr, $Arg2 : expr $(,)?
    )
    =>
    {
      $crate::Make2::make_2( $Arg1, $Arg2 );
    };

    (
      $Arg1 : expr, $Arg2 : expr, $Arg3 : expr $(,)?
    )
    =>
    {
      $crate::Make3::make_3( $Arg1, $Arg2, $Arg3 );
    };

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
            make!( $( $Rest )+ )
          )
        )
      );
    };

  }

  pub use make;
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

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::internal::
  {

    Make0,
    Make1,
    Make2,
    Make3,

    make,

  };
}
