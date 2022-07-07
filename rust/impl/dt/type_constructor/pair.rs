/// Internal namespace.
pub( crate ) mod private
{
  use crate::exposed::*;

  ///
  /// Pair type constructor.
  ///
  /// Should not be used directly. Instead use macro [crate::types!].
  ///

  #[ macro_export ]
  macro_rules! _pair
  {

    // pair Pair : < T1, T2 >;

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis pair $Name : ident :
      <
        $ParamName1 : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy1x2 : path )* )?,
        $ParamName2 : ident $( : $ParamTy2x1 : ident $( :: $ParamTy2xN : ident )* $( + $ParamTy2x2 : path )* )? $(,)?
      >
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      $( #[ $Meta ] )*
      $Vis struct $Name
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?,
        $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )?,
      >
      ( pub $ParamName1, pub $ParamName2 );

      // impl
      // <
      //   $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?,
      //   $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )?,
      // >
      // From
      // <(
      //   $ParamName1,
      //   $ParamName2,
      // )>
      // for $Name< $ParamName1, $ParamName2 >
      // {
      //   #[ inline ]
      //   fn from( src : ( $ParamName1, $ParamName2 ) ) -> Self
      //   {
      //     Self( src.0, src.1 )
      //   }
      // }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?,
        $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )?,
        Into1 : Into< $ParamName1 >,
        Into2 : Into< $ParamName2 >,
      >
      From
      <(
        Into1,
        Into2,
      )>
      for $Name< $ParamName1, $ParamName2 >
      {
        #[ inline ]
        fn from( src : ( Into1, Into2 ) ) -> Self
        {
          Self( src.0.into(), src.1.into() )
        }
      }

      // xxx : make the same changes for other type constructors

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?,
        $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )?
      >
      From < $Name< $ParamName1, $ParamName2 > >
      for ( $ParamName1, $ParamName2 )
      {
        #[ inline ]
        fn from( src : $Name< $ParamName1, $ParamName2 > ) -> Self
        {
          ( src.0, src.1 )
        }
      }


      $crate::_if_make!
      {

        impl
        <
          $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?,
          $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )?
        >
        $crate::Make0
        for $Name< $ParamName1, $ParamName2 >
        where
          $ParamName1 : Default,
          $ParamName2 : Default,
        {
          #[ inline ]
          fn make_0() -> Self
          {
            Self( Default::default(), Default::default() )
          }
        }


        impl
        <
          $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?,
          $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )?
        >
        $crate::Make2 < $ParamName1, $ParamName2 >
        for $Name< $ParamName1, $ParamName2 >
        {
          #[ inline ]
          fn make_2( _0 : $ParamName1, _1 : $ParamName2 ) -> Self
          {
            Self( _0, _1 )
          }
        }

      }

      // From Pair Into Element cant be implemented because of Rust restructions.

      $crate::types!{ $( $( $Rest )* )? }
    };

    // pair Pair : < T1, T2, ... >;

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis pair $Name : ident :
      <
        $ParamName1 : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy1x3 : path )* )?,
        $ParamName2 : ident $( : $ParamTy2x1 : ident $( :: $ParamTy2xN : ident )* $( + $ParamTy2x3 : path )* )?,
        $ParamName3 : ident
      $( $Rest : tt )*
    )
    =>
    {
      compile_error!
      (
        concat!
        (
          "Parametrized element should be pair and have either two or single elements\n",
          stringify!
          (
            $( #[ $Meta ] )*
            $Vis pair $Name :
            <
              $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?,
              $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )?,
              $ParamName3
            $( $Rest )*
          )
        )
      );
    };

    // pair Pair : Element1< T1, T2, ... >, Element2< T1, T2, ... >;

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis pair $Name : ident
      :
      $TypeSplit1x1 : ident $( :: $TypeSplit1xN : ident )*
      $( < $( $( $ParamName1 : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy1x2 : path )* )? ),+ )? > )?
      ,
      $TypeSplit2x1 : ident $( :: $TypeSplit2xN : ident )*
      $( < $( $ParamName2 : ident $( : $ParamTy2x1 : ident $( :: $ParamTy2xN : ident )* $( + $ParamTy2x2 : path )* )? ),* > )?
      $(,)?
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      $( #[ $Meta ] )*
      $Vis struct $Name
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
        $( $( $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )? ),* )?
      >
      (
        pub $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        pub $TypeSplit2x1 $( :: $TypeSplit2xN )* < $( $( $ParamName2 ),* )? >,
      );

      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
        $( $( $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )? ),* )?
      >
      From
      <(
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        $TypeSplit2x1 $( :: $TypeSplit2xN )* < $( $( $ParamName2 ),* )? >,
      )>
      for $Name< $( $( $( $ParamName1 ),+ , )? )? $( $( $ParamName2 ),* )? >
      {
        #[ inline ]
        fn from
        (
          src :
          (
            $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
            $TypeSplit2x1 $( :: $TypeSplit2xN )* < $( $( $ParamName2 ),* )? >,
          )
        )
        -> Self
        {
          Self( src.0, src.1 )
        }
      }

      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
        $( $( $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )? ),* )?
      >
      From< $Name< $( $( $( $ParamName1 ),+ , )? )? $( $( $ParamName2 ),* )? > >
      for
      (
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        $TypeSplit2x1 $( :: $TypeSplit2xN )* < $( $( $ParamName2 ),* )? >,
      )
      {
        #[ inline ]
        fn from
        (
          src : $Name< $( $( $( $ParamName1 ),+ , )? )? $( $( $ParamName2 ),* )? >
        )
        -> Self
        {
          ( src.0, src.1 )
        }
      }

      $crate::_if_make!
      {

        impl
        <
          $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
          $( $( $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )? ),* )?
        >
        $crate::Make2
        <
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
          $TypeSplit2x1 $( :: $TypeSplit2xN )* < $( $( $ParamName2 ),* )? >,
        >
        for $Name< $( $( $( $ParamName1 ),+ , )? )? $( $( $ParamName2 ),* )? >
        {
          #[ inline ]
          fn make_2
          (
            _0 : $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
            _1 : $TypeSplit2x1 $( :: $TypeSplit2xN )* < $( $( $ParamName2 ),* )? >,
          ) -> Self
          {
            Self( _0, _1 )
          }
        }

      }

      $crate::types!{ $( $( $Rest )* )? }
    };

    // pair Pair : < T1 >; // homopair

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis pair $Name : ident :
      <
        $ParamName1 : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy1x2 : path )* )? $(,)?
      >
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      $( #[ $Meta ] )*
      $Vis struct $Name
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      ( pub $ParamName1, pub $ParamName1 );

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      core::ops::Deref
      for $Name< $ParamName1 >
      {
        type Target = ( $ParamName1, $ParamName1 );
        #[ inline ]
        fn deref( &self ) -> &Self::Target
        {
          #[ cfg( debug_assertions ) ]
          {
            let layout1 = core::alloc::Layout::new::< Self >();
            let layout2 = core::alloc::Layout::new::< Self::Target >();
            debug_assert_eq!( layout1, layout2 );
          }
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            core::mem::transmute::< _, _ >( self )
          }
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      core::ops::DerefMut
      for $Name< $ParamName1 >
      {
        #[ inline ]
        fn deref_mut( &mut self ) -> &mut Self::Target
        {
          #[ cfg( debug_assertions ) ]
          {
            let layout1 = core::alloc::Layout::new::< Self >();
            let layout2 = core::alloc::Layout::new::< Self::Target >();
            debug_assert_eq!( layout1, layout2 );
          }
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            core::mem::transmute::< _, _ >( self )
          }
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      From<( $ParamName1, $ParamName1 )>
      for $Name< $ParamName1 >
      {
        #[ inline ]
        fn from( src : ( $ParamName1, $ParamName1 ) ) -> Self
        {
          Self( src.0, src.1 )
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      From< $Name< $ParamName1 > >
      for ( $ParamName1, $ParamName1 )
      {
        #[ inline ]
        fn from( src : $Name< $ParamName1 > ) -> Self
        {
          ( src.0, src.1 )
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      From< [ $ParamName1 ; 2 ] >
      for $Name< $ParamName1 >
      where
        $ParamName1 : Clone,
      {
        #[ inline ]
        fn from( src : [ $ParamName1 ; 2 ] ) -> Self
        {
          Self( src[ 0 ].clone(), src[ 1 ].clone() )
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      From< $Name< $ParamName1 > >
      for [ $ParamName1 ; 2 ]
      {
        #[ inline ]
        fn from( src : $Name< $ParamName1 > ) -> Self
        {
          [ src.0, src.1 ]
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      From< &[ $ParamName1 ] >
      for $Name< $ParamName1 >
      where
        $ParamName1 : Clone,
      {
        #[ inline ]
        fn from( src : &[ $ParamName1 ] ) -> Self
        {
          debug_assert_eq!( src.len(), 2 );
          Self( src[ 0 ].clone(), src[ 1 ].clone() )
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      From< $ParamName1 >
      for $Name< $ParamName1 >
      where
        $ParamName1 : Clone,
      {
        #[ inline ]
        fn from( src : $ParamName1 ) -> Self
        {
          Self( src.clone(), src.clone() )
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      $crate::CloneAsTuple< ( $ParamName1, $ParamName1 ) >
      for $Name< $ParamName1 >
      where
        $ParamName1 : Clone,
      {
        #[ inline ]
        fn clone_as_tuple( &self ) -> ( $ParamName1, $ParamName1 )
        {
          ( self.0.clone(), self.1.clone() )
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      $crate::CloneAsArray< $ParamName1, 2 >
      for $Name< $ParamName1 >
      where
        $ParamName1 : Clone,
      {
        #[ inline ]
        fn clone_as_array( &self ) -> [ $ParamName1 ; 2 ]
        {
          [ self.0.clone(), self.1.clone() ]
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      $crate::AsTuple< ( $ParamName1 , $ParamName1 ) >
      for $Name< $ParamName1 >
      {
        #[ inline ]
        fn as_tuple( &self ) -> &( $ParamName1, $ParamName1 )
        {
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            core::mem::transmute::< &_, &( $ParamName1, $ParamName1 ) >( self )
          }
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      $crate::AsArray< $ParamName1, 2 >
      for $Name< $ParamName1 >
      {
        #[ inline ]
        fn as_array( &self ) -> &[ $ParamName1 ; 2 ]
        {
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            core::mem::transmute::< &_, &[ $ParamName1 ; 2 ] >( self )
          }
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      $crate::AsSlice< $ParamName1 >
      for $Name< $ParamName1 >
      {
        #[ inline ]
        fn as_slice( &self ) -> &[ $ParamName1 ]
        {
          &$crate::AsArray::as_array( self )[ .. ]
        }
      }

      $crate::_if_make!
      {

        impl
        <
          $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
        >
        $crate::Make0
        for $Name< $ParamName1 >
        where
          $ParamName1 : Default,
        {
          #[ inline ]
          fn make_0() -> Self
          {
            Self( Default::default(), Default::default() )
          }
        }

        impl
        <
          $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
        >
        $crate::Make1< $ParamName1 >
        for $Name< $ParamName1 >
        where
          $ParamName1 : Clone,
        {
          #[ inline ]
          fn make_1( _0 : $ParamName1 ) -> Self
          {
            Self( _0.clone(), _0.clone() )
          }
        }

        impl
        <
          $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
        >
        $crate::Make2< $ParamName1, $ParamName1 >
        for $Name< $ParamName1 >
        {
          #[ inline ]
          fn make_2( _0 : $ParamName1, _1 : $ParamName1 ) -> Self
          {
            Self( _0, _1 )
          }
        }

      }

      $crate::types!{ $( $( $Rest )* )? }
    };

    // pair Pair : Element1< T1, T2, ... >; // homopair

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis pair $Name : ident
      :
      $TypeSplit1x1 : ident $( :: $TypeSplit1xN : ident )*
      $( < $( $( $ParamName1 : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy1x2 : path )* )? ),+ )? > )?
      $(,)?
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      $( #[ $Meta ] )*
      $Vis struct $Name
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ )? )?
      >
      (
        pub $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        pub $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
      );

      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
      >
      core::ops::Deref
      for $Name
      < $( $( $( $ParamName1 ),+ )? )? >
      {
        type Target =
        (
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        );
        #[ inline ]
        fn deref( &self ) -> &Self::Target
        {
          #[ cfg( debug_assertions ) ]
          {
            let layout1 = core::alloc::Layout::new::< Self >();
            let layout2 = core::alloc::Layout::new::< Self::Target >();
            debug_assert_eq!( layout1, layout2 );
          }
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            core::mem::transmute::< _, _ >( self )
          }
        }
      }

      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
      >
      core::ops::DerefMut
      for $Name
      < $( $( $( $ParamName1 ),+ )? )? >
      {
        #[ inline ]
        fn deref_mut( &mut self ) -> &mut Self::Target
        {
          #[ cfg( debug_assertions ) ]
          {
            let layout1 = core::alloc::Layout::new::< Self >();
            let layout2 = core::alloc::Layout::new::< Self::Target >();
            debug_assert_eq!( layout1, layout2 );
          }
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            core::mem::transmute::< _, _ >( self )
          }
        }
      }

      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
      >
      From
      <(
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
      )>
      for $Name
      < $( $( $( $ParamName1 ),+ )? )? >
      {
        #[ inline ]
        fn from
        (
          src :
          (
            $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
            $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
          )
        )
        -> Self
        {
          Self( src.0, src.1 )
        }
      }

      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
      >
      From< $Name< $( $( $( $ParamName1 ),+ )? )? > >
      for
      (
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
      )
      {
        #[ inline ]
        fn from
        (
          src : $Name< $( $( $( $ParamName1 ),+ )? )? >
        )
        -> Self
        {
          ( src.0, src.1 )
        }
      }

      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
      >
      From
      <[
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > ; 2
      ]>
      for $Name
      < $( $( $( $ParamName1 ),+ )? )? >
      where
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > : Clone,
      {
        #[ inline ]
        fn from
        (
          src :
          [
            $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > ; 2
          ]
        )
        -> Self
        {
          Self( src[ 0 ].clone(), src[ 1 ].clone() )
        }
      }

      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
      >
      From< $Name< $( $( $( $ParamName1 ),+ )? )? > >
      for
      [
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > ; 2
      ]
      {
        #[ inline ]
        fn from
        (
          src : $Name< $( $( $( $ParamName1 ),+ )? )? >
        )
        -> Self
        {
          [ src.0, src.1 ]
        }
      }

      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
      >
      From
      <&[
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >
      ]>
      for $Name
      < $( $( $( $ParamName1 ),+ )? )? >
      where
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > : Clone,
      {
        #[ inline ]
        fn from
        (
          src :
          &[
            $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >
          ]
        )
        -> Self
        {
          Self( src[ 0 ].clone(), src[ 1 ].clone() )
        }
      }

      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
      >
      $crate::CloneAsTuple
      <(
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
      )>
      for $Name
      < $( $( $( $ParamName1 ),+ )? )? >
      where
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > : Clone,
      {
        #[ inline ]
        fn clone_as_tuple( &self ) ->
        (
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        )
        {
          ( self.0.clone(), self.1.clone() )
        }
      }

      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
      >
      $crate::CloneAsArray
      <
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        2
      >
      for $Name
      < $( $( $( $ParamName1 ),+ )? )? >
      where
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > : Clone,
      {
        #[ inline ]
        fn clone_as_array( &self ) ->
        [
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > ; 2
        ]
        {
          [ self.0.clone(), self.1.clone() ]
        }
      }

      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
      >
      $crate::AsTuple
      <(
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
      )>
      for $Name
      < $( $( $( $ParamName1 ),+ )? )? >
      // where
      //   $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > : Clone,
      {
        #[ inline ]
        fn as_tuple( &self ) ->
        &(
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        )
        {
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            core::mem::transmute::< _, _ >( self )
          }
        }
      }

      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
      >
      $crate::AsArray
      <
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        2
      >
      for $Name
      < $( $( $( $ParamName1 ),+ )? )? >
      // where
      //   $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > : Clone,
      {
        #[ inline ]
        fn as_array( &self ) ->
        &[
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > ; 2
        ]
        {
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            core::mem::transmute::< _, _ >( self )
          }
        }
      }

      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
      >
      $crate::AsSlice
      <
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >
      >
      for $Name
      < $( $( $( $ParamName1 ),+ )? )? >
      // where
      //   $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > : Clone,
      {
        #[ inline ]
        fn as_slice( &self ) ->
        &[
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >
        ]
        {
          &$crate::AsArray::as_array( self )[ .. ]
        }
      }

      $crate::_if_make!
      {

        impl
        <
          $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
        >
        $crate::Make1
        <
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >
        >
        for $Name< $( $( $( $ParamName1 ),+ )? )? >
        where
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > : Clone,
        {
          #[ inline ]
          fn make_1( _0 : $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > ) -> Self
          {
            Self( _0.clone(), _0.clone() )
          }
        }

        impl
        <
          $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
        >
        $crate::Make2
        <
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        >
        for $Name< $( $( $( $ParamName1 ),+ )? )? >
        where
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > : Clone,
        {
          #[ inline ]
          fn make_2
          (
            _0 : $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
            _1 : $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
          )
          -> Self
          {
            Self( _0.clone(), _1.clone() )
          }
        }

      }

      $crate::types!{ $( $( $Rest )* )? }
    };

  }

  //

  // trace_macros!( true );
  types!
  {

    ///
    /// Type constructor to wrap two types into a tuple.
    ///
    /// ### Sample
    /// ```
    /// let i32_and_f32_in_tuple = type_constructor::Pair::< i32, f32 >::from( ( 13, 13.0 ) );
    /// dbg!( i32_and_f32_in_tuple );
    /// // vec_of_i32_in_tuple = Pair( 13, 13.0 )
    /// ```
    ///

    #[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
    pub pair Pair : < T1, T2 >;

    ///
    /// Type constructor to wrap pair of the same type.
    ///
    /// ### Sample
    /// ```
    /// let two_i32_in_tuple = type_constructor::HomoPair::< i32 >::from( ( 13, 31 ) );
    /// dbg!( two_i32_in_tuple );
    /// // vec_of_i32_in_tuple = HomoPair( 13, 31 )
    /// ```
    ///

    #[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
    pub pair HomoPair : < T >;

  }
  // trace_macros!( false );

  pub use _pair;
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
  pub use super::private::
  {
    _pair,
  };
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
    Pair,
    HomoPair,
  };
}
