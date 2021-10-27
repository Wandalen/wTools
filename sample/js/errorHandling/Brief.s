let _ = require( 'wTools' );

function decrement( i )
{
  try
  {
    if( i <= 0 )
    throw _.error.brief( 'Please, specify a positive number.' );
    return i-1;
  }
  catch( err )
  {
    console.log( err );
  }
}

decrement( 0 );

/*
  Brief error report for end user.
  Brief error report does not have stacks and other diagnostic information.
*/

