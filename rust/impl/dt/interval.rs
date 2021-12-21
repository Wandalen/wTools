#![ warn( missing_docs ) ]

/* xxx : qqq : for rust : move */
/* xxx : qqq : for rust : implement trait IntervalAdapter for standard ranges */
/* xxx : qqq : for rust : implement iterator */
/* qqq : for rust : cover */

//!
//! Interval adapter for both open/closed implementations of intervals ( ranges ).
//!
//! # sample
//!
//! ``` rust sample test
//! use winterval::*;
//!
//! let src = 2..5;
//! assert_eq!( src.closed(), ( 2, 4 ) );
//!
//! let src = 2..=4;
//! assert_eq!( src.closed(), ( 2, 4 ) );
//! ```
//!

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
