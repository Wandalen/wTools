use core::ops::Index;

struct StructNamed<T>
{
  a : T,
}

impl<T> Index<usize> for StructNamed<T>
{
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.a,
            _ => panic!("Index out of bounds"),
        }
    }
}

include!( "./only_test/struct_named.rs" );

