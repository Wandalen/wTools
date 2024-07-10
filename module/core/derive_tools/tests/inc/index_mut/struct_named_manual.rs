use core::ops::{ Index, IndexMut };

#[ allow( dead_code ) ]
struct StructNamed< T >
{
  a : Vec< T >
}

impl< T > Index< usize > for StructNamed< T >
{
  type Output = T;

  fn index( &self, index : usize ) -> &Self::Output 
  {
    &self.a[ index ]
  }
}

impl< T > IndexMut< usize > for StructNamed< T >
{
  fn index_mut( &mut self, index : usize ) -> &mut Self::Output 
  {
    &mut self.a[ index ]
  }
}


include!( "./only_test/struct_named.rs" );
