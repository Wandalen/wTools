use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use strs_tools::string::split;

/// Quick benchmark for testing the benchmark runner functionality
fn bench_quick_split( c : &mut Criterion )
{
  let input = "test:data:for:quick:benchmark";
  let delimiter = vec![ ":" ];
  
  c.bench_function( "quick_split_test", |b|
  {
    b.iter( ||
    {
      let result : Vec< _ > = split()
        .src( black_box( input ) )
        .delimeter( delimiter.clone() )
        .perform()
        .collect();
      black_box( result )
    } );
  } );
}

criterion_group!( benches, bench_quick_split );
criterion_main!( benches );