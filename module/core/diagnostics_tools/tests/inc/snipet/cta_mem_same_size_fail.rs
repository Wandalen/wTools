use test_tools::diagnostics_tools::*;

fn main()
{
  struct Int( i16 );
  let ins1 = Int( 31 );
  let ins2 = 13_i32;
  cta_mem_same_size!( ins1, ins2 );
}
