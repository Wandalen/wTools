#[ test ]
fn phantom()
{
  let _ = StructTupleEmpty::< bool >( core::marker::PhantomData );
}