#[ allow( dead_code ) ]
struct StructTupleEmpty();

impl From< StructTupleEmpty > for ()
{
  fn from( _ : StructTupleEmpty ) -> ()
  {
    ()
  }
}

include!( "./only_test/struct_tuple_empty.rs" );
