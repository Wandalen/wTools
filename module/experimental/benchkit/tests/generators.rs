//! Test generators functionality

#[ cfg(feature = "integration") ]
use benchkit ::prelude :: *;
#[ cfg(feature = "data_generators") ]
#[ allow(unused_imports) ]
use benchkit ::generators :: *;

#[ test ]
#[ cfg(feature = "data_generators") ]
fn test_data_size()
{
  assert_eq!(DataSize ::Small.size(), 10);
  assert_eq!(DataSize ::Medium.size(), 100);
  assert_eq!(DataSize ::Large.size(), 1000);
  assert_eq!(DataSize ::Huge.size(), 10000);
  assert_eq!(DataSize ::Custom(42).size(), 42);
}

#[ test ]
#[ cfg(feature = "data_generators") ]
fn test_list_generation()
{
  let small_list = generate_list_data(DataSize ::Small);
  let parts: Vec< &str > = small_list.split(',').collect();
  assert_eq!(parts.len(), 10);
  assert_eq!(parts[0], "item1");
  assert_eq!(parts[9], "item10");
}

#[ test ]
#[ cfg(feature = "data_generators") ]
fn test_map_generation()
{
  let map_data = generate_map_data(DataSize ::Small);
  assert!(map_data.contains("key1=value1"));
  assert!(map_data.contains("key10=value10"));
}

#[ test ]
#[ cfg(feature = "data_generators") ]
fn test_seeded_generator()
{
  let mut gen1 = SeededGenerator ::new(42);
  let mut gen2 = SeededGenerator ::new(42);
  
  // Same seed should produce same sequence
  assert_eq!(gen1.random_string(10), gen2.random_string(10));
  assert_eq!(gen1.random_int(1, 100), gen2.random_int(1, 100));
}

#[ test ]
#[ cfg(feature = "data_generators") ]
fn test_parsing_test_data()
{
  let args = ParsingTestData ::command_args(DataSize ::Small);
  assert!(args.contains("--arg1 value1"));
  
  let csv = ParsingTestData ::csv_data(3, 2);
  let lines: Vec< &str > = csv.lines().collect();
  assert_eq!(lines.len(), 4); // header + 3 rows
  assert_eq!(lines[0], "column1,column2");
}