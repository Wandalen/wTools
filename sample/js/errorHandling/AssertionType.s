
let _ = require( '../..' );

function div( a, b )
{
  _.assert( b !== 0 );
  _.assert( _.intIs( a ) );
  _.assert( _.intIs( b ) );
  return ( a - ( a % b ) ) / b;
}

console.log( `The result of division : ${ div( 13, 5 ) }` );
/* output : The result of division : 2 */

/*
The example above restricts the type of arguments.
It does the same thing typing in typed languages does.
Arguments `a` allowed to be an integer as well as argument `b`.
Additionally, it is forbidden to be zero for argument `b`.
*/
