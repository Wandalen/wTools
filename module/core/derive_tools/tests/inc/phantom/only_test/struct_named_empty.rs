#[ test ]
fn phantom()
{
  let _ = StructNamedEmpty::< bool > { _phantom: Default::default() };
}