//! Comprehensive benchmarks for specialized string splitting algorithms.
//!
//! This benchmark suite measures the performance improvements delivered by
//! Task 007 specialized algorithm implementations compared to generic algorithms.

use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use strs_tools::string::specialized::{ 
  smart_split, SingleCharSplitIterator, BoyerMooreSplitIterator
};
use strs_tools::string;

/// Generate test data for benchmarks
fn generate_test_data() -> (String, String, String) {
  let single_char_data = "word1,word2,word3,word4,word5,word6,word7,word8,word9,word10".repeat(100);
  let multi_char_data = "field1::field2::field3::field4::field5::field6::field7::field8".repeat(100);
  let mixed_data = "key=value,item::subitem,path/to/file,param?query#anchor".repeat(100);
  
  (single_char_data, multi_char_data, mixed_data)
}

/// Benchmark SingleChar vs Generic for comma splitting
fn bench_single_char_vs_generic(c: &mut Criterion) {
  let (single_char_data, _, _) = generate_test_data();
  
  let mut group = c.benchmark_group("single_char_splitting");
  
  // Generic algorithm baseline
  group.bench_function("generic_comma_split", |b| {
    b.iter(|| {
      let count = string::split()
        .src(&single_char_data)
        .delimeter(",")
        .perform()
        .count();
      black_box(count)
    })
  });
  
  // Specialized SingleChar algorithm
  group.bench_function("single_char_optimized", |b| {
    b.iter(|| {
      let count = SingleCharSplitIterator::new(&single_char_data, ',', false)
        .count();
      black_box(count)
    })
  });
  
  // Smart split (should automatically choose SingleChar)
  group.bench_function("smart_split_comma", |b| {
    b.iter(|| {
      let count = smart_split(&single_char_data, &[","])
        .count();
      black_box(count)
    })
  });
  
  group.finish();
}

/// Benchmark Boyer-Moore vs Generic for multi-character patterns
fn bench_boyer_moore_vs_generic(c: &mut Criterion) {
  let (_, multi_char_data, _) = generate_test_data();
  
  let mut group = c.benchmark_group("multi_char_splitting");
  
  // Generic algorithm baseline
  group.bench_function("generic_double_colon", |b| {
    b.iter(|| {
      let count = string::split()
        .src(&multi_char_data)
        .delimeter("::")
        .perform()
        .count();
      black_box(count)
    })
  });
  
  // Specialized Boyer-Moore algorithm
  group.bench_function("boyer_moore_optimized", |b| {
    b.iter(|| {
      let count = BoyerMooreSplitIterator::new(&multi_char_data, "::")
        .count();
      black_box(count)
    })
  });
  
  // Smart split (should automatically choose Boyer-Moore)
  group.bench_function("smart_split_double_colon", |b| {
    b.iter(|| {
      let count = smart_split(&multi_char_data, &["::"])
        .count();
      black_box(count)
    })
  });
  
  group.finish();
}

/// Benchmark different input sizes to show scaling characteristics
fn bench_scaling_characteristics(c: &mut Criterion) {
  let sizes = vec![100, 1000, 10000];
  
  for size in sizes {
    let comma_data = format!("item{},", size/10).repeat(size);
    let colon_data = format!("field{}::", size/10).repeat(size);
    
    let mut group = c.benchmark_group(&format!("scaling_{}_items", size));
    
    // Single character scaling
    group.bench_function("single_char_specialized", |b| {
      b.iter(|| {
        let count = SingleCharSplitIterator::new(&comma_data, ',', false)
          .count();
        black_box(count)
      })
    });
    
    group.bench_function("single_char_generic", |b| {
      b.iter(|| {
        let count = string::split()
          .src(&comma_data)
          .delimeter(",")
          .perform()
          .count();
        black_box(count)
      })
    });
    
    // Multi character scaling
    group.bench_function("boyer_moore_specialized", |b| {
      b.iter(|| {
        let count = BoyerMooreSplitIterator::new(&colon_data, "::")
          .count();
        black_box(count)
      })
    });
    
    group.bench_function("boyer_moore_generic", |b| {
      b.iter(|| {
        let count = string::split()
          .src(&colon_data)
          .delimeter("::")
          .perform()
          .count();
        black_box(count)
      })
    });
    
    group.finish();
  }
}

/// Benchmark realistic unilang parsing scenarios
fn bench_unilang_scenarios(c: &mut Criterion) {
  // Typical unilang command patterns
  let list_parsing = "item1,item2,item3,item4,item5".repeat(200);
  let namespace_parsing = "math::operations::add::execute".repeat(100);
  
  let mut group = c.benchmark_group("unilang_scenarios");
  
  // List parsing (comma-heavy, perfect for SingleChar)
  group.bench_function("unilang_list_generic", |b| {
    b.iter(|| {
      let count = string::split()
        .src(&list_parsing)
        .delimeter(",")
        .perform()
        .count();
      black_box(count)
    })
  });
  
  group.bench_function("unilang_list_specialized", |b| {
    b.iter(|| {
      let count = smart_split(&list_parsing, &[","])
        .count();
      black_box(count)
    })
  });
  
  // Namespace parsing (:: patterns, perfect for Boyer-Moore)
  group.bench_function("unilang_namespace_generic", |b| {
    b.iter(|| {
      let count = string::split()
        .src(&namespace_parsing)
        .delimeter("::")
        .perform()
        .count();
      black_box(count)
    })
  });
  
  group.bench_function("unilang_namespace_specialized", |b| {
    b.iter(|| {
      let count = smart_split(&namespace_parsing, &["::"])
        .count();
      black_box(count)
    })
  });
  
  group.finish();
}

/// Benchmark string processing throughput
fn bench_string_processing_throughput(c: &mut Criterion) {
  // Create larger datasets for throughput measurement
  let large_comma_data = "field1,field2,field3,field4,field5,field6,field7,field8".repeat(10000);
  let large_colon_data = "ns1::ns2::ns3::class::method::args::param".repeat(5000);
  
  let mut group = c.benchmark_group("throughput");
  
  // SingleChar throughput
  group.bench_function("single_char_throughput", |b| {
    b.iter(|| {
      let mut total_len = 0usize;
      for result in SingleCharSplitIterator::new(&large_comma_data, ',', false) {
        total_len += result.as_str().len();
      }
      black_box(total_len)
    })
  });
  
  // Boyer-Moore throughput
  group.bench_function("boyer_moore_throughput", |b| {
    b.iter(|| {
      let mut total_len = 0usize;
      for result in BoyerMooreSplitIterator::new(&large_colon_data, "::") {
        total_len += result.as_str().len();
      }
      black_box(total_len)
    })
  });
  
  // Generic throughput for comparison
  group.bench_function("generic_comma_throughput", |b| {
    b.iter(|| {
      let mut total_len = 0usize;
      for result in string::split().src(&large_comma_data).delimeter(",").perform() {
        total_len += result.string.len();
      }
      black_box(total_len)
    })
  });
  
  group.bench_function("generic_colon_throughput", |b| {
    b.iter(|| {
      let mut total_len = 0usize;
      for result in string::split().src(&large_colon_data).delimeter("::").perform() {
        total_len += result.string.len();
      }
      black_box(total_len)
    })
  });
  
  group.finish();
}

criterion_group!(
  benches,
  bench_single_char_vs_generic,
  bench_boyer_moore_vs_generic,
  bench_scaling_characteristics,
  bench_unilang_scenarios,
  bench_string_processing_throughput
);

criterion_main!(benches);