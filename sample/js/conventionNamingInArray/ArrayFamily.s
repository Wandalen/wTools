let _ = require( 'wTools' );

//

// array[action]
var result = _.arrayAppend( [ 1, 2 ], [ 3, 4 ] );
console.log( 'Without ed' );
console.log( result );
// returns [ 1, 2, [ 3, 4 ] ] - dstArray

// array[action]ed
var result = _.arrayAppended( [ 1, 2 ], [ 3, 4 ] );
console.log( 'With ed' );
console.log( result );
// returns 2 - index

// array[action]
var result = _.arrayPrepend( [ 1, 2 ], [ 3, 4 ] );
console.log( 'Without ed' );
console.log( result );
// returns [ [ 3, 4 ], 1, 2 ] - dstArray

// array[action]ed
var result = _.arrayPrepended( [ 1, 2 ], [ 3, 4 ] );
console.log( 'With ed' );
console.log( result );
// returns 0 - index

//

// array[action][second]
var result = _.arrayRemoveElement( [ 1, 2, 2, 4, 2, 5 ], 2 );
console.log( 'Without [how many]' );
console.log( result );
// returns [ 1, 4, 5 ]

// array[action][second]Once
var result = _.arrayRemoveElementOnce( [ 1, 2, 2, 4, 2, 5 ], 2 );
console.log( 'With [Once]' );
console.log( result );
// returns [ 1, 2, 4, 2, 5 ]

// array[action][second]OnceStrictly
var result = _.arrayRemoveElementOnceStrictly( [ 1, 4, 2, 5 ], 2 );
console.log( 'With [OnceStrictly]' );
console.log( result );
// returns [ 1, 4, 5 ]

//

// array[action]Element
var result = _.arrayRemoveElement( [ 1, 2, 2, 4, 2, 5 ], [ 2, [ 4 ] ] );
console.log( 'second as element' );
console.log( result );
// returns [ 1, 2, 2, 4, 2, 5 ], routine treats element [ 2, [ 4 ] ] as scalar

// array[action]Array
var result = _.arrayRemoveArray( [ 1, 2, 2, 4, 2, 5 ], [ 2, [ 4 ] ] );
console.log( 'second as array' );
console.log( result );
// returns [ 1, 4, 5 ], routine treats inner array [ 4 ] as scalar

// array[action]Arrays
var result = _.arrayRemoveArrays( [ 1, 2, 2, 4, 2, 5 ], [ 2, [ 4 ] ] );
console.log( 'second as arrays' );
console.log( result );
// returns [ 1, 5 ]

// array[action]
var result = _.arrayFlatten( [ 1, 2, 2, 4, 2, 5 ], [ 2, [ 4 ] ] );
console.log( 'arrayFlatten' );
console.log( result );
// returns [ 1, 2, 2, 4, 2, 5, 2, 4 ]
