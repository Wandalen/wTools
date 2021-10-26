let _ = require( 'wTools' );

// manually vectorized routine
function getFullName( user )
{
  if( _.arrayIs( user ) )
  {
    const fullNames = [];
    user.forEach( ( u ) => fullNames.push( `${u.firstName} ${u.lastName}` ) )
    return fullNames;
  }
  else
  {
    return `${user.firstName} ${user.lastName}`;
  }
}

let user = { firstName : 'John', lastName : 'Smith', age : 30 }
let users =
[
  { firstName : 'John', lastName : 'Smith', age : 30 },
  { firstName : 'Samantha', lastName : 'Blum', age : 25 },
  { firstName : 'Edvard', lastName : 'Pitt', age : 33 }
];

console.log( getFullName( user ) );
// John Smith
console.log( getFullName( users ) );
// [ 'John Smith', 'Samantha Blum', 'Edvard Pitt' ]
