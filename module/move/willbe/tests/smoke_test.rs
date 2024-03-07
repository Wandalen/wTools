
// #[ cfg( feature = "default" ) ]
#[ test ]
fn local_smoke_test()
{
 ::test_tools::smoke_test_for_local_run();
}

// #[ cfg( feature = "default" ) ]
#[ test ]
fn published_smoke_test()
{
 ::test_tools::smoke_test_for_published_run();
}
