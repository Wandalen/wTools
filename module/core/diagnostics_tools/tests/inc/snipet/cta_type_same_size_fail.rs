use test_tools::diagnostics_tools::*;

fn main()
{
  struct Int( i16 );
  cta_type_same_size!( Int, u32 );
}
