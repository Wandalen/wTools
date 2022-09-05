/// Internal namespace.
pub( crate ) mod private
{
  ///
  /// Get bool like value.
  ///

  #[ derive( Debug, PartialEq ) ]
  pub enum BoolLike
  {
    /// Variant for true-like values.
    True,
    /// Variant for false-like values.
    False,
  }

  impl Default for BoolLike
  {
    fn default() -> Self { BoolLike::False }
  }

  impl From< BoolLike > for bool
  {
    fn from( bool_like : BoolLike ) -> Self
    {
      match bool_like
      {
        BoolLike::True => true,
        BoolLike::False => false,
      }
    }
  }

  ///
  /// Method to get bool like value from current type.
  ///

  pub trait ToBoolLike
  {
    /// Get bool like value.
    fn to_bool_like( &self ) -> BoolLike;
  }

  //

  impl ToBoolLike for &str
  {
    fn to_bool_like( &self ) -> BoolLike
    {
      let bool_like = match self.parse::< bool >()
      {
        Ok( x ) => if x { BoolLike::True } else { BoolLike::False },
        Err( _ ) =>
        {
          match self.parse::< i32 >()
          {
            Ok( y ) => if y == 1 { BoolLike::True } else { BoolLike::False },
            Err(_err) => BoolLike::False,
          }
        },
      };

      bool_like
    }
  }

  //

  impl ToBoolLike for String
  {
    fn to_bool_like( &self ) -> BoolLike
    {
      let bool_like = match self.parse::< bool >()
      {
        Ok( x ) => if x { BoolLike::True } else { BoolLike::False },
        Err( _ ) =>
        {
          match self.parse::< i32 >()
          {
            Ok( y ) => if y == 1 { BoolLike::True } else { BoolLike::False },
            Err( _err ) => BoolLike::False,
          }
        },
      };

      bool_like
    }
  }
}

//

crate::mod_interface!
{
  prelude use BoolLike;
  prelude use ToBoolLike;
}
