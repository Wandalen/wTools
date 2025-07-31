use unilang::prelude::*;

fn main() {
    println!("Testing unilang public API...");
    
    // Test 1: Create basic types
    let mut registry = CommandRegistry::new();
    let _kind = Kind::String;
    let _attrs = ArgumentAttributes::default();
    
    // Test 2: Create a command definition
    let cmd = CommandDefinition::former()
        .name("test_cmd")
        .namespace(String::new())
        .description("Test command".to_string())
        .end();
    
    // Test 3: Create a simple routine
    let routine = Box::new(|_cmd, _ctx| {
        Ok(OutputData {
            content: "Success!".to_string(),
            format: "text".to_string(),
        })
    });
    
    // Test 4: Register command
    registry.command_add_runtime(&cmd, routine).unwrap();
    
    // Test 5: Use Pipeline
    let pipeline = Pipeline::new(registry);
    let result = pipeline.process_command_simple("test_cmd");
    
    assert!(result.success, "Command should execute successfully");
    assert_eq!(result.outputs[0].content, "Success!");
    
    println!("✅ All tests passed! The unilang public API is working correctly.");
}