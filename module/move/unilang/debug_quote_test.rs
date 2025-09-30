use unilang_parser::*;

fn main() {
    let parser = Parser::new(UnilangParserOptions::default());

    println!("Testing malformed quotes...");

    let test_cases = vec![
        r#".malformed content::"unclosed quote"#,
        r#".malformed content:unclosed quote""#,
        r#".malformed content::"properly closed""#,
    ];

    for (i, test_case) in test_cases.iter().enumerate() {
        println!("\nTest case {}: {}", i + 1, test_case);
        match parser.parse_single_instruction(test_case) {
            Ok(instruction) => {
                println!("  SUCCESS: Parsed successfully");
                println!("    Command: {:?}", instruction.command_path_slices);
                if !instruction.named_arguments.is_empty() {
                    for (name, args) in &instruction.named_arguments {
                        for arg in args {
                            println!("    Named arg: {} = '{}'", name, arg.value);
                        }
                    }
                }
                if !instruction.positional_arguments.is_empty() {
                    for arg in &instruction.positional_arguments {
                        println!("    Positional arg: '{}'", arg.value);
                    }
                }
            }
            Err(err) => {
                println!("  ERROR: {}", err);
            }
        }
    }
}