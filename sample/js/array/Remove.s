if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;


// Input element
var got = _.arrayRemoveElement( [ 1, 2, 3, 3, 4 ], 3 );
logger.log( 'Two elements removed', got )

// Input element + *ed pattern
var got = _.arrayRemovedElement( [ 1, 2, 3, 3, 4 ], 3 );
logger.log( 'Num of elements removed', got )

// Input Array
var got = _.arrayRemoveArray( [ 1, 2, 3, 4 ], [ 3, 4 ] );
logger.log( 'One array of elements removed', got )

// Input array of arrays
var got = _.arrayRemoveArrays( [ 1, 2, 3, 4 ], [ [ 3 ], [ 4, 5 ] ] );
logger.log( 'One array of arrays  removed', got )

// evaluator
var got = _.arrayRemoveElement( [ [ 1 ], [ 2 ] ], [ 1 ], ( e ) => e[ 0 ] );
logger.log( 'Use of evaluator', got )

//equalizer
function onEqualize( a, b )
{
  return a.num === b.num;
}
var got = _.arrayRemoveElement( [ { num : 1 }, { num : 2 }, { num : 3 } ], { num : 2 }, onEqualize );
logger.log( 'Use of equalizer', got )
