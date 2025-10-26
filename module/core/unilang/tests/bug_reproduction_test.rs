//! Direct reproduction of quoted multiword bug with conflicting parameter types

use unilang::*;

#[test]
fn test_bug_reproduction_with_integer_parameter_conflict()
{
  // Reproduce the EXACT scenario from the bug report:
  // - Command with String parameter "cmd"
  // - Command with Integer parameter "threads" (alias "t")
  // - Input: cmd::"echo test"  
  // - Bug: "test" gets matched to "threads" via alias "t" and fails Integer parsing

  let mut cmd_args = vec![];
  
  // cmd parameter (String)
  let mut cmd_arg = ArgumentDefinition::new("cmd", Kind::String);
  cmd_arg.attributes.hint = Some("Command to execute".to_string());
  cmd_args.push(cmd_arg);
  
  // threads parameter (Integer with alias "t")  
  let mut threads_arg = ArgumentDefinition::new("threads", Kind::Integer);
  threads_arg.attributes.hint = Some("Number of threads".to_string());
  threads_arg.attributes.aliases = vec!["t".to_string()];
  threads_arg.attributes.optional = true;
  cmd_args.push(threads_arg);
  
  let mut cmd_def = CommandDefinition::new(".test.foreach");
  cmd_def.description = Some("Simulates .crates.for.each".to_string());
  cmd_def.arguments = cmd_args;

  let parser = unilang_parser::UnilangParser::new(Default::default());
  
  // Test the exact input from bug report
  println!("\n=== BUG REPRODUCTION TEST ===");
  println!("Testing: cmd::\"echo test\" (quoted multi-word value)");
  
  let input = r#"cmd::"echo test""#;
  let full_cmd = format!(".test.foreach {}", input);
  println!("Full command: {}", full_cmd);
  
  let parse_result = parser.parse_single_instruction(&full_cmd);
  
  match parse_result {
    Ok(instruction) => {
      println!("✅ Parser accepted the instruction");
      println!("   Instruction: {:?}", instruction);
      
      // Now validate semantically
      let mut registry = CommandRegistry::new();
      registry.register(cmd_def);
      
      let validation_result = registry.validate(&instruction);
      
      match validation_result {
        Ok(validated) => {
          println!("✅ Semantic validation SUCCESS");
          if let Some(cmd_values) = validated.named_arguments.get("cmd") {
            if !cmd_values.is_empty() {
              println!("   cmd value: {:?}", cmd_values[0]);
              if let Value::String(s) = &cmd_values[0] {
                assert_eq!(s, "echo test", "Expected 'echo test' but got '{}'", s);
                println!("\n✅ BUG IS FIXED: Correctly parsed multi-word quoted value");
              }
            }
          }
        }
        Err(e) => {
          let err_msg = format!("{}", e);
          println!("❌ Semantic validation FAILED: {}", err_msg);
          
          if err_msg.contains("invalid digit") {
            panic!("\n🐛 BUG CONFIRMED!\n   Parser split 'echo test' incorrectly\n   'test' was matched to 'threads' parameter (via alias 't')\n   Then failed to parse 'test' as Integer\n   Error: {}", err_msg);
          } else {
            panic!("\n❌ Unexpected validation error: {}", err_msg);
          }
        }
      }
    }
    Err(e) => {
      panic!("\n❌ Parser FAILED (unexpected): {}", e);
    }
  }
  
  println!("\n✅ TEST PASSED: Bug is FIXED or doesn't exist in current code");
}
