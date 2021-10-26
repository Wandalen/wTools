// routine to vectorize
function getFullName( user )
{
  return `${user.firstName} ${user.lastName}`;
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
// undefined undefined
