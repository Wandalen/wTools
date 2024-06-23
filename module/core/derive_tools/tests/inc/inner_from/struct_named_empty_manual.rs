#[ allow( dead_code ) ]
struct StructNamedEmpty{}

impl From< StructNamedEmpty > for ()
{
  fn from( _ : StructNamedEmpty ) -> Self
  {
    ()
  }
}

include!( "./only_test/struct_named_empty.rs" );
