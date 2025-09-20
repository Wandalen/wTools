#![allow(clippy::all)]
#![allow(clippy::all)]
//! Ergonomic CLI Export and Aggregation
//!
//! This example demonstrates an intuitive, hard-to-misuse approach to:
//! 1. Exporting CLI commands from individual crates/modules
//! 2. Combining multiple CLIs with optional prefixes into a single aggregating CLI
//!
//! The design prioritizes:
//! - Ergonomic API that's easy to understand
//! - Type safety that prevents common mistakes
//! - Minimal boilerplate
//! - Clear separation of concerns
//!
//! NOTE: Temporarily commented out due to API mismatches with current implementation.

fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!("This example is temporarily disabled due to API mismatches.");
  Ok(())
}

/*

use unilang::prelude::*;
use std::collections::HashMap;

// =============================================================================
// CLI Export Utility - Makes Exporting Commands Ergonomic
// =============================================================================

/// A container for exporting CLI commands from a module
/// This makes it easy to package up commands for use in other crates
#[ allow( missing_debug_implementations ) ]
pub struct CliModule
{
  name: String,
  commands: Vec<(CommandDefinition, Box<dyn Fn(unilang::semantic::VerifiedCommand, unilang::interpreter::ExecutionContext) -> Result<unilang::data::OutputData, unilang::data::ErrorData> + Send + Sync>)>,
  description: String,
}

impl CliModule
{
  /// Create a new CLI module for export
  pub fn new(name: &str) -> Self
  {
  Self {
  name: name.to_string(),
  commands: Vec::new(),
  description: String::new(),
 }
 }

  /// Set description for this CLI module
  pub fn description(mut self, desc: &str) -> Self
  {
  self.description = desc.to_string();
  self
 }

  /// Add a command to this module (builder pattern)
  pub fn command(
  mut self,
  mut definition: CommandDefinition,
  routine: Box<dyn Fn(unilang::semantic::VerifiedCommand, unilang::interpreter::ExecutionContext) -> Result<unilang::data::OutputData, unilang::data::ErrorData> + Send + Sync>
 ) -> Self {
  // Ensure help is enabled for exported commands
  definition.auto_help_enabled = true;
  self.commands.push((definition, routine));
  self
 }

  /// Helper to add a simple command with less boilerplate
  pub fn simple_command<F>(
  self,
  name: &str,
  description: &str,
  args: Vec<ArgumentDefinition>,
  handler: F
 ) -> Self
  where
  F: Fn(unilang::semantic::VerifiedCommand, unilang::interpreter::ExecutionContext) -> Result<unilang::data::OutputData, unilang::data::ErrorData> + Send + Sync + 'static
  {
  let command_name = if name.starts_with('.') {
  name.to_string()
 } else {
  format!(".{}", name)
 };

  let cmd = CommandDefinition::former()
  .name(&command_name)
  .description(description.to_string())
  .arguments(args)
  .end();

  self.command(cmd, Box::new(handler))
 }

  /// Get the module name
  pub fn name(&self) -> &str
  {
  &self.name
 }

  /// Get the module description
  pub fn description_text(&self) -> &str
  {
  &self.description
 }

  /// Get all commands in this module
  pub fn commands(&self) -> &[(CommandDefinition, Box<dyn Fn(unilang::semantic::VerifiedCommand, unilang::interpreter::ExecutionContext) -> Result<unilang::data::OutputData, unilang::data::ErrorData> + Send + Sync>)]
  {
  &self.commands
 }
}

// =============================================================================
// CLI Aggregator - Combines Multiple CLI Modules
// =============================================================================

/// Aggregates multiple CLI modules into a single unified CLI
/// Handles prefixing, help generation, and conflict resolution
#[ allow( missing_debug_implementations ) ]
pub struct CliAggregator
{
  registry: CommandRegistry,
  modules: HashMap<String, String>, // module name -> prefix
}

impl CliAggregator
{
  /// Create a new CLI aggregator
  pub fn new() -> Self
  {
  #[ allow( deprecated ) ]
  let mut registry = CommandRegistry::new();
  registry.enable_help_conventions(true);
  Self {
  registry,
  modules: HashMap::new(),
 }
 }

  /// Add a CLI module with an optional prefix
  ///
  /// # Arguments
  /// * `module` - The CLI module to add
  /// * `prefix` - Optional prefix (e.g., "math" creates .math.add)
  ///              If None, commands are added without prefix
  ///
  /// # Returns
  /// * Ok(()) on success
  /// * Err if there are naming conflicts or registration failures
  pub fn add_module(
  mut self,
  module: CliModule,
  prefix: Option<&str>
 ) -> Result<Self, unilang::Error> {
  // Record the module
  self.modules.insert(
  module.name().to_string(),
  prefix.unwrap_or("").to_string()
 );

  // Add each command from the module
  for (mut command_def, routine) in module.commands
  {
  // Apply prefix if specified
  if let Some(prefix) = prefix
  {
  command_def.namespace = format!(".{}", prefix);
 }

  // Register the command
  self.registry.register_with_auto_help(command_def, routine)?;
 }

  Ok(self)
 }

  /// Add a main command for the aggregated CLI
  pub fn main_command<F>(
  mut self,
  name: &str,
  description: &str,
  handler: F
 ) -> Result<Self, unilang::Error>
  where
  F: Fn(unilang::semantic::VerifiedCommand, unilang::interpreter::ExecutionContext) -> Result<unilang::data::OutputData, unilang::data::ErrorData> + Send + Sync + 'static
  {
  let command_name = if name.starts_with('.') {
  name.to_string()
 } else {
  format!(".{}", name)
 };

  let cmd = CommandDefinition::former()
  .name(&command_name)
  .description(description.to_string())
  .arguments(vec![])
  .end();

  self.registry.register_with_auto_help(cmd, Box::new(handler))?;
  Ok(self)
 }

  /// Build the final aggregated CLI registry
  pub fn build(self) -> CommandRegistry
  {
  self.registry
 }

  /// Get information about registered modules
  pub fn modules(&self) -> &HashMap<String, String>
  {
  &self.modules
 }
}

// =============================================================================
// Example CLI Modules - Demonstrating Export Pattern
// =============================================================================

/// Math operations CLI module
/// This would typically be in a separate crate: `math_cli`
pub mod math_cli {
  use super::*;

  /// Export the math CLI module
  /// This is the main export function that other crates would use
  pub fn export() -> CliModule
  {
  CliModule::new("math")
  .description("Mathematical operations")
  .simple_command(
  "add",
  "Add two numbers",
  vec![
  ArgumentDefinition {
  name: "a".to_string(),
  description: "First number".to_string(),
  kind: Kind::Integer,
  hint: "First number".to_string(),
  attributes: ArgumentAttributes::default(),
  validation_rules: vec![],
  aliases: vec![],
  tags: vec![],
 },
  ArgumentDefinition {
  name: "b".to_string(),
  description: "Second number".to_string(),
  kind: Kind::Integer,
  hint: "Second number".to_string(),
  attributes: ArgumentAttributes::default(),
  validation_rules: vec![],
  aliases: vec![],
  tags: vec![],
 },
 ],
  |cmd, _ctx| {
  let a = match cmd.arguments.get("a").unwrap() {
  Value::Integer(val) => *val,
  _ => return Err(unilang::data::ErrorData {
      code: "TYPE_ERROR".to_string(),
      message: "Expected integer for 'a'".to_string(),
      source: None,
 }),
 };
  let b = match cmd.arguments.get("b").unwrap() {
  Value::Integer(val) => *val,
  _ => return Err(unilang::data::ErrorData {
      code: "TYPE_ERROR".to_string(),
      message: "Expected integer for 'b'".to_string(),
      source: None,
 }),
 };

  Ok(unilang::data::OutputData {
  content: format!("{} + {} = {}", a, b, a + b),
  format: "text".to_string(),
 })
 }
 )
  .simple_command(
  "multiply",
  "Multiply two numbers",
  vec![
  ArgumentDefinition {
  name: "x".to_string(),
  description: "First number".to_string(),
  kind: Kind::Integer,
  hint: "First number".to_string(),
  attributes: ArgumentAttributes::default(),
  validation_rules: vec![],
  aliases: vec![],
  tags: vec![],
 },
  ArgumentDefinition {
  name: "y".to_string(),
  description: "Second number".to_string(),
  kind: Kind::Integer,
  hint: "Second number".to_string(),
  attributes: ArgumentAttributes::default(),
  validation_rules: vec![],
  aliases: vec![],
  tags: vec![],
 },
 ],
  |cmd, _ctx| {
  let x = match cmd.arguments.get("x").unwrap() {
  Value::Integer(val) => *val,
  _ => return Err(unilang::data::ErrorData {
      code: "TYPE_ERROR".to_string(),
      message: "Expected integer for 'x'".to_string(),
      source: None,
 }),
 };
  let y = match cmd.arguments.get("y").unwrap() {
  Value::Integer(val) => *val,
  _ => return Err(unilang::data::ErrorData {
      code: "TYPE_ERROR".to_string(),
      message: "Expected integer for 'y'".to_string(),
      source: None,
 }),
 };

  Ok(unilang::data::OutputData {
  content: format!("{} × {} = {}", x, y, x * y),
  format: "text".to_string(),
 })
 }
 )
 }
}

/// File operations CLI module
/// This would typically be in a separate crate: `file_cli`
pub mod file_cli {
  use super::*;

  /// Export the file CLI module
  pub fn export() -> CliModule
  {
  CliModule::new("file")
  .description("File system operations")
  .simple_command(
  "list",
  "List files in directory",
  vec![
  ArgumentDefinition {
  name: "path".to_string(),
  description: "Directory path".to_string(),
  kind: Kind::String,
  hint: "Directory path".to_string(),
  attributes: ArgumentAttributes {
      optional: true,
      default: Some(".".to_string()),
      ..ArgumentAttributes::default()
 },
  validation_rules: vec![],
  aliases: vec![],
  tags: vec![],
 },
 ],
  |cmd, _ctx| {
  let path = match cmd.arguments.get("path") {
  Some(Value::String(p)) => p.clone(),
  _ => ".".to_string(),
 };

  Ok(unilang::data::OutputData {
  content: format!("📁 Listing files in '{}':\n  📄 document.txt\n  📄 readme.md\n  📁 src/", path),
  format: "text".to_string(),
 })
 }
 )
  .simple_command(
  "copy",
  "Copy a file",
  vec![
  ArgumentDefinition {
  name: "source".to_string(),
  description: "Source file path".to_string(),
  kind: Kind::String,
  hint: "Source file".to_string(),
  attributes: ArgumentAttributes::default(),
  validation_rules: vec![],
  aliases: vec!["src".to_string()],
  tags: vec![],
 },
  ArgumentDefinition {
  name: "destination".to_string(),
  description: "Destination file path".to_string(),
  kind: Kind::String,
  hint: "Destination file".to_string(),
  attributes: ArgumentAttributes::default(),
  validation_rules: vec![],
  aliases: vec!["dest", "dst"].into_iter().map(String::from).collect(),
  tags: vec![],
 },
 ],
  |cmd, _ctx| {
  let source = match cmd.arguments.get("source") {
  Some(Value::String(s)) => s,
  _ => return Err(unilang::data::ErrorData {
      code: "MISSING_ARG".to_string(),
      message: "Source file required".to_string(),
      source: None,
 }),
 };
  let dest = match cmd.arguments.get("destination") {
  Some(Value::String(d)) => d,
  _ => return Err(unilang::data::ErrorData {
      code: "MISSING_ARG".to_string(),
      message: "Destination file required".to_string(),
      source: None,
 }),
 };

  Ok(unilang::data::OutputData {
  content: format!("📋 Copied '{}' to '{}'", source, dest),
  format: "text".to_string(),
 })
 }
 )
 }
}

/// Database operations CLI module
/// This would typically be in a separate crate: `db_cli`
pub mod db_cli {
  use super::*;

  /// Export the database CLI module
  pub fn export() -> CliModule
  {
  CliModule::new("database")
  .description("Database management operations")
  .simple_command(
  "connect",
  "Connect to database",
  vec![
  ArgumentDefinition {
  name: "host".to_string(),
  description: "Database host".to_string(),
  kind: Kind::String,
  hint: "DB host".to_string(),
  attributes: ArgumentAttributes {
      default: Some("localhost".to_string()),
      optional: true,
      ..ArgumentAttributes::default()
 },
  validation_rules: vec![],
  aliases: vec![],
  tags: vec![],
 },
  ArgumentDefinition {
  name: "port".to_string(),
  description: "Database port".to_string(),
  kind: Kind::Integer,
  hint: "DB port".to_string(),
  attributes: ArgumentAttributes {
      default: Some("5432".to_string()),
      optional: true,
      ..ArgumentAttributes::default()
 },
  validation_rules: vec![],
  aliases: vec![],
  tags: vec![],
 },
 ],
  |cmd, _ctx| {
  let host = match cmd.arguments.get("host") {
  Some(Value::String(h)) => h.clone(),
  _ => "localhost".to_string(),
 };
  let port = match cmd.arguments.get("port") {
  Some(Value::Integer(p)) => *p,
  _ => 5432,
 };

  Ok(unilang::data::OutputData {
  content: format!("🔌 Connected to database at {}:{}", host, port),
  format: "text".to_string(),
 })
 }
 )
 }
}

// =============================================================================
// Demonstration: Ergonomic CLI Aggregation
// =============================================================================

fn main() -> Result<(), unilang::Error>
{
  println!("🚀 Ergonomic CLI Export and Aggregation Demo\n");

  // Step 1: Create an aggregated CLI by importing modules
  println!("📦 Building aggregated CLI from multiple modules...");

  let aggregated_cli = CliAggregator::new()
  // Add math module with "math" prefix -> .math.add, .math.multiply
  .add_module(math_cli::export(), Some("math"))?

  // Add file module with "fs" prefix -> .fs.list, .fs.copy
  .add_module(file_cli::export(), Some("fs"))?

  // Add database module with "db" prefix -> .db.connect
  .add_module(db_cli::export(), Some("db"))?

  // Add a main info command for the aggregated CLI
  .main_command("info", "Show information about this aggregated CLI", |_cmd, _ctx| {
  Ok(unilang::data::OutputData {
  content: "🎯 Unified CLI v1.0\n\nAvailable modules:\n  📊 Math operations (.math.*)\n  📁 File operations (.fs.*)\n  🗄️  Database operations (.db.*)\n\nUse '.' to list all commands or '.module.command.help' for help.".to_string(),
  format: "text".to_string(),
 })
 })?

  .build();

  let pipeline = Pipeline::new(aggregated_cli);

  // Step 2: Demonstrate the aggregated CLI in action
  println!("✅ CLI built successfully!\n");
  println!("🧮 Testing Math Module:");

  let result = pipeline.process_command_simple(".math.add a::15 b::25");
  println!("   Command: .math.add a::15 b::25");
  println!("   Result: {}\n", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

  let result = pipeline.process_command_simple(".math.multiply x::7 y::8");
  println!("   Command: .math.multiply x::7 y::8");
  println!("   Result: {}\n", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

  println!("📁 Testing File Module:");
  let result = pipeline.process_command_simple(".fs.list path::/home/user");
  println!("   Command: .fs.list path::/home/user");
  println!("   Result: {}\n", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

  let result = pipeline.process_command_simple(".fs.copy source::readme.txt dest::backup.txt");
  println!("   Command: .fs.copy source::readme.txt dest::backup.txt");
  println!("   Result: {}\n", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

  println!("🗄️ Testing Database Module:");
  let result = pipeline.process_command_simple(".db.connect host::production.db port::5432");
  println!("   Command: .db.connect host::production.db port::5432");
  println!("   Result: {}\n", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

  println!("ℹ️ Testing Main CLI Info:");
  let result = pipeline.process_command_simple(".info");
  println!("   Command: .info");
  println!("   Result: {}\n", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

  // Step 3: Demonstrate help system works perfectly
  println!("❓ Testing Help System:");

  // Auto-generated help
  let result = pipeline.process_command_simple(".math.add.help");
  println!("   Command: .math.add.help (auto-generated)");
  println!("   Help available: {}", result.success);

  // ?? parameter
  let result = pipeline.process_command_simple(".fs.copy \"??\"");
  println!("   Command: .fs.copy \"??\" (help parameter)");
  println!("   Help available: {}", result.success);

  // Traditional ? operator
  let result = pipeline.process_command_simple(".db.connect ?");
  println!("   Command: .db.connect ? (help operator)");
  println!("   Help available: {}", result.success);

  // List all commands
  println!("\n📋 Listing all available commands:");
  let result = pipeline.process_command_simple(".");
  println!("   Command: .");
  println!("   Success: {}\n", result.success);

  println!("🎉 Summary:");
  println!("  ✅ Successfully exported CLIs from 3 separate modules");
  println!("  ✅ Combined them with intuitive prefixes (math, fs, db)");
  println!("  ✅ All help functionality works seamlessly");
  println!("  ✅ Type safety and validation maintained");
  println!("  ✅ Ergonomic API that's hard to misuse");

  Ok(())
}

*/