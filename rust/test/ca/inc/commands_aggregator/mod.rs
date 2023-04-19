use super::*;

use wca::
{
  Parser,
  GrammarConverter, ExecutorConverter,

  CommandsAggregator,
  Routine,
  HelpVariants
};

// mod basic;

#[test]
fn static_grammar() {
  // TODO: rework it to generate by macro
  let ca = CommandsAggregator::former()
  .grammar
  (
    &phf::phf_map!
    (
      "command" => &[
        wca::StaticGrammarCommand
        {
          hint : "",
          long_hint : "",
          phrase : "command",
          subjects : &[],
          properties : phf::phf_map!(),
          properties_aliases : phf::phf_map!(),
        }
      ]
    )
  )
  .executor(
  [
    ( "command".to_owned(), Routine::new( | _ | { println!( "Command" ); Ok( () ) } ) ),
  ])
  .build();

  ca.perform( ".command" ).unwrap();
}
