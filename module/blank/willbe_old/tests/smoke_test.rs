

#[ test ]
fn local_smoke_test()
{
  ::test_tools::smoke_test_for_local_run();
}


#[ test ]
#[ ignore ]
fn published_smoke_test()
{
  ::test_tools::smoke_test_for_published_run();
}
