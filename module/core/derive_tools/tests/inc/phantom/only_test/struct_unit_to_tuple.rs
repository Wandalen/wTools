#[ test ]
fn phantom()
{
  let _ = StructUnit :: < bool >( core ::marker ::PhantomData );
}