#[ test ]
fn phantom()
{
  let _ = BoundsInlined::< String, i32 > { _phantom: core::marker::PhantomData };
}