#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/winterval/latest/winterval/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Internal namespace.
#[ cfg( feature = "enabled" ) ]
mod private
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use core::ops::Bound;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
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
        Bound::Unbounded => 0.into(),
        // Bound::Unbounded => isize::MIN.into(),
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
  /// `NonIterableInterval` it does not implement iterator unlike `IterableInterval`.
  /// `IterableInterval` inherits all methods of `NonIterableInterval`.
  ///
  /// Non-iterable intervals have either one or several unbound endpoints.
  /// For example, interval `core::ops::RangeFull` has no bounds and represents the range from minus infinity to plus infinity.
  ///
  pub trait NonIterableInterval< T = isize >
  where
    // Self : IntoIterator< Item = T >,
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
      self.right().into_right_closed()
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
  /// Interval adapter. Interface to interval-like structures.
  ///
  /// `NonIterableInterval` it does not implement iterator unlike `IterableInterval`.
  /// `IterableInterval` inherits all methods of `NonIterableInterval`.
  ///

  pub trait IterableInterval< T = isize >
  where
    Self : IntoIterator< Item = T > + NonIterableInterval< T >,
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
  }

  impl< T, NonIterableIntervalType > IterableInterval< T >
  for NonIterableIntervalType
  where
    NonIterableIntervalType : NonIterableInterval< T >,
    Self : IntoIterator< Item = T > + NonIterableInterval< T >,
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
  }

  ///
  /// Canonical implementation of interval. Other implementations of interval is convertible to it.
  ///
  /// Both [core::ops::Range], [core::ops::RangeInclusive] are convertable to [crate::Interval]
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
    #[ inline( always ) ]
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
  // impl IterableInterval
  //

  // impl< T, All > NonIterableInterval< T > for All
  // where
  //   T : EndPointTrait< T >,
  //   isize : Into< T >,
  //   Interval< T > : From< Self >,
  //   All : Clone,
  // {
  //   #[ inline( always ) ]
  //   fn left( &self ) -> Bound< T >
  //   {
  //     Interval::from( self.clone() )._left
  //   }
  //   #[ inline( always ) ]
  //   fn right( &self ) -> Bound< T >
  //   {
  //     Interval::from( self.clone() )._right
  //   }
  // }

  impl< T > NonIterableInterval< T >
  for Interval< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn left( &self ) -> Bound< T >
    {
      self._left
    }
    #[ inline( always ) ]
    fn right( &self ) -> Bound< T >
    {
      self._right
    }
  }

  impl< T > NonIterableInterval< T >
  for core::ops::Range< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn left( &self ) -> Bound< T >
    {
      Bound::Included( self.start )
    }
    #[ inline( always ) ]
    fn right( &self ) -> Bound< T >
    {
      Bound::Excluded( self.end )
    }
  }

  impl< T > NonIterableInterval< T >
  for core::ops::RangeInclusive< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn left( &self ) -> Bound< T >
    {
      Bound::Included( *self.start() )
    }
    #[ inline( always ) ]
    fn right( &self ) -> Bound< T >
    {
      Bound::Included( *self.end() )
    }
  }

  impl< T > NonIterableInterval< T >
  for core::ops::RangeTo< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn left( &self ) -> Bound< T >
    {
      Bound::Unbounded
    }
    #[ inline( always ) ]
    fn right( &self ) -> Bound< T >
    {
      Bound::Excluded( self.end )
    }
  }

  impl< T > NonIterableInterval< T >
  for core::ops::RangeToInclusive< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn left( &self ) -> Bound< T >
    {
      Bound::Unbounded
    }
    #[ inline( always ) ]
    fn right( &self ) -> Bound< T >
    {
      Bound::Included( self.end )
    }
  }

  impl< T > NonIterableInterval< T >
  for core::ops::RangeFrom< T >
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn left( &self ) -> Bound< T >
    {
      Bound::Included( self.start )
    }
    #[ inline( always ) ]
    fn right( &self ) -> Bound< T >
    {
      Bound::Unbounded
    }
  }

  impl< T > NonIterableInterval< T >
  for core::ops::RangeFull
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn left( &self ) -> Bound< T >
    {
      Bound::Unbounded
    }
    #[ inline( always ) ]
    fn right( &self ) -> Bound< T >
    {
      Bound::Unbounded
    }
  }

  impl< T > NonIterableInterval< T >
  for ( T, T )
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn left( &self ) -> Bound< T >
    {
      Bound::Included( self.0 )
    }
    #[ inline( always ) ]
    fn right( &self ) -> Bound< T >
    {
      Bound::Included( self.1 )
    }
  }

  impl< T > NonIterableInterval< T >
  for ( Bound< T >, Bound< T > )
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn left( &self ) -> Bound< T >
    {
      self.0
    }
    #[ inline( always ) ]
    fn right( &self ) -> Bound< T >
    {
      self.1
    }
  }

  impl< T > NonIterableInterval< T >
  for [ T ; 2 ]
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn left( &self ) -> Bound< T >
    {
      Bound::Included( self[ 0 ] )
    }
    #[ inline( always ) ]
    fn right( &self ) -> Bound< T >
    {
      Bound::Included( self[ 1 ] )
    }
  }

  impl< T > NonIterableInterval< T >
  for [ Bound< T > ; 2 ]
  where
    T : EndPointTrait< T >,
    isize : Into< T >,
  {
    #[ inline( always ) ]
    fn left( &self ) -> Bound< T >
    {
      self[ 0 ]
    }
    #[ inline( always ) ]
    fn right( &self ) -> Bound< T >
    {
      self[ 1 ]
    }
  }

  // =
  // from for std
  // =

  macro_rules! impl_interval_from
  {
    {} => {};
    {
      $Type : ty
    }
    =>
    {
      impl< T > From< $Type >
      for Interval< T >
      where
        T : EndPointTrait< T >,
        isize : Into< T >,
      {
        #[ inline( always ) ]
        fn from( src : $Type ) -> Self
        {
          let _left = NonIterableInterval::left( &src );
          let _right = NonIterableInterval::right( &src );
          Self { _left, _right }
        }
      }
    };
    {
      $Type : ty
      , $( $Rest : tt )*
    }
    =>
    {
      impl_interval_from!{ $Type }
      impl_interval_from!{ $( $Rest )* }
    };
  }

  impl_interval_from!
  {
    core::ops::Range< T >,
    core::ops::RangeInclusive< T >,
    core::ops::RangeTo< T >,
    core::ops::RangeToInclusive< T >,
    core::ops::RangeFrom< T >,
    core::ops::RangeFull,
    ( T, T ),
    ( Bound< T >, Bound< T > ),
    [ T ; 2 ],
    [ Bound< T > ; 2 ],
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

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
// #[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
}

/// Parented namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*;
  #[ doc( inline ) ]
  pub use private::
  {
    Bound,
    BoundExt,
    EndPointTrait,
    Interval,
    // IterableInterval,
    // NonIterableInterval,
    // IntoInterval,
  };
}

// #[ doc( inline ) ]
#[ allow( unused_imports ) ]
// #[ cfg( feature = "enabled" ) ]
// #[ allow( unused_imports ) ]
// pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  #[ doc( inline ) ]
  pub use private::
  {
    IterableInterval,
    NonIterableInterval,
    IntoInterval,
  };
}
