use core::ops::{ Index, IndexMut };

#[ allow( dead_code ) ]
struct StructMultipleNamed< T >
{
  a : Vec< T >,
  b : Vec< T >,
}

impl< T > Index< usize > for StructMultipleNamed< T >
{
  type Output = T;

  fn index( &self, index : usize ) -> &Self::Output 
  {
    &self.b[ index ]
  }
}

impl< T > IndexMut< usize > for StructMultipleNamed< T >
{
  fn index_mut( &mut self, index : usize ) -> &mut Self::Output 
  {
    &mut self.b[ index ]
  }
}


include!( "./only_test/struct_multiple_named.rs" );

