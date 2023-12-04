
//!
//! Master seed.
//!

/// Internal namespace.
pub( crate ) mod private
{

  /// Master seed.
  #[ derive( Clone, Debug, PartialEq, Eq ) ]
  pub struct Seed( String );

  impl Seed
  {
    /// Creates new seed from a string.
    pub fn new< IntoString >( value : IntoString ) -> Self
    where
      IntoString : Into< String >,
    {
      Self( value.into() )
    }

    /// Used for simplifying seed creation from a [`u64`] seed.
    pub fn from_integer( src : u64 ) -> Self
    {
      Self( format!( "master_seed_{}", src ) )
    }

    /// Returns inner seed string value.
    pub fn into_inner( self ) -> String
    {
      self.0
    }
  }

  impl Default for Seed
  {
    fn default() -> Self
    {
      Self( "master_seed".to_owned() )
    }
  }

  impl< IntoString > From< IntoString > for Seed
  where
    IntoString : Into< String >,
  {
    #[ inline( always ) ]
    fn from( src : IntoString ) -> Self
    {
      Self::new( src )
    }
  }


}

crate::mod_interface!
{
  orphan use Seed;
}
