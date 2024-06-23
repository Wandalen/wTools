#[ allow( dead_code ) ]
struct StructUnit;

impl From< StructUnit > for ()
{
  fn from( _ : StructUnit ) -> ()
  {
    ()
  }
}

include!( "./only_test/struct_unit.rs" );
