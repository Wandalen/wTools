/// Internal namespace.
mod internal
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
      pair $Name : ident :
      <
        $ParamName1 : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy1x2 : path )* )?,
        $ParamName2 : ident $( : $ParamTy2x1 : ident $( :: $ParamTy2xN : ident )* $( + $ParamTy2x2 : path )* )? $(,)?
      >
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      $( #[ $Meta ] )*
      pub struct $Name
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?,
        $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )?
      >
      ( pub $ParamName1, pub $ParamName2 );

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?,
        $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )?
      >
      From
      <(
        $ParamName1,
        $ParamName2,
      )>
      for $Name< $ParamName1, $ParamName2 >
      {
        fn from( src : ( $ParamName1, $ParamName2 ) ) -> Self
        {
          Self( src.0, src.1 )
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?,
        $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )?
      >
      From < $Name< $ParamName1, $ParamName2 > >
      for ( $ParamName1, $ParamName2 )
      {
        fn from( src : $Name< $ParamName1, $ParamName2 > ) -> Self
        {
          ( src.0, src.1 )
        }
      }

      #[ cfg( feature = "make" ) ]
      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?,
        $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )?
      >
      Make0
      for $Name< $ParamName1, $ParamName2 >
      where
        $ParamName1 : Default,
        $ParamName2 : Default,
      {
        fn make_0() -> Self
        {
          Self( Default::default(), Default::default() )
        }
      }

      #[ cfg( feature = "make" ) ]
      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?,
        $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )?
      >
      Make2 < $ParamName1, $ParamName2 >
      for $Name< $ParamName1, $ParamName2 >
      {
        fn make_2( _0 : $ParamName1, _1 : $ParamName2 ) -> Self
        {
          Self( _0, _1 )
        }
      }

      // From Pair Into Element cant be implemented because of Rust restructions.

      $crate::types!{ $( $( $Rest )* )? }
    };

    // pair Pair : < T1, T2, ... >;

    (
      $( #[ $Meta : meta ] )*
      pair $Name : ident :
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
            pair $Name :
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
      pair $Name : ident
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
      pub struct $Name
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
        fn from
        (
          src : $Name< $( $( $( $ParamName1 ),+ , )? )? $( $( $ParamName2 ),* )? >
        )
        -> Self
        {
          ( src.0, src.1 )
        }
      }

      #[ cfg( feature = "make" ) ]
      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
        $( $( $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )? ),* )?
      >
      Make2
      <
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        $TypeSplit2x1 $( :: $TypeSplit2xN )* < $( $( $ParamName2 ),* )? >,
      >
      for $Name< $( $( $( $ParamName1 ),+ , )? )? $( $( $ParamName2 ),* )? >
      {
        fn make_2
        (
          _0 : $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
          _1 : $TypeSplit2x1 $( :: $TypeSplit2xN )* < $( $( $ParamName2 ),* )? >,
        ) -> Self
        {
          Self( _0, _1 )
        }
      }

      $crate::types!{ $( $( $Rest )* )? }
    };

    // pair Pair : < T1 >; // homopair

    (
      $( #[ $Meta : meta ] )*
      pair $Name : ident :
      <
        $ParamName1 : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy1x2 : path )* )? $(,)?
      >
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      $( #[ $Meta ] )*
      pub struct $Name
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
        fn deref( &self ) -> &Self::Target
        {
          #[ cfg( debug_assertions ) ]
          {
            let layout1 = std::alloc::Layout::new::< Self >();
            let layout2 = std::alloc::Layout::new::< Self::Target >();
            debug_assert_eq!( layout1, layout2 );
          }
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            std::mem::transmute::< _, _ >( self )
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
        fn deref_mut( &mut self ) -> &mut Self::Target
        {
          #[ cfg( debug_assertions ) ]
          {
            let layout1 = std::alloc::Layout::new::< Self >();
            let layout2 = std::alloc::Layout::new::< Self::Target >();
            debug_assert_eq!( layout1, layout2 );
          }
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            std::mem::transmute::< _, _ >( self )
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
        fn from( src : $ParamName1 ) -> Self
        {
          Self( src.clone(), src.clone() )
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      CloneAsTuple< ( $ParamName1, $ParamName1 ) >
      for $Name< $ParamName1 >
      where
        $ParamName1 : Clone,
      {
        fn clone_as_tuple( &self ) -> ( $ParamName1, $ParamName1 )
        {
          ( self.0.clone(), self.1.clone() )
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      CloneAsArray< $ParamName1, 2 >
      for $Name< $ParamName1 >
      where
        $ParamName1 : Clone,
      {
        fn clone_as_array( &self ) -> [ $ParamName1 ; 2 ]
        {
          [ self.0.clone(), self.1.clone() ]
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      AsTuple< ( $ParamName1 , $ParamName1 ) >
      for $Name< $ParamName1 >
      {
        fn as_tuple( &self ) -> &( $ParamName1, $ParamName1 )
        {
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            std::mem::transmute::< &_, &( $ParamName1, $ParamName1 ) >( self )
          }
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      AsArray< $ParamName1, 2 >
      for $Name< $ParamName1 >
      {
        fn as_array( &self ) -> &[ $ParamName1 ; 2 ]
        {
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            std::mem::transmute::< &_, &[ $ParamName1 ; 2 ] >( self )
          }
        }
      }

      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      AsSlice< $ParamName1 >
      for $Name< $ParamName1 >
      {
        fn as_slice( &self ) -> &[ $ParamName1 ]
        {
          &self.as_array()[ .. ]
        }
      }

      #[ cfg( feature = "make" ) ]
      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      Make0
      for $Name< $ParamName1 >
      where
        $ParamName1 : Default,
      {
        fn make_0() -> Self
        {
          Self( Default::default(), Default::default() )
        }
      }

      #[ cfg( feature = "make" ) ]
      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      Make1< $ParamName1 >
      for $Name< $ParamName1 >
      where
        $ParamName1 : Clone,
      {
        fn make_1( _0 : $ParamName1 ) -> Self
        {
          Self( _0.clone(), _0.clone() )
        }
      }

      #[ cfg( feature = "make" ) ]
      impl
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      Make2< $ParamName1, $ParamName1 >
      for $Name< $ParamName1 >
      {
        fn make_2( _0 : $ParamName1, _1 : $ParamName1 ) -> Self
        {
          Self( _0, _1 )
        }
      }

      $crate::types!{ $( $( $Rest )* )? }
    };

    // pair Pair : Element1< T1, T2, ... >; // homopair

    (
      $( #[ $Meta : meta ] )*
      pair $Name : ident
      :
      $TypeSplit1x1 : ident $( :: $TypeSplit1xN : ident )*
      $( < $( $( $ParamName1 : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy1x2 : path )* )? ),+ )? > )?
      $(,)?
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      $( #[ $Meta ] )*
      pub struct $Name
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
        fn deref( &self ) -> &Self::Target
        {
          #[ cfg( debug_assertions ) ]
          {
            let layout1 = std::alloc::Layout::new::< Self >();
            let layout2 = std::alloc::Layout::new::< Self::Target >();
            debug_assert_eq!( layout1, layout2 );
          }
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            std::mem::transmute::< _, _ >( self )
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
        fn deref_mut( &mut self ) -> &mut Self::Target
        {
          #[ cfg( debug_assertions ) ]
          {
            let layout1 = std::alloc::Layout::new::< Self >();
            let layout2 = std::alloc::Layout::new::< Self::Target >();
            debug_assert_eq!( layout1, layout2 );
          }
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            std::mem::transmute::< _, _ >( self )
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
      CloneAsTuple
      <(
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
      )>
      for $Name
      < $( $( $( $ParamName1 ),+ )? )? >
      where
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > : Clone,
      {
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
      CloneAsArray
      <
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        2
      >
      for $Name
      < $( $( $( $ParamName1 ),+ )? )? >
      where
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > : Clone,
      {
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
      AsTuple
      <(
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
      )>
      for $Name
      < $( $( $( $ParamName1 ),+ )? )? >
      where
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > : Clone,
      {
        fn as_tuple( &self ) ->
        &(
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        )
        {
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            std::mem::transmute::< _, _ >( self )
          }
        }
      }

      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
      >
      AsArray
      <
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        2
      >
      for $Name
      < $( $( $( $ParamName1 ),+ )? )? >
      where
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > : Clone,
      {
        fn as_array( &self ) ->
        &[
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > ; 2
        ]
        {
          /* Safety : in case of homopair it is safe to assume that layout is the same. Homopair does not have to have #[repr(C)]. */
          unsafe
          {
            std::mem::transmute::< _, _ >( self )
          }
        }
      }

      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
      >
      AsSlice
      <
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >
      >
      for $Name
      < $( $( $( $ParamName1 ),+ )? )? >
      where
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > : Clone,
      {
        fn as_slice( &self ) ->
        &[
          $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >
        ]
        {
          &self.as_array()[ .. ]
        }
      }

      #[ cfg( feature = "make" ) ]
      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
      >
      Make1
      <
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >
      >
      for $Name< $( $( $( $ParamName1 ),+ )? )? >
      where
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > : Clone,
      {
        fn make_1( _0 : $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > ) -> Self
        {
          Self( _0.clone(), _0.clone() )
        }
      }

      #[ cfg( feature = "make" ) ]
      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
      >
      Make2
      <
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
      >
      for $Name< $( $( $( $ParamName1 ),+ )? )? >
      where
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? > : Clone,
      {
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

      $crate::types!{ $( $( $Rest )* )? }
    };

  }

  types!
  {

    ///
    /// Type constructor to wrap two types into a tuple.
    ///

    #[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
    pair Pair : < T1, T2 >;

    ///
    /// Type constructor to wrap pair of the same type.
    ///

    #[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
    pair HomoPair : < T >;

    // xxx : samples

  }

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
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::internal::
  {

    _pair,

    Pair,
    HomoPair,

  };
}
