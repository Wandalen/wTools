use criterion::{ black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput };
use strs_tools::string::split;

/// Stable benchmark with fixed test data (no generation loops)
fn bench_stable_operations( c : &mut Criterion )
{
  let mut group = c.benchmark_group( "stable_operations" );
  
  // Pre-defined test data to avoid generation issues
  let test_cases = [
    ( "small", "a:b:c:d:e", 100 ),
    ( "medium", "namespace:command:arg:value:option:flag:param:config:setting:property", 1000 ),
    ( "large", "namespace:command:arg:value:option:flag:param:config:setting:property:attribute:field:member:method:function:class:struct:enum:trait:impl:use:pub:mod:fn:let:const:static:mut:ref:move:async:await:loop:while:for:if:else:match:return:break:continue", 10000 ),
  ];
  
  for ( name, input, size ) in test_cases
  {
    // Repeat the input to reach approximate target size
    let mut test_input = String::new();
    while test_input.len() < size
    {
      test_input.push_str( input );
      test_input.push( ' ' );
    }
    test_input.truncate( size );
    
    group.throughput( Throughput::Bytes( test_input.len() as u64 ) );
    
    // Single delimiter test
    group.bench_with_input(
      BenchmarkId::new( "single_colon", name ),
      &test_input,
      |b, input|
      {
        b.iter( ||
        {
          let result : Vec< _ > = split()
            .src( black_box( input ) )
            .delimeter( vec![ ":" ] )
            .perform()
            .collect();
          black_box( result )
        } );
      }
    );
    
    // Multi delimiter test
    group.bench_with_input(
      BenchmarkId::new( "multi_delim", name ),
      &test_input,
      |b, input|
      {
        b.iter( ||
        {
          let result : Vec< _ > = split()
            .src( black_box( input ) )
            .delimeter( vec![ ":", " ", "," ] )
            .perform()
            .collect();
          black_box( result )
        } );
      }
    );
  }
  
  group.finish();
}

criterion_group!( benches, bench_stable_operations );
criterion_main!( benches );