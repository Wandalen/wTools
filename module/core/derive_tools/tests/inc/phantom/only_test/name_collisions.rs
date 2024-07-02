#[ test ]
fn phantom()
{
  let _ = NameCollisions::< bool > { a : "boo".into(), b : 3, _phantom: Default::default() };
}