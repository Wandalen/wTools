#[allow(clippy::used_underscore_binding, clippy::all, warnings)]
#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]


#[ test ]
fn subform()
{

  let got = Parent::former()
  .children2( "a" ).end()
  .children2( "b" ).end()
  .form();

  let children = collection_tools::vec!
  [
    Child { name : "a".to_string(), data : false },
    Child { name : "b".to_string(), data : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}
