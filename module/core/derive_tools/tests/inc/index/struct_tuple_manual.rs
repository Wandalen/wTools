use core::ops::Index;

#[ allow( dead_code) ]
struct StructTuple< T >( Vec< T > );

impl< T > Index< usize > for StructTuple< T >
{
  type Output = T;

  fn index( &self, index : usize ) -> &Self::Output 
  {
    &self.0[ index ]   
  }
}

include!( "./only_test/struct_tuple.rs" );
