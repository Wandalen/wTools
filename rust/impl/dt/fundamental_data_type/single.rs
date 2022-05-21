/// Internal namespace.
mod internal
{
  use crate::exposed::*;

  ///
  /// Type constructor to define tuple wrapping a given type.
  ///
  /// Quite often you need to wrap a given type into new one.
  /// For example if orphan rule became and obstacle one should introduce a new type wrapping foreing one.
  /// Type constructr `types!` does exaclty that and auto-implement traits From, Into and Deref for the constructed type.
  ///
  /// ### Sample :: single line single.
  ///
  /// To define your own single use macro `types!`. Single-line definition looks like that.
  ///
  /// ```rust
  /// use fundamental_data_type::prelude::*;
  /// types!( single MySingle : i32 );
  /// let x = MySingle( 13 );
  /// println!( "x : {}", x.0 );
  /// ```
  ///
  /// It gererates code:
  ///
  /// ```rust
  /// use fundamental_data_type::prelude::*;
  ///
  /// pub struct MySingle( pub i32 );
  ///
  /// impl core::ops::Deref for MySingle
  /// {
  ///   type Target = i32;
  ///   fn deref( &self ) -> &Self::Target
  ///   {
  ///     &self.0
  ///   }
  /// }
  /// impl From< i32 > for MySingle
  /// {
  ///   fn from( src : i32 ) -> Self
  ///   {
  ///     Self( src )
  ///   }
  /// }
  /// impl From< MySingle > for i32
  /// {
  ///   fn from( src : MySingle ) -> Self
  ///   {
  ///     src.0
  ///   }
  /// }
  ///
  /// let x = MySingle( 13 );
  /// println!( "x : {}", x.0 );
  /// ```
  ///
  /// ### Sample :: single with derives and attributes.
  ///
  /// It's possible to define attributes as well as derives.
  ///
  /// ```rust
  /// use fundamental_data_type::prelude::*;
  /// types!
  /// {
  ///   /// This is also attribute and macro understands it.
  ///   #[ derive( Debug ) ]
  ///   single MySingle : i32;
  /// }
  /// let x = MySingle( 13 );
  /// dbg!( x );
  /// ```
  ///
  /// It gererates code:
  ///
  /// ```rust
  /// use fundamental_data_type::prelude::*;
  ///
  /// /// This is also attribute and macro understands it.
  /// #[ derive( Debug ) ]
  /// pub struct MySingle( pub i32 );
  ///
  /// impl core::ops::Deref for MySingle
  /// {
  ///   type Target = i32;
  ///   fn deref( &self ) -> &Self::Target
  ///   {
  ///     &self.0
  ///   }
  /// }
  /// impl From< i32 > for MySingle
  /// {
  ///   fn from( src : i32 ) -> Self
  ///   {
  ///     Self( src )
  ///   }
  /// }
  /// impl From< MySingle > for i32
  /// {
  ///   fn from( src : MySingle ) -> Self
  ///   {
  ///     src.0
  ///   }
  /// }
  ///
  /// let x = MySingle( 13 );
  /// dbg!( x );
  /// ```
  ///
  /// ### Sample :: single with struct instead of macro.
  ///
  /// Sometimes it's sufficient to use common type instead of defining a brand new.
  /// You may use paramtetrized struct `Single< T >` instead of macro `types!` if that is the case.
  ///
  /// ```rust
  /// use fundamental_data_type::prelude::*;
  /// let x = Single::< i32 >( 13 );
  /// dbg!( x );
  /// ```
  ///
  /// ### Sample :: single with parametrized element.
  ///
  /// Element of tuple could be parametrized.
  ///
  /// ```rust
  /// use fundamental_data_type::prelude::*;
  /// types!
  /// {
  ///   #[ derive( Debug ) ]
  ///   single MySingle : std::sync::Arc< T : Copy >;
  /// }
  /// let x = MySingle( std::sync::Arc::new( 13 ) );
  /// dbg!( x );
  /// ```
  ///
  /// It gererates code:
  ///
  /// ```rust
  /// use fundamental_data_type::*;
  ///
  /// #[ derive( Debug ) ]
  /// pub struct MySingle< T : Copy >( pub std::sync::Arc< T > );
  ///
  /// impl<T: Copy> core::ops::Deref for MySingle< T >
  /// {
  ///   type Target = std::sync::Arc< T >;
  ///   fn deref( &self ) -> &Self::Target
  ///   {
  ///     &self.0
  ///   }
  /// }
  /// impl< T : Copy > From< std::sync::Arc< T > > for MySingle< T >
  /// {
  ///   fn from( src : std::sync::Arc<T>) -> Self {
  ///     Self( src )
  ///   }
  /// }
  /// impl< T : Copy > From< MySingle< T > > for std::sync::Arc< T >
  /// {
  ///   fn from(src: MySingle<T>) -> Self
  ///   {
  ///     src.0
  ///   }
  /// }
  ///
  /// let x = MySingle( std::sync::Arc::new( 13 ) );
  /// ```
  ///
  /// ### Sample :: single with parametrized tuple.
  ///
  /// Instead of parametrizing the element it's possible to define a parametrized tuple.
  ///
  ///
  /// ```rust
  /// use fundamental_data_type::prelude::*;
  /// types!
  /// {
  ///   #[ derive( Debug ) ]
  ///   single MySingle : < T : Copy >;
  /// }
  /// let x = MySingle( 13 );
  /// dbg!( x );
  /// ```
  ///
  /// It gererates code:
  ///
  /// ```rust
  /// #[ derive( Debug ) ]
  /// pub struct MySingle< T : Copy >( pub T );
  ///
  /// impl< T : Copy > core::ops::Deref
  /// for MySingle< T >
  /// {
  ///   type Target = T;
  ///   fn deref( &self ) -> &Self::Target
  ///   {
  ///     &self.0
  ///   }
  /// }
  ///
  /// impl< T : Copy > From< T >
  /// for MySingle< T >
  /// {
  ///   fn from( src : T ) -> Self
  ///   {
  ///     Self( src )
  ///   }
  /// }
  ///
  /// let x = MySingle( 13 );
  /// dbg!( 13 );
  /// ```
  ///

  // #[ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

  #[ macro_export ]
  macro_rules! types
  {

    // No more.

    (
    )
    =>
    {
    };

    // No more.

    (
      ;
    )
    =>
    {
    };

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

      impl< $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? > From< $ParamName >
      for $Name
      < $ParamName >
      {
        fn from( src : $ParamName ) -> Self
        {
          Self( src )
        }
      }

      // From Single Into Element cant be implemented because of Rust restructions.

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
      < $( $ParamName : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy2 : path )* )? ),* >
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      $( #[ $Meta ] )*
      pub struct $Name
      < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* >
      ( pub $TypeSplit1 $( :: $TypeSplitN )* < $( $ParamName ),* > );

      impl< $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* > core::ops::Deref
      for $Name
      < $( $ParamName ),* >
      {
        type Target = $TypeSplit1 $( :: $TypeSplitN )* < $( $ParamName ),* >;
        fn deref( &self ) -> &Self::Target
        {
          &self.0
        }
      }

      impl< $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* >
      From< $TypeSplit1 $( :: $TypeSplitN )* < $( $ParamName ),* > >
      for $Name
      < $( $ParamName ),* >
      {
        fn from( src : $TypeSplit1 $( :: $TypeSplitN )* < $( $ParamName ),* > ) -> Self
        {
          Self( src )
        }
      }

      impl< $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? ),* >
      From< $Name< $( $ParamName ),* > >
      for $TypeSplit1 $( :: $TypeSplitN )* < $( $ParamName ),* >
      {
        fn from( src : $Name< $( $ParamName ),* > ) -> Self
        {
          src.0
        }
      }

      $crate::types!{ $( $( $Rest )* )? }
    };

    // single Single : Element;

    (
      $( #[ $Meta : meta ] )*
      single $Name : ident : $TypeSplit1 : ident $( :: $TypeSplitN : ident )*
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      $crate::types!
      (
        $( #[ $Meta ] )*
        single $Name : $TypeSplit1 $( :: $TypeSplitN )* <>;
        // $( ; $( $Rest )* )?
      );
      $crate::types!{ $( $( $Rest )* )? }
    };

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

      // impl
      // <
      //   $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?,
      //   $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )?
      // >
      // core::ops::Deref
      // for $Name< $ParamName1, $ParamName2 >
      // {
      //   type Target = ( $ParamName1, $ParamName2 );
      //   fn deref( &self ) -> &Self::Target
      //   {
      //     &( self.0, self.1 )
      //   }
      // }

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
          "Parametrized element should be pair, because Pair has exactly two elements\n",
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
        $( $( $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )? )* )?
      >
      (
        pub $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        pub $TypeSplit2x1 $( :: $TypeSplit2xN )* < $( $( $ParamName2 ),* )? >,
      );

      // xxx : add version for single type
      // impl
      // <
      //   $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
      //   $(, $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )? )*
      // >
      // core::ops::Deref
      // for $Name
      // < $( $( $( $ParamName1 ),+ , )? )? $( $( $ParamName2 )* )? >
      // {
      //   type Target =
      //   (
      //     $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $ParamName1 ),* >,
      //     $TypeSplit2x1 $( :: $TypeSplit2xN )* < $( $ParamName2 ),* >,
      //   );
      //   fn deref( &self ) -> &Self::Target
      //   {
      //     // let layout1 = std::alloc::Layout::new::< Self >();
      //     // let layout2 = std::alloc::Layout::new::< Self::Target >();
      //     // dbg!( layout1 );
      //     // dbg!( layout2 );
      //     unsafe
      //     {
      //       std::mem::transmute::< &Self, &Self::Target >( self )
      //     }
      //   }
      // }

      impl
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
        $( $( $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )? )* )?
      >
      From
      <(
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        $TypeSplit2x1 $( :: $TypeSplit2xN )* < $( $( $ParamName2 ),* )? >,
      )>
      for $Name
      < $( $( $( $ParamName1 ),+ , )? )? $( $( $ParamName2 )* )? >
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
        $( $( $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )? )* )?
      >
      From< $Name< $( $( $( $ParamName1 ),+ , )? )? $( $( $ParamName2 )* )? > >
      for
      (
        $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        $TypeSplit2x1 $( :: $TypeSplit2xN )* < $( $( $ParamName2 ),* )? >,
      )
      {
        fn from
        (
          src : $Name< $( $( $( $ParamName1 ),+ , )? )? $( $( $ParamName2 )* )? >
        )
        -> Self
        {
          ( src.0, src.1 )
        }
      }

      $crate::types!{ $( $( $Rest )* )? }
    };

    // xxx : cover test case of lack of args

//     // pair Pair : Element1, Element2;
//
//     (
//       $( #[ $Meta : meta ] )*
//       pair $Name : ident :
//         $TypeSplit1x1 : ident $( :: $TypeSplit1xN : ident )*,
//         $TypeSplit2x1 : ident $( :: $TypeSplit2xN : ident )* $(,)?
//       $( ; $( $Rest : tt )* )?
//     )
//     =>
//     {
//       $crate::types!
//       (
//         $( #[ $Meta ] )*
//         pair $Name :
//           $TypeSplit1x1 $( :: $TypeSplit1xN )*<>,
//           $TypeSplit2x1 $( :: $TypeSplit2xN )*<>;
//       );
//       $crate::types!{ $( $( $Rest )* )? }
//     };

    // bad syntax

    (
      $( $Rest : tt )*
    )
    =>
    {
      compile_error!
      (
        concat!
        (
          "Bad syntax.\n",
          "Expects : {kind} {name} : {type}.\n",
          "For example : `single MySingle : std::sync::Arc< T : Copy >`.\n",
          "But got:\n",
          stringify!
          (
            $( $Rest )*
          ),
        )
      );
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
    /// use fundamental_data_type::prelude::*;
    /// let x = Single::< i32 >( 13 );
    /// dbg!( x );
    /// ```
    ///

    #[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
    single Single : < T >;

    ///
    /// Type constructor to wrap two types into a tuple.
    ///

    #[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
    pair Pair : < T1, T2 >;

  }

  pub use types;
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
    types,
    Single,
    Pair,
  };
}
