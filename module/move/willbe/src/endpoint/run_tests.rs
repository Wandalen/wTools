/// Internal namespace.
mod private {
  use std::path::Path;

	use crate::{ wtools, process::{ self, CmdReport } };

  use wtools::error::Result;

	/// run all tests in all crates
	pub fn run_tests( dir : &Path ) -> Result< CmdReport >
	{
		let output = process::start_sync("cargo test", dir)?;

		Ok( output )
	}
}

crate::mod_interface!
{
  /// run all tests in all crates
  prelude use run_tests;
}