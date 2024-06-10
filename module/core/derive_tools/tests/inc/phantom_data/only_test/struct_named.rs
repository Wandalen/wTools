#[ test ]
fn phantom_data()
{
  let _ = StructNamed::< bool >{ a : "boo".into(), b : 3, _phantom: Default::default() };
}