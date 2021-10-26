if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;

var dst = [ { val : 3 }, { val : 0 }, { val : 5 } ];
var evalutor1 = ( e ) => e.val;
var evalutor2 = ( e ) => e;
var result = _.arrayRemoveElement( dst, 0, evalutor1, evalutor2 );
console.log( result );
/* log : [ { val : 3 }, { val : 5 } ] */

