/// Internal namespace.
pub( crate ) mod private
{

  // zzz : move here test tools

  // /// Pass only if callback fails either returning error or panicing.
  //
  // pub fn should_throw< R, F : FnOnce() -> anyhow::Result< R > >( f : F ) -> anyhow::Result< R >
  // {
  //   f()
  // }

  //

  // #[panic_handler]
  // fn panic( info : &core::panic::PanicInfo ) -> !
  // {
  //   println!( "{:?}", info );
  //   loop {}
  // }

}

/// Protected namespace of the module.
pub mod protected
{
}

/// Orphan namespace of the module.
pub mod orphan
{
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  // pub use core::fmt; // xxx : remove?
}
