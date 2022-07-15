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

crate::mod_interface!
{
}
