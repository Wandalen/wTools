use wtools::diagnostics::*;

fn main()
{
  struct Int( i16 );
  cta_type_same_size!( Int, u32 );
}
