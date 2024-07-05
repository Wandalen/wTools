use core::ops::Index;

#[ allow( dead_code ) ]
struct StructNamed< T >
{
  a: Vec< T >
}

impl< T > Index< usize > for StructNamed< T >
{
  type Output = T;

  fn index( &self, index : usize ) -> &Self::Output 
  {
    &self.a[ index ]
  }
}

include!( "./only_test/struct_named.rs" );
