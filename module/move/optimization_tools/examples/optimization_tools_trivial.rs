//! Example of hybrid optimization for custom problem, with implementation of traits for hybrid optimization.
//! 
//! Problem: Given a set of items, each with a weight, determine the subset of items with the total weight which is closest to a given baseline.
//! 


use optimization_tools::hybrid_optimizer::*;

use deterministic_rand::{ Hrng, Rng, seq::IteratorRandom };
use iter_tools::Itertools;

// Create struct that represents candidate solution and implement trait Individual for it.
#[ derive( Debug, PartialEq, Clone ) ]
pub struct SubsetPerson 
{
  pub subset : Vec< bool >,
  pub value_diff : usize,
}

impl SubsetPerson
{
  pub fn new( subset : Vec< bool > ) -> Self
  {
    Self { subset, value_diff : 0 }
  }
}

impl Individual for SubsetPerson
{
  fn fitness( &self ) -> usize
  {
    self.value_diff
  }

  fn is_optimal( &self ) -> bool 
  {
    self.value_diff == 0
  }

  fn update_fitness( &mut self, value : f64 ) 
  {
    self.value_diff = value as usize;
  }
}

// Create struct that represents problem, and implement trait InitialProblem for it.
// Associated item is SubsetPerson created above.
#[ derive( Debug, Clone ) ]
pub struct SubsetProblem
{
  pub items : Vec< usize >,
  pub baseline : usize,
}

impl InitialProblem for SubsetProblem
{
  type Person = SubsetPerson;

  fn get_random_person( &self, hrng : Hrng ) -> SubsetPerson 
  {
    let mut subset = vec![ false; self.items.len() ];

    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    let number_of_elements = rng.gen_range( 1..subset.len() );
    let positions = ( 0..subset.len() ).choose_multiple( &mut *rng, number_of_elements );

    for position in positions
    {
      subset[ position ] = true;
    }

    let mut person = SubsetPerson::new( subset );
    let diff = self.evaluate( &person );
    person.update_fitness( diff );

    person
  }

  fn evaluate( &self, person : &SubsetPerson ) -> f64 
  {
    let mut sum = 0;
    for i in 0..person.subset.len()
    {
      if person.subset[ i ] == true
      {
        sum += self.items[ i ];
      }
    }

    self.baseline.abs_diff( sum ) as f64
  }
}

// Create crossover operator for custom problem, implement CrossoverOperator trait for it.
#[ derive( Debug, Clone ) ]
pub struct SubsetCrossover;
impl CrossoverOperator for SubsetCrossover
{
  type Person = SubsetPerson;
  fn crossover( &self, hrng : Hrng, parent1 : &Self::Person, parent2 : &Self::Person ) -> Self::Person 
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    let point = ( 1..parent1.subset.len() - 2 ).choose( &mut *rng ).unwrap();
    let child = parent1.subset.iter().cloned().take( point ).chain( parent2.subset.iter().cloned().skip( point ) ).collect_vec();

    SubsetPerson::new( child )
  }
}

// Create mutation operator for custom problem, implement MutationOperator trait for it.
#[ derive( Debug, Clone ) ]
pub struct SubsetMutation;
impl MutationOperator for SubsetMutation
{
  type Person = SubsetPerson;
  type Problem = SubsetProblem;

  fn mutate( &self, hrng : Hrng, person : &mut Self::Person, _context : &Self::Problem ) 
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    //remove random item
    loop 
    {
      let position = ( 0..person.subset.len() ).choose( &mut *rng ).unwrap();
      if person.subset[ position ] == true
      {
        person.subset[ position ] = false;
        break;
      }
    }

    //add random item
    loop 
    {
      let position = ( 0..person.subset.len() ).choose( &mut *rng ).unwrap();
      if person.subset[ position ] == false
      {
        person.subset[ position ] = true;
        break;
      }
    }
  }
}

fn main()
{
  // Initialize custom problem.
  let items = vec![ 3, 5, 9, 12, 43, 32, 18 ];
  let init_problem = SubsetProblem { items : items.clone(), baseline : 41 }; 
  
  // Initialize hybrid optimization problem, using custom problem and custom operators.
  let problem = Problem::new( init_problem, SubsetCrossover, SubsetMutation );  
    
  // Create new hybrid optimizer with default configuration, and hybrid optimization problem.
  let optimizer = HybridOptimizer::new( Config::default(), problem )
  // If desired, update certain configuration values for optimizer.
  .set_population_size( 100 )
  .set_dynasties_limit( 100 );
  
  // Perform optimization of given problem. Result includes best found solution and reason for termination 
  // of optimization process.
  let ( reason, solution ) = optimizer.optimize();
  
  // Print results.
  println!( "reason : {:?}", reason );
  
  if let Some( solution ) = solution
  {
    print!( "subset : " );
    for i in 0..solution.subset.len()
    {
      if solution.subset[ i ] == true
      {
        print!("{} ", items[ i ] );
      }
    }
    println!();
    println!( "difference : {:?}", solution.value_diff );
  }
}