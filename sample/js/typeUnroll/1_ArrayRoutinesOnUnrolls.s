if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;

// creating unroll array
var unroll = _.unroll.make([ 2, 3, 4 ]);

var result = _.arrayAppend( unroll, 5 );
/* returns : [ 2, 3, 4, 5 ] */
console.log( _.unroll.is( result ) );
/* log : true */

unroll.push( 'str' );
console.log( _.unroll.is( unroll ) );
/* log : true */

