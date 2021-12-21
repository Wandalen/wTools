#![ allow( missing_docs ) ]

/* xxx : qqq : for rust : move */
/* xxx : qqq : for rust : implement trait IntervalAdapter for standard ranges */
/* qqq : for rust : cover */

///
/// Interface of interval.
///
/// Interval adapter is implemented for [core::ops::Range], [core::ops::RangeInclusive] as well as for [crate::Interval].
///

pub trait IntervalAdapter< T = isize >
where
  T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
  isize : Into< T >,
{
  fn first( &self ) -> T;
  fn last( &self ) -> T;
  fn len( &self ) -> T
  {
    let one : T = 1.into();
    self.last() - self.first() + one
  }
  fn closed( &self ) -> ( T, T )
  {
    ( self.first(), self.last() )
  }
  fn closed_open( &self ) -> ( T, T )
  {
    let one : T = 1.into();
    ( self.first(), self.last() + one )
  }
  fn first_len( &self ) -> ( T, T )
  {
    ( self.first(), self.len() )
  }
}

///
/// Alternative implementation of interval. Both [core::ops::Range], [core::ops::RangeInclusive] are convertable to [crate::Interval]
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
