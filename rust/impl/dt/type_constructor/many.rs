/// Internal namespace.
mod internal
{
  use crate::exposed::*;

  ///
  /// Type constructor of many.
  ///
  /// Should not be used directly. Instead use macro [crate::types!].
  ///

  #[ macro_export ]
  macro_rules! _many
  {

    // many Many : < T >;

    (
      $( #[ $Meta : meta ] )*
      many $Name : ident :
      < $ParamName : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy2 : path )* )? >
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      $( #[ $Meta ] )*
      pub struct $Name
      < $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      ( pub std::vec::Vec< $ParamName > );

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? > core::ops::Deref
      for $Name
      < $ParamName >
      {
        type Target = std::vec::Vec< $ParamName >;
        fn deref( &self ) -> &Self::Target
        {
          &self.0
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? > core::ops::DerefMut
      for $Name
      < $ParamName >
      {
        fn deref_mut( &mut self ) -> &mut Self::Target
        {
          &mut self.0
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      From< $ParamName >
      for $Name
      < $ParamName >
      {
        fn from( src : $ParamName ) -> Self
        {
          Self( vec![ src ] )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      From< ( $ParamName, ) >
      for $Name
      < $ParamName >
      {
        fn from( src : ( $ParamName, ) ) -> Self
        {
          Self( vec![ src.0 ] )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )?, const N : usize >
      From< [ $ParamName ; N ] >
      for $Name
      < $ParamName >
      where
        $ParamName : Clone,
      {
        fn from( src : [ $ParamName ; N ] ) -> Self
        {
          Self( std::vec::Vec::from( src ) )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      From< &[ $ParamName ] >
      for $Name
      < $ParamName >
      where
        $ParamName : Clone,
      {
        fn from( src : &[ $ParamName ] ) -> Self
        {
          Self( std::vec::Vec::from( src ) )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      AsSlice< $ParamName >
      for $Name < $ParamName >
      {
        fn as_slice( &self ) -> &[ $ParamName ]
        {
          &self[ .. ]
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      Make0
      for $Name < $ParamName >
      {
        fn make_0() -> Self
        {
          Self( std::vec::Vec::new() )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      Make1< $ParamName >
      for $Name < $ParamName >
      {
        fn make_1( _0 : $ParamName ) -> Self
        {
          Self( vec![ _0 ] )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      Make2< $ParamName, $ParamName >
      for $Name < $ParamName >
      {
        fn make_2( _0 : $ParamName, _1 : $ParamName ) -> Self
        {
          Self( vec![ _0, _1 ] )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      Make3< $ParamName, $ParamName, $ParamName >
      for $Name < $ParamName >
      {
        fn make_3( _0 : $ParamName, _1 : $ParamName, _2 : $ParamName ) -> Self
        {
          Self( vec![ _0, _1, _2 ] )
        }
      }

      $crate::types!{ $( $( $Rest )* )? }
    };

    // many Many : < T1, ... >;

    (
      $( #[ $Meta : meta ] )*
      many $Name : ident :
      < $ParamName : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy2 : path )* )? ,
      $( $Rest : tt )*
    )
    =>
    {
      compile_error!
      (
        concat!
        (
          "Parametrized element should be many, because Many has only one element\n",
          stringify!
          (
            $( #[ $Meta ] )*
            $Name :
            < $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ,
            $( $Rest )*
          )
        )
      );
    };

    // many Many : Element< T1, T2, ... >;

    (
      $( #[ $Meta : meta ] )*
      many $Name : ident : $TypeSplit1 : ident $( :: $TypeSplitN : ident )*
      $( < $( $ParamName : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy2 : path )* )? ),* > )?
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      $( #[ $Meta ] )*
      pub struct $Name
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      ( pub std::vec::Vec< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? > );

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      core::ops::Deref
      for $Name
      $( < $( $ParamName ),* > )?
      {
        type Target = std::vec::Vec< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? >;
        fn deref( &self ) -> &Self::Target
        {
          &self.0
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      core::ops::DerefMut
      for $Name
      $( < $( $ParamName ),* > )?
      {
        fn deref_mut( &mut self ) -> &mut Self::Target
        {
          &mut self.0
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      From
      < $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? >
      for $Name
      $( < $( $ParamName ),* > )?
      {
        fn from( src : $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ) -> Self
        {
          Self( vec![ src ] )
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      From
      < ( $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? , ) >
      for $Name
      $( < $( $ParamName ),* > )?
      {
        fn from( src : ( $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? , ) ) -> Self
        {
          Self( vec![ src.0 ] )
        }
      }

      impl
      < $( $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? , )* )? const N : usize >
      From
      < [ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ; N ] >
      for $Name
      $( < $( $ParamName ),* > )?
      where
        $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? : Clone,
      {
        fn from( src : [ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ; N ] ) -> Self
        {
          Self( std::vec::Vec::from( src ) )
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      From
      < &[ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ] >
      for $Name
      $( < $( $ParamName ),* > )?
      where
        $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? : Clone,
      {
        fn from( src : &[ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ] ) -> Self
        {
          debug_assert_eq!( src.len(), 1 ); // xxx : add test with long slice
          Self( std::vec::Vec::from( src ) )
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      AsSlice< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? >
      for
      $Name $( < $( $ParamName ),* > )?
      where
        $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? : Clone,
      {
        fn as_slice( &self ) -> &[ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ]
        {
          &self[ .. ]
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      Make0
      for
      $Name $( < $( $ParamName ),* > )?
      {
        fn make_0() -> Self
        {
          Self( std::vec::Vec::< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? >::new() )
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      Make1< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? >
      for
      $Name $( < $( $ParamName ),* > )?
      {
        fn make_1
        (
          _0 : $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
        )
        -> Self
        {
          Self( vec![ _0 ] )
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      Make2
      <
        $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
        $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
      >
      for
      $Name $( < $( $ParamName ),* > )?
      {
        fn make_2
        (
          _0 : $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
          _1 : $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
        )
        -> Self
        {
          Self( vec![ _0, _1 ] )
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      Make3
      <
        $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
        $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
        $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
      >
      for
      $Name $( < $( $ParamName ),* > )?
      {
        fn make_3
        (
          _0 : $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
          _1 : $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
          _2 : $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
        )
        -> Self
        {
          Self( vec![ _0, _1, _2 ] )
        }
      }

      $crate::types!{ $( $( $Rest )* )? }
    };

  }

  types!
  {

    ///
    /// Type constructor to wrap a another type into a tuple.
    ///

    // xxx : sample

    #[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
    many Many : < T >;

  }

  pub use _many;
}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::internal::
  {

    _many,
    Many,

  };
}
