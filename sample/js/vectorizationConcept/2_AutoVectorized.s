let _ = require( 'wTools' );

// routine to vectorize
function getFullName( user )
{
  return `${user.firstName} ${user.lastName}`;
}

// automatically vectorized routine by vectorize( func, n ),
// func - scalar routine, n - number of parameters for vectorized routine
let getFullName2 = _.vectorize( getFullName, 1 );

let user = { firstName : 'John', lastName : 'Smith', age : 30 }
let users =
[
  { firstName : 'John', lastName : 'Smith', age : 30 },
  { firstName : 'Samantha', lastName : 'Blum', age : 25 },
  { firstName : 'Edvard', lastName : 'Pitt', age : 33 }
];

console.log( getFullName2( user ) );
// John Smith
console.log( getFullName2( users ) );
// [ 'John Smith', 'Samantha Blum', 'Edvard Pitt' ]
