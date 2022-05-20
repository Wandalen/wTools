/// Internal namespace.
mod internal
{
  use crate::exposed::*;

  // xxx : module

  ///
  /// Macro to declare another type wrapping a single another type into a tuple.
  ///
  /// Auto-implement traits From, Into and Dereference for the wrapper.
  ///

  #[ macro_export ]
  macro_rules! single
  {

    (
      $( #[ $Meta : meta ] )*
      $Name : ident :
      < $( $ParamName : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy2 : path )* )? )* >
      $(;)?
    )
    =>
    {
      $( #[ $Meta ] )*
      pub struct $Name
      < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? )* >
      ( pub $( $ParamName )* );

      impl< $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? )* > core::ops::Deref
      for $Name
      < $( $ParamName )* >
      {
        type Target = $( $ParamName )*;
        fn deref( &self ) -> &Self::Target
        {
          &self.0
        }
      }

      impl< $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? )* > From< $( $ParamName )* >
      for $Name
      < $( $ParamName )* >
      {
        fn from( src : $( $ParamName )* ) -> Self
        {
          Self( src )
        }
      }

      // impl< $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? )* > From< $Name< $( $ParamName )* > >
      // for $( $ParamName )*
      // {
      //   fn from( src : $Name< $( $ParamName )* > ) -> Self
      //   {
      //     src.0
      //   }
      // }

    };

    (
      $( #[ $Meta : meta ] )*
      $Name : ident : $TypeSplit1 : ident $( :: $TypeSplitN : ident )*
      < $( $ParamName : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy2 : path )* )? )* >
      $(;)?
    )
    =>
    {
      $( #[ $Meta ] )*
      pub struct $Name
      < $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? )* >
      ( pub $TypeSplit1 $( :: $TypeSplitN )* < $( $ParamName )* > );

      impl< $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? )* > core::ops::Deref
      for $Name
      < $( $ParamName )* >
      {
        type Target = $TypeSplit1 $( :: $TypeSplitN )* < $( $ParamName )* >;
        fn deref( &self ) -> &Self::Target
        {
          &self.0
        }
      }

      impl< $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? )* > From< $TypeSplit1 $( :: $TypeSplitN )* < $( $ParamName )* > >
      for $Name
      < $( $ParamName )* >
      {
        fn from( src : $TypeSplit1 $( :: $TypeSplitN )* < $( $ParamName )* > ) -> Self
        {
          Self( src )
        }
      }

      impl< $( $ParamName $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy2 )* )? )* > From< $Name< $( $ParamName )* > >
      for $TypeSplit1 $( :: $TypeSplitN )* < $( $ParamName )* >
      {
        fn from( src : $Name< $( $ParamName )* > ) -> Self
        {
          src.0
        }
      }

    };

    (
      $( #[ $Meta : meta ] )*
      $Name : ident : $TypeSplit1 : ident $( :: $TypeSplitN : ident )* $(;)?
      // $Name : ident : $Type : ty $(;)?
    )
    =>
    {
      $crate::single!
      (
        $( #[ $Meta ] )*
        $Name : $TypeSplit1 $( :: $TypeSplitN )* <>;
      );

    };

  }

  single!
  {

    ///
    /// A type wrapping a single another type into a tuple.
    ///

    #[ derive( Debug, Clone, PartialEq, Eq ) ]
    Single : < T >;

  }

  pub use single;
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
    single,
    Single,
  };
}
