#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/winterval/latest/winterval/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

// #![ feature( step_trait ) ]

/* zzz : consider https://doc.rust-lang.org/std/ops/trait.RangeBounds.html */
/* zzz : implement iterator */

//!
//! Interval adapter for both open/closed implementations of intervals ( ranges ).
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Internal namespace.
#[ cfg( not( feature = "no_std" ) ) ]
pub( crate ) mod private
{

  #[ doc( inline ) ]
  pub use core::ops::Bound;
  #[ doc( inline ) ]
  pub use core::ops::RangeBounds;

  use core::cmp::{ PartialEq, Eq };
  use core::ops::{ Sub, Add };

  // xxx : seal it

  /// Extend bound adding few methods.
  pub trait BoundExt< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    /// Convert bound to an integer to resemble left bound of a closed interval.
    fn into_left_closed( &self ) -> T;
    /// Convert bound to an integer to resemble right bound of a closed interval.
    fn into_right_closed( &self ) -> T;
  }

  impl< T > BoundExt< T > for Bound< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn into_left_closed( &self ) -> T
    {
      match self
      {
        Bound::Included( v ) => *v,
        Bound::Excluded( v ) => *v + 1.into(),
        Bound::Unbounded => isize::MIN.into(),
      }
    }
    #[ inline( always ) ]
    fn into_right_closed( &self ) -> T
    {
      match self
      {
        Bound::Included( v ) => *v,
        Bound::Excluded( v ) => *v - 1.into(),
        Bound::Unbounded => isize::MAX.into(),
      }
    }
  }

  /// Enpoint of an interval, aka bound of a range.
  /// Special trait to avoid repeating all the bound on endpoint.
  pub trait EndPointTrait< T >
  where
    Self : core::cmp::PartialOrd + Sub< Output = T > + Add< Output = T > + Clone + Copy + Sized,
  {
  }

  impl< T, All > EndPointTrait< T > for All
  where
    Self : core::cmp::PartialOrd + Sub< Output = T > + Add< Output = T > + Clone + Copy + Sized,
  {
  }

  ///
  /// Interval adapter. Interface to interval-like structures.
  ///

  pub trait IntervalAdapter< T = isize >
  where
    Self : IntoIterator< Item = T >,
    T : EndPointTrait< T >,
    isize : Into< T >,
  {

    /// The left endpoint of the interval, as is.
    fn left( &self ) -> Bound< T >;
    /// The right endpoint of the interval, as is.
    fn right( &self ) -> Bound< T >;
    /// Interval in closed format as pair of numbers.
    /// To convert open endpoint to closed add or subtract one.
    #[ inline( always ) ]
    fn bounds( &self ) -> ( Bound< T >, Bound< T > )
    {
      ( self.left(), self.right() )
    }

    /// The left endpoint of the interval, converting interval into closed one.
    #[ inline( always ) ]
    fn closed_left( &self ) -> T
    {
      self.left().into_left_closed()
    }
    /// The right endpoint of the interval, converting interval into closed one.
    #[ inline( always ) ]
    fn closed_right( &self ) -> T
    {
      self.left().into_right_closed()
    }
    /// Length of the interval, converting interval into closed one.
    #[ inline( always ) ]
    fn closed_len( &self ) -> T
    {
      let one : T = 1.into();
      self.closed_right() - self.closed_left() + one
    }
    /// Interval in closed format as pair of numbers, converting interval into closed one.
    #[ inline( always ) ]
    fn closed( &self ) -> ( T, T )
    {
      ( self.closed_left(), self.closed_right() )
    }

    /// Convert to interval in canonical format.
    #[ inline( always ) ]
    fn canonical( &self ) -> Interval< T >
    {
      Interval::new( self.left(), self.right() )
    }

  }

  ///
  /// Canonical implementation of interval. Other implementations of interval is convertible to it.
  ///
  /// Both [Range], [RangeInclusive] are convertable to [crate::Interval]
  ///

  #[ derive( PartialEq, Eq, Debug, Clone, Copy ) ]
  pub struct Interval< T = isize >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    _left : Bound< T >,
    _right : Bound< T >,
  }

  impl< T > Interval< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    /// Constructor of an interval. Expects closed interval in arguments.
    pub fn new( left : Bound< T >, right : Bound< T > ) -> Self
    {
      Self { _left : left, _right : right }
    }
    /// Convert to interval in canonical format.
    #[ inline( always ) ]
    pub fn iter< It >( &self ) -> impl Iterator< Item = T >
    {
      ( &self ).into_iter()
    }
  }

  // =
  // IntoIterator for Interval
  // =

  impl< T > IntoIterator for Interval< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    type Item = T;
    type IntoIter = IntervalIterator< T >;
    #[ inline( always ) ]
    fn into_iter( self ) -> Self::IntoIter
    {
      IntervalIterator::new( self )
    }
  }

  impl< T > IntoIterator for &Interval< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    type Item = T;
    type IntoIter = IntervalIterator< T >;
    #[ inline( always ) ]
    fn into_iter( self ) -> Self::IntoIter
    {
      IntervalIterator::new( *self )
    }
  }

  #[ derive( Debug ) ]
  pub struct IntervalIterator< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    current : T,
    right : T,
  }

  impl< T > IntervalIterator< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    /// Constructor.
    pub fn new( ins : Interval< T > ) -> Self
    {
      let current = ins._left.into_left_closed();
      let right = ins._right.into_right_closed();
      Self { current, right }
    }
  }

  impl< T > Iterator for IntervalIterator< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    type Item = T;
    fn next( &mut self ) -> Option< Self::Item >
    {
      if self.current <= self.right
      {
        let result = Some( self.current );
        self.current = self.current + 1.into();
        result
      }
      else
      {
        None
      }
    }
  }

  //
  // impl IntervalAdapter
  //

  impl< T > IntervalAdapter< T >
  for Interval< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    fn left( &self ) -> Bound< T >
    {
      self._left
    }
    fn right( &self ) -> Bound< T >
    {
      self._right
    }
    fn closed_left( &self ) -> T
    {
      self._left.into_left_closed()
    }
    fn closed_right( &self ) -> T
    {
      self._right.into_right_closed()
    }
  }

  impl< T > IntervalAdapter< T >
  for core::ops::Range< T >
  where
    core::ops::Range< T > : Iterator< Item = T >,
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    fn left( &self ) -> Bound< T >
    {
      Bound::Included( self.start )
    }
    fn right( &self ) -> Bound< T >
    {
      Bound::Excluded( self.end )
    }
    fn closed_left( &self ) -> T
    {
      self.start
    }
    fn closed_right( &self ) -> T
    {
      let one : T = 1.into();
      self.end - one
    }
  }

  impl< T > IntervalAdapter< T >
  for core::ops::RangeInclusive< T >
  where
    core::ops::RangeInclusive< T > : Iterator< Item = T >,
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    fn left( &self ) -> Bound< T >
    {
      Bound::Included( *self.start() )
    }
    fn right( &self ) -> Bound< T >
    {
      Bound::Included( *self.end() )
    }
    fn closed_left( &self ) -> T
    {
      *self.start()
    }
    fn closed_right( &self ) -> T
    {
      *self.end()
    }
  }

  impl< T > IntervalAdapter< T >
  for core::ops::RangeTo< T >
  where
    core::ops::RangeTo< T > : Iterator< Item = T >,
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    fn left( &self ) -> Bound< T >
    {
      Bound::Unbounded
    }
    fn right( &self ) -> Bound< T >
    {
      Bound::Included( self.end )
    }
  }

  // =
  // from for std
  // =

  impl< T > From< core::ops::Range< T > >
  for Interval< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn from( src : core::ops::Range< T > ) -> Self
    {
      Self { _left : Bound::Included( src.start ), _right : Bound::Excluded( src.end ) }
    }
  }

  //

  impl< T > From< core::ops::RangeInclusive< T > >
  for Interval< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn from( src : core::ops::RangeInclusive< T > ) -> Self
    {
      Self { _left : Bound::Included( *src.start() ), _right : Bound::Included( *src.end() ) }
    }
  }

  //

  impl< T > From< core::ops::RangeTo< T > >
  for Interval< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn from( src : core::ops::RangeTo< T > ) -> Self
    {
      Self { _left : Bound::Unbounded, _right : Bound::Included( src.end ) }
    }
  }

  //

  impl< T > From< ( T, T ) >
  for Interval< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn from( src : ( T, T ) ) -> Self
    {
      Self { _left : Bound::Included( src.0 ), _right : Bound::Included( src.1 ) }
    }
  }

  //

  impl< T > From< ( Bound< T >, Bound< T > ) >
  for Interval< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn from( src : ( Bound< T >, Bound< T > ) ) -> Self
    {
      Self { _left : src.0, _right : src.1 }
    }
  }

  //

  impl< T > From< [ T ; 2 ] >
  for Interval< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn from( src : [ T ; 2 ] ) -> Self
    {
      Self { _left : Bound::Included( src[ 0 ] ), _right : Bound::Included( src[ 1 ] ) }
    }
  }

  //

  impl< T > From< [ Bound< T > ; 2 ] >
  for Interval< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn from( src : [ Bound< T > ; 2 ] ) -> Self
    {
      Self { _left : src[ 0 ], _right : src[ 1 ] }
    }
  }

  /// Convert it into canonical interval.
  pub trait IntoInterval< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    /// Convert it into canonical interval.
    fn into_interval( self ) -> Interval< T >;
  }

  impl< T, All > IntoInterval< T > for All
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
    Interval< T > : From< Self >,
  {
    fn into_interval( self ) -> Interval< T >
    {
      From::from( self )
    }
  }

// //   macro_rules! impl_interval_into_interval
// //   {
// //     {
// //       $( $Type : tt )*
// //     }
// //     =>
// //     {
// //       impl< T > IntoInterval< T > for $( $Type )*< T >
// //       where
// //         T : EndPointTrait< T >,
// //         isize : Into< T >,
// //         $( $Type )*< T > : Iterator< Item = T >,
// //       {
// //         fn into_interval( self ) -> Interval< T >
// //         {
// //           IntervalAdapter::< T >::canonical( &self )
// //         }
// //       }
// //     }
// //   }

}


/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;
  #[ cfg( not( feature = "no_std" ) ) ]
  #[ doc( inline ) ]
  pub use super::private::
  {
    Bound,
    EndPointTrait,
    IntervalAdapter,
    Interval,
    IntoInterval,
  };
}

#[ doc( inline ) ]
pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
