//! Parser-specific data generation utilities
//!
//! This module provides specialized data generators for parser benchmarking,
//! including command syntax generation, argument patterns, error cases,
//! and realistic parser workloads.

use crate ::data_generation :: { DataComplexity, DataGenerator };
use std ::collections ::HashMap;

/// Parser command generator with syntax-aware patterns
#[ derive(Debug, Clone) ]
pub struct ParserCommandGenerator
{
  /// Command complexity level
  pub complexity: CommandComplexity,
  /// Maximum nesting depth for command hierarchies  
  pub max_depth: usize,
  /// Maximum arguments per command
  pub max_arguments: usize,
  /// Command separator patterns
  pub separators: Vec< String >,
  /// Argument patterns to use
  pub argument_patterns: Vec< ArgumentPattern >,
}

/// Command complexity levels for parser testing
#[ derive(Debug, Clone, Copy, PartialEq, Eq, Hash) ]
pub enum CommandComplexity
{
  /// Simple commands with minimal arguments
  Simple,
  /// Standard commands with moderate complexity
  Standard,
  /// Complex commands with many arguments and nesting
  Complex,
  /// Comprehensive commands with all features and edge cases
  Comprehensive,
}

/// Argument pattern types for realistic command generation
#[ derive(Debug, Clone, PartialEq) ]
pub enum ArgumentPattern
{
  /// Positional arguments (value1 value2)
  Positional,
  /// Named arguments (`key ::value`)
  Named,
  /// Quoted arguments ("value with spaces")
  Quoted,
  /// Array arguments ([item1,item2,item3])
  Array,
  /// Nested arguments (`key :: {sub ::value}`)
  Nested,
  /// Mixed patterns combining multiple types
  Mixed,
}

impl Default for ParserCommandGenerator
{
  fn default() -> Self
  {
  Self
  {
   complexity: CommandComplexity ::Standard,
   max_depth: 3,
   max_arguments: 5,
   separators: vec![";;".to_string(), ";".to_string()],
   argument_patterns: vec![
  ArgumentPattern ::Positional,
  ArgumentPattern ::Named,
  ArgumentPattern ::Quoted,
 ],
 }
 }
}

impl ParserCommandGenerator
{
  /// Create a new parser command generator
  #[ must_use ]
  pub fn new() -> Self
  {
  Self ::default()
 }
  
  /// Set command complexity level
  #[ must_use ]
  pub fn complexity(mut self, complexity: CommandComplexity) -> Self
  {
  self.complexity = complexity;
  self
 }
  
  /// Set maximum nesting depth
  #[ must_use ]
  pub fn max_depth(mut self, depth: usize) -> Self
  {
  self.max_depth = depth;
  self
 }
  
  /// Set maximum arguments per command
  #[ must_use ]
  pub fn max_arguments(mut self, args: usize) -> Self
  {
  self.max_arguments = args;
  self
 }
  
  /// Add argument pattern
  #[ must_use ]
  pub fn with_pattern(mut self, pattern: ArgumentPattern) -> Self
  {
  if !self.argument_patterns.contains(&pattern)
  {
   self.argument_patterns.push(pattern);
 }
  self
 }
  
  /// Generate a single command
  #[ must_use ]
  pub fn generate_command(&self, index: usize) -> String
  {
  let command_path = self.generate_command_path(index);
  let arguments = self.generate_arguments(index);
  
  if arguments.is_empty()
  {
   command_path
 }
  else
  {
   format!("{command_path} {}", arguments.join(" "))
 }
 }
  
  /// Generate multiple commands
  #[ must_use ]
  pub fn generate_commands(&self, count: usize) -> Vec< String >
  {
  (0..count).map(|i| self.generate_command(i)).collect()
 }
  
  /// Generate batch command string with separators
  #[ must_use ]
  pub fn generate_batch_commands(&self, count: usize) -> String
  {
  let commands = self.generate_commands(count);
  let separator = &self.separators[0]; // Use first separator
  commands.join(&format!(" {separator} "))
 }
  
  /// Generate error cases for parser robustness testing
  #[ must_use ]
  pub fn generate_error_cases(&self, count: usize) -> Vec< String >
  {
  let error_patterns = [
   "invalid..double.dot",
   "trailing.dot.",
   ".leading.dot",
   "empty ::value :: ",
   "unclosed\"quote",
   "bad :::triple.colon",
   "spaces in command",
   "special@#$chars",
   "unicode🦀command",
   "",  // Empty command
 ];
  
  (0..count)
   .map(|i| {
  let base_pattern = error_patterns[i % error_patterns.len()];
  match self.complexity
  {
   CommandComplexity ::Simple => base_pattern.to_string(),
   CommandComplexity ::Standard => format!("{base_pattern} arg ::value"),
   CommandComplexity ::Complex => format!("{base_pattern} arg1 ::value1 arg2 :: \"complex value\""),
   CommandComplexity ::Comprehensive => format!("{base_pattern} arg1 ::value1 arg2 :: [item1,item2] nested :: {{key ::value}}"),
 }
 })
   .collect()
 }
  
  /// Generate realistic parser workload with distribution
  #[ must_use ]
  pub fn generate_workload(&self, total_count: usize) -> ParserWorkload
  {
  let distribution = Self ::get_complexity_distribution();
  let mut commands = Vec ::with_capacity(total_count);
  let mut complexity_counts = HashMap ::new();
  
  for i in 0..total_count
  {
   let complexity_level = Self ::select_complexity_by_distribution(i, &distribution);
   let generator = self.clone().complexity(complexity_level);
   let command = generator.generate_command(i);
   
   commands.push(command);
   *complexity_counts.entry(complexity_level).or_insert(0) += 1;
 }
  
  // Add some error cases for robustness testing
  #[ allow(clippy ::cast_possible_truncation, clippy ::cast_sign_loss) ]
  let error_count = (total_count as f32 * 0.05) as usize; // 5% error cases
  let mut error_cases = self.generate_error_cases(error_count);
  commands.append(&mut error_cases);
  
  ParserWorkload
  {
   commands,
   complexity_distribution: complexity_counts,
   total_characters: 0, // Will be calculated
   average_command_length: 0.0,
   error_case_count: error_count,
 }
 }
  
  // Private helper methods
  
  fn generate_command_path(&self, index: usize) -> String
  {
  let namespaces = ["system", "user", "data", "config", "service", "log", "backup", "monitor"];
  let actions = ["create", "update", "delete", "list", "show", "execute", "process", "analyze"];
  let entities = ["record", "file", "service", "task", "report", "session", "cache", "index"];
  
  let depth = match self.complexity
  {
   CommandComplexity ::Simple => 1,
   CommandComplexity ::Standard => 2,
   CommandComplexity ::Complex => self.max_depth.min(3),
   CommandComplexity ::Comprehensive => self.max_depth,
 };
  
  let mut path_parts = Vec ::with_capacity(depth);
  
  path_parts.push(namespaces[index % namespaces.len()]);
  
  if depth > 1
  {
   path_parts.push(entities[(index / namespaces.len()) % entities.len()]);
 }
  
  if depth > 2
  {
   path_parts.push(actions[(index / (namespaces.len() * entities.len())) % actions.len()]);
 }
  
  if depth > 3
  {
   // Add more specific sub-commands for comprehensive complexity
   let specifics = ["detailed", "quick", "batch", "async"];
   path_parts.push(specifics[index % specifics.len()]);
 }
  
  path_parts.join(".")
 }
  
  fn generate_arguments(&self, index: usize) -> Vec< String >
  {
  let arg_count = match self.complexity
  {
   CommandComplexity ::Simple => (index % 2).max(0),
   CommandComplexity ::Standard => (index % 3) + 1,
   CommandComplexity ::Complex => (index % self.max_arguments) + 2,
   CommandComplexity ::Comprehensive => (index % self.max_arguments) + 3,
 };
  
  let mut arguments = Vec ::new();
  
  for i in 0..arg_count
  {
   let pattern = &self.argument_patterns[i % self.argument_patterns.len()];
   let arg = Self ::generate_argument_by_pattern(pattern, index, i);
   arguments.push(arg);
 }
  
  arguments
 }
  
  fn generate_argument_by_pattern(pattern: &ArgumentPattern, cmd_index: usize, arg_index: usize) -> String
  {
  match pattern
  {
   ArgumentPattern ::Positional => format!("pos_arg_{arg_index}"),
   ArgumentPattern ::Named =>
  {
  let value = cmd_index % 100;
  format!("param{arg_index} ::value{value}")
 },
   ArgumentPattern ::Quoted => format!("description :: \"Command {cmd_index} argument {arg_index}\""),
   ArgumentPattern ::Array =>
  {
  let item1 = arg_index;
  let item2 = arg_index + 1;
  let item3 = arg_index + 2;
  format!("items :: [\"item{item1}\",\"item{item2}\",\"item{item3}\"]") 
 },
   ArgumentPattern ::Nested =>
  {
  let timeout = (cmd_index % 10) + 1;
  let retries = (arg_index % 3) + 1;
  format!("config :: {{timeout :: {timeout},retries :: {retries}}}")
 },
   ArgumentPattern ::Mixed =>
  {
  match arg_index % 3
  {
   0 =>
  {
  let value = cmd_index % 100;
  format!("param{arg_index} ::value{value}")
 },
   1 => format!("description :: \"Command {cmd_index} argument {arg_index}\""),
   _ =>
  {
  let item1 = arg_index;
  let item2 = arg_index + 1;
  let item3 = arg_index + 2;
  format!("items :: [\"item{item1}\",\"item{item2}\",\"item{item3}\"]") 
 },
 }
 }
 }
 }
  
  fn get_complexity_distribution() -> Vec< (CommandComplexity, f32) >
  {
  // Realistic distribution based on typical CLI usage
  vec![
   (CommandComplexity ::Simple, 0.3),      // 30% simple commands
   (CommandComplexity ::Standard, 0.5),    // 50% standard commands
   (CommandComplexity ::Complex, 0.15),    // 15% complex commands
   (CommandComplexity ::Comprehensive, 0.05), // 5% comprehensive commands
 ]
 }
  
  fn select_complexity_by_distribution(index: usize, distribution: &[ (CommandComplexity, f32)]) -> CommandComplexity
  {
  let mut cumulative = 0.0;
  let normalized_index = (index as f32) / 100.0 % 1.0; // Normalize to 0-1 range
  
  for (complexity, weight) in distribution
  {
   cumulative += weight;
   if normalized_index <= cumulative
   {
  return *complexity;
 }
 }
  
  // Fallback to standard complexity
  CommandComplexity ::Standard
 }
}

/// Generated parser workload with metadata
#[ derive(Debug, Clone) ]
pub struct ParserWorkload
{
  /// Generated commands
  pub commands: Vec< String >,
  /// Distribution of complexity levels
  pub complexity_distribution: HashMap< CommandComplexity, usize >,
  /// Total characters across all commands
  pub total_characters: usize,
  /// Average command length
  pub average_command_length: f64,
  /// Number of error cases included
  pub error_case_count: usize,
}

impl ParserWorkload
{
  /// Calculate workload statistics
  pub fn calculate_statistics( &mut self )
  {
  self.total_characters = self.commands.iter().map(std ::string ::String ::len).sum();
  self.average_command_length = self.total_characters as f64 / self.commands.len() as f64;
 }
  
  /// Get workload summary
  #[ must_use ]
  pub fn summary( &self ) -> String
  {
  let mut summary = String ::new();
  
  summary.push_str("Parser Workload Summary: \n");
  summary.push_str(&format!("- Total commands: {}\n", self.commands.len()));
  summary.push_str(&format!("- Total characters: {}\n", self.total_characters));
  summary.push_str(&format!("- Average length: {:.1} chars/command\n", self.average_command_length));
  summary.push_str(&format!("- Error cases: {} ({:.1}%)\n", 
  self.error_case_count,
  self.error_case_count as f64 / self.commands.len() as f64 * 100.0));
  
  summary.push_str("- Complexity distribution: \n");
  for (complexity, count) in &self.complexity_distribution
  {
   let percentage = *count as f64 / (self.commands.len() - self.error_case_count) as f64 * 100.0;
   summary.push_str(&format!("  * {complexity:?} : {count} ({percentage:.1}%)\n"));
 }
  
  summary
 }
  
  /// Get sample commands for preview
  #[ must_use ]
  pub fn sample_commands(&self, count: usize) -> Vec< &String >
  {
  self.commands.iter().take(count).collect()
 }
}

/// Enhanced `DataGenerator` with parser-specific extensions
impl DataGenerator
{
  /// Generate unilang commands with enhanced patterns
  #[ must_use ]
  pub fn generate_enhanced_unilang_commands(&self, count: usize) -> Vec< String >
  {
  let generator = ParserCommandGenerator ::new()
   .complexity( match self.complexity 
  {
  DataComplexity ::Simple => CommandComplexity ::Simple,
  DataComplexity ::Medium => CommandComplexity ::Standard,
  DataComplexity ::Complex => CommandComplexity ::Complex,
  DataComplexity ::Full => CommandComplexity ::Comprehensive,
 })
   .with_pattern(ArgumentPattern ::Named)
   .with_pattern(ArgumentPattern ::Quoted)
   .with_pattern(ArgumentPattern ::Array);
   
  generator.generate_commands(count)
 }
  
  /// Generate parser test scenarios with specific patterns
  #[ must_use ]
  pub fn generate_parser_scenarios(&self, scenario_type: &str, count: usize) -> Vec< String >
  {
  let generator = ParserCommandGenerator ::new()
   .complexity(CommandComplexity ::Standard);
   
  match scenario_type
  {
   "batch_processing" => vec![generator.generate_batch_commands(count)],
   "error_handling" => generator.generate_error_cases(count),
   "performance_stress" =>
  {
  let mut workload = generator.generate_workload(count);
  workload.calculate_statistics();
  workload.commands
 },
   _ => generator.generate_commands(count),
 }
 }
}

