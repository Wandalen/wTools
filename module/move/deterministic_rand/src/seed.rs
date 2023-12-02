
//!
//! Master seed.
//!

/// Internal namespace.
pub( crate ) mod private
{

  // use std::cmp::Ordering;
  // #[ cfg( not( feature = "determinism" ) ) ]
  // use std::{ ops::Deref, ops::DerefMut };
  // #[ cfg( feature = "determinism" ) ]
  // use std::sync::{ Arc, Mutex, RwLock };
  // #[ cfg( feature = "determinism" ) ]
  // use std::vec::IntoIter;
  //
  // #[ cfg( feature = "determinism" ) ]
  // use iter_tools::exposed::Itertools;
  //
  // #[ cfg( feature = "determinism" ) ]
  // use rand_chacha::ChaCha8Rng;
  //
  // pub use rand::{ SeedableRng, Rng, RngCore, seq::SliceRandom };

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

}

crate::mod_interface!
{
  orphan use Seed;
}
