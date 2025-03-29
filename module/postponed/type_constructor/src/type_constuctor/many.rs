/// Define a private namespace for all its items.
mod private
{
  use crate::exposed::*;

  #[ cfg( feature = "no_std" ) ]
  extern crate core;
  #[ cfg( all( feature = "no_std", feature = "use_alloc" ) ) ]
  extern crate alloc;

  #[ cfg( any( not( feature = "no_std" ), not( feature = "use_alloc" ) ) ) ]
  /// Alias of Vec for internal usage.
  pub use std::vec::Vec as _Vec;
  #[ cfg( all( feature = "no_std", feature = "use_alloc" ) ) ]
  /// Alias of Vec for internal usage.
  pub use alloc::vec::Vec as _Vec;

  /// Alias of Vec for internal usage.
  #[ macro_export ]
  macro_rules! _vec
  {
    ( $( $Rest:tt )* )
    =>
    {{
      let result;
      #[ cfg( any( not( feature = "no_std" ), not( feature = "use_alloc" ) ) ) ]
      {
        result = std::vec!( $( $Rest )* );
      }
      #[ cfg( all( feature = "no_std", feature = "use_alloc" ) ) ]
      {
        extern crate alloc;
        result = alloc::vec!( $( $Rest )* );
      }
      result
    }}
  }

  ///
  /// Type constructor of many.
  ///
  /// Should not be used directly. Instead use macro [crate::types!].
  /// Type constructor `many` is available if eiter feature `use_std` or feature `use_alloc` is enabled. Also feature `many` should be enabled.
  ///

  #[ macro_export ]
  macro_rules! _many
  {

    // many Many : < T >;

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis many $Name : ident :
      < $ParamName : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy2 : path )* )? >
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      $( #[ $Meta ] )*
      $Vis struct $Name
      < $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      ( pub $crate::_Vec< $ParamName > );

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? > core::ops::Deref
      for $Name
      < $ParamName >
      {
        type Target = $crate::_Vec< $ParamName >;
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

      // impl< Collection > From< Collection > for Polygons
      // where
      //   Collection : IntoIterator< Item = Polygon >,
      // {
      //   fn from( src : Collection ) -> Self
      //   {
      //     Self( src.into_iter().collect::< Vec< Polygon > >() )
      //   }
      // }
      // zzz

      impl< Collection, IntoT, $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      From< Collection >
      for $Name< $ParamName >
      where
        Collection : IntoIterator< Item = IntoT >,
        IntoT : Into< $ParamName >,
      {
        #[ inline ]
        fn from( src : Collection ) -> Self
        {
          Self( src.into_iter().map( | e | e.into() ).collect::< $crate::_Vec< $ParamName > >() )
        }
      }

//       impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
//       From< $ParamName >
//       for $Name< $ParamName >
//       {
//         #[ inline ]
//         fn from( src : $ParamName ) -> Self
//         {
//           Self( $crate::_vec![ src ] )
//         }
//       }
//
//       impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
//       From< &$ParamName >
//       for $Name
//       < $ParamName >
//       where
//         $ParamName : Clone,
//       {
//         #[ inline ]
//         fn from( src : &$ParamName ) -> Self
//         {
//           Self( $crate::_vec![ src.clone() ] )
//         }
//       }
//
//       impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
//       From< ( $ParamName, ) >
//       for $Name
//       < $ParamName >
//       {
//         #[ inline ]
//         fn from( src : ( $ParamName, ) ) -> Self
//         {
//           Self( $crate::_vec![ src.0 ] )
//         }
//       }
//
//       impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )?, const N : usize >
//       From< [ $ParamName ; N ] >
//       for $Name
//       < $ParamName >
//       {
//         #[ inline ]
//         fn from( src : [ $ParamName ; N ] ) -> Self
//         {
//           Self( $crate::_Vec::from( src ) )
//         }
//       }
//
//       impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
//       From< &[ $ParamName ] >
//       for $Name
//       < $ParamName >
//       where
//         $ParamName : Clone,
//       {
//         #[ inline ]
//         fn from( src : &[ $ParamName ] ) -> Self
//         {
//           Self( $crate::_Vec::from( src ) )
//         }
//       }

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
      $crate::AsSlice< $ParamName >
      for $Name < $ParamName >
      {
        #[ inline ]
        fn as_slice( &self ) -> &[ $ParamName ]
        {
          &self[ .. ]
        }
      }

      $crate::_if_from!
      {

//         impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
//         $crate::From_0
//         for $Name < $ParamName >
//         {
//           #[ inline ]
//           fn from_0() -> Self
//           {
//             Self( $crate::_Vec::new() )
//           }
//         }
//
//         impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
//         $crate::From_1< $ParamName >
//         for $Name < $ParamName >
//         {
//           #[ inline ]
//           fn from_1( _0 : $ParamName ) -> Self
//           {
//             Self( $crate::_vec![ _0 ] )
//           }
//         }

//         impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
//         $crate::From_2< $ParamName, $ParamName >
//         for $Name < $ParamName >
//         {
//           #[ inline ]
//           fn from_2( _0 : $ParamName, _1 : $ParamName ) -> Self
//           {
//             Self( $crate::_vec![ _0, _1 ] )
//           }
//         }
//
//         impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? >
//         $crate::From_3< $ParamName, $ParamName, $ParamName >
//         for $Name < $ParamName >
//         {
//           #[ inline ]
//           fn from_3( _0 : $ParamName, _1 : $ParamName, _2 : $ParamName ) -> Self
//           {
//             Self( $crate::_vec![ _0, _1, _2 ] )
//           }
//         }

      }

      $crate::types!{ $( $( $Rest )* )? }
    };

    // many Many : < T1, ... >;

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis many $Name : ident :
      < $ParamName : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy2 : path )* )? ,
      $( $Rest : tt )*
    )
    =>
    {
      compile_error!
      (
        concat!
        (
          "Parametrized element should be single, because Many has only one element\n",
          stringify!
          (
            $( #[ $Meta ] )*
            $Vis many $Name :
            < $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ,
            $( $Rest )*
          )
        )
      );
    };

    // many Many : Element< T1, T2, ... >;

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis many $Name : ident : $TypeSplit1 : ident $( :: $TypeSplitN : ident )*
      $( < $( $ParamName : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy2 : path )* )? ),* > )?
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      $( #[ $Meta ] )*
      $Vis struct $Name
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      ( pub $crate::_Vec< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? > );

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      core::ops::Deref
      for $Name
      $( < $( $ParamName ),* > )?
      {
        type Target = $crate::_Vec< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? >;
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
      < Collection, Item, $( $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* )? >
      From< Collection >
      for $Name
      $( < $( $ParamName ),* > )?
      where
        Collection : IntoIterator< Item = Item >,
        Item : Into< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? >,
      {
        #[ inline ]
        fn from( src : Collection ) -> Self
        {
          let src2 = src
          .into_iter()
          .map( | e | e.into() )
          .collect::< $crate::_Vec< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? > >();
          Self( src2 )
        }
      }

      // impl
      // < 'a, Collection, $( $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* )? >
      // From< Collection >
      // for $Name
      // $( < $( $ParamName ),* > )?
      // where
      //   Collection : IntoIterator< Item = &'a $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? >,
      // {
      //   #[ inline ]
      //   fn from( src : Collection ) -> Self
      //   {
      //     let src2 = src
      //     .into_iter()
      //     .map( | e | *e )
      //     .collect::< $crate::_Vec< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? > >();
      //     Self( src2 )
      //   }
      // }

      // yyy
//       impl
//       $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
//       From
//       < $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? >
//       for $Name
//       $( < $( $ParamName ),* > )?
//       {
//         #[ inline ]
//         fn from( src : $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ) -> Self
//         {
//           Self( $crate::_vec![ src ] )
//         }
//       }
//
//       impl
//       < __FromRef $( , $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* )? >
//       From
//       < &__FromRef >
//       for $Name
//       $( < $( $ParamName ),* > )?
//       where
//         __FromRef : Clone,
//         Self : From< __FromRef >,
//       {
//         #[ inline ]
//         fn from( src : &__FromRef ) -> Self
//         {
//           From::from( ( *src ).clone() )
//         }
//       }
//
//       impl
//       $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
//       From
//       < ( $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? , ) >
//       for $Name
//       $( < $( $ParamName ),* > )?
//       {
//         #[ inline ]
//         fn from( src : ( $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? , ) ) -> Self
//         {
//           Self( $crate::_vec![ src.0 ] )
//         }
//       }
//
//       impl
//       < $( $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? , )* )? const N : usize >
//       From
//       < [ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ; N ] >
//       for $Name
//       $( < $( $ParamName ),* > )?
//       {
//         #[ inline ]
//         fn from( src : [ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ; N ] ) -> Self
//         {
//           Self( $crate::_Vec::from( src ) )
//         }
//       }
//
//       impl
//       $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
//       From
//       < &[ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ] >
//       for $Name
//       $( < $( $ParamName ),* > )?
//       where
//         $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? : Clone,
//       {
//         #[ inline ]
//         fn from( src : &[ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ] ) -> Self
//         {
//           Self( $crate::_Vec::from( src ) )
//         }
//       }
      // yyy

      impl
      $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
      $crate::AsSlice< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? >
      for
      $Name $( < $( $ParamName ),* > )?
      {
        #[ inline ]
        fn as_slice( &self ) -> &[ $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? ]
        {
          &self[ .. ]
        }
      }

      $crate::_if_from!
      {

        impl
        $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
        $crate::From_0
        for
        $Name $( < $( $ParamName ),* > )?
        {
          #[ inline ]
          fn from_0() -> Self
          {
            Self( $crate::_Vec::< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? >::new() )
          }
        }

        impl
        $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
        $crate::From_1< $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )? >
        for
        $Name $( < $( $ParamName ),* > )?
        {
          #[ inline ]
          fn from_1
          (
            _0 : $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
          )
          -> Self
          {
            Self( $crate::_vec![ _0 ] )
          }
        }

        impl
        $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
        $crate::From_2
        <
          $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
          $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
        >
        for
        $Name $( < $( $ParamName ),* > )?
        {
          #[ inline ]
          fn from_2
          (
            _0 : $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
            _1 : $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
          )
          -> Self
          {
            Self( $crate::_vec![ _0, _1 ] )
          }
        }

        impl
        $( < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > )?
        $crate::From_3
        <
          $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
          $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
          $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
        >
        for
        $Name $( < $( $ParamName ),* > )?
        {
          #[ inline ]
          fn from_3
          (
            _0 : $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
            _1 : $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
            _2 : $TypeSplit1 $( :: $TypeSplitN )* $( < $( $ParamName ),* > )?,
          )
          -> Self
          {
            Self( $crate::_vec![ _0, _1, _2 ] )
          }
        }

      }

      $crate::types!{ $( $( $Rest )* )? }
    };

  }

  types!
  {

    ///
    /// Type constructor to wrap a vector.
    ///
    /// ### Basic use-case.
    /// ```rust
    /// let vec_of_i32_in_tuple = type_constructor::Many::< i32 >::from( [ 1, 2, 3 ] );
    /// dbg!( vec_of_i32_in_tuple );
    /// // vec_of_i32_in_tuple = Many([ 1, 2, 3 ])
    /// ```
    ///

    #[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
    pub many Many : < T >;

  }

  pub use _vec;
  pub use _many;
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
    _many,
    _vec,
    _Vec,
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
    Many,
  };
}
