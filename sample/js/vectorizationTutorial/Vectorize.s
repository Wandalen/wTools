let _ = require( 'wTools' );

// scalar routine
function sum( a )
{
  return a + 5;
}

// vectorized routine
var o =
{
  routine : sum,
  vectorizingMapKeys : 1,
  vectorizingMapVals : 1,
  select : 1,
}
let sumV = _.vectorize( o );

// log examples
console.log( sumV( [ 0, 1, 2, 'a' ] ) );
console.log( sumV( { a : 'a', b : 4, c : 5 } ) );


// scalar routine
function arr()
{
  return _.longSlice( arguments );
}

// vectorized routine
var o =
{
  routine : arr,
  vectorizingMapKeys : 1,
  vectorizingMapVals : 0,
  select : 2,
}
let changeKeys = _.vectorize( o );

// log examples
console.log( changeKeys( [ 0, 1, 2, 'a' ], [ 1, 2, 3, 4 ] ) );
console.log( changeKeys( [ 1, 2, 3 ], { a : 'a', b : 4 } ) );

// vectorized routine
var o =
{
  routine : arr,
  vectorizingArray : 1,
  vectorizingMapKeys : 0,
  vectorizingMapVals : 0,
  select : 2,
}
let arrV = _.vectorize( o );

// log examples
console.log( arrV( [ 0, 1, 2, 'a' ], [ 1, 2, 3, 4 ] ) );
console.log( arrV( { a : 5 }, [ 1, 2, 3 ] ) );
console.log( arrV( { a : 5 }, 1 ) );

// vectorized routine
var o =
{
  routine : arr,
  vectorizingArray : 1,
  select : 'a',
}
let selectByKey = _.vectorize( o );

// log examples
console.log( selectByKey( [ 0, 1, 2, 'a' ] ) );
console.log( selectByKey( { a : 2, b : [ 3, 4 ] } ) );
console.log( selectByKey( { a : [ 5, 4, 3 ], b : [ 1, 2 ], c : 0 } ) );

// vectorized routine
var o =
{
  routine : arr,
  vectorizingArray : 1,
  select : [ 'a', 'b' ],
}
let selectByKeys = _.vectorize( o );

// log examples
let result = selectByKeys( { a : [ 5, 4, 3 ], b : [ 'a', 2 ] } );
console.log( result );
for( let i = 0; i < result.length; i++ )
console.log( result[ i ] );
