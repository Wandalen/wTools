if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;

var typedArray = new Uint8Array( [ 1, 2, 3, 4, 5, 5 ] );

// Input element
var dst =  new Int8Array( [ 1, 2, 3, '4', '4', 1, 2, 1, 5 ] );

var got  = _.longOnce( dst, function( a, b )
{
  return a === b;
} );

logger.log( got )
