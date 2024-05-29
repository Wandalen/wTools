use super::*;
use the_module::exposed::*;

tests_impls!
{
  fn simple()
  {
    fn command( () : (), args : Args, props : Props) -> Result< (), () >
    {
      Ok( () )
    }

    fn command2( () : (), args : Args, props : Props ) -> Result< (), () >
    {
      Ok( () )
    }

    fn echo( () : (), args : Args, props : Props ) -> Result< (), () >
    {
      Ok( () )
    }

    let ca = the_module::cui( () ).command( command ).command( command2 ).command( echo.arg( "string", Type::String ) ).build();

    a_id!( (), ca.perform( ".command2 .help" ).unwrap() );

    a_id!( (), ca.perform( ".help command" ).unwrap() );
    a_id!( (), ca.perform( ".help command2" ).unwrap() );
    a_id!( (), ca.perform( ".help help" ).unwrap() );

    a_id!( (), ca.perform( ".help.command" ).unwrap() );
    a_id!( (), ca.perform( ".help.command2" ).unwrap() );
    a_id!( (), ca.perform( ".help.help" ).unwrap() );

    a_true!( ca.perform( ".help.help.help" ).is_err() );
    a_true!( ca.perform( ".echo 34" ).is_ok() );
    a_true!( ca.perform( ".echo" ).is_err() );
  }
}

tests_index!
{
  simple
}
