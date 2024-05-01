//! Contains implementation of hybrid optimization using Simulated Annealing and Genetic optimization methods.
//! 

use crate::*;
#[ cfg( feature="static_plot" ) ]
use crate::plot::{ PlotDescription, PlotOptions, plot };
use iter_tools::Itertools;
use std::ops::RangeInclusive;
use rayon::iter::{ ParallelIterator, IndexedParallelIterator};
use deterministic_rand::{ Seed, seq::{ SliceRandom, IteratorRandom } };
use derive_tools::exposed::Display;
use optimal_params_search::OptimalProblem;

mod gen_alg;
pub use gen_alg::*;
mod sim_anneal;
pub use sim_anneal::*;

/// Pause execution of optimizer.
pub fn sleep()
{
  std::thread::sleep( std::time::Duration::from_secs( 5 ) );
}

/// Represents the reasons for the termination or proceeding with the Sudoku solving.
#[ derive( PartialEq, Eq, Clone, Copy, Debug, Display ) ]
pub enum Reason
{
  /// Optimization process was finished with optimal result.
  GoodEnough,
  /// Optimization process finished due to reaching limit of resets.
  ResetLimit,
  /// Optimization process finished due to reaching limit of dynasties.
  DynastiesLimit,
}

/// Configuration for Hybrid Optimizer.
#[ derive( Debug ) ]
pub struct Config
{
  /// Max amount of mutations in dynasty.
  pub sa_mutations_per_dynasty_limit : usize,

  /// Max allowed number of resets.
  pub reset_limit : usize,

  /// Number of fittest individuals that will be cloned to new population.
  pub elite_selection_rate : f64,

  /// Number of individuals that will be replaced by crossover operation.
  pub crossover_rate : f64,

  /// Probabilistic measure of a individual mutation likelihood.
  pub mutation_rate : f64,

  /// Recalculate fitness on every iteration.
  pub fitness_recalculation : bool,

  /// Max number of iteration without improvement in population.
  pub max_stale_iterations : usize,

  /// Hierarchical random numbers generator.
  pub hrng : Hrng,

  /// Percent of population selected for next cycle of optimization.
  pub population_percent : f64,

  /// Max number of dynasties, termination condition.
  pub dynasties_limit : usize,

  /// Number of Individuals in initial generation of solutions.
  pub population_size : usize,
}

impl Default for Config
{
  fn default() -> Self 
  {
    Self
    {
        max_stale_iterations : 100,
        sa_mutations_per_dynasty_limit : 300,
        reset_limit : 1_000,
        crossover_rate : 0.5,
        fitness_recalculation : false,
        mutation_rate : 0.25,
        elite_selection_rate : 0.25,
        hrng : Hrng::master_with_seed( Seed::default() ),
        dynasties_limit : 10_000,
        population_size : 10_000,
        population_percent : 1.0,
    }
  }
}

/// Specific optimization problem for Hybrid Optimizer.
#[ derive( Debug ) ]
pub struct Problem< S : InitialProblem, C, M >
{
  /// Temperature update operator.
  pub sa_temperature_schedule : Box< dyn TemperatureSchedule >,

  /// Crossover genetic operator, which defines how new Individuals are produced by combiniting traits of Individuals from current generation.
  pub ga_crossover_operator : C,

  /// Selection genetic operator, which defines how Individuals from current generation are selected to be breeders of new generation.
  pub ga_selection_operator : Box< dyn SelectionOperator< < S as InitialProblem >::Person > >,

  /// Struct responsible for creation of initial population.
  pub seeder : S,

  /// Mutation operator, randomly changes person's genome to introduce diversity into population.
  pub mutation_operator : M,
}

impl< S : InitialProblem, C, M > Problem< S, C, M >
{
  /// Create new instance of optimization problem for Hybrid Optimizer.
  pub fn new( initial : S, crossover_operator : C, mutation_operator : M ) -> Self
  where TournamentSelection : SelectionOperator< < S as InitialProblem >::Person >
  {
    let selection_operator = Box::new( TournamentSelection
    {
      size : 2,
      selection_pressure : 0.85,
    } );

    Self
    {
      seeder : initial,
      sa_temperature_schedule : Box::new( LinearTempSchedule
        {
          coefficient : ( 0.999 ).into(),
          constant : 0f64.into(),
          reset_increase_value : 1f64.into()
        } ),
      ga_crossover_operator : crossover_operator,
      ga_selection_operator : selection_operator,
      mutation_operator : mutation_operator,
    }
  }
}

/// Represents hybrid optimization method with both Simulated Annealing and Genetic Algorithm.
#[ derive( Debug ) ]
pub struct HybridOptimizer< S : InitialProblem, C, M >
{
  /// Configuration of Hybrid Optimizer.
  config : Config,

  /// Specific optimization problem.
  problem : Problem< S, C, M >,
}

impl< S : InitialProblem + Sync, C : CrossoverOperator::< Person = < S as InitialProblem>::Person >, M > HybridOptimizer< S, C, M >
where M : MutationOperator::< Person = < S as InitialProblem >::Person > + Sync,
  M : MutationOperator::< Problem = S > + Sync
{
  /// Create new instance of hybrid optimizer using given problem and configuration.
  pub fn new( config : Config, problem : Problem<S, C, M> ) -> Self
  {
    Self
    {
      config,
      problem,
    }
  }
  /// Set size of initial population.
  pub fn set_population_size( mut self, size : usize ) -> Self
  {
    self.config.population_size = size;
    self
  }

  /// Set max dynasties number.
  pub fn set_dynasties_limit( mut self, limit : usize ) -> Self
  {
    self.config.dynasties_limit = limit;
    self
  }

  /// Set temperature schedule for optimization.
  pub fn set_sa_temp_schedule( mut self, schedule : Box< dyn TemperatureSchedule > ) -> Self
  {
    self.problem.sa_temperature_schedule = schedule;
    self
  }

  /// Set selection operator.
  pub fn set_selection_operator( mut self, selection_op : Box< dyn SelectionOperator< < S as InitialProblem >::Person > > ) -> Self
  {
    self.problem.ga_selection_operator = selection_op;
    self
  }

  /// Set max amount of mutations per one dynasty.
  pub fn set_sa_max_mutations_per_dynasty( mut self, number : usize ) -> Self
  {
    self.config.sa_mutations_per_dynasty_limit = number;
    self
  }

  /// Set mutation rate for GA.
  pub fn set_population_proportions( mut self, proportions : PopulationModificationProportions< f64, f64, f64 > ) -> Self
  {
    self.config.mutation_rate = proportions.mutation_rate();
    self.config.elite_selection_rate = proportions.elite_selection_rate();
    self.config.crossover_rate = proportions.crossover_rate();
    self
  }

  /// Set stale iterations limit.
  pub fn set_max_stale_iterations( mut self, limit : usize ) -> Self
  {
    self.config.max_stale_iterations = limit;
    self
  }

  /// Perform hybrid SA/GA optimization.
  pub fn optimize( &self ) -> ( Reason, Option< < S as InitialProblem >::Person > )
  {
    let mut population = self.problem.seeder.initial_population( self.config.hrng.clone(), self.config.population_size );
    population.sort_by( | p1, p2 | p1.fitness().cmp( &p2.fitness() ) );
    let mut dynasties_number = 0;
    let mut stale_generations = 0;
    let mut prev_best = population[ 0 ].clone();
    let mut temperature = self.initial_temperature();
    let mut reset_number = 0;

    loop
    {
      if dynasties_number > self.config.dynasties_limit
      {
        
        return ( Reason::DynastiesLimit, [ prev_best, population[ 0 ].clone() ].into_iter().min_by( | p1, p2 | p1.fitness().cmp( &p2.fitness() ) ) );
      }

      if self.population_has_solution( &population )
      {
        return ( Reason::GoodEnough, Some( population[ 0 ].clone() ) );
      }

      if reset_number > self.config.reset_limit
      {
        population = self.problem.seeder.initial_population( self.config.hrng.clone(), self.config.population_size );
        temperature = self.initial_temperature();
      }

      if stale_generations > self.config.max_stale_iterations
      {
        if temperature > self.initial_temperature()
        {
          population = self.problem.seeder.initial_population( self.config.hrng.clone(), self.config.population_size );
          temperature = self.initial_temperature();
          reset_number = 0;
        }
        else
        {
          temperature = self.problem.sa_temperature_schedule.reset_temperature( temperature );
          reset_number += 1;
        }
      }
      
      if population[ 0 ].fitness() < prev_best.fitness()
      {
        stale_generations = 0;
        
        {
          prev_best = population[ 0 ].clone();
        }
      }
      else
      {
        stale_generations += 1;
      }

      let mut new_population = Vec::with_capacity( population.len() );

      new_population.extend(
        population
        .iter()
        .cloned()
        .take( ( ( population.len() as f64 ) * self.config.elite_selection_rate ) as usize )
     );
      for i in ( ( ( population.len() as f64 ) * self.config.elite_selection_rate ) as usize )..population.len()
      {
        let mut person = self.evolve( population[ i ].clone(), &population, &temperature );

        person.update_fitness( self.problem.seeder.evaluate( &person ) );
        if person.is_optimal()
        {
          return ( Reason::GoodEnough, Some( person.clone() ) );
        }

        new_population.push( person );
      }

      new_population.sort_by( | p1, p2 | p1.fitness().cmp( &p2.fitness() ) );
      temperature = self.problem.sa_temperature_schedule.calculate_next_temp( temperature );

      population = new_population.into_iter().take( ( population.len() as f64 * self.config.population_percent ) as usize ).collect_vec();
      
      dynasties_number += 1;
    }
  }

  /// Check if candidate person represents vital state.
  fn is_vital
  ( 
    &self, 
    person : &< S as InitialProblem >::Person, 
    candidate : &< S as InitialProblem >::Person, 
    temperature : &Temperature 
  ) -> bool
  {
    let rng_ref = self.config.hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    let cost_difference = 0.5 + candidate.fitness() as f64 - person.fitness() as f64;
    let threshold = ( - cost_difference / temperature.unwrap() ).exp();

    let rand : f64 = rng.gen();
    rand < threshold  
  }

  /// Check if population has solution.
  fn population_has_solution( &self, population : &Vec< < S as InitialProblem >::Person > ) -> bool
  {
    for person in population
    {
      if person.is_optimal()
      {
        return true;
      }
    }
    false
  }

  /// Update person using crossover operator or mutation.
  fn evolve
  ( 
    &self, 
    person : < S as InitialProblem >::Person, 
    population : &Vec< < S as InitialProblem >::Person >,
    temperature : &Temperature,
  ) -> < S as InitialProblem >::Person
  {
 
    let rng_ref = self.config.hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    let operator = [ ( 0, self.config.mutation_rate ), ( 1, self.config.crossover_rate ) ]
    .choose_weighted( &mut *rng, | item | item.1 )
    .unwrap()
    .0
    ; 
    drop( rng );

    let mut child = 
    if operator == 1
    {
      let parent1 = self.problem.ga_selection_operator.select( self.config.hrng.clone(), &population );
      let parent2 = self.problem.ga_selection_operator.select( self.config.hrng.clone(), &population );
      let candidate = self.problem.ga_crossover_operator.crossover( self.config.hrng.clone(), parent1, parent2 );
      if self.is_vital( &person, &candidate, temperature )
      {
        candidate
      }
      else
      {
        person.clone()
      }
    }
    else
    {
      let mut n_mutations : usize = 0;
      let mut expected_number_of_mutations = 4;

      loop
      {
        if n_mutations > self.config.sa_mutations_per_dynasty_limit
        {
          {
            return person.clone();
          }
        }
    
        let hrng = self.config.hrng.clone();
        let mutation_op = &self.problem.mutation_operator;
        let mutation_context = &self.problem.seeder;

        let candidates = rayon::iter::repeat( () )
        .take( expected_number_of_mutations )
        .enumerate()
        .map( | ( i, _ ) | hrng.child( i ) )
        .flat_map( | hrng | 
          {
            let mut candidate = person.clone();
            mutation_op.mutate( hrng.clone(), &mut candidate, mutation_context );
        
            let rng_ref = hrng.rng_ref();
            let mut rng = rng_ref.lock().unwrap();
        
            let cost_difference = 0.5 + candidate.fitness() as f64 - person.fitness() as f64;
            let threshold = ( - cost_difference / temperature.unwrap() ).exp();
        
            log::trace!
            (
              "cost : {}  | cost_difference : {cost_difference} | temperature : {}",
              person.fitness(),
              temperature,
            );
            let rand : f64 = rng.gen();
            let vital = rand < threshold;  
            if vital
            {
              let emoji = if cost_difference > 0.0
              {
                "🔼"
              }
              else if cost_difference < 0.0
              {
                "✔️"
              }
              else
              {
                "🔘"
              };
              log::trace!( " {emoji} vital | rand( {rand} ) < threshold( {threshold} )" );
              if cost_difference == 0.0
              {
                // sleep();
              }
              Some( candidate )
            }
            else
            {
              log::trace!( " ❌ non-vital | rand( {rand} ) > threshold( {threshold} )" );
              None
            }
              
          } )
        .collect::< Vec< _ > >()
        ;

        if candidates.len() > 0
        {
          let rng_ref = self.config.hrng.rng_ref();
          let mut rng = rng_ref.lock().unwrap();
          
          if let Some( index ) = ( 0..candidates.len() - 1 ).choose( &mut *rng )
          {
            break candidates[ index ].clone()
          }
          else 
          {
            break candidates[ 0 ].clone()
          }
        }

        n_mutations += expected_number_of_mutations;
        if expected_number_of_mutations < 32
        {
          expected_number_of_mutations += 4;
        }
      }
    };

    if self.config.fitness_recalculation
    {
      child.update_fitness( self.problem.seeder.evaluate( &child ) );
    }

    child
  }

  /// Calculate the initial temperature for the optimization process.
  pub fn initial_temperature( &self ) -> Temperature
  {
    use statrs::statistics::Statistics;
    let rand_person = self.problem.seeder.get_random_person( self.config.hrng.clone() );
    const N : usize = 16;
    let mut costs : [ f64 ; N ] = [ 0.0 ; N ];
    for i in 0..N
    {
      let mut person2 = rand_person.clone();
      self.problem.mutation_operator.mutate( self.config.hrng.clone(), &mut person2, &self.problem.seeder );
      costs[ i ] = self.problem.seeder.evaluate( &person2 ) as f64;
    }
    costs[..].std_dev().into()
  }

}

/// Starting parameters for optimal parameters search for hybrid optimization configuration.
pub fn starting_params_for_hybrid() -> Result< OptimalProblem< RangeInclusive< f64 > >, optimal_params_search::Error >
{
  let opt_problem = OptimalProblem::new()
  .add( Some( String::from( "temperature decrease factor" ) ), Some( 0.0..=1.0 ), Some( 0.999 ), Some( 0.0002 ) )?
  .add( Some( String::from( "mutation per dynasty" ) ), Some( 10.0..=200.0 ), Some( 100.0 ), Some( 20.0 ) )?
  .add( Some( String::from( "mutation rate" ) ), Some( 0.0..=1.0 ), Some( 0.25 ), Some( 0.1 ) )?
  .add( Some( String::from( "crossover rate" ) ), Some( 0.0..=1.0 ), Some( 0.5 ), Some( 0.2 ) )?
  .add( Some( String::from( "max stale iterations" ) ), Some( 1.0..=100.0 ), Some( 30.0 ), Some( 5.0 ) )?
  .add( Some( String::from( "population size" ) ), Some( 1.0..=1000.0 ), Some( 300.0 ), Some( 200.0 ) )?
  .add( Some( String::from( "dynasties limit" ) ), Some( 100.0..=2000.0 ), Some( 1000.0 ), Some( 300.0 ) )?
  ;

  Ok( opt_problem )
}

/// Starting parameters for optimal parameters search for SA optimization configuration.
pub fn starting_params_for_sa() -> Result< OptimalProblem< RangeInclusive< f64 > >, optimal_params_search::Error >
{
  let opt_problem = OptimalProblem::new()
  .add( Some( String::from( "temperature decrease factor" ) ), Some( 0.0..=1.0 ), Some( 0.999 ), Some( 0.0002 ) )?
  .add( Some( String::from( "mutation per dynasty" ) ), Some( 10.0..=200.0 ), Some( 100.0 ), Some( 20.0 ) )?
  .add( Some( String::from( "mutation rate" ) ), Some( 1.0..=1.0 ), Some( 1.0 ), Some( 0.0 ) )?
  .add( Some( String::from( "crossover rate" ) ), Some( 0.0..=0.0 ), Some( 0.0 ), Some( 0.0 ) )?
  .add( Some( String::from( "max stale iterations" ) ), Some( 1.0..=100.0 ), Some( 30.0 ), Some( 5.0 ) )?
  .add( Some( String::from( "population size" ) ), Some( 1.0..=1.0 ), Some( 1.0 ), Some( 0.0 ) )?
  .add( Some( String::from( "dynasties limit" ) ), Some( 100.0..=5000.0 ), Some( 1000.0 ), Some( 300.0 ) )?
  ;

  Ok( opt_problem )
}

/// Starting parameters for optimal parameters search for GA optimization configuration.
pub fn starting_params_for_ga() -> Result< OptimalProblem< RangeInclusive< f64 > >, optimal_params_search::Error >
{
  let opt_problem = OptimalProblem::new()
  .add( Some( String::from( "temperature decrease factor" ) ), Some( 0.0..=1.0 ), Some( 0.999 ), Some( 0.0002 ) )?
  .add( Some( String::from( "mutation per dynasty" ) ), Some( 10.0..=200.0 ), Some( 100.0 ), Some( 20.0 ) )?
  .add( Some( String::from( "mutation rate" ) ), Some( 0.1..=1.0 ), Some( 0.25 ), Some( 0.1 ) )?
  .add( Some( String::from( "crossover rate" ) ), Some( 0.1..=1.0 ), Some( 0.5 ), Some( 0.2 ) )?
  .add( Some( String::from( "max stale iterations" ) ), Some( 1.0..=100.0 ), Some( 30.0 ), Some( 5.0 ) )?
  .add( Some( String::from( "population size" ) ), Some( 10.0..=2000.0 ), Some( 300.0 ), Some( 200.0 ) )?
  .add( Some( String::from( "dynasties limit" ) ), Some( 100.0..=2000.0 ), Some( 1000.0 ), Some( 300.0 ) )?
  ;

  Ok( opt_problem )
}