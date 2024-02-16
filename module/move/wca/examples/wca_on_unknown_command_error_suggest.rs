//! Using this feature, when calling a command with an invalid name, the error text will contain a sentence with a correction, e.g. if you type:
//! ```shell
//! cargo r --features on_unknown_command_error_suggest --example wca_on_unknown_command_error_suggest .echoo
//! ```
//! you will see the message:
//! ```shell
//! Validation error. Can not identify a command.
//! Details: Command not found. Maybe you mean `.echo`?
//! ```
#[ cfg( feature = "on_unknown_command_error_suggest" ) ]
fn main() {
		use wca::prelude::*;

		let ca = CommandsAggregator::former()
		.grammar(
		[
		  Command::former()
			.phrase("echo")
			.hint("prints all subjects and properties")
			.subject("Subject", Type::String, true)
			.property("property", "simple property", Type::String, true)
			.form(),
		] )
		.executor(
		[
		  ( "echo".to_owned(), Routine::new( | ( args, props ) |
			{
			  println!( "= Args\n{args:?}\n\n= Properties\n{props:?}\n" );
				Ok(())
			} )
			),
		] )
		.build();

		let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
		match ca.perform( args.join( " " ) ) {
				Ok( _ ) => {}
				Err( err ) => println!( "{err}" ),
		};
}

