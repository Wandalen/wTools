# Vectorization

Vectorization is the process of converting an algorithm from operating on a single input value at a time
to operating on a set of input values([vector](./Vector.md)) at one time.

A vectorized routine is a routine that can perform operations on a set of input values,
transmitted as a [vector](./Vector.md). The vectorized routine should not lose the ability to process
scalar data.

### Automatic vectorization

Vectorization can be performed using the routine [vectorize](../tutorial/Vectorize.md).

The user object is passed to the `getFullName` routine, and its full name is returned as a string.
```js
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
```
After performing the automatic vectorization of the `getFullName` routine, it can also process the data vector.
```js
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
// [ 'John Smith', 'Samantha Blum', 'Edward Pitt' ]
```

### Manual vectorization

Data operations can be complex and not be amenable to automatic vectorization or make it inappropriate.
In this case, you need to vectorize the routines manually.

For example, consider a routine that can actually be easily automatically vectorized, but it demonstrates well
manual vectorization idea.
```js
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

```
It is necessary to make it possible to process both one user object and an array of user objects.
To do this, we can change the operations on the input data as follows.
```js
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
```

[Back to content](../README.md#Concepts)
