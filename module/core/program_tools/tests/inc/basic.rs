#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn basic()
{
  use the_module::program;

  let _plan = program::Plan::former()
    .program()
      // .source().file_path( "main.rs" ).data( program::GetData::FromStr( "fn main() { println( \"hello!\" ) }" ) ).end()
      .source().file_path( "main.rs" ).data( "fn main() { println( \"hello!\" ) }" ).end()
    .end()
  .end();

}
