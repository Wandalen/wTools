#[ test ]
fn phantom()
{
  let _ = BoundsMixed::< String, i32 > { _phantom: Default::default() };
}