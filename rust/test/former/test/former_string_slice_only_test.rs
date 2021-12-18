
//

fn test_complex() -> anyhow::Result< () >
{

  let command = Struct1::former()
  .form();
  // dbg!( &command );

  let expected = Struct1
  {
    string_slice_1 : "",
  };
  // assert_eq!( command, expected );

  Ok( () )
}

//

#[ test ]
fn main_test()
{
  test_complex().unwrap();
}
