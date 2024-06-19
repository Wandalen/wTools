use std::marker::PhantomData;

#[ allow( dead_code ) ]
struct SendSyncType< T >
{
  a: T,
  _phantom: PhantomData< T >,
}

include!( "./only_test/send_sync_type.rs" );