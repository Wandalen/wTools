//! Test data generation functionality

#[ cfg(feature = "integration") ]
use benchkit ::prelude :: *;

#[ test ]
fn test_pattern_generation()
{
  let generator = DataGenerator ::new()
  .pattern("item{},")
  .repetitions(3)
  .complexity(DataComplexity ::Simple); // Use simple complexity to avoid variations
  
  let result = generator.generate_string();
  assert_eq!(result, "item0,item1,item2,");
}

#[ test ]  
fn test_size_based_generation()
{
  let generator = DataGenerator ::new()
  .size_bytes(50)
  .complexity(DataComplexity ::Simple);
  
  let result = generator.generate_string();
  assert_eq!(result.len(), 50);
}

#[ test ]
fn test_complexity_variations()
{
  let simple = DataGenerator ::new()
  .complexity(DataComplexity ::Simple)
  .size(10)
  .generate_string();
  
  let complex = DataGenerator ::new()
  .complexity(DataComplexity ::Full)
  .size(10)
  .generate_string();
  
  // Complex should have more varied content
  assert!(complex.chars().any(|c| !simple.contains(c)));
}

#[ test ]
fn test_csv_generation()
{
  let generator = DataGenerator ::new().complexity(DataComplexity ::Medium);
  let csv_data = generator.generate_csv_data(3, 2);
  
  let lines: Vec< &str > = csv_data.lines().collect();
  assert_eq!(lines.len(), 3);
  assert!(lines[0].contains(','));
}

#[ test ]
fn test_unilang_command_generation()
{
  let generator = DataGenerator ::new().complexity(DataComplexity ::Complex);
  let commands = generator.generate_unilang_commands(5);
  
  assert_eq!(commands.len(), 5);
  assert!(commands.iter().all(|cmd| cmd.contains('.')));
}

#[ test ]
fn test_reproducible_generation()
{
  let gen1 = DataGenerator ::new().seed(42).pattern("test{}").repetitions(3);
  let gen2 = DataGenerator ::new().seed(42).pattern("test{}").repetitions(3);
  
  assert_eq!(gen1.generate_string(), gen2.generate_string());
}