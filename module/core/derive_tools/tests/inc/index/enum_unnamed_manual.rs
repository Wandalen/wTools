 use core::ops::Index;

#[ allow( dead_code) ]
enum Enum < T >
{
  IndexVector( Vec< T > ),
}

impl< T > Index< usize > for Enum< T >
{
    type Output = T;
    
    fn index( &self, index: usize ) -> &Self::Output
    {   
        match index
        {   
            0 => match self
            {
                Enum::IndexVector( a ) => &a[0],
            },
            1 => match self
            {
                Enum::IndexVector( a ) => &a[1],
            },
            _ => panic!( "Index out of bounds" ),
        }
    }
}

include!( "./only_test/enum_unnamed.rs" );
