if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;

var unroll1 = _.unroll.make([ '5' ]);
var unroll2 = _.unroll.make([ 's', 3, [ 4 ] ]);

// creating an unroll-array from a given array
var result = _.unroll.from([ 1, 2, unroll1, unroll2 ]);
/* returns : [ 1, 2, '5', 's', 3, [ 4 ] ] */
console.log( _.unroll.is( result ) );
/* log : true */

