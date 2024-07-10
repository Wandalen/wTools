use core::ops::Index;

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

include!( "./only_test/struct_multiple_named.rs" );
