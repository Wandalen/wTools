

#[ test ]
#[ ignore ]
fn local_smoke_test()
{
  ::test_tools::test::smoke_test::smoke_test_for_local_run();
}


#[ test ]
fn published_smoke_test()
{
  ::test_tools::test::smoke_test::smoke_test_for_published_run();
}
