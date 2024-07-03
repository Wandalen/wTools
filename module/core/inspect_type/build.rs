//! To have information about channel of Rust compiler.

use rustc_version::{ version, version_meta, Channel };

fn main()
{
  // Assert we haven't travelled back in time
  assert!( version().unwrap().major >= 1 );

  // Set cfg flags depending on release channel
  match version_meta().unwrap().channel
  {
    Channel::Stable =>
    {
      println!("cargo:rustc-cfg=RUSTC_IS_STABLE");
      println!("cargo:rustc-check-cfg=cfg(RUSTC_IS_STABLE)");
    }
    Channel::Beta =>
    {
      println!("cargo:rustc-cfg=RUSTC_IS_BETA");
      println!("cargo:rustc-check-cfg=cfg(RUSTC_IS_BETA)");
    }
    Channel::Nightly =>
    {
      println!("cargo:rustc-cfg=RUSTC_IS_NIGHTLY");
      println!("cargo:rustc-check-cfg=cfg(RUSTC_IS_NIGHTLY)");
    }
    Channel::Dev =>
    {
      println!("cargo:rustc-cfg=RUSTC_IS_DEV");
      println!("cargo:rustc-check-cfg=cfg(RUSTC_IS_DEV)");
    }
  }

}
