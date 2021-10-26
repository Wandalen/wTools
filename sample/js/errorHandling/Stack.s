let _ = require( 'wTools' );

function program()
{
  function decrement( num, dec )
  {
    console.log( _.introspector.stack() );
    return num - dec;
  }

  function divideAndDecrement( n )
  {
    return decrement( n / 2, 2 );
  }

  divideAndDecrement( 0 );
}

program();

/*
Simplest sample to illustrate sync stack.
*/
