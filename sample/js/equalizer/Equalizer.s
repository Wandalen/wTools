if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;

var dst = [ { val : 3 }, { val : 5 }, { val : 0 } ];

function equalize( e1, e2 )
{
  return e1.val === e2.val;
}

var result = _.arrayRemoveElement( dst, { val : 5 }, equalize );
console.log( result );
/* log : [ { val : 3 }, { val : 0 } ] */

