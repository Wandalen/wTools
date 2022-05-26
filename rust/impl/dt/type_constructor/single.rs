/// Internal namespace.
mod internal
{
  use crate::exposed::*;

  ///
  /// Type constructor of single.
  ///
  /// Should not be used directly. Instead use macro [crate::types!].
  ///

  #[ macro_export ]
  macro_rules! _single
  {

    // single Single : < T >;

    (
      $( #[ $Meta : meta ] )*
      single $Name : ident :
      < $ParamName : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy2 : path )* )? >
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      $( #[ $Meta ] )*
      pub struct $Name
      < $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      ( pub $ParamName );

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? > core::ops::Deref
      for $Name
      < $ParamName >
      {
        type Target = $ParamName;
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
          Self( src )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      From< ( $ParamName, ) >
      for $Name
      < $ParamName >
      {
        fn from( src : ( $ParamName, ) ) -> Self
        {
          Self( src.0 )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      From< $Name< $ParamName > >
      for ( $ParamName, )
      {
        fn from( src : $Name< $ParamName > ) -> Self
        {
          ( src.0, )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      From< [ $ParamName ; 1 ] >
      for $Name
      < $ParamName >
      where
        $ParamName : Clone,
      {
        fn from( src : [ $ParamName ; 1 ] ) -> Self
        {
          Self( src[ 0 ].clone() )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      From< $Name< $ParamName > >
      for [ $ParamName ; 1 ]
      {
        fn from( src : $Name< $ParamName > ) -> Self
        {
          [ src.0 ]
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
          debug_assert_eq!( src.len(), 1 );
          Self( src[ 0 ].clone() )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      CloneAsTuple< ( $ParamName, ) >
      for $Name < $ParamName >
      where
        $ParamName : Clone,
      {
        fn clone_as_tuple( &self ) -> ( $ParamName, )
        {
          ( self.0.clone(), )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      CloneAsArray< $ParamName, 1 >
      for $Name < $ParamName >
      where
        $ParamName : Clone,
      {
        fn clone_as_array( &self ) -> [ $ParamName ; 1 ]
        {
          [ self.0.clone() ; 1 ]
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      AsTuple< ( $ParamName, ) >
      for $Name < $ParamName >
      {
        fn as_tuple( &self ) -> &( $ParamName, )
        {
          /* Safety : in case of single elemet it is safe to assume that layout is the same. It does not have to have #[repr(C)]. */
          unsafe
          {
            std::mem::transmute::< _, _ >( self )
          }
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      AsArray< $ParamName, 1 >
      for $Name < $ParamName >
      {
        fn as_array( &self ) -> &[ $ParamName ; 1 ]
        {
          /* Safety : in case of single elemet it is safe to assume that layout is the same. It does not have to have #[repr(C)]. */
          unsafe
          {
            std::mem::transmute::< _, _ >( self )
          }
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      AsSlice< $ParamName >
      for $Name < $ParamName >
      {
        fn as_slice( &self ) -> &[ $ParamName ]
        {
          &self.as_array()[ .. ]
        }
      }

      #[ cfg( feature = "make" ) ]
      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      Make0
      for $Name < $ParamName >
      where $ParamName : Default
      {
        fn make_0() -> Self
        {
          Self( Default::default() )
        }
      }

      #[ cfg( feature = "make" ) ]
      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      Make1< $ParamName >
      for $Name < $ParamName >
      {
        fn make_1( _0 : $ParamName ) -> Self
        {
          Self( _0 )
        }
      }

      // From Single Into Element cant be implemented because of Rust restrictions.

      $crate::types!{ $( $( $Rest )* )? }
    };

    // single Single : < T1, ... >;

    (
      $( #[ $Meta : meta ] )*
      single $Name : ident :
      < $ParamName : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy2 : path )* )? ,
      $( $Rest : tt )*
    )
    =>
    {
      compile_error!
      (
        concat!
        (
          "Parametrized element should be single, because Single has only one element\n",
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

    // single Single : Element< T1, T2, ... >;

    (
      $( #[ $Meta : meta ] )*
      single $Name : ident : $TypeSplit1 : ident $( :: $TypeSplitN : ident )*
      $( < $( $ParamName : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy2 : path )* )? ),* > )?
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      $( #[ $Meta ] )*
      pub struct $Name
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      ( pub $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? );

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      core::ops::Deref
      for $Name
      $( < $( $ParamName ),* > )?
      {
        type Target = $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?;
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
          Self( src )
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      From
      < $Name $( < $( $ParamName ),* > )? >
      for $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?
      {
        fn from( src : $Name $( < $( $ParamName ),* > )? ) -> Self
        {
          src.0
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
          Self( src.0 )
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      From
      < [ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ; 1 ] >
      for $Name
      $( < $( $ParamName ),* > )?
      where
        $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? : Clone,
      {
        fn from( src : [ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ; 1 ] ) -> Self
        {
          Self( src[ 0 ].clone() )
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
          debug_assert_eq!( src.len(), 1 );
          Self( src[ 0 ].clone() )
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      CloneAsTuple< ( $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?, ) >
      for
      $Name $( < $( $ParamName ),* > )?
      where
        $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? : Clone,
      {
        fn clone_as_tuple( &self ) -> ( $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?, )
        {
          ( self.0.clone(), )
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      CloneAsArray< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? , 1 >
      for
      $Name $( < $( $ParamName ),* > )?
      where
        $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? : Clone,
      {
        fn clone_as_array( &self ) -> [ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ; 1 ]
        {
          [ self.0.clone() ]
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      AsTuple< ( $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?, ) >
      for
      $Name $( < $( $ParamName ),* > )?
      where
        $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? : Clone,
      {
        fn as_tuple( &self ) -> &( $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?, )
        {
          /* Safety : in case of single elemet it is safe to assume that layout is the same. It does not have to have #[repr(C)]. */
          unsafe
          {
            std::mem::transmute::< _, _ >( self )
          }
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      AsArray< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? , 1 >
      for
      $Name $( < $( $ParamName ),* > )?
      where
        $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? : Clone,
      {
        fn as_array( &self ) -> &[ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ; 1 ]
        {
          /* Safety : in case of single elemet it is safe to assume that layout is the same. It does not have to have #[repr(C)]. */
          unsafe
          {
            std::mem::transmute::< _, _ >( self )
          }
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
          &self.as_array()[ .. ]
        }
      }

      #[ cfg( feature = "make" ) ]
      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      Make1< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? >
      for
      $Name $( < $( $ParamName ),* > )?
      {
        fn make_1( _0 : $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ) -> Self
        {
          Self( _0 )
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
    /// ### Sample :: struct instead of macro.
    ///
    /// Sometimes it's sufficient to use common type instead of defining a brand new one.
    /// You may use paramtetrized struct `fundamental_data_type::Single< T >` instead of macro `fundamental_data_type::types!` if that is the case.
    ///
    /// ```rust
    /// use type_constructor::prelude::*;
    /// let x = Single::< i32 >( 13 );
    /// dbg!( x );
    /// ```
    ///

    #[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
    single Single : < T >;

  }

  pub use _single;
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

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::internal::
  {

    _single,
    Single,

  };
}