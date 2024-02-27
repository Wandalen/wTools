#[ test ]
fn basic()
{

  let got = HashMapWrap::former().insert( 1, 11 ).end();
  let exp = HashMapWrap::new( hmap!{ 1 => 11 } );
  a_id!( got, exp );

}