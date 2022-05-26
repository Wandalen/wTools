
use wtools_published as wtools;

#[ test ]
fn wtools_published()
{
  if let Ok( value ) = std::env::var( "WITH_SMOKE" )
  {
    if value != "false" && value != "local"
    {
      #[ path = "./wtools_common.rs" ]
      mod wtools_common;
      wtools_common::wtools_time();
    }
  }
}
