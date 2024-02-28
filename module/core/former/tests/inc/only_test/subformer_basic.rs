#[ test ]
fn basic()
{

  let got = Command::< &str >::former()
  .hint( "a" )
  .subject( "b" )
  .form();
  let exp = Command::< &str >
  {
    hint : "a".to_string(),
    subject : "b".to_string(),
    properties : hset!{},
  };
  a_id!( got, exp );

}
