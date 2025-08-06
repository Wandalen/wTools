use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use strs_tools::string::split;

/// Ultra-minimal benchmark that cannot hang
fn bench_minimal_split( c : &mut Criterion )
{
  c.bench_function( "minimal_split", |b|
  {
    b.iter( ||
    {
      let result : Vec< _ > = split()
        .src( black_box( "a:b:c" ) )
        .delimeter( vec![ ":" ] )
        .perform()
        .collect();
      black_box( result )
    } );
  } );
}

criterion_group!( benches, bench_minimal_split );
criterion_main!( benches );