if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;

// Input element
var got = _.arrayPrependElement( [ 1, 2, 3, 3, 4 ], 3 );
logger.log( 'Element prepended', got )

// Input element + *Once pattern
var got = _.arrayPrependOnce( [ 1, 2, 3, 4 ], 0 );
logger.log( 'Element prepend with *Once ', got )

var got = _.arrayPrependOnce( [ 1, 2, 3, 4 ], 3 );
logger.log( 'Element not prepended because already in src', got )

// Input element + *OnceStrictly pattern
var got = _.arrayPrependOnceStrictly( [ 1, 2, 3, 4 ], [ 1 ] );
logger.log( 'Element prepended with OnceStrictly', got )

// Decomment to test - Should throw error because of *Strictly
//var got = _.arrayPrependOnceStrictly( [ 1, 2, 3, 4 ], 1 );
//logger.log( 'Should throw error', got )
