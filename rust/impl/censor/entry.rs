use std::env;
use wcensor_mid::*;

fn main()
{
  let params = InstructionParseParams::new();
  let instruction = instruction_parse_from_splits( &params, env::args().skip( 1 ) );
  println!( "{:?}", instruction );
}
