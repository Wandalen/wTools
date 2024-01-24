//! Solver of linear programming problems by Simplex Method.
//! 

use std::collections::HashSet;
use iter_tools::Itertools;
use super::linear_problem::{ Problem, BasicSolution };

/// Extreme point of feasible region.
#[ derive( Clone, Debug ) ]
pub struct ExtremePoint
{
  /// Basic variables indices.
  bv : Vec< usize >,
  /// Extreme point coordinates.
  pub point : Vec< f64 >,
  /// Value of function to optimize.
  z : f64,
}

impl PartialEq for ExtremePoint {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

impl Eq for ExtremePoint {}

impl Default for ExtremePoint
{
  fn default() -> Self 
  {
    Self { bv : Vec::new(), point : Vec::new(), z : f64::MAX }
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
  /// Calculates extreme points of linear problem.
  pub fn extreme_points ( p : &mut Problem ) -> Vec< ExtremePoint >
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

    let mut final_result = Vec::with_capacity(number_of_basic_solutions as usize);
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
  pub fn solve( &self, p : Problem ) -> Vec< ExtremePoint >
  {
    let basic_variables_number = p.var_coeffs.len();

    let p = p.normalized();
    let mut table = Vec::new();

    let mut z_coeff = p.variables.iter().map( | var | -var.coefficient ).collect_vec();

    z_coeff.push( 0.0 );
    table.push( z_coeff );

    for i in 0..p.coeffs.shape()[ 0 ]
    {
      let vec_rhs = p.coeffs.row( i ).clone();
      let mut vec_rhs = vec_rhs.to_slice().unwrap().to_vec();
      vec_rhs.push( p.rhs[ i ] );
      table.push( vec_rhs );
    }

    let mut points = Vec::new();
    let mut candidate = ExtremePoint::default();

    loop 
    {
      let mut bv_pos = Vec::new();
      let mut nbv_pos = Vec::new();

      for j in 0..table[ 0 ].len() - 1
      {
        let mut is_bv = true;
        let mut non_zero_count = 0;
        for i in 1..table.len()
        {
          if table[ i ][ j ].abs() != 0.0
          {
            non_zero_count += 1;
            if table[ i ][ j ].abs() != 1.0
            {
              is_bv = false;
            }
          } 
        }

        if is_bv && non_zero_count == 1
        {
          bv_pos.push( j + 1 );
          for i in 1..table.len()
          {
            if table[ i ][ j ] == -1.0
            {
              for k in 0..table[ 0 ].len()
              {
                table[ i ][ k ] = - table[ i ][ k ];
              }
              }
          }
        } 
        else
        {
          nbv_pos.push( j + 1 );
        }
      }

      let mut initial_bs = BasicSolution 
      { 
        bv_values: vec![ -1.0; basic_variables_number ], 
        bv: bv_pos, 
        nbv: nbv_pos,
      };

      let rows = initial_bs.nbv.len();
      let columns = initial_bs.bv.len();

      let mut m = ndarray::Array::zeros( ( rows, columns ) );
      for ( index, bv ) in initial_bs.bv.iter().enumerate() 
      {
        for i in 0..m.shape()[ 1 ] 
        {
          m.row_mut( i )[ index ] = p.coeffs.row( i )[ bv - 1 ];
        }
      }
    
      let b = faer::Mat::from_fn( p.rhs.len(), 1, | i, _ | p.rhs[ i ] );
      let m = faer::IntoFaer::into_faer( m.view() );
      let lu = faer::FaerMat::partial_piv_lu( &m );
        
      let solution = faer::sparse::solvers::SpSolver::solve( &lu, &b );

      initial_bs.bv_values = solution.col_as_slice( 0 ).iter().map( | a | *a ).collect_vec();

      let initial_point = ExtremePoint::new( initial_bs.clone(), p.variables.iter().map( | var | var.coefficient ).collect_vec() );

      let mut min_coeff = f64::MAX;
      let mut pos = 0;
      for ( index, coeff ) in table[ 0 ].iter().enumerate()
      {
        if initial_bs.nbv.contains( &( index + 1 ) )
        {
          if coeff < &min_coeff
          {
            min_coeff = *coeff;
            pos = index + 1;
          }
        }
      }

      if candidate.z == initial_point.z
      {
        if !points.contains( &initial_point )
        {
          points.push(initial_point.clone());
        }
        break;
      }

      if min_coeff == 0.0
      {
        if !points.contains( &initial_point )
        {
          points.push(initial_point.clone());
        }
        if points.len() > initial_bs.bv.len()
        {
          break;
        }
      }

      if min_coeff > 0.0
      {
        points.push( initial_point.clone() );
        break;
      }
      candidate = initial_point;

      let mut var_row = 1;
      let mut r = table[ 1 ].last().unwrap() / table[ 1 ][ pos - 1 ];
      for i in 2..table.len()
      {
        let row_r = table[ i ].last().unwrap() / table[ i ][ pos - 1 ];
        if row_r < r 
        {
          r = row_r;
          var_row = i;
        }
      }
      

      let mut new_table = table.clone();
      for i in 0..table[ 0 ].len()
      {
        new_table[ var_row ][ i ] = table[ var_row ][ i ] / table[ var_row ][ pos - 1 ];
      }

      for i in 0..table.len()
      {
        if i == var_row
        {
          continue;
        }
        let coeff = table[ i ][ pos - 1 ];
        for j in 0..table[ 0 ].len()
        {
          new_table[ i ][ j ] = table[ i ][ j ] - new_table[ var_row ][ j ] * coeff;
        }
      }
      table = new_table;
    };

    points
  }
}
