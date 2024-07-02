
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
#[ allow( unused_imports ) ]
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    cta_true,
  };
}
