#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

/* zzz : consider https://doc.rust-lang.org/std/ops/trait.RangeBounds.html */
/* zzz : implement iterator */

//!
//! Interval adapter for both open/closed implementations of intervals ( ranges ).
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Internal namespace.
mod internal
{

  ///
  /// Interval adapter. Interface to interval-like structures.
  ///

  pub trait IntervalAdapter< T = isize >
  where
    T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
    isize : Into< T >,
  {
    /// the first element of the ( closed ) interval
    fn first( &self ) -> T;
    /// the last element of the ( closed ) interval
    fn last( &self ) -> T;
    /// number of discrete elements in the interval
    fn len( &self ) -> T
    {
      let one : T = 1.into();
      self.last() - self.first() + one
    }
    /// interval in closed format as pair of numbers
    fn closed( &self ) -> ( T, T )
    {
      ( self.first(), self.last() )
    }
    /// interval in closed-open format as pair of numbers
    fn closed_open( &self ) -> ( T, T )
    {
      let one : T = 1.into();
      ( self.first(), self.last() + one )
    }
    /// interval in first-length format as pair of numbers
    fn first_len( &self ) -> ( T, T )
    {
      ( self.first(), self.len() )
    }
  }

  ///
  /// Alternative implementation of interval.
  ///
  /// Both [core::ops::Range], [core::ops::RangeInclusive] are convertable to [crate::Interval]
  ///

  #[ derive( PartialEq, Debug ) ]
  pub struct Interval< T = isize >
  where
    T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
    isize : Into< T >,
  {
    _first : T,
    _last : T,
  }

  impl< T > Interval< T >
  where
    T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
    isize : Into< T >,
  {
    /// Constructor of an interval. Expects closed interval in arguments.
    pub fn new( first : T, last : T ) -> Self
    {
      Self { _first : first, _last : last }
    }
  }

  impl< T > IntervalAdapter< T >
  for Interval< T >
  where
    T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
    isize : Into< T >,
  {
    fn first( &self ) -> T
    {
      self._first
    }
    fn last( &self ) -> T
    {
      self._last
    }
  }

  //
  // IntervalAdapter for std
  //

  impl< T > IntervalAdapter< T >
  for ::core::ops::Range< T >
  where
    T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
    isize : Into< T >,
  {
    fn first( &self ) -> T
    {
      self.start
    }
    fn last( &self ) -> T
    {
      let one : T = 1.into();
      self.end - one
    }
  }

  impl< T > IntervalAdapter< T >
  for ::core::ops::RangeInclusive< T >
  where
    T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
    isize : Into< T >,
  {
    fn first( &self ) -> T
    {
      *self.start()
    }
    fn last( &self ) -> T
    {
      *self.end()
    }
  }

  //
  // from for std
  //

  impl< T > From< ::core::ops::Range< T > >
  for Interval< T >
  where
    T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
    isize : Into< T >,
  {
    fn from( src : ::core::ops::Range< T > ) -> Self
    {
      let one : T = 1.into();
      Self { _first : src.start, _last : src.end - one }
    }
  }

  //

  impl< T > From< ::core::ops::RangeInclusive< T > >
  for Interval< T >
  where
    T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
    isize : Into< T >,
  {
    fn from( src : ::core::ops::RangeInclusive< T > ) -> Self
    {
      Self { _first : *src.start(), _last : *src.end() }
    }
  }

}

/// Own namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::internal::
  {
    IntervalAdapter,
    Interval,
  };
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
