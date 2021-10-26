if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;

var unroll1 = _.unroll.make([ 7, [ 2 ] ]);
var unroll2 = _.unroll.make([ 0, 1, 'str' ]);

var result = _.unrollAppend( unroll1, unroll2 );
/* returns : [ 7, [ 2 ], 0, 1, 'str' ] */
console.log( _.unroll.is( result ) );
/* log : true */

