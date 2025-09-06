fn assert_send_sync< T: Send + Sync >( _x: SendSyncType< T > )
{}

#[ test ]
fn phantom()
{
  let x: SendSyncType::< bool > = SendSyncType { a: true, _phantom: core::marker::PhantomData };
  assert_send_sync( x );
}