#[ test ]
fn basic()
{

  // let got = HashMapWrap::new( hmap!{ "abc" => "def" } );
  // let exp = HashMapWrap::< &str, &str >::former().insert( "abc", "def" );
  // a_id!( got, exp );

  let got = HashMapWrap::< &str, &str >::former().insert( "abc", "def" ).form();
  let exp = hmap!{ "abc" => "def" };
  a_id!( got, exp );

  let got = HashMapWrap::< &str, &str >::former().insert( "a", "b" ).replace( hmap!{ "abc" => "def" } ).form();
  let exp = hmap!{ "abc" => "def" };
  a_id!( got, exp );

  let got = HashMapWrap::< &str, &str >::former().insert( "abc", "def" ).end();
  let exp = hmap!{ "abc" => "def" };
  a_id!( got, exp );

  let got = HashMapWrap::< &str, &str >::former().container( hmap!{ "abc" => "def" } ).form();
  let exp = hmap!{ "abc" => "def" };
  a_id!( got, exp );

}
