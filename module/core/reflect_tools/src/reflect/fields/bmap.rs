//!
//! Implement fields for BTreeMap.
//!

// qqq : xxx : implement for other containers

use crate::*;
use std::borrow::Cow;
use collection_tools::Bmap;

impl< K, V, Borrowed > Fields< K, &'_ Borrowed > for Bmap< K, V >
where
  Borrowed : std::borrow::ToOwned + 'static + ?Sized,
  V : std::borrow::Borrow< Borrowed >,
{

  type Key< 'k > = &'k K
  where Self : 'k, K : 'k;

  type Val< 'v > = &'v Borrowed
  where Self : 'v, V : 'v;

  fn fields< 's >( &'s self ) -> impl IteratorTrait< Item = ( Self::Key< 's >, Self::Val< 's > ) >
  {
    self.iter().map( move | ( key, val ) | ( key, val.borrow() ) )
  }

}

impl< K, V, Borrowed > Fields< K, Option< Cow< '_, Borrowed > > > for Bmap< K, V >
where
  Borrowed : std::borrow::ToOwned + 'static + ?Sized,
  V : std::borrow::Borrow< Borrowed >,
{

  type Key< 'k > = &'k K
  where Self : 'k, K : 'k;

  type Val< 'v > = Option< Cow< 'v, Borrowed > >
  where Self : 'v, V : 'v;

  fn fields< 's >( &'s self ) -> impl IteratorTrait< Item = ( Self::Key< 's >, Self::Val< 's > ) >
  {
    self.iter().map( move | ( key, val ) | ( key, Some( Cow::Borrowed( val.borrow() ) ) ) )
  }

}

impl< K, V, Borrowed, Marker > Fields< K, OptionalCow< '_, Borrowed, Marker > > for Bmap< K, V >
where
  Borrowed : std::borrow::ToOwned + 'static + ?Sized,
  V : std::borrow::Borrow< Borrowed >,
  Marker : Clone + Copy + 'static,
{

  type Key< 'k > = &'k K
  where Self : 'k, K : 'k;

  type Val< 'v > = OptionalCow< 'v, Borrowed, Marker >
  where Self : 'v, V : 'v;

  fn fields< 's >( &'s self ) -> impl IteratorTrait< Item = ( Self::Key< 's >, Self::Val< 's > ) >
  {
    self.iter().map( move | ( key, val ) | ( key, OptionalCow::from( val.borrow() ) ) )
  }

}
