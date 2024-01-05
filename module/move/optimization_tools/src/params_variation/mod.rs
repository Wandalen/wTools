use std::time::{ Instant, Duration };
use std::thread;
use std::sync::mpsc;

pub fn get_time_for_step< T, F >
( 
  proc : F,
  starting_value : T,
  step : T,
  number_of_iterations : usize, 
) -> Vec< ( Duration, T ) >
where F : Fn( T ), T : Clone + std::ops::Add<Output = T>
{
  let mut results: Vec<(Duration, T)> = Vec::new();
  let mut input = starting_value;
  loop
  {
    let mut current_results: Vec< Duration > = Vec::new();
    for _ in 0..number_of_iterations
    {
      let val = input.clone();
      let now = Instant::now();
      proc( val );
      let elapsed = now.elapsed();
      current_results.push( elapsed );
    }
    let size = current_results.len() as u128;
    let average = current_results
    .into_iter()
    .fold(0, | acc, elem | acc + elem.as_millis() / size )
    ;
    if results.len() == 0 || (*results.last().unwrap()).0.as_millis() >= average
    {
      results.push( ( Duration::from_millis( average as u64 ), input.clone() ) );

      input = input + step.clone();
    }
    else 
    {
      results.push( ( Duration::from_millis( average as u64 ), input.clone() ) );
      break;
    }
  }
  results
} 

pub fn get_time_for_input< T, F >
( 
  proc : F, 
  data : Vec< T >, 
  max_execution_time : Duration, 
  number_of_iterations : usize, 
) -> Vec< ( Duration, T ) >
where F : Fn( T ) + Send + Sync + 'static, T : Clone + Send + Sync + 'static
{
  let mut results = Vec::new();
  for input in data
  {
    let mut current_results: Vec< Option< Duration > > = Vec::new();
    for _ in 0..number_of_iterations
    {
      thread::scope( | s | 
      {
        let p = &proc;
        let input = input.clone();
        let ( sender, receiver ) = mpsc::channel();
        let t = s.spawn( move || 
        {
          let now = Instant::now();
          p( input );
          let elapsed = now.elapsed();
          match sender.send( elapsed ) 
          {
            Ok( () ) => {}, 
            Err( _ ) => {}, 
          }
        } );

        match receiver.recv_timeout(max_execution_time)
        {
          Ok( duration ) => current_results.push( Some( duration ) ),
          Err( mpsc::RecvTimeoutError::Timeout ) => {
            drop( receiver );
            drop( t );
            current_results.push( None )
          },
          Err( mpsc::RecvTimeoutError::Disconnected ) => unreachable!(),
        }
      } );
    }
    let size = current_results.len() as u128;
    let average = current_results
    .into_iter()
    .flatten()
    .fold(0, | acc, elem | acc + elem.as_millis() / size )
    ;

    results.push( ( Duration::from_millis( average as u64 ), input.clone() ) );
  }
  //results.sort_by( | a, b | a.0.cmp( &b.0 ) );
  results
} 

pub fn get_time_no_limit< T, F >( data : Vec< T >, proc : F ) -> Vec< Duration >
where F : Fn( T )
{
  let mut results = Vec::new();
  for input in data
  {
    let now = Instant::now();
    proc( input );
    let elapsed = now.elapsed();
    results.push( elapsed );
  }
  results
} 
