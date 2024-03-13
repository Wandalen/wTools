//! qqq : write proper description
use for_each::for_each;

fn main()
{

  for_each!
  {
    dbg where
    @Prefix { "prefix".to_string() + }
    @Postfix { + "postfix" }
    @Each "a" "b" "c"
  };

  // generates
  dbg!( "prefix".to_string() + "a" + "postfix" );
  dbg!( "prefix".to_string() + "b" + "postfix" );
  dbg!( "prefix".to_string() + "c" + "postfix" );

}