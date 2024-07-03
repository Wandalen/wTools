use core::ops::Index;

struct StructTuple< T >( T, T );

impl< T > Index< usize > for StructTuple< T > 
{
  type Output = T;

  fn index( &self, index: usize ) -> &Self::Output 
  {
    match index 
    {
      0 => &self.0,
      1 => &self.1,
      _ => panic!( "Index out of bounds" ),
    }
  }
}

include!( "./only_test/struct_tuple.rs" );
