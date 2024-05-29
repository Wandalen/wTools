
#[ test ]
fn scalar()
{

  let children = collection_tools::vec!
  [
    Child { name : "a".to_string(), data : false },
    Child { name : "b".to_string(), data : false },
  ];
  let got = Parent::former()
  .children( children )
  .form();

  let children = collection_tools::vec!
  [
    Child { name : "a".to_string(), data : false },
    Child { name : "b".to_string(), data : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}
