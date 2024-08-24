//!
//! Implement fields for vector.
//!

use crate::*;
use std::borrow::Cow;
use collection_tools::Vec;

impl< V, Borrowed > Fields< usize, &'_ Borrowed > for Vec< V >
where
  Borrowed : std::borrow::ToOwned + 'static + ?Sized,
  V : std::borrow::Borrow< Borrowed >,
{

  type Key< 'k > = usize
  where Self : 'k, usize : 'k;

  type Val< 'v > = &'v Borrowed
  where Self : 'v, V : 'v;

  fn fields< 's >( &'s self ) -> impl IteratorTrait< Item = ( Self::Key< 's >, Self::Val< 's > ) >
  {
    self.iter().enumerate().map( move | ( key, val ) | ( key, val.borrow() ) )
  }

}

impl< V, Borrowed > Fields< usize, Option< Cow< '_, Borrowed > > > for Vec< V >
where
  Borrowed : std::borrow::ToOwned + 'static + ?Sized,
  V : std::borrow::Borrow< Borrowed >,
{

  type Key< 'k > = usize
  where Self : 'k, usize : 'k;

  type Val< 'v > = Option< Cow< 'v, Borrowed > >
  where Self : 'v;

  fn fields< 's >( &'s self ) -> impl IteratorTrait< Item = ( Self::Key< 's >, Self::Val< 's > ) >
  {
    self.iter().enumerate().map( move | ( key, val ) | ( key, Some( Cow::Borrowed( val.borrow() ) ) ) )
  }

}

impl< V, Borrowed, Marker > Fields< usize, OptionalCow< '_, Borrowed, Marker > > for Vec< V >
where
  Borrowed : std::borrow::ToOwned + 'static + ?Sized,
  V : std::borrow::Borrow< Borrowed >,
  Marker : Clone + Copy + 'static,
{

  type Key< 'k > = usize
  where Self : 'k, usize : 'k;

  type Val< 'v > = OptionalCow< 'v, Borrowed, Marker >
  where Self : 'v;

  fn fields< 's >( &'s self ) -> impl IteratorTrait< Item = ( Self::Key< 's >, Self::Val< 's > ) >
  {
    self.iter().enumerate().map( move | ( key, val ) | ( key, OptionalCow::from( val.borrow() ) ) )
  }

}
