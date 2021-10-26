
let _ = require( '../..' );

function div( a, b )
{
  _.assert( b !== 0 );
  return a / b;
}

div( 5, 0 );

/*
Somewhat over trivial example of using asserts.
Asserts is test check built-in source code of the unit of testing.
*/
