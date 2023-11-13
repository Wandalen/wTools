/// Internal namespace.
mod private {
  use crate::wtools;

	use wca::{ Args, Props };
  use wtools::error::Result;

	/// run all tests in all crates
	pub fn run_tests( ( _, _ ) : ( Args, Props ) ) -> Result< () >
	{
		let output = std::process::Command::new( "cargo" )
		.arg( "test" )
		.output()
		.expect( "Error while running tests" );

		if output.status.success() 
		{
			println!( "All tests were successfully executed" );
		} 
		else 
		{
			eprintln!( "Error while executing tests" );

			eprintln!( "stderr: {:?}", String::from_utf8_lossy( &output.stderr ) );
		}

		Ok( () )
	}
}

crate::mod_interface!
{
  /// run all tests in all crates
  prelude use run_tests;
}