#[ test ]
fn basic()
{

  let got = HashMapWrap::< &str, &str >::former().insert( "abc", "def" ).form();
  let exp = HashMapWrap::< &str, &str >::new( hmap!{ "abc" => "def" } );
  a_id!( got, exp );

}
