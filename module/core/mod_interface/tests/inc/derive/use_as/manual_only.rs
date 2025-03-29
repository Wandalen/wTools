
use layer_x as layer_a;

#[doc(inline)]
#[allow(unused_imports)]
pub use own :: * ;

#[doc = r" Own namespace of the module."]
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;

  #[doc(inline)]
  #[allow(unused_imports)]
  pub use super :: orphan :: * ;

  #[doc(inline)]
  #[allow(unused_imports)]
  #[doc = " layer_a"]
  pub use super :: layer_x :: orphan :: * ;

}

#[doc = r" Orphan namespace of the module."]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;

  #[doc(inline)]
  #[allow(unused_imports)]
  pub use super :: exposed :: * ;

}

#[doc = r" Exposed namespace of the module."]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[doc(inline)]
  #[allow(unused_imports)]
  pub use super :: prelude :: * ;

  #[doc(inline)]
  #[allow(unused_imports)]
  #[doc = " layer_a"]
  pub use super :: layer_x :: exposed :: * ;
}

#[doc = r" Prelude to use essentials: `use my_module::prelude::*`."]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  #[doc(inline)]
  #[allow(unused_imports)]
  #[doc = " layer_a"]
  pub use super :: layer_x :: prelude :: * ;
}
