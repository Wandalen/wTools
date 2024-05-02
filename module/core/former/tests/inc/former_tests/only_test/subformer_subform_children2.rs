
#[ test ]
fn subform()
{

  let got = Parent::former()
  .children2( "a" ).end()
  .children2( "b" ).end()
  .form();

  let children = vec!
  [
    Child { name : "a".to_string(), is_mandatory : false },
    Child { name : "b".to_string(), is_mandatory : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}
