//!
//! Implement fields for vector.
//!

use crate::*;
use std::borrow::Cow;
use collection_tools::HashMap;

impl< K, V, Borrowed > Fields< K, &'_ Borrowed > for HashMap< K, V >
where
  K : core::hash::Hash + core::cmp::Eq,
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

impl< K, V, Borrowed > Fields< K, Option< Cow< '_, Borrowed > > > for HashMap< K, V >
where
  K : core::hash::Hash + core::cmp::Eq,
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

impl< K, V, Borrowed, Marker > Fields< K, OptionalCow< '_, Borrowed, Marker > > for HashMap< K, V >
where
  K : core::hash::Hash + core::cmp::Eq,
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

// impl< K, V > Fields< &'_ K, &'_ V > for HashMap< K, V >
// where
//   K : core::hash::Hash + core::cmp::Eq,
//   V : std::borrow::ToOwned,
// {
//
//   type Key< 'k > = &'k K
//   where Self : 'k, K : 'k;
//
//   type Val< 'v > = &'v V
//   where Self : 'v, V : 'v;
//
//   fn fields( &self ) -> impl IteratorTrait< Item = ( Self::Key< '_ >, Self::Val< '_ > ) >
//   {
//     self.iter()
//   }
//
// }
//
// impl< K, V > Fields< &'_ K, Option< Cow< '_, V > > > for HashMap< K, V >
// where
//   K : core::hash::Hash + core::cmp::Eq,
//   V : std::borrow::ToOwned,
// {
//
//   type Key< 'k > = &'k K
//   where Self : 'k, K : 'k;
//
//   type Val< 'v > = Option< Cow< 'v, V > >
//   where Self : 'v, V : 'v;
//
//   fn fields( &self ) -> impl IteratorTrait< Item = ( Self::Key< '_ >, Self::Val< '_ > ) >
//   {
//     self.iter().map( move | ( key, val ) | ( key, Some( Cow::Borrowed( val ) ) ) )
//   }
//
// }
//
// impl< K, V, Marker > Fields< &'_ K, crate::OptionalCow< '_, V, Marker > > for HashMap< K, V >
// where
//   K : core::hash::Hash + core::cmp::Eq,
//   V : std::borrow::ToOwned,
//   Marker : Clone + Copy + 'static,
// {
//
//   type Key< 'k > = &'k K
//   where Self : 'k, K : 'k;
//
//   type Val< 'v > = crate::OptionalCow< 'v, V, Marker >
//   where Self : 'v, V : 'v;
//
//   fn fields( &self ) -> impl IteratorTrait< Item = ( Self::Key< '_ >, Self::Val< '_ > ) >
//   {
//     self.iter().map( move | ( key, val ) | ( key, crate::OptionalCow::from( val ) ) )
//   }
//
// }
