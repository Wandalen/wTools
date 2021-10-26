let _ = require( 'wTools' );

// scalar routine
function sum( a, b )
{
  return Number( a ) + Number( b );
}

// manually written vectorized routine
function sum_v1( a, b )
{
  let result = [];
  for( let i = 0; i < a.length; i++ )
  result[ i ] = sum( a[ i ], b[ i ] );
  return result;
}

let o =
{
  routine : sum,
  select : 2,
}
// automatically vectorized routine by vectorize()
let sum_v2 = _.vectorize( o );

// vectors
let a = [ 1, 3, 4, 6, 9 ];
let b = [ 4, 1, 0, 7, 3 ];

//scalar

let c = 1;
let d = 2;

// log results
console.log( sum_v1( a, b ) );
console.log( sum_v2( a, b ) );

console.log( sum_v1( c, d ) );
console.log( sum_v2( c, d ) );
