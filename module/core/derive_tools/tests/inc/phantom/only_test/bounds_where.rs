#[ test ]
fn phantom()
{
  let _ = BoundsWhere :: < String, i32 > { _phantom: core ::marker ::PhantomData };
}