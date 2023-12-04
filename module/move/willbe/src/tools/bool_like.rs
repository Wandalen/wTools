/// Internal namespace.
pub( crate ) mod private
{
  use core::str::FromStr;

  ///
  /// Get bool like value.
  ///

  // xxx : qqq : for Bohdan : move to wca and use there
  #[ derive( Debug, PartialEq, Eq ) ]
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

  /// Not a string like.
  #[ derive( Debug, PartialEq, Eq ) ]
  pub struct StringLikeError;

  impl FromStr for BoolLike
  {
    type Err = StringLikeError;

    fn from_str( s : &str ) -> Result<Self, Self::Err>
    {
      match s.parse::< bool >()
      {
        Ok( b ) => if b { Ok( BoolLike::True ) } else { Ok( BoolLike::False ) },
        Err( _ ) =>
        {
          match s.parse::< usize >()
          {
            Ok( i ) => if i == 0 { Ok( BoolLike::False ) } else { Ok( BoolLike::True ) },
            Err( _err ) => Err( StringLikeError ),
          }
        },
      }
    }
  }

}

//

crate::mod_interface!
{
  orphan use BoolLike;
}
