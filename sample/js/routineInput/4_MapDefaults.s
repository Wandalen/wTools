if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;

function arrayOfRandomInts( o )
{
  if( !o.times )
  o.times = arrayOfRandomInts.defaults.times;

  let result = [];
  for( let i = 0; i < o.times; i++ )
  result.push( _.intRandom([ -100, 100 ]) );

  return result;
}
arrayOfRandomInts.defaults =
{
  times : 1
}

console.log( arrayOfRandomInts( {} ) );
/* log array with 1 random integer : [ 23 ] */
console.log( arrayOfRandomInts({ times : 2 }) );
/* log array with 2 random integers : [ -57, 93 ] */

