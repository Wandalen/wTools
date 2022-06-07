
pub( crate ) mod private
{

  ///
  /// Macro to compare meta condition is true at compile-time.
  ///
  /// ### Sample
  ///
  /// ``` rust
  /// use diagnostics_tools::prelude::*;
  /// cta_true!( any( feature = "compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) );
  /// ```
  ///

  #[ macro_export ]
  macro_rules! cta_true
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

  pub use cta_true;
}

/// Protected namespace of the module.
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
  pub use super::private::
  {
    cta_true,
  };
}
