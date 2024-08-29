
//!
//! Master seed.
//!

/// Internal namespace.
mod private
{
  #[ cfg( feature = "no_std" ) ]
  extern crate alloc;
  #[ cfg( feature = "no_std" ) ]
  use alloc::string;

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

    /// Random string as seed.
    pub fn random() -> Self
    {
      use rand::{ distributions::Alphanumeric, Rng };
      let str : String = rand::thread_rng()
      .sample_iter( &Alphanumeric )
      .take( 16 )
      .map(char::from)
      .collect();
      debug_assert!( str.len() > 0 );
      Self( str )
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
