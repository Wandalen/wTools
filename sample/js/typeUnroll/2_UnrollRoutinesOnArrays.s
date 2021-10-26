if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;

var arr = [ 0, 1, 2, 3 ];

var result = _.unrollPrepend( arr, 4 );
/* returns : [ 4, 0, 1, 2, 3 ] */
console.log( _.unroll.is( result ) );
/* log : false */

