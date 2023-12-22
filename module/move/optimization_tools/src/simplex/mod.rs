//! Contains implementation of Simplex optimization method.
//! 

use std::{ vec, collections::{HashSet, BinaryHeap}, env::var };
use iter_tools::Itertools;
//use ndarray;

mod drawing;


/// Represents linear problem.
#[ derive( Clone, Debug ) ]
pub struct Problem 
{
    /// Coefficients of variables in function to optimize.
    pub var_coeffs : Vec< f64 >,
    /// Set of inequation constraints.
    pub constraints : Vec< Constraint >,
    /// Min allowed values for variables.
    pub mins : Vec< f64 >,
    /// Max allowed values for variables.
    pub maxs : Vec< f64 >,
}

impl Problem 
{
  /// Create new linear problem.
  pub fn new( var_coeffs : Vec< f64 >, constraints : Vec< Constraint >, mins : Vec< f64 >, maxs : Vec< f64 > ) -> Self
  {
    Self { var_coeffs, constraints, mins, maxs }
  }

  pub fn read( opt_function: &str, constraints_str: Vec<&str> ) -> Problem
  {
    use exmex::{ prelude::*, DeepEx, ops_factory, BinOp, MakeOperators, Operator};

    ops_factory!(
      BitwiseOpsFactory,
        bool,
        Operator::make_bin(
            "<=",
            BinOp {
                apply: |a, b| a <= b,
                prio: 0,
                is_commutative: false,
            }
        )
    );

    let mut z_coeffs = Vec::new();

    let z_expr = FlatEx::<f64>::parse(opt_function).unwrap();
    let var_number = z_expr.var_indices_ordered().len();
    for val in 0..var_number
    {
      let deep_ex = z_expr.clone().to_deepex().unwrap();
      let coeff = deep_ex.partial(val).unwrap();
      z_coeffs.push(coeff.eval(vec![0.0; var_number].as_slice()).unwrap());
    }
  
  let mut constraints = Vec::new();
    for constraint in &constraints_str
    {
      let mut left_hand = "";
      let mut right_hand = "";
      let mut comp = Comp::Less;
      if constraint.contains("<=")
      {
        (left_hand, right_hand) = constraint.split("<=").collect_tuple().unwrap();
      }

      if constraint.contains(">=")
      {
        (left_hand, right_hand) = constraint.split(">=").collect_tuple().unwrap();
        comp = Comp::Greater;
      }
      
      let mut coeffs = Vec::new();
      let expr = FlatEx::<f64>::parse(left_hand).unwrap();
      let var_number = expr.var_indices_ordered().len();
      for val in 0..var_number
      {
        let deep_ex = expr.clone().to_deepex().unwrap();
        let coeff = deep_ex.partial(val).unwrap();
        coeffs.push(coeff.eval(vec![0.0; var_number].as_slice()).unwrap());
        
      }
      constraints.push( Constraint {
        coefs: coeffs,
        value: FlatEx::<f64>::parse(right_hand).unwrap().eval(&[]).unwrap(),
        comparison: comp,
      });
    }

    dbg!(Problem {
      constraints,
      var_coeffs: z_coeffs,
      mins: Vec::new(),
      maxs: Vec::new(),
    })
  }

  fn normalize( &mut self )
  {
    let mut equations_coefficients = Vec::new();
    for i in 1..= self.constraints.len()
    {
      let mut coeffs = self.constraints[ i - 1 ].coefs.clone();
      for _ in 1..=self.constraints.len()
      {
        coeffs.push( 0.0 );
      }
      match self.constraints[ i-1 ].comparison
      {
        Comp::Less => 
        {
            coeffs[ self.var_coeffs.len() + i - 1 ] = 1.0;
        }
        Comp::Greater =>
        {
            coeffs[ self.var_coeffs.len() + i - 1 ] = -1.0;
        }
        Comp::Equal => {}
      }
      equations_coefficients.push( coeffs );
    }

    let new_constraints = self.constraints
    .iter()
    .enumerate()
    .map( | ( i, constraint ) | 
      Constraint::new(equations_coefficients[ i ].clone(), constraint.value, Comp::Equal ) )
    .collect_vec()
    ;

    self.constraints = new_constraints;
  }
}

/// Represents inequation constraint.
#[ derive( Clone, Debug ) ]
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
#[ derive( Clone, Debug ) ]
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

/// Extreme point of feasible region.
#[ derive( Clone, Debug, PartialEq ) ]
pub struct ExtremePoint
{
//   problem_var_coeffs : Vec< f64 >,
  /// Basic variables indices.
  bv : Vec< usize >,
  /// Extreme point coordinates.
  point : Vec< f64 >,
  z : f64,
}

impl Eq for ExtremePoint {}

impl Default for ExtremePoint
{
  fn default() -> Self 
  {
    Self { bv : Vec::new(), point : Vec::new() , z : 0.0 }
  }
}

impl ExtremePoint
{

  pub fn new( solution : BasicSolution, problem_coeffs : Vec< f64 > ) -> Self
  {
    let m = solution.bv.len();
    let mut point = vec![ 0.0; m ];
    for index in 1..= m
    {
      if solution.bv.contains( &index )
      {
        point[ index - 1 ] = solution.bv_values[ solution.bv.iter().position( | a | *a == index ).unwrap() ];
      }
    }

    let z = problem_coeffs
    .iter()
    .zip( &point )
    .fold( 0.0, | sum, elem | sum + elem.0 * elem.1 )
    ;

    Self
    {
      bv : solution.bv,
      point,
      z,
    }
  }
  /// Checks if two extreme points is adjacent.
  pub fn is_adjacent( &self, other : &ExtremePoint ) -> bool
  {
    let bv = self.bv.iter().collect::< HashSet< _ > >();
    let other_bv = other.bv.iter().collect::< HashSet< _ > >();
    if bv.intersection( &other_bv ).collect_vec().len() == bv.len() - 1
    {
      return true;
    }
    false
  }
}

#[ derive( Clone, Debug ) ]
pub struct BasicSolution
{
  /// Non-basic variables indices.
  nbv : Vec< usize >,
  /// Basic variables indices.
  bv : Vec< usize >,
  bv_values : Vec< f64 >,
}

impl PartialOrd for ExtremePoint 
{
  fn partial_cmp( &self, other : &Self ) -> Option< std::cmp::Ordering > 
  {
    Some( self.z.partial_cmp( &other.z ).unwrap() )
  }
}

impl Ord for ExtremePoint 
{
  fn cmp( &self, other : &Self ) -> std::cmp::Ordering
  {
    self.z.partial_cmp( &other.z ).unwrap()
  }
}

/// Implementation of Simplex method solver.
#[ derive( Debug ) ]
pub struct SimplexSolver {}

impl SimplexSolver
{
  fn extreme_points ( p : &mut Problem ) -> Vec< ExtremePoint >
  {
    let bfs = Self::basic_feasible_solutions( p.clone() );
    let extreme_points = bfs
    .into_iter()
    .map( | s | ExtremePoint::new( s, p.var_coeffs.clone() ) )
    .collect::< Vec< ExtremePoint > >()
    ;

    extreme_points
  }
  fn basic_feasible_solutions( mut p : Problem ) -> Vec< BasicSolution >
  {
    let total_variables_number = p.var_coeffs.len() + p.constraints.len();
    let basic_variables_number = p.var_coeffs.len();
    let non_basic_variables_number = p.constraints.len();
    let number_of_basic_solutions : u128 = ( 1..=total_variables_number as u128 ).product::< u128 >() 
      / ( ( 1..=basic_variables_number as u128 ).product::< u128 >() * ( 1..=non_basic_variables_number as u128 ).product::< u128 >() );

    p.normalize();

    let mut bs = vec![ BasicSolution 
      { 
        bv_values: vec![ -1.0; basic_variables_number ], 
        bv: vec![ 0; basic_variables_number ], 
        nbv: vec![ 0; non_basic_variables_number ]
      }; 
      number_of_basic_solutions as usize ];

    let mut result = ( 1..=total_variables_number )
    .into_iter()
    .map(| elem | 
    {
      let mut h = HashSet::new(); 
      h.insert(elem); 
      h
    })
    .collect_vec()
    ;

    for _ in 0..basic_variables_number
    {
      result = ( 1..=total_variables_number )
      .cartesian_product( result.clone()).map( | ( elem, mut set ) | 
      {
        set.insert( elem );
        set
      })
      .collect_vec()
      ;
    }

    let mut result = result
    .into_iter()
    .filter( | set | set.len() == basic_variables_number )
    .collect_vec()
    ;

    let mut final_result = Vec::new();
    while let Some( combination ) = result.pop() 
    {
      if !result.contains( &combination )
      {
        final_result.push( combination );
      }
    }

    for ( index, bs ) in bs.iter_mut().enumerate()
    {
      bs.bv = final_result[ index ].clone().iter().map( | elem | *elem ).collect_vec();
      bs.bv.sort();
    }

    for basic_solution in bs.iter_mut() 
    {
      let indices = ( 1..=total_variables_number ).into_iter().collect::< HashSet< _ > >();
      let bv_set = basic_solution.bv.clone().into_iter().collect::< HashSet< _ > >();
      let set = indices.difference( &bv_set );
      basic_solution.nbv = set.into_iter().map( | elem | *elem ).collect_vec();
    }
    for basic_solution in bs.iter_mut() 
    {
      let rows = basic_solution.nbv.len();
      let columns = basic_solution.bv.len();
      let mut vec_of_coeffs = vec![ 0.0; rows * columns ];
        
      for ( index, bv ) in basic_solution.bv.iter().enumerate() 
      {
        for i in 0..p.constraints.len() 
        {
          vec_of_coeffs[ i * columns + index ] = p.constraints[ i ].coefs[ bv - 1 ];
        }
      }

      
      let v = p.constraints.iter().map(|c| c.value).collect::<Vec<_>>();

      let m1: ndarray::Array2<f64> = ndarray::Array2::from_shape_vec((rows, columns), vec_of_coeffs).unwrap();
      let b : ndarray::Array1<f64> = ndarray::ArrayBase::from_vec(v.clone());

      let b = ndarray_linalg::Solve::solve_into(&m1, b);

      if let Ok( solution ) = b
      {
        basic_solution.bv_values = solution.iter().map( | a | *a ).collect_vec();
      }
    }

    bs.into_iter().filter_map( | b_s | 
      {
        for b_value in b_s.bv_values.iter() 
        {
          if b_value < &0.0
          {
            return None;
          }
        }
        Some( b_s )
      }
    ).collect_vec()

  }

  /// Solves linear problem using Simplex method.
  pub fn solve( &self, mut p : Problem ) -> ExtremePoint
  {
    //let bfs = Self::basic_feasible_solutions( p.clone() );
    let extreme_points = Self::extreme_points(&mut p);
    let mut queue: std::collections::BinaryHeap<ExtremePoint> = extreme_points.into_iter().collect::< BinaryHeap< _ > >();
    let max_point = queue.pop().unwrap();

    max_point
  }
}

#[ cfg( test ) ]
mod simplex_tests {
  use super::*;

  #[ test ]
  fn constraint() 
  {
    let c = Constraint::new( vec![ 1.0, 2.0 ], 4.0, Comp::Greater );
    assert_eq!( c.value, 4.0 );
  }

  #[ test ]
  fn problem() 
  {
    let p = Problem::new
    ( 
      vec![ 3.0, 2.0 ], 
      vec![ Constraint::new( vec![ 2.0, 1.0 ], 9.0, Comp::Less ), Constraint::new( vec![ 1.0, 2.0 ], 9.0, Comp::Less ) ],
      Vec::new(), 
      Vec::new()
    );
    let c = Constraint::new( vec![ 1.0, 2.0 ], 4.0, Comp::Greater );
    assert_eq!( c.value, 4.0 );

    let solution = SimplexSolver{}.solve( p );
    assert_eq!( solution.point, vec![ 3.0, 3.0 ] )
  }

  #[ test ]
  fn problem3d_2() 
  {
    let p = Problem::new
    ( 
      vec![ 0.0, 0.0, 1.0 ], 
      vec!
      [ 
        Constraint::new( vec![ 1.0, 2.0, 0.0 ], 2.0, Comp::Less ), 
        Constraint::new( vec![ 0.0, 3.0, 1.0 ], 3.0, Comp::Less ),
        Constraint::new( vec![ 3.0, 0.0, 2.0 ], 6.0, Comp::Less ),
      ],
      Vec::new(), 
      Vec::new()
    );

    let solution = SimplexSolver{}.solve( p );
    assert_eq!( solution.point, vec![ 0.0, 0.0, 3.0 ] )
  }

  #[ test ]
  fn problem_draw() 
  {
    let mut p = Problem::new
    ( 
      vec![ 3.0, 2.0 ], 
      vec![ Constraint::new( vec![ 2.0, 1.0 ], 9.0, Comp::Less ), Constraint::new( vec![ 1.0, 2.0 ], 9.0, Comp::Less ) ],
      Vec::new(), 
      Vec::new()
    );

    let ex_points = SimplexSolver::extreme_points(&mut p);
    drawing::draw_problem(&p, ex_points);
  }

}