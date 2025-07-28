// T1.3: Unit struct (should not compile)
pub struct UnitStruct;

impl std::ops::Add for UnitStruct 
{
  type Output = Self;
  fn add( self, _rhs : Self ) -> Self::Output 
  {
    self.0 + _rhs.0
  }
}

fn main()
{
}
