if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;

var dst = [ { val : 3 }, { val : 0 }, { val : 5 } ];
var result = _.arrayRemoveElement( dst, 0, ( e ) => e.val );
console.log( result );
/* log : [ { val : 3 }, { val : 0 }, { val : 5 } ] */

