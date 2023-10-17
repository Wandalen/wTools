#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/winterval/latest/winterval/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

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
  pub use std::ops::RangeBounds;

  ///
  /// Interval adapter. Interface to interval-like structures.
  ///

  pub trait IntervalAdapter< T = isize >
  where
    T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
    isize : Into< T >,
  {

    /// The left endpoint of the interval, as is.
    fn left( &self ) -> T;
    /// The right endpoint of the interval, as is.
    fn right( &self ) -> T;
    /// Interval in closed format as pair of numbers.
    /// To convert open endpoint to closed add or subtract one.
    fn pair( &self ) -> ( T, T )
    {
      ( self.left(), self.right() )
    }

    /// The left endpoint of the interval, converting interval into closed one.
    fn closed_left( &self ) -> T;
    /// The right endpoint of the interval, converting interval into closed one.
    fn closed_right( &self ) -> T;
    /// Length of the interval, converting interval into closed one.
    fn closed_len( &self ) -> T
    {
      let one : T = 1.into();
      self.closed_right() - self.closed_left() + one
    }
    /// Interval in closed format as pair of numbers, converting interval into closed one.
    fn closed( &self ) -> ( T, T )
    {
      ( self.closed_left(), self.closed_right() )
    }

    /// Interval as pair of endpoints.
    /// To convert open endpoint to closed add or subtract one.
    fn bounds( &self ) -> ( Bound< T >, Bound< T > );

    // /// Interval in closed format as pair of numbers.
    // /// To convert open endpoint to closed add or subtract one.
    // fn iter( &self ) -> impl Iterator< Item = T >
    // {
    //   let bounds = self.bounds();
    //   match bounds
    //   {
    //     ( Bound::Included( left ), Bound::Included( right ) ) => left..=right,
    //     ( Bound::Included( left ), Bound::Excluded( right ) ) => left..right,
    //     _ => 0..0,
    //   }
    // }

    // /// Interval in left-length format as pair of numbers, assumed it is closed.
    // fn left_len( &self ) -> ( T, T )
    // {
    //   ( self.closed_left(), self.closed_len() )
    // }

  }

  ///
  /// Alternative implementation of interval.
  ///
  /// Both [core::ops::Range], [core::ops::RangeInclusive] are convertable to [crate::Interval]
  ///

  #[ derive( PartialEq, Eq, Debug ) ]
  pub struct Interval< T = isize >
  where
    T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
    isize : Into< T >,
  {
    _left : Bound< T >,
    _right : Bound< T >,
  }

  impl< T > Interval< T >
  where
    T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
    isize : Into< T >,
  {
    /// Constructor of an interval. Expects closed interval in arguments.
    pub fn new( left : Bound< T >, right : Bound< T > ) -> Self
    {
      Self { _left : left, _right : right }
    }
  }

  impl< T > IntervalAdapter< T >
  for Interval< T >
  where
    T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
    isize : Into< T >,
  {
    fn left( &self ) -> T
    {
      match self._left
      {
        Bound::Included( v ) => v,
        Bound::Excluded( v ) => v,
        Bound::Unbounded => isize::MIN.into(),
      }
    }
    fn right( &self ) -> T
    {
      match self._right
      {
        Bound::Included( v ) => v,
        Bound::Excluded( v ) => v,
        Bound::Unbounded => isize::MAX.into(),
      }
    }
    fn closed_left( &self ) -> T
    {
      match self._left
      {
        Bound::Included( v ) => v,
        Bound::Excluded( v ) => v + 1.into(),
        Bound::Unbounded => isize::MIN.into(),
      }
    }
    fn closed_right( &self ) -> T
    {
      match self._right
      {
        Bound::Included( v ) => v,
        Bound::Excluded( v ) => v - 1.into(),
        Bound::Unbounded => isize::MAX.into(),
      }
    }
    fn bounds( &self ) -> ( Bound< T >, Bound< T > )
    {
      ( self._left, self._right )
    }
    // fn closed( &self ) -> ( T, T )
    // {
    //   ( closed_left(), closed_right() )
    // }

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
    fn left( &self ) -> T
    {
      self.start
    }
    fn right( &self ) -> T
    {
      self.end
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
    fn bounds( &self ) -> ( Bound< T >, Bound< T > )
    {
      ( Bound::Included( self.left() ), Bound::Excluded( self.right() ) )
    }
  }

  impl< T > IntervalAdapter< T >
  for ::core::ops::RangeInclusive< T >
  where
    T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
    isize : Into< T >,
  {
    fn left( &self ) -> T
    {
      *self.start()
    }
    fn right( &self ) -> T
    {
      *self.end()
    }
    fn closed_left( &self ) -> T
    {
      *self.start()
    }
    fn closed_right( &self ) -> T
    {
      *self.end()
    }
    fn bounds( &self ) -> ( Bound< T >, Bound< T > )
    {
      ( Bound::Included( self.left() ), Bound::Included( self.right() ) )
    }
  }

  //
  // from for std
  //

  // impl< T > RangeBounds< T >
  // for Interval
  // where
  //   // IntervalAdapterType : IntervalAdapter< T >,
  //   Interval : IntervalAdapter< T >,
  //   T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
  //   isize : Into< T >,
  // {
  //   // Required methods
  //   fn start_bound(&self) -> Bound<&T>
  //   {
  //     let bound = self.bounds().0;
  //     match bound
  //     {
  //       Bound::Included( v ) => Bound::Included( &v ),
  //       Bound::Excluded( v ) => Bound::Excluded( &v ),
  //       Bound::Unbounded => Bound::Unbounded,
  //     }
  //     // self.bounds().0.as_ref()
  //   }
  //   fn end_bound(&self) -> Bound<&T>
  //   {
  //     self.bounds().1.as_ref()
  //   }
  // }

  // impl< T, RangeBoundType > From< RangeBoundType >
  // for Interval< T >
  // where
  //   RangeBoundType : RangeBounds< T >,
  //   T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
  //   isize : Into< T >,
  // {
  //   fn from( src : ::core::ops::Range< T > ) -> Self
  //   {
  //     Self { _left : Bound::Included( src.start ), _right : Bound::Excluded( src.end ) }
  //   }
  // }

  impl< T > From< ::core::ops::Range< T > >
  for Interval< T >
  where
    T : std::ops::Sub< Output = T > + std::ops::Add< Output = T > + Copy,
    isize : Into< T >,
  {
    fn from( src : ::core::ops::Range< T > ) -> Self
    {
      Self { _left : Bound::Included( src.start ), _right : Bound::Excluded( src.end ) }
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
      Self { _left : Bound::Included( *src.start() ), _right : Bound::Included( *src.end() ) }
    }
  }

  // xxx : qqq2 : implement rest Froms
  // xxx : qqq2 : std range Froms interval

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
    IntervalAdapter,
    Interval,
  };
}

#[ doc( inline ) ]
pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
