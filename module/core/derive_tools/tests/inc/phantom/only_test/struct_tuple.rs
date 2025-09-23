#[ test ]
fn phantom()
{
  let _ = StructTuple :: < bool >( "boo".into(), 3, core ::marker ::PhantomData );
}
