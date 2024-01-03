//! Parser for linear programming problem.
//! 

use super::linear_problem::{ Problem, Variable, Constraint, Comp };
use exmex::{ prelude::*, ops_factory, BinOp, MakeOperators, Operator };
use iter_tools::Itertools;
use std::collections::HashSet;

/// Parses linear programming problem from str to Problem struct.
#[ derive( Debug ) ]
pub struct ProblemParser {}

impl ProblemParser
{
  /// Creates Problem struct from objective function and constraints passed as string slices.
  pub fn parse( opt_function : &str, constraints_str : Vec< &str > ) -> Problem
  {
    ops_factory!
    (
      BitwiseOpsFactory,
      bool,
      Operator::make_bin
      (
        "<=",
        BinOp 
        {
          apply : | a, b | a <= b,
          prio : 0,
          is_commutative : false,
        }
      )
    );
  
    let mut z_coeffs = Vec::new();
    
    let z_expr = FlatEx::< f64 >::parse( opt_function ).unwrap();
    let var_number = z_expr.var_indices_ordered().len();
    let var_names = z_expr.var_names().into_iter().cloned().collect::< HashSet< _ > >();
    for val in 0..var_number
    {
      let deep_ex = z_expr.clone().to_deepex().unwrap();
      let coeff = deep_ex.partial( val ).unwrap();
      z_coeffs.push( coeff.eval( vec![ 0.0; var_number ].as_slice() ).unwrap() );
    }
      
    let mut constraints = Vec::new();
    for constraint in &constraints_str
    {
      let mut left_hand = "";
      let mut right_hand = "";
      let mut comp = Comp::Less;
      if constraint.contains( "<=" )
      {
        ( left_hand, right_hand ) = constraint.split( "<=" ).collect_tuple().unwrap();
      }
    
      if constraint.contains( ">=" )
      {
        ( left_hand, right_hand ) = constraint.split( ">=" ).collect_tuple().unwrap();
        comp = Comp::Greater;
      }
        
      let mut coeffs = Vec::new();
      let mut expr = FlatEx::< f64 >::parse( left_hand ).unwrap();
        
      let con_var_names = expr.var_names();
      let con_var_names = con_var_names.into_iter().cloned().collect::< HashSet< _ > >();
      let unused_vars = var_names.difference( &con_var_names );
      for unused_var in unused_vars
      {
        expr = expr.operate_binary( FlatEx::< f64 >::parse
        (
          ( String::from( "0*" ) + unused_var ).as_str() 
        ).unwrap(), "+" )
        .unwrap()
        ;
      }
      let var_number = expr.var_indices_ordered().len();
      for val in 0..var_number
      {
        let deep_ex = expr.clone().to_deepex().unwrap();
        let coeff = deep_ex.partial( val ).unwrap();
        coeffs.push( coeff.eval( vec![ 0.0; var_number ].as_slice() ).unwrap() );
      }
      constraints.push( Constraint 
      {
        coefs : coeffs,
        value : FlatEx::< f64 >::parse( right_hand ).unwrap().eval( &[] ).unwrap(),
        comparison : comp,
      } );
    }

    let variables = z_coeffs
    .into_iter()
    .map( | coeff | Variable::new( coeff ).min( 0.0 ) )
    .collect_vec()
    ;

    Problem::new
    (
      variables,
      constraints,
    )
  }
}