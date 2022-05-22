/// Internal namespace.
mod internal
{
  use crate::exposed::*;

  // xxx : check doc

  ///
  /// Type constructor of single.
  ///
  /// Should not be used directly. Instead use macro [crate::type!].
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

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::internal::
  {

    _single,
    Single,

  };
}