 use core::ops::Index;

#[ allow( dead_code) ]

enum EnumNamed < T >
{
  A { a : Vec< T > },
}

impl< T > Index< usize > for EnumNamed< T >
{
  type Output = T;
    
  fn index( &self, index: usize ) -> &Self::Output
  {   
    match index
    {   
      0 => match self 
      {
        EnumNamed::A { a, .. } => &a[0],          
      },
      1 => match self 
      {
        EnumNamed::A { a, .. } => &a[1],
      },
      2 => match self 
      {
        EnumNamed::A { a, .. } => &a[2],
      },
      _ => panic!( "Index out of bounds" ),
    }
  }
}

include!( "./only_test/enum_named.rs" );
