
#[ test ]
fn scalar()
{

  let children = vec!
  [
    Child { name : "a".to_string(), is_mandatory : false },
    Child { name : "b".to_string(), is_mandatory : false },
  ];
  let got = Parent::former()
  .children3( children )
  .form();

  let children = vec!
  [
    Child { name : "a".to_string(), is_mandatory : false },
    Child { name : "b".to_string(), is_mandatory : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}
