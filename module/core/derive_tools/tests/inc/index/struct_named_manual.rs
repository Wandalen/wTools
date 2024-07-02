use core::ops::Index;

struct StructNamed
{
  a : bool,
}

impl Index<usize> for StructNamed
{
    type Output = bool;
    
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.a,
            _ => panic!("Index out of bounds"),
        }
    }
}

include!( "./only_test/struct_named.rs" );

