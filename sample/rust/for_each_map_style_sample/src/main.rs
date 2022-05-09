use for_each::for_each;

fn main()
{

  for_each!
  {
    dbg where
    @PREFIX { "prefix".to_string() + }
    @POSTFIX { + "postfix" }
    @EACH "a" "b" "c"
  };

  // generates
  dbg!( "prefix".to_string() + "a" + "postfix" );
  dbg!( "prefix".to_string() + "b" + "postfix" );
  dbg!( "prefix".to_string() + "c" + "postfix" );

}