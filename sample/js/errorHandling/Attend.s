let _ = require( 'wTools' );

try
{
  throw _.error.err( 'Simplest error sample' );
}
catch( err )
{
  _.error.attend( err );
  console.log( `Attended: ${ err }` )
}


/*
  This sample outputs nothing.
  If error should not be logged neither halt a porcess then use routine _.error.attend to change state of error object
  from `unattended` to `atteneded`.
*/
