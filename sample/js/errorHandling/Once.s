let _ = require( '../..' );

function decrement( i )
{
  try
  {
    if( i <= 0 )
    throw _.error.err( 'negative!' );
    return i-1;
  }
  catch( err )
  {
    let error = _.error.err( err, '\nFailed to decrement' );
    if( _.error.once( error ) )
    console.log( error );
    throw error;
  }
}

function divide( i )
{
  try
  {
    if( i % 2 === 1 )
    throw _.error.err( 'odd!' );
    return decrement( i / 2 );
  }
  catch( err )
  {
    let error = _.error.err( err );
    if( _.error.once( error ) )
    console.log( error );
    throw error;
  }
}

try
{
  divide( 0 );
}
catch( err )
{
  let error = _.error.err( err );
  if( _.error.once( error ) )
  console.log( error );
}

/*
  Error logged once.
  States of an error make possible to account it and avoid logging second time.
*/
