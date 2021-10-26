if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;


// Input element
var got = _.arrayFlatten( [ 1, 2, 3, 3, 4 ], 3 );
logger.log( 'Array flattened', got )

// Input element + *Once pattern
var got = _.arrayFlattenOnce( [ 1, 2, 3, 4 ], 0 );
logger.log( 'Array flattened with *Once ', got )

var got = _.arrayFlattenOnce( [ 1, 2, 3, 4 ], 3 );
logger.log( 'Array flattened with repeated element', got )

// Input element + *OnceStrictly pattern
var got = _.arrayFlattenOnceStrictly( [ 1, 2, 3, 4 ], [ 5 ] );
logger.log( 'Array flattened with OnceStrictly', got )

// Decomment to test - Should throw error because of *Strictly
//var got = _.arrayFlattenOnceStrictly( [ 1, 2, 3, 4 ], 1 );
//logger.log( 'Should throw error', got )
