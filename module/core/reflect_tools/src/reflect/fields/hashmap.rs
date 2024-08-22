//!
//! Implement fields for vector.
//!

use crate::*;
use std::borrow::Cow;
use collection_tools::HashMap;

impl< K, V > Fields< K, &'_ V > for HashMap< K, V >
where
  K : core::hash::Hash + core::cmp::Eq,
  V : std::borrow::ToOwned,
{

  type Key< 'k > = &'k K
  where Self : 'k, K : 'k;

  type Val< 'v > = &'v V
  where Self : 'v, V : 'v;

  fn fields( &self ) -> impl IteratorTrait< Item = ( Self::Key< '_ >, Self::Val< '_ > ) >
  {
    self.into_iter()
  }

}

impl< K, V > Fields< K, Option< Cow< '_, V > > > for HashMap< K, V >
where
  K : core::hash::Hash + core::cmp::Eq,
  V : std::borrow::ToOwned,
{

  type Key< 'k > = &'k K
  where Self : 'k, K : 'k;

  type Val< 'v > = Option< Cow< 'v, V > >
  where Self : 'v, V : 'v;

  fn fields( &self ) -> impl IteratorTrait< Item = ( Self::Key< '_ >, Self::Val< '_ > ) >
  {
    self.into_iter().map( move | ( key, val ) | ( key, Some( Cow::Borrowed( val ) ) ) )
  }

}

impl< K, V, Marker > Fields< K, crate::OptionalCow< '_, V, Marker > > for HashMap< K, V >
where
  K : core::hash::Hash + core::cmp::Eq,
  V : std::borrow::ToOwned,
  Marker : Clone + Copy + 'static,
{

  type Key< 'k > = &'k K
  where Self : 'k, K : 'k;

  type Val< 'v > = crate::OptionalCow< 'v, V, Marker >
  where Self : 'v, V : 'v;

  fn fields( &self ) -> impl IteratorTrait< Item = ( Self::Key< '_ >, Self::Val< '_ > ) >
  {
    self.into_iter().map( move | ( key, val ) | ( key, crate::OptionalCow::from( val ) ) )
  }

}
