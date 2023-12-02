
//!
//! Hierarchical random number generators itself.
//!
//! There are two versions of HRNG: deterministic and non-deterministic.
//! Both have the same interface and are interchengable by switching on/off a feature `determinsim`.
//!

/// Internal namespace.
pub( crate ) mod private
{

  use crate::*;
  // use std::cmp::Ordering;
  #[ cfg( not( feature = "determinism" ) ) ]
  use std::{ ops::Deref, ops::DerefMut };
  // #[ cfg( feature = "determinism" ) ]
  // use std::sync::{ Arc, Mutex, RwLock };
  // #[ cfg( feature = "determinism" ) ]
  // use std::vec::IntoIter;

  // #[ cfg( feature = "determinism" ) ]
  // use iter_tools::exposed::Itertools;

  // #[ cfg( feature = "determinism" ) ]
  // use rand_chacha::ChaCha8Rng;

  // pub use rand::{ SeedableRng, Rng, RngCore, seq::SliceRandom };

  // /// Generator under mutex and reference counter.
  // #[ cfg( feature = "determinism" ) ]
  // pub type SharedGenerator = Arc< Mutex< ChaCha8Rng > >;

  /// Emulates behavior of `Arc<Mutex<ThreadRng>>` for compatibility.
  #[ cfg( not( feature = "determinism" ) ) ]
  #[ derive( Debug ) ]
  pub struct SharedGenerator;

  #[ cfg( not( feature = "determinism" ) ) ]
  impl SharedGenerator
  {
    /// Emulate lock of a mutex.
    #[ inline( always ) ]
    pub fn lock( &self ) -> SharedGeneratorLock
    {
      SharedGeneratorLock
    }
  }

  /// Emulates behavior of `Arc<Mutex<ThreadRng>>` for compatibility.
  #[ cfg( not( feature = "determinism" ) ) ]
  #[ derive( Debug) ]
  pub struct SharedGeneratorLock;

  #[ cfg( not( feature = "determinism" ) ) ]
  impl SharedGeneratorLock
  {
    /// Emulate unwrap of a result of guard produced my locking a mutex.
    #[ inline( always ) ]
    pub fn unwrap( &self ) -> DerefRng
    {
      DerefRng( rand::thread_rng() )
    }
  }

  /// Placeholder structure that is used when `determinism` feature is not enabled.
  ///
  /// Used for code compatibility for both deterministic and non-deterministic modes.
  #[ cfg( not( feature = "determinism" ) ) ]
  #[ derive( Debug ) ]
  pub struct DerefRng( rand::rngs::ThreadRng );

  #[ cfg( not( feature = "determinism" ) ) ]
  impl Deref for DerefRng
  {
    type Target = rand::rngs::ThreadRng;
    #[ inline( always ) ]
    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  #[ cfg( not( feature = "determinism" ) ) ]
  impl DerefMut for DerefRng
  {
    fn deref_mut( &mut self ) -> &mut Self::Target
    {
      &mut self.0
    }
  }

  impl Default for Hrng
  {
    fn default() -> Self
    {
      Hrng::master()
    }
  }

  /// Placeholder of a deterministic hierarchical random number generator
  /// for then the `determinism` feature is not enabled
  ///
  /// Always returns `rand::thread_rng`
  #[ cfg( not( feature = "determinism" ) ) ]
  #[ derive( Debug, Clone ) ]
  pub struct Hrng;

  #[ cfg( not( feature = "determinism" ) ) ]
  impl Hrng
  {
    /// Construct master hierarchical random number generator with default seed phrase.
    #[ inline( always ) ]
    pub fn master() -> Self
    {
      Self
    }

    /// Construct hierarchical random number generator with help of seed phrase.
    #[ inline( always ) ]
    pub fn master_with_seed( _ : Seed ) -> Self
    {
      Self
    }

    /// Get arc on current generator.
    #[ inline( always ) ]
    pub fn rng( &self ) -> SharedGenerator
    {
      SharedGenerator
    }

    /// Creates new child hierarchical random number generator by index seed.
    #[ inline( always ) ]
    pub fn child( &self, _ : usize ) -> Self
    {
      Self
    }

    /// Returns number of children created by this generator.
    #[ inline( always ) ]
    pub fn children_len( &self ) -> usize
    {
      0
    }

    /// Returns current index of the generator.
    #[ inline( always ) ]
    pub fn index( &self ) -> usize
    {
      0
    }
  }

}

crate::mod_interface!
{
  orphan use Hrng;
}
