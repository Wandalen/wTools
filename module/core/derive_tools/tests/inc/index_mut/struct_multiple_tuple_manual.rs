use core::ops::{ Index, IndexMut };

#[ allow( dead_code ) ]
struct StructMultipleTuple< T >( bool, Vec< T > );

impl< T > Index< usize > for StructMultipleTuple< T >
{
  type Output = T;

  fn index( &self, index : usize ) -> &Self::Output 
  {
    &self.1[ index ]
  }
}

impl< T > IndexMut< usize > for StructMultipleTuple< T >
{
  fn index_mut( &mut self, index : usize ) -> &mut Self::Output 
  {
    &mut self.1[ index ]
  }
}


include!( "./only_test/struct_multiple_tuple.rs" );


