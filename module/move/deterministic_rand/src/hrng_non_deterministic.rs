
//!
//! Hierarchical random number generators itself.
//!
//! There are two versions of HRNG: deterministic and non-deterministic.
//! Both have the same interface and are interchengable by switching on/off a feature `determinsim`.
//!

/// Internal namespace.
mod private
{

  use crate::*;
  use core::{ ops::Deref, ops::DerefMut };

  /// Emulates behavior of `Arc<Mutex< ThreadRng >>` for compatibility.

  #[ derive( Debug ) ]
  pub struct SharedGenerator;


  impl SharedGenerator
  {
    /// Emulate lock of a mutex.
    #[ inline( always ) ]
    pub fn lock( &self ) -> SharedGeneratorLock
    {
      SharedGeneratorLock
    }
  }

  /// Emulates behavior of `Arc<Mutex< ThreadRng >>` for compatibility.

  #[ derive( Debug) ]
  pub struct SharedGeneratorLock;

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

  #[ derive( Debug ) ]
  pub struct DerefRng( rand::rngs::ThreadRng );

  impl Deref for DerefRng
  {
    type Target = rand::rngs::ThreadRng;
    #[ inline( always ) ]
    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

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

  #[ derive( Debug, Clone ) ]
  pub struct Hrng;

  impl Hrng
  {

    /// Construct master hierarchical random number generator with default seed phrase.
    ///
    /// ### Example
    /// ```
    /// use deterministic_rand::{ Hrng, Rng };
    /// let hrng = Hrng::master();
    /// let rng_ref = hrng.rng_ref();
    /// let mut rng = rng_ref.lock().unwrap();
    /// let got : u64 = rng.gen();
    /// ```

    #[ inline( always ) ]
    pub fn master() -> Self
    {
      Self
    }

    /// Construct hierarchical random number generator with help of seed phrase.
    ///
    /// ### Example
    /// ```
    /// use deterministic_rand::{ Hrng, Rng };
    /// let hrng = Hrng::master_with_seed( "master1".into() );
    /// let rng_ref = hrng.rng_ref();
    /// let mut rng = rng_ref.lock().unwrap();
    /// let got : u64 = rng.gen();
    /// ```

    #[ cfg( not( feature = "no_std" ) ) ]
    #[ inline( always ) ]
    pub fn master_with_seed( _ : Seed ) -> Self
    {
      Self
    }

    /// Get a reference to the current random number generator using a reference counter and mutex.
    ///
    /// Returns a shared `Arc<Mutex< Generator >>`.
    ///
    /// ### Example
    ///
    /// ```
    /// # use deterministic_rand::{ Hrng, Rng };
    /// # let hrng = Hrng::default();
    /// let rng_ref = hrng.rng_ref();
    /// let mut rng = rng_ref.lock().unwrap();
    /// let got : u64 = rng.gen();
    /// ```

    #[ inline( always ) ]
    pub fn rng_ref( &self ) -> SharedGenerator
    {
      SharedGenerator
    }

    /// Creates new child hierarchical random number generator by index seed.
    #[ inline( always ) ]
    pub fn child( &self, _ : usize ) -> Self
    {
      Self
    }

//     /// Creates new child hierarchical random number generator by index seed, index is deduced from the contexst.
//     /// Index is new child is index of current newest child plus one.
//     pub fn child_new( &self ) -> Self
//     {
//       self.child( 0 )
//     }

    /// Returns number of children created by this generator.
    #[ inline( always ) ]
    pub fn _children_len( &self ) -> usize
    {
      0
    }

//     /// Returns current index of the generator.
//     #[ inline( always ) ]
//     pub fn index( &self ) -> usize
//     {
//       0
//     }
  }

}

crate::mod_interface!
{
  orphan use Hrng;
}
