function logMsgs( msg1, msg2 )
{
  if( msg1 && !msg2 )
  console.log( msg1 );

  if( !msg1 && msg2 )
  console.log( msg2, arguments[ 1 ] );  // two variants of calls of signed parameter

  if( arguments[ 2 ] )
  console.log( arguments[ 2 ] );
}

logMsgs( 'one' );
/* log : one */
logMsgs( null, 'two' );
/* log : two two */
logMsgs( null, undefined, 'three' );
/* log : three */
logMsgs( 'one', 'two', 'three', 'four', 'five' );
/* log : three */

