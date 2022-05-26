
mod internal
{

  ///
  /// Macro to compar meta condition is true at compile-time.
  ///
  /// ### Sample
  ///
  /// ``` rust
  ///
  /// ```

  #[ cfg( feature = "ct" ) ]
  #[macro_export]
  macro_rules! ct_true
  {
    () => {};
    (
      $( $Cond : meta )+, $Msg : expr $(,)?
    ) =>
    {
      #[ cfg( not( $( $Cond )+ ) ) ]
      compile_error!( $Msg );
    };
    (
      $( $Cond : tt )*
    )
    =>
    {
      #[ cfg( not( $( $Cond )* ) ) ]
      compile_error!
      (
        concat!
        (
          "Does not hold :\n  ",
          stringify!( $( $Cond )* ),
        )
      );
    };
  }

  pub use ct_true;
}

/// Own namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

#[ doc( inline ) ]
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

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "assertions" ) ]
  #[ doc( inline ) ]
  pub use ::pretty_assertions::assert_eq as a_id;
  #[ cfg( feature = "assertions" ) ]
  #[ doc( inline ) ]
  pub use ::pretty_assertions::assert_ne as a_not_id;

  pub use super::internal::
  {
    ct_true,
  };
}
