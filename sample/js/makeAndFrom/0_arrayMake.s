if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;

var srcArray = [ 'a', 'b', 'c' ];

var gotArray = _.arrayMake( srcArray );
console.log( gotArray ); /* log : [ 'a', 'b', 'c' ] */
console.log( gotArray === srcArray ); /* log : false */
