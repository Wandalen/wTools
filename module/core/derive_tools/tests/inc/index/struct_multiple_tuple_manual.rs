use core::ops::Index;

#[ allow( dead_code) ]
struct StructTupleMultiple< T >( bool, Vec< T > );

impl< T > Index< usize > for StructTupleMultiple< T >
{
  type Output = T;

  fn index( &self, index : usize ) -> &Self::Output 
  {
    &self.1[ index ]   
  }
}

include!( "./only_test/struct_multiple_tuple.rs" );

