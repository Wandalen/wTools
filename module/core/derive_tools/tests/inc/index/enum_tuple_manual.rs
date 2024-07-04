 use core::ops::Index;

#[ allow( dead_code) ]

enum EnumTuple < T >
{
  A ( T ),
  B ( T )
}

impl< T > Index< usize > for EnumTuple< T >
{
    type Output = T;
    
    fn index( &self, index: usize ) -> &Self::Output
    {   
        match index
        {   
            0 => match self
            {
                EnumTuple::A( a ) | EnumTuple::B( a ) => a,
            },
            _ => panic!( "Index out of bounds" ),
        }
    }
}

include!( "./only_test/enum_tuple.rs" );
