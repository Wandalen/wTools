use super::*;

#[ allow( dead_code ) ]
#[ the_module::phantom ]
struct SendSyncType< T >
{
  a: T,
}

include!( "./only_test/send_sync_type.rs" );