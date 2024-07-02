#[ test ]
fn phantom()
{
  let _ = BoundsInlined::< String, i32 > { _phantom: Default::default() };
}