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
    properties : std::collections::HashMap::< &str, String >::new(),
  };
  a_id!( got, exp );

}
