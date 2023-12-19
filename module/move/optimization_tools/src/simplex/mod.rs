#[ derive( Clone ) ]
pub struct Problem 
{
    obj_coeffs : Vec< f64 >,
    constraints : Vec< Constraint >,
}

impl Problem 
{
  pub fn new( obj_coeffs : Vec< f64 >, constraints : Vec< Constraint > ) -> Self
  {
    Self { obj_coeffs, constraints }
  }
}

#[ derive( Clone ) ]
pub struct Constraint 
{
  coefs : Vec< f64 >,
  value : f64,
}

impl Constraint 
{
  pub fn new( coefs : Vec< f64 >, value : f64 ) -> Self
  {
    Self
    {
    coefs,
    value,
    }
  }
}

pub struct SimplexSolver 
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constraint() {
        let c = Constraint::new(vec![1.0, 2.0], 4.0);
        assert_eq!(c.value, 4.0);
    }

}