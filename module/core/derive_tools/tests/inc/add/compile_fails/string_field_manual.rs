// T1.4: Named struct with String (should not compile)
pub struct StringStruct 
{ 
  x : String 
}

impl std::ops::Add for StringStruct 
{
  type Output = Self;
  fn add( self, rhs : Self ) -> Self::Output 
  {
    StringStruct 
    {
      x : self.x + rhs.x, // will not compile with rhs: String. Could compile with &String or &str
    }
  }
}

impl std::ops::Sub for StringStruct 
{
  type Output = Self;
  fn sub( self, rhs : Self ) -> Self::Output 
  {
    StringStruct 
    {
      x : self.x - rhs.x,
    }
  }
}

fn main()
{
}
