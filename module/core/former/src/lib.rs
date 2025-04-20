#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/former/latest/former/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

// xxx : introduce body( struct/enum ) attribute `standalone_constructors` which create stand-alone, top-level constructors for struct/enum. for struct it's always single function, for enum it's as many functions as enum has vartianys. if there is no `arg_for_constructor` then constructors expect exaclty zero arguments. start from implementations without respect of attribute attribute `arg_for_constructor`. by default `standalone_constructors` is false
// xxx : introduce field attribute to mark an attribute `arg_for_constructor` as an argument which should be used in constructing functions ( either standalone consturcting function or associated with struct ). in case of enums attribute `arg_for_constructor` is attachable only to fields of variant and attempt to attach attribute `arg_for_constructor` to variant must throw understandable error. name standalone constructor of struct the same way struct named, but snake case and for enums the same name variant is named, but snake case. by default it's false.

// xxx : add to readme example with enums
// xxx : disable and phase out attribute "[ perform( fn method_name<...> () -> OutputType ) ]"
// xxx : split out crate component model
// xxx : fix commented out tests

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  pub use former_types;
  pub use former_meta;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use former_meta as derive;
}

/// Parented namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use former_meta::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use former_types::exposed::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use former_types::prelude::*;

}
