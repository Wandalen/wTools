use std::{vec, collections::HashSet};

use iter_tools::Itertools;

#[ derive( Clone ) ]
pub struct Problem 
{
    var_coeffs : Vec< f64 >,
    constraints : Vec< Constraint >,
    mins : Vec< f64 >,
    maxs : Vec< f64 >
}

impl Problem 
{
  pub fn new( var_coeffs : Vec< f64 >, constraints : Vec< Constraint >, mins : Vec< f64 >, maxs : Vec< f64 > ) -> Self
  {
    Self { var_coeffs, constraints, mins, maxs }
  }
}

#[ derive( Clone ) ]
pub struct Constraint 
{
  coefs : Vec< f64 >,
  value : f64,
  comparison : Comp,
}

#[ derive( Clone ) ]
pub enum Comp
{
  Less,
  Greater,
  Equal,
}

impl Constraint 
{
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

#[ derive( Clone, Debug, PartialEq ) ]
struct ExtremePoint
{
  /// Basic variables indices.
  bv : Vec< usize >,
  point : Vec< f64 >,
}

impl Default for ExtremePoint
{
  fn default() -> Self 
  {
    Self { bv : Vec::new(), point : Vec::new() }
  }
}

impl ExtremePoint
{
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
struct BasicSolution
{
  /// Non-basic variables indices.
  nbv : Vec< usize >,
  /// Basic variables indices.
  bv : Vec< usize >,
  bv_values : Vec< f64 >,
}

impl From< BasicSolution > for ExtremePoint
{
  fn from(solution: BasicSolution) -> Self {
    let m = solution.bv.len();
    let mut point = vec![ 0.0; m ];
    for index in 1..= m
    {
        if solution.bv.contains( &index )
        {
        point[ index ] = solution.bv_values[ solution.bv[ index ] ];
        }
    }
    Self
    {
        bv : solution.bv,
        point,
    }
  }
}

pub struct SimplexSolver {}

impl SimplexSolver
{
  fn basic_feasible_solutions( p : Problem ) -> Vec< BasicSolution >
  {
    let total_variables_number = p.var_coeffs.len() + p.constraints.len();
    let basic_variables_number = p.var_coeffs.len();
    let non_basic_variables_number = p.constraints.len();
    let number_of_basic_solutions : u128 = ( 1..total_variables_number as u128 ).product::< u128 >() 
      / ( ( 1..basic_variables_number as u128 ).product::< u128 >() * ( 1..non_basic_variables_number as u128 ).product::< u128 >() );

        
    let mut equations_coefficients = Vec::new();
    for i in 1..= p.constraints.len()
    {
      let mut coeffs = p.constraints[ i - 1 ].coefs.clone();
      for _ in 1..=p.constraints.len()
      {
        coeffs.push( 0.0 );
      }
      match p.constraints[ i-1 ].comparison
      {
        Comp::Less => 
        {
            coeffs[ p.constraints.len() + i - 1 ] = 1.0;
        }
        Comp::Greater =>
        {
            coeffs[ p.constraints.len() + i - 1 ] = -1.0;
        }
        Comp::Equal => {}
      }
      equations_coefficients.push( coeffs );
    }

    let mut bs = vec![ BasicSolution 
      { 
        bv_values: vec![ 0.0; basic_variables_number ], 
        bv: vec![ 0; basic_variables_number ], 
        nbv: vec![ 0; non_basic_variables_number ]
      }; 
      number_of_basic_solutions as usize ];

    let mut iter = bs.iter_mut();
    for i in 1..=total_variables_number 
    {
      for j in i..=total_variables_number
      {
        if i != j  {
          ( *iter.next().unwrap() ).nbv = vec![ i, j ];
        }
      }
    }

    for basic_solution in bs.iter_mut() 
    {
      let mut e = 1;
      for basic_var in basic_solution.bv.iter_mut()
      {
        loop {
          if !basic_solution.nbv.contains( &e ) 
          {
            *basic_var = e;
            e+= 1;
            break;
          } 
          else { e += 1; }
        }
      }
    }

    for basic_solution in bs.iter_mut() 
    {
      let mut vec_of_coeffs = Vec::new();
        
      for bv in basic_solution.bv.iter() 
      {
        for i in 0..basic_solution.bv.len() 
        {
          vec_of_coeffs.push( equations_coefficients[ i ][ bv - 1 ] );
        }
      }
      let dimension = basic_solution.bv.len();

      let m = nalgebra::DMatrix::from_vec( dimension, dimension, vec_of_coeffs );
      let inverse = m.try_inverse().unwrap();
      let const_m = nalgebra::DMatrix::from_vec( dimension, 1, p.constraints.iter().map(|c| c.value).collect::<Vec<_>>());
      let solutions = inverse * const_m;
      basic_solution.bv_values = solutions.iter().map(|a| *a).collect_vec();
    }

    dbg!( bs.into_iter().filter_map( | b_s | 
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
    ).collect_vec())

    }

    fn solve( &self, p : Problem )
    {
      let m = p.constraints.len();
      let bfs = Self::basic_feasible_solutions( p.clone() );

    //   let extreme_points = bfs.into_iter().map( | solution |
    //     {
    //       let mut ex_point = vec![ 0.0; m ];
    //       for i in 1..=m
    //       {
    //         if solution.bv.contains( &i )
    //         {
    //           ex_point[i] = solution.bv_values[ solution.bv[ i ] ];
    //         }
    //       }
            
    //     } ).collect_vec();
      let mut max_z = f64::MIN;
      let mut max_point = bfs[0].clone().into();
      let extreme_points = bfs.into_iter().map( | s | s.into() ).collect::< Vec< ExtremePoint > >();
      let mut ex_point = extreme_points[0].clone();
      let mut old_point = ExtremePoint::default();
      loop
      {
        let mut z = 0.0;
        for i in 0..p.var_coeffs.len()
        {
          z += p.var_coeffs[ i ] * ex_point.point[ i ];
        }
        if z > max_z 
        {
            max_z = z;
            max_point = ex_point.clone();
            let new_ex_point = extreme_points.iter().find( | p | p.is_adjacent( &ex_point ) ).unwrap().clone();
            //todo
        }
      }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constraint() {
        let c = Constraint::new(vec![1.0, 2.0], 4.0, Comp::Greater);
        assert_eq!(c.value, 4.0);
    }

    #[test]
    fn problem() {
        let p = Problem::new( vec![3.0,2.0], vec![
            Constraint::new(vec![2.0, 1.0], 9.0, Comp::Less),
            Constraint::new(vec![1.0, 2.0], 9.0, Comp::Less),
            ], Vec::new(), Vec::new());
        let c = Constraint::new(vec![1.0, 2.0], 4.0, Comp::Greater);
        assert_eq!(c.value, 4.0);

        let solve = SimplexSolver::bacis_feasible_solutions(p);
    }

}