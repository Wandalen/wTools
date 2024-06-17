#[ allow( dead_code ) ]
struct StructUnit;

impl AsRef< () > for StructUnit
{
  fn as_ref( &self ) -> &()
  {
    &()
  }
}

include!( "./only_test/struct_unit.rs" );
