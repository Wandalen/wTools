//! This is a basic CLI application for the `unilang` module.
//! It demonstrates how to initialize the command registry,
//! parse command-line arguments, and execute commands.

use std::collections::HashMap;
use unilang::data::ArgumentAttributes;
use unilang::registry::CommandRegistry;
use unilang::data::{CommandDefinition, ArgumentDefinition, Kind, ErrorData, OutputData};
use unilang_parser::{Parser, UnilangParserOptions};
use unilang::semantic::{SemanticAnalyzer, VerifiedCommand};
use unilang::interpreter::{Interpreter, ExecutionContext};
use std::env;
use unilang::help::HelpGenerator;

/// Sample routine for the "echo" command.
#[allow(clippy::unnecessary_wraps)]
fn echo_routine(_verified_command: VerifiedCommand, _context: ExecutionContext) -> Result<OutputData, ErrorData> {
  println!("Echo command executed!");
  Ok(OutputData {
    content: "Echo command executed!".to_string(),
    format: "text".to_string(),
  })
}

/// Sample routine for the "add" command.
#[allow(clippy::needless_pass_by_value)]
fn add_routine(verified_command: VerifiedCommand, _context: ExecutionContext) -> Result<OutputData, ErrorData> {
  let a = verified_command
    .arguments
    .get("a")
    .ok_or_else(|| ErrorData {
      code: "MISSING_ARGUMENT".to_string(),
      message: "Argument 'a' not found".to_string(),
    })?
    .as_integer()
    .ok_or_else(|| ErrorData {
      code: "INVALID_ARGUMENT_TYPE".to_string(),
      message: "Argument 'a' is not an integer".to_string(),
    })?;
  let b = verified_command
    .arguments
    .get("b")
    .ok_or_else(|| ErrorData {
      code: "MISSING_ARGUMENT".to_string(),
      message: "Argument 'b' not found".to_string(),
    })?
    .as_integer()
    .ok_or_else(|| ErrorData {
      code: "INVALID_ARGUMENT_TYPE".to_string(),
      message: "Argument 'b' is not an integer".to_string(),
    })?;
  println!("Result: {}", a + b);
  Ok(OutputData {
    content: format!("Result: {}", a + b),
    format: "text".to_string(),
  })
}

/// Sample routine for the "cat" command.
#[allow(clippy::needless_pass_by_value)]
fn cat_routine(verified_command: VerifiedCommand, _context: ExecutionContext) -> Result<OutputData, ErrorData> {
  let path = verified_command
    .arguments
    .get("path")
    .ok_or_else(|| ErrorData {
      code: "MISSING_ARGUMENT".to_string(),
      message: "Argument 'path' not found".to_string(),
    })?
    .as_path()
    .ok_or_else(|| ErrorData {
      code: "INVALID_ARGUMENT_TYPE".to_string(),
      message: "Argument 'path' is not a path".to_string(),
    })?;
  let content = std::fs::read_to_string(path).map_err(|e| ErrorData {
    code: "FILE_READ_ERROR".to_string(),
    message: format!("Failed to read file: {e}"),
  })?;
  println!("{content}");
  Ok(OutputData {
    content,
    format: "text".to_string(),
  })
}

#[allow(clippy::too_many_lines)]
fn main() -> Result<(), unilang::error::Error> {
  let args: Vec<String> = env::args().collect();

  let mut registry = CommandRegistry::new();

  // Register sample commands
  let echo_def = CommandDefinition::former()
    .name("echo")
    .hint("Echoes a message.")
    .description("Echoes a message back to the console. Useful for testing connectivity or displaying simple text.")
    .status("stable")
    .version("1.0.0")
    .tags(vec!["utility".to_string(), "debug".to_string()])
    .aliases(vec!["e".to_string()])
    .permissions(vec!["public".to_string()])
    .idempotent(true)
    .form();
  registry

    .command_add_runtime(&echo_def, Box::new(echo_routine))
    .expect("Failed to register echo command");

  let add_def = CommandDefinition::former()

    .name("add")
    .hint("Adds two integers.")
    .description("Performs addition on two integer arguments and returns the sum.")
    .status("stable")
    .version("1.0.0")
    .tags(vec!["math".to_string(), "arithmetic".to_string()])
    .aliases(vec!["plus".to_string()])
    .permissions(vec!["public".to_string()])
    .idempotent(true)
    .arguments(vec![
      ArgumentDefinition::former()
        .name("a")
        .hint("The first integer operand.")
        .kind(Kind::Integer)
        .attributes(
          ArgumentAttributes::former()
            .is_default_arg(false)
            .optional(false)
            .multiple(false)
            .interactive(false)
            .sensitive(false)
            .form(),
        )
        .validation_rules(vec!["min:0".to_string()])
        .tags(vec!["operand".to_string()])
        .form(),
      ArgumentDefinition::former()
        .name("b")
        .hint("The second integer operand.")
        .kind(Kind::Integer)
        .attributes(
          ArgumentAttributes::former()
            .is_default_arg(false)
            .optional(false)
            .multiple(false)
            .interactive(false)
            .sensitive(false)
            .form(),
        )
        .validation_rules(vec!["min:0".to_string()])
        .tags(vec!["operand".to_string()])
        .form(),
    ])

    .form();
  registry

    .command_add_runtime(&add_def, Box::new(add_routine))
    .expect("Failed to register add command");

  let cat_def = CommandDefinition::former()
    .name("cat")
    .hint("Prints content of a file.")
    .description("Reads the content of a specified file and prints it to the console.")
    .status("stable")
    .version("1.0.0")
    .tags(vec!["file".to_string(), "io".to_string()])
    .aliases(vec!["type".to_string()])
    .permissions(vec!["public".to_string()])
    .idempotent(true)
    .arguments(vec![ArgumentDefinition::former()
      .name("path")
      .hint("The path to the file to read.")
      .kind(Kind::Path)
      .attributes(
        ArgumentAttributes::former()
          .is_default_arg(false)
          .optional(false)
          .multiple(false)
          .interactive(false)
          .sensitive(false)
          .form(),
      )
      .validation_rules(vec![])
      .tags(vec!["input".to_string(), "file".to_string()])
      .form()])

    .form();
  registry

    .command_add_runtime(&cat_def, Box::new(cat_routine))
    .expect("Failed to register cat command");


  let help_generator = HelpGenerator::new(&registry);

  if args.len() < 2 {
    println!("{}", help_generator.list_commands());
    eprintln!("Usage: {0} <command> [args...]", args[0]);
    return Ok(());
  }
  let mut processed_args = args.clone();
  let mut command_name_or_alias = processed_args[1].clone();

  // New alias resolution logic
  let mut alias_map: HashMap<String, String> = HashMap::new();
  for (command_name, command_def) in &registry.commands {
    for alias in &command_def.aliases {
      alias_map.insert(alias.clone(), command_name.clone());
    }
  }

  if let Some(canonical_name) = alias_map.get(&command_name_or_alias) {
    command_name_or_alias = canonical_name.clone();
    processed_args[1].clone_from(canonical_name); // Replace alias with canonical name in args
  }

  let command_name = &command_name_or_alias; // Use the resolved command name


  if command_name == "--help" || command_name == "help" {
    if args.len() == 2 {
      println!("{}", help_generator.list_commands());
    } else if args.len() == 3 {
      let mut specific_command_name = args[2].clone();
      if let Some(canonical_name) = alias_map.get(&specific_command_name) {
        specific_command_name = canonical_name.clone();
      }
      if let Some(help_message) = help_generator.command(&specific_command_name) {
        println!("{help_message}");
      } else {
        eprintln!("Error: Command '{specific_command_name}' not found for help.");
        std::process::exit(1);
      }
    } else {
      eprintln!("Error: Invalid usage of help command. Use `help` or `help <command_name>`.");
      std::process::exit(1);
    }
    return Ok(());
  }

  let parser = Parser::new(UnilangParserOptions::default());
  let command_input_str = processed_args[1..].join(" ");
  let instruction = parser.parse_single_instruction(&command_input_str)?;
  let instructions = &[instruction][..];

  let semantic_analyzer = SemanticAnalyzer::new(instructions, &registry);

  let result = semantic_analyzer.analyze().and_then(|verified_commands| {
    let mut context = ExecutionContext::default();
    let interpreter = Interpreter::new(&verified_commands, &registry);
    interpreter.run(&mut context)
  });

  match result {
    Ok(_) => Ok(()),
    Err(e) => {
      eprintln!("Error: {e}");
      std::process::exit(1);
    }
  }
}
