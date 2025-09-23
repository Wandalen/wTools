use unilang_parser::*;

fn main() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = ".region.buy_castle coord::1,1";
    let result = parser.parse_single_instruction(input);

    match result {
        Ok(instruction) => {
            println!("Parsed successfully!");
            println!("Command path: {:?}", instruction.command_path_slices);
            println!("Named arguments: {:?}", instruction.named_arguments);
        }
        Err(e) => {
            println!("Parse error: {:?}", e);
        }
    }
}