// T1.5: Generic struct (should not compile)
pub struct GenericStruct< T > 
{ 
  x : T 
}

impl< T > std::ops::Add for GenericStruct< T > 
{
  type Output = Self;
  fn add( self, other : Self ) -> Self::Output 
  {
    GenericStruct 
    {
      x : self.x + other.x, // Will not compile unless T: Add
    }
  }
}

impl< T > std::ops::Sub for GenericStruct< T > 
{
  type Output = Self;
  fn sub( self, other : Self ) -> Self::Output 
  {
    GenericStruct 
    {
      x : self.x - other.x, // Will not compile unless T: Sub
    }
  }
}

fn main()
{
}
