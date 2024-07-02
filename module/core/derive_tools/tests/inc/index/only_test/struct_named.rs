use super::*;

#[ test ]
fn index()
{
  let x = StructNamed { a : true};

  a_id!(x[0], true);
}

