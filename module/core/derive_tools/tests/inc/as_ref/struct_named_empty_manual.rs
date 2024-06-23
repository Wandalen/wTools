#[ allow( dead_code ) ]
struct StructNamedEmpty{}

impl AsRef< () > for StructNamedEmpty
{
  fn as_ref( &self ) -> &()
  {
    &()
  }
}

include!( "./only_test/struct_named_empty.rs" );
