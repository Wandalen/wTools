/// Define a private namespace for all its items.
mod private
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

    // pub single Single : < T >;

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis single $Name : ident :
      < $ParamName : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy2 : path )* )? >
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      $( #[ $Meta ] )*
      $Vis struct $Name
      < $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      ( pub $ParamName );

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? > core::ops::Deref
      for $Name
      < $ParamName >
      {
        type Target = $ParamName;
        #[ inline ]
        fn deref( &self ) -> &Self::Target
        {
          &self.0
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? > core::ops::DerefMut
      for $Name
      < $ParamName >
      {
        #[ inline ]
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
        #[ inline ]
        fn from( src : $ParamName ) -> Self
        {
          Self( src )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      From< &$ParamName >
      for $Name
      < $ParamName >
      where
        $ParamName : Clone,
      {
        #[ inline ]
        fn from( src : &$ParamName ) -> Self
        {
          Self( src.clone() )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      From< ( $ParamName, ) >
      for $Name
      < $ParamName >
      {
        #[ inline ]
        fn from( src : ( $ParamName, ) ) -> Self
        {
          Self( src.0 )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      From< $Name< $ParamName > >
      for ( $ParamName, )
      {
        #[ inline ]
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
        #[ inline ]
        fn from( src : [ $ParamName ; 1 ] ) -> Self
        {
          Self( src[ 0 ].clone() )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      From< $Name< $ParamName > >
      for [ $ParamName ; 1 ]
      {
        #[ inline ]
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
        #[ inline ]
        fn from( src : &[ $ParamName ] ) -> Self
        {
          debug_assert_eq!( src.len(), 1 );
          Self( src[ 0 ].clone() )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      $crate::CloneAsTuple< ( $ParamName, ) >
      for $Name < $ParamName >
      where
        $ParamName : Clone,
      {
        #[ inline ]
        fn clone_as_tuple( &self ) -> ( $ParamName, )
        {
          ( self.0.clone(), )
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      $crate::CloneAsArray< $ParamName, 1 >
      for $Name < $ParamName >
      where
        $ParamName : Clone,
      {
        #[ inline ]
        fn clone_as_array( &self ) -> [ $ParamName ; 1 ]
        {
          [ self.0.clone() ; 1 ]
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      $crate::AsTuple< ( $ParamName, ) >
      for $Name < $ParamName >
      {
        #[ inline ]
        fn as_tuple( &self ) -> &( $ParamName, )
        {
          // to be deprecated
          /* Safety : in case of single elemet it is safe to assume that layout is the same. It does not have to have #[repr(C)]. */
          #[ allow( unsafe_code ) ]
          unsafe
          {
            core::mem::transmute::< _, _ >( self )
          }
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      $crate::AsArray< $ParamName, 1 >
      for $Name < $ParamName >
      {
        #[ inline ]
        fn as_array( &self ) -> &[ $ParamName ; 1 ]
        {
          // to be deprecated
          /* Safety : in case of single elemet it is safe to assume that layout is the same. It does not have to have #[repr(C)]. */
          #[ allow( unsafe_code ) ]
          unsafe
          {
            core::mem::transmute::< _, _ >( self )
          }
        }
      }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      $crate::AsSlice< $ParamName >
      for $Name < $ParamName >
      {
        #[ inline ]
        fn as_slice( &self ) -> &[ $ParamName ]
        {
          &$crate::AsArray::as_array( self )[ .. ]
        }
      }

      // $crate::_if_from!
      // {
      //   impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      //   $crate::From_0
      //   for $Name < $ParamName >
      //   where $ParamName : Default
      //   {
      //     #[ inline ]
      //     fn from_0() -> Self
      //     {
      //       Self( Default::default() )
      //     }
      //   }
      //
      //
      //   impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      //   $crate::From_1< $ParamName >
      //   for $Name < $ParamName >
      //   {
      //     #[ inline ]
      //     fn from_1( _0 : $ParamName ) -> Self
      //     {
      //       Self( _0 )
      //     }
      //   }
      // }

      // From Single Into Element cant be implemented because of Rust restrictions.

      $crate::types!{ $( $( $Rest )* )? }
    };

    // pub single Single : < T1, ... >;

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis single $Name : ident :
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
            $Vis single $Name :
            < $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ,
            $( $Rest )*
          )
        )
      );
    };

    // pub single Single : Element< T1, T2, ... >;

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis single $Name : ident : $TypeSplit1 : ident $( :: $TypeSplitN : ident )*
      $( < $( $ParamName : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy2 : path )* )? ),* > )?
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      $( #[ $Meta ] )*
      $Vis struct $Name
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      ( pub $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? );

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      core::ops::Deref
      for $Name
      $( < $( $ParamName ),* > )?
      {
        type Target = $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?;
        #[ inline ]
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
        #[ inline ]
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
        #[ inline ]
        fn from( src : $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ) -> Self
        {
          Self( src )
        }
      }

      impl
      < __FromRef $( , $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* )? >
      From
      < &__FromRef >
      for $Name
      $( < $( $ParamName ),* > )?
      where
        __FromRef : Clone,
        Self : From< __FromRef >,
      {
        #[ inline ]
        fn from( src : &__FromRef ) -> Self
        {
          From::from( ( *src ).clone() )
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      From
      < $Name $( < $( $ParamName ),* > )? >
      for $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?
      {
        #[ inline ]
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
        #[ inline ]
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
        #[ inline ]
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
        #[ inline ]
        fn from( src : &[ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ] ) -> Self
        {
          debug_assert_eq!( src.len(), 1 );
          Self( src[ 0 ].clone() )
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      $crate::CloneAsTuple< ( $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?, ) >
      for
      $Name $( < $( $ParamName ),* > )?
      where
        $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? : Clone,
      {
        #[ inline ]
        fn clone_as_tuple( &self ) -> ( $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?, )
        {
          ( self.0.clone(), )
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      $crate::CloneAsArray< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? , 1 >
      for
      $Name $( < $( $ParamName ),* > )?
      where
        $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? : Clone,
      {
        #[ inline ]
        fn clone_as_array( &self ) -> [ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ; 1 ]
        {
          [ self.0.clone() ]
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      $crate::AsTuple< ( $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?, ) >
      for
      $Name $( < $( $ParamName ),* > )?
      {
        #[ inline ]
        fn as_tuple( &self ) -> &( $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?, )
        {
          // to be deprecated
          /* Safety : in case of single elemet it is safe to assume that layout is the same. It does not have to have #[repr(C)]. */
          #[ allow( unsafe_code ) ]
          unsafe
          {
            core::mem::transmute::< _, _ >( self )
          }
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      $crate::AsArray< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? , 1 >
      for
      $Name $( < $( $ParamName ),* > )?
      {
        #[ inline ]
        fn as_array( &self ) -> &[ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ; 1 ]
        {
          // to be deprecated
          /* Safety : in case of single elemet it is safe to assume that layout is the same. It does not have to have #[repr(C)]. */
          #[ allow( unsafe_code ) ]
          unsafe
          {
            core::mem::transmute::< _, _ >( self )
          }
        }
      }

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      $crate::AsSlice< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? >
      for
      $Name $( < $( $ParamName ),* > )?
      {
        #[ inline ]
        fn as_slice( &self ) -> &[ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ]
        {
          &$crate::AsArray::as_array( self )[ .. ]
        }
      }

      $crate::_if_from!
      {
        impl
        $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
        $crate::From_1< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? >
        for
        $Name $( < $( $ParamName ),* > )?
        {
          #[ inline ]
          fn from_1( _0 : $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ) -> Self
          {
            Self( _0 )
          }
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
    /// ### Basic Use Case :: struct instead of macro.
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
    pub single Single : < T >;

  }

  pub use _single;
}

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use orphan::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use prelude::*;
  #[ doc( inline ) ]
  pub use private::
  {
    _single,
  };
}


/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  #[ doc( inline ) ]
  pub use private::
  {
    Single,
  };
}
