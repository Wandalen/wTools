
use wtools_local as wtools;

#[ test ]
fn wtools_local()
{
  if let Ok( value ) = std::env::var( "WITH_SMOKE" )
  {
    if value != "false" && value != "published"
    {
      #[ path = "./wtools_common.rs" ]
      mod wtools_common;
      wtools_common::wtools_time();
    }
  }
}
