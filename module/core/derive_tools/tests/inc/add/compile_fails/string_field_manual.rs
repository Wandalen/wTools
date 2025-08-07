// T1.4: Named struct with String (should not compile)
pub struct StringStruct 
{ 
  x : String 
}

impl std::ops::Add for StringStruct 
{
  type Output = Self;
  fn add( self, other : Self ) -> Self::Output 
  {
    StringStruct 
    {
      x : self.x + other.x, // will not compile with other: String. Could compile with &String or &str
    }
  }
}

impl std::ops::Sub for StringStruct 
{
  type Output = Self;
  fn sub( self, other : Self ) -> Self::Output 
  {
    StringStruct 
    {
      x : self.x - other.x,
    }
  }
}

fn main()
{
}
