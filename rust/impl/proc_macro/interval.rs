#![ warn( missing_docs ) ]

// use crate::num::*;

/* xxx : qqq : for rust : move */
/* qqq : for rust : cover */

pub trait Interval< T = isize >
where
  T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
  // < T as SignedOf >::Unsigned : std::ops::Sub< Output = < T as SignedOf >::Unsigned >,
  isize : Into< T >,
{
  fn first( &self ) -> T;
  fn last( &self ) -> T;
  fn len( &self ) -> T
  {
    let one : T = 1.into();
    self.last() - self.first() + one
  }
}

pub struct IntervalInclusive< T = isize >
where
  T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
  isize : Into< T >,
{
  _first : T,
  _last : T,
}

impl< T > IntervalInclusive< T >
where
  T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
  isize : Into< T >,
{
  pub fn new( first : T, last : T ) -> Self
  {
    Self { _first : first, _last : last }
  }
}

impl< T > Interval< T >
for IntervalInclusive< T >
where
  T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
  // < T as SignedOf >::Unsigned : std::ops::Sub< Output = < T as SignedOf >::Unsigned >,
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
for IntervalInclusive< T >
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
for IntervalInclusive< T >
where
  T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
  isize : Into< T >,
{
  fn from( src : ::core::ops::RangeInclusive< T > ) -> Self
  {
    Self { _first : *src.start(), _last : *src.end() }
  }
}
