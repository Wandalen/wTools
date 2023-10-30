
pub( crate ) mod private
{

  ///
  /// Macro to compare meta condition is true at compile-time.
  ///
  /// ### Basic use-case.
  ///
  /// ``` rust
  /// use diagnostics_tools::prelude::*;
  /// cta_true!( any( feature = "diagnostics_compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) );
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
      core::compile_error!( $Msg );
    };
    (
      $( $Cond : tt )*
    )
    =>
    {
      #[ cfg( not( $( $Cond )* ) ) ]
      core::compile_error!
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

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    cta_true,
  };
}
