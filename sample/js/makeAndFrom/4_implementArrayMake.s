if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;

console.log( '4_implementArrayMake.s' );

function arrayMake( src )
{
  if( src === null || src === undefined )
  return new Array();

  if( _.number.is( src ) )
  return new Array( src );

  if( src.length === 1 )
  return [ src[ 0 ] ];
  else
  return Array.apply( Array, src );
}

module.exports = arrayMake;
