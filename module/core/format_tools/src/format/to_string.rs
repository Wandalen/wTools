//!
//! Flexible ToString augmentation.
//!

/// Internal namespace.
pub( crate ) mod private
{

  pub use super::
  {
    aref::{ Ref, _Ref },
  };

  use std::
  {
    fmt,
    borrow::Cow,
  };

  // ==

  /// Marker type for using Debug formatting.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct WithDebug;

  /// Marker type for using Display formatting.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct WithDisplay;

  /// Marker type for usign Well formatting.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct WithWell;

  // ==

  /// Trait to convert a type to a string using a specified formatting method.
  pub trait ToStringWith< 'a, How >
  {
    /// Converts the type to a string using the specified formatting method.
    fn to_string_with( &'a self ) -> Cow< 'a, str >;
  }

  impl< 'a, T > ToStringWith< 'a, WithDebug > for T
  where
    T : fmt::Debug,
  {
    /// Converts the type to a string using Debug formatting.
    #[ inline ]
    fn to_string_with( &'a self ) -> Cow< 'a, str >
    {
      Cow::Owned( format!( "{:?}", self ) )
    }
  }

  // impl< 'a, T > ToStringWith< 'a, WithDebug > for &T
  // where
  //   T : fmt::Debug,
  // {
  //   /// Converts the type to a string using Debug formatting.
  //   #[ inline ]
  //   fn to_string_with( &'a self ) -> Cow< 'a, str >
  //   {
  //     ToStringWith::< 'a, WithDebug >::to_string_with( *self )
  //   }
  // }

  impl< 'a, T > ToStringWith< 'a, WithDisplay > for T
  where
    T : fmt::Display,
  {
    /// Converts the type to a string using Display formatting.
    #[ inline ]
    fn to_string_with( &'a self ) -> Cow< 'a, str >
    {
      ( &Ref::< 'a, T, WithDisplay >::from( self ) )._display_string()
      // Cow::Owned( format!( "{}", self ) )
    }
  }

  trait _DisplayString< 'a >
  {
    fn _display_string( self ) -> Cow< 'a, str >;
  }

  impl< 'a, T > _DisplayString< 'a > for _Ref< 'a, T, WithDisplay >
  where
    T : fmt::Display,
  {
    #[ inline ]
    fn _display_string( self ) -> Cow< 'a, str >
    {
      // panic!( "a" );
      Cow::Owned( format!( "{}", self.0 ) )
    }
  }

  // xxx : not only String

  impl< 'a > _DisplayString< 'a > for Ref< 'a, String, WithDisplay >
  where
    String : fmt::Display,
  {
    #[ inline ]
    fn _display_string( self ) -> Cow< 'a, str >
    {
      panic!( "xxx" );
      Cow::Borrowed( self.0.0 )
    }
  }

  impl< 'a > _DisplayString< 'a > for Ref< 'a, &String, WithDisplay >
  where
    String : fmt::Display,
  {
    #[ inline ]
    fn _display_string( self ) -> Cow< 'a, str >
    {
      panic!( "yyy" );
      Cow::Borrowed( self.0.0 )
    }
  }

  impl< 'a > _DisplayString< 'a > for Ref< 'a, &&String, WithDisplay >
  where
    String : fmt::Display,
  {
    #[ inline ]
    fn _display_string( self ) -> Cow< 'a, str >
    {
      panic!( "yyy" );
      Cow::Borrowed( self.0.0 )
    }
  }

  impl< 'a > _DisplayString< 'a > for Ref< 'a, str, WithDisplay >
  where
    str : fmt::Display,
  {
    #[ inline ]
    fn _display_string( self ) -> Cow< 'a, str >
    {
      panic!( "zzz1" );
      Cow::Borrowed( self.0.0 )
    }
  }

  impl< 'a > _DisplayString< 'a > for Ref< 'a, &str, WithDisplay >
  where
    str : fmt::Display,
  {
    #[ inline ]
    fn _display_string( self ) -> Cow< 'a, str >
    {
      panic!( "zzz2" );
      Cow::Borrowed( self.0.0 )
    }
  }

  // xxx
  fn _f< 'a, T : 'a >()
  where
    Ref::< 'a, T, WithDisplay > : _DisplayString< 'a >,
  {
  }

  // xxx : clean

//   #[ test ]
//   fn borrowed_string_test()
//   {
//
//     let src = "string".to_string();
//     let got = ToStringWith::< WithDisplay >::to_string_with( &src );
//     let exp : Cow< '_, str > = Cow::Borrowed( "string" );
//     assert_eq!( got, exp );
//     assert!( matches!( got, Cow::Borrowed( _ ) ) );
//
//   }

}

mod aref;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
  #[ doc( inline ) ]
  pub use private::
  {
    Ref,
  };
}

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
    WithDebug,
    WithDisplay,
    WithWell,
    ToStringWith,
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
}
