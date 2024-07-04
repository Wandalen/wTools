 use core::ops::Index;

#[ allow( dead_code) ]

enum EnumNamed < T >
{
  A { a : T, b : T },
  B { a : T, b : T },
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
        EnumNamed::A { a, .. } | EnumNamed::B { a, .. } => a,          
      },
      1 => match self 
      {
        EnumNamed::A { b, .. } | EnumNamed::B { b, .. } => b,
      },
      _ => panic!( "Index out of bounds" ),
    }
  }
}

include!( "./only_test/enum_named.rs" );
