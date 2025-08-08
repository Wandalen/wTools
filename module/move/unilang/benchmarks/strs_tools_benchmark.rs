//! Benchmark for strs_tools SIMD string operations performance impact
//!
//! This benchmark measures the performance difference between standard library
//! string operations and strs_tools SIMD-optimized operations in the context
//! of unilang parsing tasks.

use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use unilang::data::Kind;

/// Generate test data for list parsing benchmarks
fn generate_list_data(items: usize) -> String {
  (1..=items).map(|i| i.to_string()).collect::<Vec<_>>().join(",")
}

/// Generate test data for map parsing benchmarks  
fn generate_map_data(entries: usize) -> String {
  (1..=entries).map(|i| format!("key{}=value{}", i, i)).collect::<Vec<_>>().join(",")
}

/// Generate test data for enum choices parsing
fn generate_enum_data(choices: usize) -> String {
  (1..=choices).map(|i| format!("choice{}", i)).collect::<Vec<_>>().join(",")
}

fn benchmark_list_parsing(c: &mut Criterion) {
  let mut group = c.benchmark_group("list_parsing");
  
  let test_cases = [
    ("small_list_10", 10),
    ("medium_list_100", 100), 
    ("large_list_1000", 1000),
    ("huge_list_10000", 10000),
  ];
  
  for (name, size) in test_cases.iter() {
    let data = generate_list_data(*size);
    let kind = Kind::List(Box::new(Kind::Integer), Some(','));
    
    group.bench_function(*name, |b| {
      b.iter(|| {
        let result = unilang::types::parse_value(black_box(&data), black_box(&kind));
        black_box(result)
      })
    });
  }
  
  group.finish();
}

fn benchmark_map_parsing(c: &mut Criterion) {
  let mut group = c.benchmark_group("map_parsing");
  
  let test_cases = [
    ("small_map_5", 5),
    ("medium_map_50", 50),
    ("large_map_500", 500),
    ("huge_map_2000", 2000),
  ];
  
  for (name, size) in test_cases.iter() {
    let data = generate_map_data(*size);
    let kind = Kind::Map(
      Box::new(Kind::String),
      Box::new(Kind::String),
      Some(','),
      Some('=')
    );
    
    group.bench_function(*name, |b| {
      b.iter(|| {
        let result = unilang::types::parse_value(black_box(&data), black_box(&kind));
        black_box(result)
      })
    });
  }
  
  group.finish();
}

fn benchmark_enum_parsing(c: &mut Criterion) {
  let mut group = c.benchmark_group("enum_parsing");
  
  let test_cases = [
    ("small_enum_3", 3),
    ("medium_enum_20", 20),
    ("large_enum_100", 100),
    ("huge_enum_500", 500),
  ];
  
  for (name, size) in test_cases.iter() {
    let choices_str = generate_enum_data(*size);
    let enum_kind_str = format!("Enum({})", choices_str);
    
    group.bench_function(*name, |b| {
      b.iter(|| {
        let result: Result<Kind, _> = black_box(&enum_kind_str).parse();
        black_box(result)
      })
    });
  }
  
  group.finish();
}

fn benchmark_complex_scenario(c: &mut Criterion) {
  let mut group = c.benchmark_group("complex_parsing");
  
  // Simulate a complex command with multiple list and map arguments
  let complex_data = vec![
    ("list_args", "1,2,3,4,5,6,7,8,9,10", Kind::List(Box::new(Kind::Integer), Some(','))),
    ("map_config", "host=localhost,port=8080,timeout=30,retry=3", 
     Kind::Map(Box::new(Kind::String), Box::new(Kind::String), Some(','), Some('='))),
    ("file_list", "file1.txt,file2.txt,file3.txt,file4.txt,file5.txt", 
     Kind::List(Box::new(Kind::String), Some(','))),
  ];
  
  group.bench_function("mixed_parsing_scenario", |b| {
    b.iter(|| {
      for (name, data, kind) in &complex_data {
        let result = unilang::types::parse_value(black_box(data), black_box(kind));
        black_box((name, result.unwrap_or_default()));
      }
    })
  });
  
  group.finish();
}

fn benchmark_throughput(c: &mut Criterion) {
  let mut group = c.benchmark_group("throughput");
  
  // Create realistic workloads for throughput testing
  let large_list = generate_list_data(5000);
  let large_map = generate_map_data(1000);
  
  let list_kind = Kind::List(Box::new(Kind::Integer), Some(','));
  let map_kind = Kind::Map(
    Box::new(Kind::String), 
    Box::new(Kind::String), 
    Some(','), 
    Some('=')
  );
  
  group.throughput(criterion::Throughput::Bytes(large_list.len() as u64));
  group.bench_function("large_list_throughput", |b| {
    b.iter(|| {
      let result = unilang::types::parse_value(black_box(&large_list), black_box(&list_kind));
      black_box(result)
    })
  });
  
  group.throughput(criterion::Throughput::Bytes(large_map.len() as u64));
  group.bench_function("large_map_throughput", |b| {
    b.iter(|| {
      let result = unilang::types::parse_value(black_box(&large_map), black_box(&map_kind));
      black_box(result)
    })
  });
  
  group.finish();
}

/// Benchmark group for strs_tools SIMD performance testing
criterion_group!(
  benches, 
  benchmark_list_parsing, 
  benchmark_map_parsing,
  benchmark_enum_parsing,
  benchmark_complex_scenario,
  benchmark_throughput
);

criterion_main!(benches);