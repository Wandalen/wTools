
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
  #[ cfg( not( feature = "no_std" ) ) ]
  use std::sync::{ Arc, Mutex, RwLock };
  use rand_chacha::ChaCha8Rng;

  ///
  /// Generator under mutex and reference counter.
  ///

  pub type SharedGenerator = Arc< Mutex< ChaCha8Rng > >;
  // qqq : parametrize, use ChaCha8Rng by default, but allow to specify other

  /// Hierarchical random number generator.
  ///
  /// Produce deterministic random series of numbers with uniform distribution.
  /// Handy to be used for paralelism.
  ///
  /// Master random number generator produce children and each child might produce more children as much as dataflows in progam.
  ///

  #[ derive( Debug, Clone ) ]
  pub struct Hrng
  {
    /// List of child generators produced by this hierarchical random number generator.
    children : Arc< RwLock< Vec< Hrng > > >,
    /// Current main generator used for number generation.
    generator : SharedGenerator,
    /// Current generator used for child creation.
    ///
    /// Different generators are used for generating data and generating children for performance
    /// and to make sure that child with the same index of a parent produce always same sequence of random numbers.
    children_generator : SharedGenerator,
    // /// Current index of the generator in the list of children of parent.
    // index : usize,
  }

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

    pub fn master() -> Self
    {
      Self::master_with_seed( Seed::default() )
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

    pub fn master_with_seed( seed : Seed ) -> Self
    {
      let mut _generator : ChaCha8Rng = rand_seeder::Seeder::from( seed.into_inner() ).make_rng();
      let _children_generator = ChaCha8Rng::seed_from_u64( _generator.next_u64() );
      let generator = Arc::new( Mutex::new( _generator ) );
      let children_generator = Arc::new( Mutex::new( _children_generator ) );
      Self
      {
        children : Default::default(),
        generator,
        children_generator,
        // index: 0,
      }
    }

    /// Construct hierarchical random number generator with help of short seed.
    fn _with_short_seed( seed : u64 ) -> Self
    {
      let rng = ChaCha8Rng::seed_from_u64( seed );
      Self::_with_generator( rng )
    }

    /// Construct hierarchical random number generator with help of RNG.
    fn _with_generator( mut rng : ChaCha8Rng ) -> Self
    {
      // Use another sequence for seed generation to improve uniformness.
      rng.set_stream( 1 );
      let _children_generator = ChaCha8Rng::seed_from_u64( rng.next_u64() );
      rng.set_stream( 0 );
      let generator = Arc::new( Mutex::new( rng ) );
      let children_generator = Arc::new( Mutex::new( _children_generator ) );
      Self
      {
        children : Default::default(),
        generator,
        children_generator,
        // index: 0,
      }
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
      self.generator.clone()
    }

    /// Creates new child hierarchical random number generator by index seed.
    pub fn child( &self, index : usize ) -> Self
    {
      let children = self.children.read().unwrap();
      if children.len() > index
      {
        return children[ index ].clone();
      }

      // To acquire a write lock, read lock should be released first
      drop( children );
      let mut rng = self.children_generator.lock().unwrap();
      let mut children = self.children.write().unwrap();
      let len = children.len();

      // After the second lock it can happen that the child already exists.
      if len > index
      {
        return children[ index ].clone();
      }

      children.reserve( index + 1 - len );
      for _ in len..( index + 1 )
      {
        children.push( Self::_with_short_seed( rng.next_u64() ) )
      }
      children[ index ].clone()

    }

//     // xxx : remove, maybe
//     /// Creates new child hierarchical random number generator by index seed, index is deduced from the contexst.
//     /// Index is new child is index of current newest child plus one.
//     pub fn child_new( &self ) -> Self
//     {
//       self.child( self.children.read().unwrap().len() )
//     }

    /// Returns number of children created by this generator. Used only for diagnostics.
    pub fn _children_len( &self ) -> usize
    {
      self.children.read().unwrap().len()
    }

//     // xxx : remove, maybe
//     /// Returns current index of the generator.
//     pub fn index( &self ) -> usize
//     {
//       self.index
//     }
  }

  impl Default for Hrng
  {
    fn default() -> Self
    {
      Hrng::master()
    }
  }

}

crate::mod_interface!
{
  orphan use Hrng;
}
