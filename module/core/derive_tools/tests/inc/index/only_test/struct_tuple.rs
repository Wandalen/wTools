use super::*;

#[ test ]
fn index()
{
  let x = StructTuple(7);

  a_id!(x[0], 7);
}

