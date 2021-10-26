require( 'wTools' );

// original array

let origin = [ 1, 2, 'a', 'b', true, 1, [ 1 ] ];

// second argument in the routines

let elem = 1;

// routine uses argument srcArray

let srcArray = wTools.longSlice( origin );

let result = wTools.longCountElement( srcArray, elem );

console.log( 'The result is:' );
console.log( result );
console.log( 'The original array is:' );
console.log( origin );
console.log( 'srcArray changed to:' );
console.log( srcArray );

// routine uses argument dstArray

let dstArray = wTools.longSlice( origin );

let result1 = wTools.arrayAppend( dstArray, elem );

console.log( 'The result is:' );
console.log( result1 );
console.log( 'The original array is:' );
console.log( origin );
console.log( 'dstArray changed to:' );
console.log( dstArray );
