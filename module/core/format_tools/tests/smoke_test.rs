//! Smoke tests.

/// Smoke test of local version of the crate.
#[ test ]
fn local_smoke_test()
{
  ::test_tools::smoke_test_for_local_run();
}

/// Smoke test of published version of the crate.
#[ test ]
fn published_smoke_test()
{
  ::test_tools::smoke_test_for_published_run();
}
