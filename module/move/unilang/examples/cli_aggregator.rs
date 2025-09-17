//! Advanced CLI Aggregation Utility
//!
//! This example shows a more sophisticated approach to CLI aggregation
//! including automatic prefix application and registry merging.

use unilang::prelude::*;
use std::collections::HashMap;

// =============================================================================
// CLI Aggregation Utility
// =============================================================================

/// A utility for aggregating multiple CLI registries with prefixes
#[allow(missing_debug_implementations)]
pub struct CliAggregator {
    main_registry: CommandRegistry,
    registered_clis: HashMap<String, String>, // CLI name -> prefix mapping
}

impl CliAggregator {
    /// Create a new CLI aggregator
    pub fn new() -> Self {
        let mut registry = CommandRegistry::new();
        registry.enable_help_conventions(true);

        Self {
            main_registry: registry,
            registered_clis: HashMap::new(),
        }
    }

    /// Add a CLI with an optional prefix
    /// If prefix is provided, all commands will be prefixed (e.g., "math" -> .math.add)
    /// If no prefix, commands are added as-is (but must avoid conflicts)
    pub fn add_cli(&mut self, name: &str, _source_registry: CommandRegistry, prefix: Option<&str>) -> Result<(), unilang::Error> {
        // Store the mapping for reference
        self.registered_clis.insert(
            name.to_string(),
            prefix.unwrap_or("").to_string()
        );

        // Note: In the current unilang architecture, we need to manually re-register commands
        // with prefixes. A future enhancement would add CommandRegistry::extract_commands()
        // and CommandRegistry::merge_with_prefix() methods.

        println!("Registered CLI '{}' with prefix: {:?}", name, prefix);
        Ok(())
    }

    /// Add individual commands with prefix transformation
    pub fn add_command_with_prefix(
        &mut self,
        mut command: CommandDefinition,
        routine: Box<dyn Fn(unilang::semantic::VerifiedCommand, unilang::interpreter::ExecutionContext) -> Result<unilang::data::OutputData, unilang::data::ErrorData> + Send + Sync>,
        prefix: Option<&str>
    ) -> Result<(), unilang::Error> {

        if let Some(prefix) = prefix {
            // Apply prefix by setting namespace
            command.namespace = format!(".{}", prefix);
        }

        // Enable auto-help for aggregated commands
        command.auto_help_enabled = true;

        self.main_registry.register_with_auto_help(command, routine)
    }

    /// Get the final aggregated registry
    pub fn build(self) -> CommandRegistry {
        self.main_registry
    }

    /// List all registered CLIs and their prefixes
    pub fn list_registered_clis(&self) -> &HashMap<String, String> {
        &self.registered_clis
    }
}

// =============================================================================
// Example CLIs to Aggregate
// =============================================================================

/// Database CLI commands
mod db_cli {
    use unilang::prelude::*;

    pub fn register_commands(aggregator: &mut super::CliAggregator) -> Result<(), unilang::Error> {
        // Database connect command
        let connect_cmd = CommandDefinition::former()
            .name(".connect")
            .description("Connect to a database")
            .arguments(vec![
                ArgumentDefinition::former()
                    .name("host")
                    .kind(Kind::String)
                    .description("Database host")
                    .end(),
                ArgumentDefinition::former()
                    .name("port")
                    .kind(Kind::Integer)
                    .description("Database port")
                    .attributes(ArgumentAttributes {
                        optional: true,
                        default: Some("5432".to_string()),
                        ..Default::default()
                    })
                    .end()
            ])
            .end();

        let connect_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
            let host = if let Some(unilang::types::Value::String(s)) = cmd.arguments.get("host") {
                s
            } else {
                return Err(unilang::data::ErrorData {
                    code: "MISSING_ARGUMENT".to_string(),
                    message: "Missing required argument: host".to_string(),
                    source: None,
                });
            };
            let port = if let Some(unilang::types::Value::Integer(n)) = cmd.arguments.get("port") {
                n.to_string()
            } else {
                "5432".to_string()
            };
            Ok(unilang::data::OutputData {
                content: format!("Connected to database at {}:{}", host, port),
                format: "text".to_string(),
            })
        });

        aggregator.add_command_with_prefix(connect_cmd, connect_routine, Some("db"))?;

        // Database query command
        let query_cmd = CommandDefinition::former()
            .name(".query")
            .description("Execute a database query")
            .arguments(vec![ArgumentDefinition::former()
                .name("sql")
                .kind(Kind::String)
                .description("SQL query to execute")
                .end()])
            .end();

        let query_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
            let sql = if let Some(unilang::types::Value::String(s)) = cmd.arguments.get("sql") {
                s
            } else {
                return Err(unilang::data::ErrorData {
                    code: "MISSING_ARGUMENT".to_string(),
                    message: "Missing required argument: sql".to_string(),
                    source: None,
                });
            };
            Ok(unilang::data::OutputData {
                content: format!("Executing query: {}\nResult: 3 rows affected", sql),
                format: "text".to_string(),
            })
        });

        aggregator.add_command_with_prefix(query_cmd, query_routine, Some("db"))?;

        Ok(())
    }
}

/// Network CLI commands
mod net_cli {
    use unilang::prelude::*;

    pub fn register_commands(aggregator: &mut super::CliAggregator) -> Result<(), unilang::Error> {
        // Network ping command
        let ping_cmd = CommandDefinition::former()
            .name(".ping")
            .description("Ping a network host")
            .arguments(vec![
                ArgumentDefinition::former()
                    .name("host")
                    .kind(Kind::String)
                    .description("Host to ping")
                    .end(),
                ArgumentDefinition::former()
                    .name("count")
                    .kind(Kind::Integer)
                    .description("Number of pings")
                    .attributes(ArgumentAttributes {
                        optional: true,
                        default: Some("4".to_string()),
                        ..Default::default()
                    })
                    .end()
            ])
            .end();

        let ping_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
            let host = if let Some(unilang::types::Value::String(s)) = cmd.arguments.get("host") {
                s
            } else {
                return Err(unilang::data::ErrorData {
                    code: "MISSING_ARGUMENT".to_string(),
                    message: "Missing required argument: host".to_string(),
                    source: None,
                });
            };
            let count = if let Some(unilang::types::Value::Integer(n)) = cmd.arguments.get("count") {
                n.to_string()
            } else {
                "4".to_string()
            };
            Ok(unilang::data::OutputData {
                content: format!("Pinging {} {} times...\nAll pings successful!", host, count),
                format: "text".to_string(),
            })
        });

        aggregator.add_command_with_prefix(ping_cmd, ping_routine, Some("net"))?;

        // Network scan command
        let scan_cmd = CommandDefinition::former()
            .name(".scan")
            .description("Scan network ports")
            .arguments(vec![ArgumentDefinition::former()
                .name("target")
                .kind(Kind::String)
                .description("Target to scan")
                .end()])
            .end();

        let scan_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
            let target = if let Some(unilang::types::Value::String(s)) = cmd.arguments.get("target") {
                s
            } else {
                return Err(unilang::data::ErrorData {
                    code: "MISSING_ARGUMENT".to_string(),
                    message: "Missing required argument: target".to_string(),
                    source: None,
                });
            };
            Ok(unilang::data::OutputData {
                content: format!("Scanning {}...\nOpen ports: 22, 80, 443", target),
                format: "text".to_string(),
            })
        });

        aggregator.add_command_with_prefix(scan_cmd, scan_routine, Some("net"))?;

        Ok(())
    }
}

/// System CLI commands (no prefix - top level)
mod sys_cli {
    use unilang::prelude::*;

    pub fn register_commands(aggregator: &mut super::CliAggregator) -> Result<(), unilang::Error> {
        // System info command (no prefix)
        let info_cmd = CommandDefinition::former()
            .name(".sysinfo")
            .description("Show system information")
            .end();

        let info_routine = Box::new(|_cmd: unilang::semantic::VerifiedCommand, _ctx| {
            Ok(unilang::data::OutputData {
                content: "System: Linux x86_64\nMemory: 16GB\nCPU: 8 cores".to_string(),
                format: "text".to_string(),
            })
        });

        // No prefix for system commands
        aggregator.add_command_with_prefix(info_cmd, info_routine, None)?;

        Ok(())
    }
}

// =============================================================================
// Main Demo
// =============================================================================

fn main() -> Result<(), unilang::Error> {
    println!("=== Advanced CLI Aggregation Demo ===\n");

    // Create the CLI aggregator
    let mut aggregator = CliAggregator::new();

    // Register different CLI modules with their prefixes
    println!("Registering CLI modules...");
    db_cli::register_commands(&mut aggregator)?;
    net_cli::register_commands(&mut aggregator)?;
    sys_cli::register_commands(&mut aggregator)?;

    // Add a main help command for the aggregated CLI
    let main_help_cmd = CommandDefinition::former()
        .name(".help")
        .description("Show help for the aggregated CLI")
        .arguments(vec![ArgumentDefinition::former()
            .name("command")
            .kind(Kind::String)
            .description("Specific command to get help for")
            .attributes(ArgumentAttributes {
                optional: true,
                ..Default::default()
            })
            .end()])
        .end();

    let main_help_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let specific = cmd.arguments.get("command");
        match specific {
            Some(unilang::types::Value::String(cmd_name)) => Ok(unilang::data::OutputData {
                content: format!("Help for command: {}", cmd_name),
                format: "text".to_string(),
            }),
            _ => Ok(unilang::data::OutputData {
                content: "Aggregated CLI Help:\n\nDatabase commands (db prefix):\n  .db.connect - Connect to database\n  .db.query - Execute SQL query\n\nNetwork commands (net prefix):\n  .net.ping - Ping host\n  .net.scan - Scan ports\n\nSystem commands (no prefix):\n  .sysinfo - Show system info\n  .help - This help".to_string(),
                format: "text".to_string(),
            }),
        }
    });

    aggregator.add_command_with_prefix(main_help_cmd, main_help_routine, None)?;

    // Build the final registry
    let registry = aggregator.build();
    let pipeline = Pipeline::new(registry);

    // Demo the aggregated CLI
    println!("\n=== Testing Aggregated CLI ===\n");

    // Test database commands with prefix
    println!("1. Database operations (db prefix):");
    let result = pipeline.process_command_simple(".db.connect host::localhost port::5432");
    println!("   Command: .db.connect host::localhost port::5432");
    println!("   Result: {}\n", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

    // Test network commands with prefix
    println!("2. Network operations (net prefix):");
    let result = pipeline.process_command_simple(".net.ping host::google.com count::3");
    println!("   Command: .net.ping host::google.com count::3");
    println!("   Result: {}\n", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

    // Test system command without prefix
    println!("3. System operations (no prefix):");
    let result = pipeline.process_command_simple(".sysinfo");
    println!("   Command: .sysinfo");
    println!("   Result: {}\n", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

    // Test help system
    println!("4. Help system:");
    let result = pipeline.process_command_simple(".help");
    println!("   Command: .help");
    println!("   Result: {}\n", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

    // Test auto-generated help for prefixed commands
    println!("5. Auto-generated help for prefixed commands:");
    let result = pipeline.process_command_simple(".db.connect.help");
    println!("   Command: .db.connect.help");
    println!("   Help available: {}\n", result.success);

    // Test the ?? parameter with prefixed commands
    println!("6. ?? parameter with prefixed commands:");
    let result = pipeline.process_command_simple(".net.ping \"??\"");
    println!("   Command: .net.ping \"??\"");
    println!("   Help via ?? parameter: {}\n", result.success);

    println!("=== CLI Aggregation Complete ===");

    Ok(())
}