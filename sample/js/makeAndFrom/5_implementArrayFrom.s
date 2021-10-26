if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;

let arrayMake = require( './4_implementArrayMake.s' );

console.log( '5_implementArrayFrom.s' );

function arrayFrom( src )
{
  if( _.arrayIs( src ) )
  return src;
  return arrayMake( src );
}

module.exports = arrayFrom;
