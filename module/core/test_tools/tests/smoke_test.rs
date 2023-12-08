
// #[ cfg( feature = "default" ) ]
#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
#[ test ]
fn local_smoke_test()
{
  ::test_tools::smoke_test_for_local_run();
}

// #[ cfg( feature = "default" ) ]
#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
#[ test ]
fn published_smoke_test()
{
  ::test_tools::smoke_test_for_published_run();
}
