use diagnostics_tools::*;

fn main()
{
  let ins1 = ( 13, 15, 16 );
  let ins2 = ( 13, 15, 17 );
  a_not_id!( ins1, ins2 );
}
