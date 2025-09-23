#[ test ]
fn phantom()
{
  let _ = StructNamedEmpty :: < bool > { _phantom: core ::marker ::PhantomData };
}