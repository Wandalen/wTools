let _ = require( 'wTools' );
// _.include( 'wFiles' );
// uncomment to have source code in error report

try
{
  throw _.error.err( 'Simplest error sample' );
}
catch( err )
{
  let error = err;
  throw error;
}

/*
  Simplest sample of throwing errors.
  Routune _.err generates stateful error object.
*/
