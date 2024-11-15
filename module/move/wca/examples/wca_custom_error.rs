//! - 

#[ derive( Debug, error_tools::typed::Error )]
enum CustomError
{
  #[ error( "this is typed error" ) ]
  TheError,
}

fn main()
{
  let ca = wca::CommandsAggregator::former()
  .command( "error.string" )
    .hint( "Returns error as a string" )
    .routine( || { Err( format!( "this is string error" ) ) } )
  .end()
  .command( "error.typed" )
    .hint( "Returns error as a custom error" )
    .routine( || { Err( CustomError::TheError ) } )
  .end()
  .command( "error.untyped" )
    .hint( "Returns error as untyped error" )
    .routine( || { Err( error_tools::error::untyped::format_err!( "this is untyped error" ) ) } )
  .end()
  .command( "error.with_context" )
    .hint( "Returns error as untyped error with context" )
    .routine( || { Err( error_tools::error::untyped::format_err!( "this is untyped error" ).context( "with context" ) ) } )
  .end()
  .perform();

  let args: Vec< String > = std::env::args().skip( 1 ).collect();
  ca.perform( args ).unwrap();
}