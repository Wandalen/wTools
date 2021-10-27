let _ = require( 'wTools' );

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
    throw _.error.err( err, '\nFailed to decrement' );
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
    throw _.error.err( err, '\nFailed to divide' );
  }
}

divide( 0 );

/*
  Routine _.err append throws stack of the error.
*/
