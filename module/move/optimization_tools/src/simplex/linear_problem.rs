//! Structs that represent linear programming problem and its components.
//! 

use iter_tools::Itertools;
use ndarray::{ Array1, Array2, ArrayBase };

/// Variable of objective function.
#[ derive( Clone, Debug, PartialEq ) ]
pub struct Variable 
{
  /// Variable coefficient.
  pub coefficient : f64,
  /// Upper bound of variable.
  pub max : f64,
  /// Lower bound of variable.
  pub min : f64,
}

impl Variable
{
  /// Create new objective function variable with coefficient.
  pub fn new( coeff : f64 ) -> Self
  {
    Self { coefficient : coeff, min : f64::MIN, max : f64::MAX }
  }

  /// Add max value for objective function variable.
  pub fn max( self, max : f64 ) -> Self
  {
    Self { max, coefficient : self.coefficient, min : self.min }
  }

  /// Add min value for objective function variable.
  pub fn min( self, min : f64 ) -> Self
  {
    Self { min, coefficient : self.coefficient, max : self.max }
  }

  /// Check if given value satisfies max and min restrictions of variable.
  pub fn is_in_bounds( &self, val : f64 ) -> bool
  {
    if val >= self.min && val <= self.max
    {
      true
    }
    else 
    {
      false
    }
  }
}

/// Represents inequation constraint.
#[ derive( Clone, Debug, PartialEq ) ]
pub struct Constraint 
{
  /// Coefficients of variables in inequation.
  pub coefs : Vec< f64 >,
  /// Right-hand constant value.
  pub value : f64,
  /// Type of comparison.
  pub comparison : Comp,
}

/// Type of comparison in inequation.
#[ derive( Clone, Debug, PartialEq ) ]
pub enum Comp
{
  /// Less than comparison.
  Less,
  /// Greater than comparison.
  Greater,
  /// Constraint is equation.
  Equal,
}

impl Constraint 
{
  /// Create new constraint.
  pub fn new( coefs : Vec< f64 >, value : f64, comparison : Comp ) -> Self
  {
    Self
    {
      coefs,
      value,
      comparison,
    }
  }
}

/// Represents linear problem.
#[ derive( Clone, Debug ) ]
pub struct Problem 
{
  /// Coefficients of variables in function to optimize.
  pub var_coeffs : Vec< f64 >,
  /// Set of inequation constraints.
  pub constraints : Vec< Constraint >,
  variables : Vec< Variable >,
}

impl Problem 
{
  /// Create new linear problem.
  pub fn new( vars : Vec< Variable >, constraints : Vec< Constraint > ) -> Self
  {
    Self { var_coeffs : vars.iter().map( | var | var.coefficient ).collect_vec(), constraints, variables : vars }
  }

  /// Create normalized problem from linear programming problem.
  pub fn normalized( &self ) -> NormalizedProblem
  {
    let mut equations_coefficients = Vec::new();
    let mut vars = self.variables.clone();

    for i in 1..=self.constraints.len()
    {
      let mut coeffs = self.constraints[ i - 1 ].coefs.clone();
      for _ in 1..=self.constraints.len()
      {
        coeffs.push( 0.0 );
      }
      match self.constraints[ i - 1 ].comparison
      {
        Comp::Less => 
        {
          coeffs[ self.var_coeffs.len() + i - 1 ] = 1.0;
          vars.push( Variable::new( 0.0 ).min( 0.0 ) );
        }
        Comp::Greater =>
        {
          coeffs[ self.var_coeffs.len() + i - 1 ] = -1.0;
          vars.push( Variable::new( 0.0 ).min( 0.0 ) );
        }
        Comp::Equal => {}
      }
      equations_coefficients.push( coeffs );

    }

    NormalizedProblem::new
    ( 
      &equations_coefficients, 
      &self.constraints.iter().map( | c | c.value ).collect_vec(),
      &vars,
    )
  }

}

/// Represents normalized linear problem.
#[ derive( Clone, Debug ) ]
pub struct NormalizedProblem 
{
  /// Coefficients of normalized equations.
  pub coeffs : Array2< f64 >,
  /// Rhs values of equations.
  pub rhs : Array1< f64 >,
  /// Variables of objective function.
  pub variables : Vec< Variable >,
}

impl NormalizedProblem
{
  /// Create new normalized problem.
  pub fn new( matrix : &Vec< Vec< f64 > >, rhs : &Vec< f64 >, vars : &Vec< Variable > ) -> Self
  {
    Self
    {
      coeffs : Array2::from_shape_vec
      ( 
        ( matrix.len(), matrix[ 0 ].len() ), 
        matrix.iter().flat_map( | vec | vec.clone() ).collect_vec()
      ).unwrap(),
      rhs : ArrayBase::from_vec( rhs.clone() ),
      variables : vars.clone(),
    }
  }

  /// Check if basic solution is feasible.
  pub fn is_feasible_solution( &self, bs : &BasicSolution ) -> bool
  {
    for ( index, bv ) in bs.bv.iter().enumerate()
    {
      if let Some( var ) = self.variables.get( bv - 1 )
      {
        if !var.is_in_bounds( bs.bv_values[ index ] )
        {
          return false;
        }
      }
    }
    true
  }
}

/// Basic solution of linear problem.
#[ derive( Clone, Debug ) ]
pub struct BasicSolution
{
  /// Non-basic variables indices.
  pub nbv : Vec< usize >,
  /// Basic variables indices.
  pub bv : Vec< usize >,
  /// Basic variables values.
  pub bv_values : Vec< f64 >,
}