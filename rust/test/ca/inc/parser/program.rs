use super::*;

//

tests_impls!
{
  fn basic()
  {
    let parser = Parser::former().form();

    // only one command and only one namespace
    a_id!
    (
      Ok( Program { namespaces : vec!
      [
        Namespace { commands : vec!
        [
          RawCommand
          {
            name : "command".into(),
            subjects : vec![],
            properties : HashMap::new(),
          }
        ]}
      ]}),
      parser.program( ".command" )
    );

    // one command at a time in many namespaces
    a_id!
    (
      Ok( Program { namespaces : vec!
      [
        Namespace { commands : vec!
        [
          RawCommand
          {
            name : "command1".into(),
            subjects : vec![],
            properties : HashMap::new(),
          }
        ]},
        Namespace { commands : vec!
        [
          RawCommand
          {
            name : "command2".into(),
            subjects : vec![],
            properties : HashMap::new(),
          }
        ]},
        Namespace { commands : vec!
        [
          RawCommand
          {
            name : "command3".into(),
            subjects : vec![],
            properties : HashMap::new(),
          }
        ]},
      ]}),
      parser.program( ".command1 .also .command2 .also .command3" )
    );
  }
}

//

tests_index!
{
  basic,
}
