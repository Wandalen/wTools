#![cfg_attr(feature = "no_std", no_std)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png")]
#![doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
)]
#![doc(html_root_url = "https://docs.rs/variadic_from/latest/variadic_from/")]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ]

/// Internal implementation of variadic `From` traits and macro.
#[cfg(feature = "enabled")]
pub mod variadic;

/// Namespace with dependencies.
#[cfg(feature = "enabled")]
pub mod dependency {
  pub use ::variadic_from_meta;
}

#[cfg(feature = "enabled")]
#[doc(inline)]
#[allow(unused_imports)]
pub use own::*;

/// Own namespace of the module.
#[cfg(feature = "enabled")]
#[allow(unused_imports)]
pub mod own {
  use super::*;
  #[doc(inline)]
  pub use orphan::*;
}

/// Orphan namespace of the module.
#[cfg(feature = "enabled")]
#[allow(unused_imports)]
pub mod orphan {
  use super::*;
  #[doc(inline)]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[cfg(feature = "enabled")]
#[allow(unused_imports)]
pub mod exposed {
  use super::*;
  #[doc(inline)]
  pub use prelude::*;

  #[doc(inline)]
  pub use ::variadic_from_meta::*;

  #[cfg(feature = "type_variadic_from")]
  #[doc(inline)]
  pub use crate::variadic::From1;
  #[cfg(feature = "type_variadic_from")]
  #[doc(inline)]
  pub use crate::variadic::From2;
  #[cfg(feature = "type_variadic_from")]
  #[doc(inline)]
  pub use crate::variadic::From3;

  #[cfg(feature = "type_variadic_from")]
  #[doc(inline)]
  pub use crate::from;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[cfg(feature = "enabled")]
#[allow(unused_imports)]
pub mod prelude {
  use super::*;

  #[doc(no_inline)]
  pub use ::variadic_from_meta::VariadicFrom;

  #[cfg(feature = "type_variadic_from")]
  #[doc(inline)]
  pub use crate::variadic::From1;
  #[cfg(feature = "type_variadic_from")]
  #[doc(inline)]
  pub use crate::variadic::From2;
  #[cfg(feature = "type_variadic_from")]
  #[doc(inline)]
  pub use crate::variadic::From3;

  #[cfg(feature = "type_variadic_from")]
  #[doc(inline)]
  pub use crate::from;
}
