//! Solver of linear programming problems by Simplex Method.
//! 

use std::{ vec, collections::{ HashSet, BinaryHeap } };
use iter_tools::Itertools;
use super::linear_problem::{ Problem, BasicSolution };

/// Extreme point of feasible region.
#[ derive( Clone, Debug, PartialEq ) ]
pub struct ExtremePoint
{
  /// Basic variables indices.
  bv : Vec< usize >,
  /// Extreme point coordinates.
  pub point : Vec< f64 >,
  /// Value of function to optimize.
  z : f64,
}

impl Eq for ExtremePoint {}

impl Default for ExtremePoint
{
  fn default() -> Self 
  {
    Self { bv : Vec::new(), point : Vec::new(), z : 0.0 }
  }
}

impl ExtremePoint
{
  /// Create new extreme point from basic solution and coeffiicients of function to optimize.
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

  /// Calculates basic feasible solutions for linear problem.
  fn basic_feasible_solutions( p : Problem ) -> Vec< BasicSolution >
  {
    let total_variables_number = p.var_coeffs.len() + p.constraints.len();
    let basic_variables_number = p.var_coeffs.len();
    let non_basic_variables_number = p.constraints.len();
    let number_of_basic_solutions : u128 = ( 1..=total_variables_number as u128 ).product::< u128 >() 
      / ( ( 1..=basic_variables_number as u128 ).product::< u128 >() * ( 1..=non_basic_variables_number as u128 ).product::< u128 >() );

    let p = p.normalized();

    let mut bs = vec![ BasicSolution 
      { 
        bv_values: vec![ -1.0; basic_variables_number ], 
        bv: vec![ 0; basic_variables_number ], 
        nbv: vec![ 0; non_basic_variables_number ]
      }; 
      number_of_basic_solutions as usize ];

    let mut result = ( 1..=total_variables_number )
    .into_iter()
    .map( | elem | { HashSet::from( [ elem ] ) } )
    .collect_vec()
    ;

    for _ in 0..basic_variables_number
    {
      result = ( 1..=total_variables_number )
      .cartesian_product( result ).map( | ( elem, mut set ) | 
      {
        set.insert( elem );
        set
      } )
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

      let mut m = ndarray::Array::zeros( ( rows, columns ) );
      for ( index, bv ) in basic_solution.bv.iter().enumerate() 
      {
        for i in 0..m.shape()[ 1 ] 
        {
          m.row_mut( i )[ index ] = p.coeffs.row( i )[ bv - 1 ];
        }
      }
      
      let b = faer::Mat::from_fn( p.rhs.len(), 1, | i, _ | p.rhs[ i ] );
      let m = faer::IntoFaer::into_faer( m.view() );
      let lu = faer::FaerMat::partial_piv_lu( &m );
      
      let solution = faer::sparse::solvers::SpSolver::solve(&lu, &b);

      basic_solution.bv_values = solution.col_as_slice(0).iter().map( | a | *a ).collect_vec();
    }

    bs.into_iter().filter( | bs | p.is_feasible_solution( bs ) ).collect_vec()

  }

  /// Solves linear problem using Simplex method.
  pub fn solve( &self, mut p : Problem ) -> ExtremePoint
  {
    let extreme_points = Self::extreme_points( &mut p );
    let mut queue: BinaryHeap<ExtremePoint> = extreme_points.into_iter().collect::< BinaryHeap< _ > >();
    let max_point = queue.pop().unwrap();

    max_point
  }
}

#[ cfg( test ) ]
mod simplex_tests {
  use super::*;
  use crate::simplex::
  {
    linear_problem::{ Problem, Constraint, Comp, Variable },
    drawing,
  };

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
      vec![ Variable::new( 3.0 ).min( 0.0 ), Variable::new( 2.0 ).min( 0.0 ) ], 
      vec![ Constraint::new( vec![ 2.0, 1.0 ], 9.0, Comp::Less ), Constraint::new( vec![ 1.0, 2.0 ], 9.0, Comp::Less ) ],
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
      vec![ Variable::new( 0.0 ).min( 0.0 ), Variable::new( 0.0 ).min( 0.0 ), Variable::new( 1.0 ).min( 0.0 ) ], 
      vec!
      [ 
        Constraint::new( vec![ 1.0, 2.0, 0.0 ], 2.0, Comp::Less ), 
        Constraint::new( vec![ 0.0, 3.0, 1.0 ], 3.0, Comp::Less ),
        Constraint::new( vec![ 3.0, 0.0, 2.0 ], 6.0, Comp::Less ),
      ],
    );

    let solution = SimplexSolver{}.solve( p );
    assert_eq!( solution.point, vec![ 0.0, 0.0, 3.0 ] )
  }

  #[ test ]
  fn problem_draw() 
  {
    let mut p = Problem::new
    ( 
      vec![ Variable::new( 3.0 ), Variable::new( 2.0 ) ], 
      vec![ Constraint::new( vec![ 2.0, 1.0 ], 9.0, Comp::Less ), Constraint::new( vec![ 1.0, 2.0 ], 9.0, Comp::Less ) ],
    );

    let ex_points = SimplexSolver::extreme_points( &mut p );
    let _ = drawing::draw_problem( &p, ex_points );
  }
  
  #[ cfg( feature = "lp_parse" ) ]
  #[ test ]
  fn problem_parse() 
  {
    let p = Problem::new
    ( 
      vec![ Variable::new( 2.0 ).min( 0.0 ), Variable::new( -3.0 ).min( 0.0 ), Variable::new( 4.0 ).min( 0.0 ) ], 
      vec!
      [ 
        Constraint::new( vec![ 2.0, -3.0, 1.0 ], 3.0, Comp::Less ), 
        Constraint::new( vec![ 1.0, -1.0, 0.0 ], 4.0, Comp::Less ) 
      ],
    );
    let parsed = crate::parser::ProblemParser::parse( "2*x - 3*y + 4*z", vec![ "2*x -3*y +z <= 3", "-y + x <=4" ] );
    
    assert_eq!( p.var_coeffs, parsed.var_coeffs );
    assert_eq!( p.constraints, parsed.constraints );
  }

}