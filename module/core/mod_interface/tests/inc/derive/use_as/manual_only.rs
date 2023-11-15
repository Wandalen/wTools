
use layer_x as layer_a;

#[doc(inline)]
#[allow(unused_imports)]
pub use protected :: * ;

#[doc = r" Protected namespace of the module."]
pub mod protected
{

  #[doc(inline)]
  #[allow(unused_imports)]
  pub use super :: orphan :: * ;

  #[doc(inline)]
  #[allow(unused_imports)]
  #[doc = " layer_a"]
  pub use super :: layer_x :: orphan :: * ;

}

#[doc = r" Orphan namespace of the module."]
pub mod orphan
{

  #[doc(inline)]
  #[allow(unused_imports)]
  pub use super :: exposed :: * ;

}

#[doc = r" Exposed namespace of the module."]
pub mod exposed
{
  #[doc(inline)]
  #[allow(unused_imports)]
  pub use super :: prelude :: * ;

  #[doc(inline)]
  #[allow(unused_imports)]
  #[doc = " layer_a"]
  pub use super :: layer_x :: exposed :: * ;
}

#[doc = r" Prelude to use essentials: `use my_module::prelude::*`."]
pub mod prelude
{
  #[doc(inline)]
  #[allow(unused_imports)]
  #[doc = " layer_a"]
  pub use super :: layer_x :: prelude :: * ;
}
