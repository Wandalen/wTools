use super::*;
use wca::*;

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

    let ca = wca::cui( () ).command( command ).command( command2 ).command( echo.arg( "string", Type::String ) ).build();

    a_id!( Ok( () ), ca.perform( ".command2 .help" ) );

    a_id!( Ok( () ), ca.perform( ".help command" ) );
    a_id!( Ok( () ), ca.perform( ".help command2" ) );
    a_id!( Ok( () ), ca.perform( ".help help" ) );

    a_id!( Ok( () ), ca.perform( ".help.command" ) );
    a_id!( Ok( () ), ca.perform( ".help.command2" ) );
    a_id!( Ok( () ), ca.perform( ".help.help" ) );

    a_true!( ca.perform( ".help.help.help" ).is_err() );
    a_true!( ca.perform( ".echo 34" ).is_ok() );
    a_true!( ca.perform( ".echo" ).is_err() );
  }
}

tests_index!
{
  simple
}
