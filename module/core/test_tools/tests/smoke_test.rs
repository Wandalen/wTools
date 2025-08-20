//! Smoke testing of the crate.

#[ cfg( feature = "enabled" ) ]
#[cfg(not(feature = "no_std"))]
#[ test ]
fn local_smoke_test() -> Result< (), Box< dyn core::error::Error > > {
  ::test_tools::test::smoke_test::smoke_test_for_local_run()
}

#[ cfg( feature = "enabled" ) ]
#[cfg(not(feature = "no_std"))]
#[ test ]
fn published_smoke_test() -> Result< (), Box< dyn core::error::Error > > {
  ::test_tools::test::smoke_test::smoke_test_for_published_run()
}
