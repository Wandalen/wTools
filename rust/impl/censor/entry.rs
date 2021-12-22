use std::env;
use wcensor_mid::*;

fn main()
{
  let params = InstructionParseParams::new();
  instruction_parse_from_args( &params, env::args().skip( 1 ) );
  // println!( "{}", 13 );
}
