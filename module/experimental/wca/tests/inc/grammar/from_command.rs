use super :: *;

use the_module :: { parser ::Parser, Type, Value, grammar ::Dictionary, verifier ::Verifier };

//

tests_impls! {
  fn command_validation()
  {
  // init parser
  let parser = Parser;

  // init converter
  let dictionary = &Dictionary ::former()
  .command
  (
   the_module ::grammar ::Command ::former()
   .hint( "hint" )
   .long_hint( "long_hint" )
   .phrase( "command" )
   .form()
 )
  .form();
  let verifier = Verifier;

  // existed command
  let raw_command = parser.parse( [ ".command" ] ).unwrap().commands.remove( 0 );

  let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

  // not existed command
  let raw_command = parser.parse( [ ".invalid_command" ] ).unwrap().commands.remove( 0 );

  let grammar_command = verifier.to_command( dictionary, raw_command );
  a_true!( grammar_command.is_err() );

  // invalid command syntax
  let raw_command = parser.parse( [ "invalid_command" ] );
  a_true!( raw_command.is_err() );
 }

  fn subjects()
  {
  // init parser
  let parser = Parser;
  let dictionary = &Dictionary ::former()
  .command
  (
   the_module ::grammar ::Command ::former()
   .hint( "hint" )
   .long_hint( "long_hint" )
   .phrase( "command" )
   .subject().hint( "first subject" ).kind( Type ::String ).end()
   .form()
 )
  .form();

  // init converter
  let verifier = Verifier;

  // with only one subject
  let raw_command = parser.parse( [ ".command", "subject" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

  a_id!( vec![ Value ::String( "subject".to_string() ) ], grammar_command.args.0 );
  a_true!( grammar_command.props.is_empty() );

  // with more subjects that it is set
  let raw_command = parser.parse( [ ".command", "subject1", "subject2" ] ).unwrap().commands.remove( 0 );

  let grammar_command = verifier.to_command( dictionary, raw_command );
  a_true!( grammar_command.is_err() );

  // with subject and property that isn't declared
  let raw_command = parser.parse( [ ".command", "subject", "prop: value" ] ).unwrap().commands.remove( 0 );

  a_true!( verifier.to_command( dictionary, raw_command ).is_err() );

  // subject with colon when property not declared
  let raw_command = parser.parse( [ ".command", "prop: value" ] ).unwrap().commands.remove( 0 );

  let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();
  a_id!( vec![ Value ::String( "prop: value".to_string() ) ], grammar_command.args.0 );
  a_true!( grammar_command.props.is_empty() );
 }

  fn subject_type_check()
  {
  // init parser
  let parser = Parser;

  // init converter
  let dictionary = &Dictionary ::former()
  .command
  (
   the_module ::grammar ::Command ::former()
   .hint( "hint" )
   .long_hint( "long_hint" )
   .phrase( "command" )
   .subject().hint( "number value" ).kind( Type ::Number ).optional( true ).end()
   .form()
 )
  .form();
  let verifier = Verifier;

  // string when number expected
  let raw_command = parser.parse( [ ".command", "subject" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command );
  a_true!( grammar_command.is_err() );

  // valid negative float number when number expected
  let raw_command = parser.parse( [ ".command", "-3.14" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();
 }

  fn subject_with_list()
  {
  // init parser
  let parser = Parser;

  // init converter
  let dictionary = &Dictionary ::former()
  .command
  (
   the_module ::grammar ::Command ::former()
   .hint( "hint" )
   .long_hint( "long_hint" )
   .phrase( "command" )
   .subject().hint( "Subjects list" ).kind( Type ::List( Type ::String.into(), ',' ) ).optional( true ).end()
   .form()
 )
  .form();
  let verifier = Verifier;

  // with only one subject
  let raw_command = parser.parse( [ ".command", "first_subject,second_subject,third_subject" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( &dictionary, raw_command ).unwrap();

  a_id!( vec!
  [
   Value ::List( vec!
   [
  Value ::String( "first_subject".into() ),
  Value ::String( "second_subject".into() ),
  Value ::String( "third_subject".into() ),
 ])
 ], grammar_command.args.0 );
  a_true!( grammar_command.props.is_empty() );
 }

  fn subject_is_optional_basic()
  {
  // init parser
  let parser = Parser;

  // init converter
  let dictionary = &Dictionary ::former()
  .command
  (
   the_module ::grammar ::Command ::former()
   .hint( "hint" )
   .long_hint( "long_hint" )
   .phrase( "command" )
   .subject().hint( "This subject is optional" ).kind( Type ::String ).optional( true ).end()
   .form()
 )
  .form();
  let verifier = Verifier;

  // with subject
  let raw_command = parser.parse( [ ".command", "subject" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

  // without subject
  let raw_command = parser.parse( [ ".command" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();
 }

  fn preferred_non_optional_first_order()
  {
  // init parser
  let parser = Parser;

  // init converter
  let dictionary = &Dictionary ::former()
  .command
  (
   the_module ::grammar ::Command ::former()
   .hint( "hint" )
   .long_hint( "long_hint" )
   .phrase( "command" )
   .subject().hint( "This subject is optional and type number" ).kind( Type ::Number ).optional( true ).end()
   .subject().hint( "This subject is required and type that accepts the optional one" ).kind( Type ::String ).optional( false ).end()
   .form()
 )
  .form();
  let verifier = Verifier;

  // second subject is required, but missing
  let raw_command = parser.parse( [ ".command", "42" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command );
  a_true!( grammar_command.is_err(), "subject identifies as first subject" );

  // first subject is missing
  let raw_command = parser.parse( [ ".command", "valid_string" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

  // both subjects exists
  let raw_command = parser.parse( [ ".command", "42", "string" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

  // first subject not a number, but both arguments exists
  let raw_command = parser.parse( [ ".command", "not_a_number", "string" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command );
  a_true!( grammar_command.is_err(), "first subject not a number" );
 }

  fn properties()
  {
  // init parser
  let parser = Parser;

  // init converter
  let dictionary = &Dictionary ::former()
  .command
  (
   the_module ::grammar ::Command ::former()
   .hint( "hint" )
   .long_hint( "long_hint" )
   .phrase( "command" )
   .subject().hint( "Any string." ).kind( Type ::String ).optional( true ).end()
   .form()
 )
  .form();
  let verifier = Verifier;

  // with subject
  let raw_command = parser.parse( [ ".command", "prop1: value1" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

  a_id!( vec![ Value ::String( "prop1: value1".to_string() ) ], grammar_command.args.0 );
  a_true!( grammar_command.props.0.is_empty() );

  // with different subject
  let raw_command = parser.parse( [ ".command", "prop1: value" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

  a_id!( vec![ Value ::String( "prop1: value".to_string() ) ], grammar_command.args.0 );
  a_true!( grammar_command.props.0.is_empty() );

  // without subject
  let raw_command = parser.parse( [ ".command" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

  a_true!( grammar_command.args.0.is_empty() );
  a_true!( grammar_command.props.0.is_empty() );
 }

  fn property_type_check()
  {
  // init parser
  let parser = Parser;

  // init converter
  let dictionary = &Dictionary ::former()
  .command
  (
   the_module ::grammar ::Command ::former()
   .hint( "hint" )
   .long_hint( "long_hint" )
   .phrase( "command" )
   .subject().hint( "Any string." ).kind( Type ::String ).optional( true ).end()
   .property( "prop" ).hint( "Number property" ).kind( Type ::Number ).optional( true ).end()
   .routine( || println!( "hello" ) )
   .form()
 )
  .form();
  let verifier = Verifier;

  // string when number expected
  let raw_command = parser.parse( [ ".command", "prop: Property" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command );
  a_true!( grammar_command.is_err() );

  // valid negative float number when number expected
  let raw_command = parser.parse( [ ".command", "prop: -3.14" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();
 }

  fn property_with_list()
  {
  // init parser
  let parser = Parser;

  // init converter
  let dictionary = &Dictionary ::former()
  .command
  (
   the_module ::grammar ::Command ::former()
   .hint( "hint" )
   .long_hint( "long_hint" )
   .phrase( "command" )
   .subject().hint( "Any string." ).kind( Type ::String ).optional( true ).end()
   .property( "prop" ).hint( "Numbers list property" ).kind( Type ::List( Type ::Number.into(), ',' ) ).optional( true ).end()
   .routine( || println!( "hello" ) )
   .form()
 )
  .form();
  let verifier = Verifier;

  // with only one subject
  let raw_command = parser.parse( [ ".command", "prop: 1,2,3" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

  a_true!( grammar_command.args.0.is_empty() );
  a_id!
  (
   vec![ 1.0, 2.0, 3.0 ],
   Vec :: < f64 > ::from( grammar_command.props.0[ "prop" ].clone() )
 );
 }

  fn alias_property()
  {
  // init parser
  let parser = Parser;

  // init converter
  let dictionary = &Dictionary ::former()
  .command
  (
   the_module ::grammar ::Command ::former()
   .hint( "hint" )
   .long_hint( "long_hint" )
   .phrase( "command" )
   .subject().hint( "Any string." ).kind( Type ::String ).optional( true ).end()
   .form()
 )
  .form();
  let verifier = Verifier;

  // with subject
  let raw_command = parser.parse( [ ".command", "property: value" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

  a_id!( vec![ Value ::String( "property: value".to_string() ) ], grammar_command.args.0 );
  a_true!( grammar_command.props.0.is_empty() );

  // with different subject
  let raw_command = parser.parse( [ ".command", "prop: value" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

  a_id!( vec![ Value ::String( "prop: value".to_string() ) ], grammar_command.args.0 );
  a_true!( grammar_command.props.0.is_empty() );

  // with another subject
  let raw_command = parser.parse( [ ".command", "p: value" ] ).unwrap().commands.remove( 0 );
  let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

  a_id!( vec![ Value ::String( "p: value".to_string() ) ], grammar_command.args.0 );
  a_true!( grammar_command.props.0.is_empty() );
 }
}

//

tests_index! {
  command_validation,
  subjects,
  subject_type_check,
  subject_with_list,
  subject_is_optional_basic,
  preferred_non_optional_first_order,
  properties,
  property_type_check,
  property_with_list,
  alias_property,
}
