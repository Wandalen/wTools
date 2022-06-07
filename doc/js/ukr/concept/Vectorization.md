# Векторизація

Векторизація - це процес перетворення алгоритму від операцій над одним значенням за раз, до операцій над набором
значень([вектором](./Vector.md)) за один раз.

Векторизована рутина - це рутина, яка може виконувати операції над масивом вхідних даних,
переданих у вигляді [вектора](./Vector.md). Векторизована рутина не повинна втрачати здатність обробляти
скалярні дані.

### Автоматична векторизація

Векторизацію можливо виконати за допомогою рутини [vectorize](../tutorial/Vectorize.md).

У рутину `getFullName` передається об'єкт user, а повертається його повне ім'я у вигляді рядка.
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
Після виконання автоматичної векторизації рутини `getFullName`, вона здатна обробляти також вектор із даними.

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
// [ 'John Smith', 'Samantha Blum', 'Edvard Pitt' ]
```  

### Ручна векторизація

Операції над даними можуть бути складними і не піддаватись автоматичній векторизації або робити її недоцільною. 
В такому разі можливо векторизувати рутини вручну.

Для прикладу розглянемо рутину, яку, насправді, можна легко автоматично векторизувати, проте вона гарно демонструє ідею
ручної векторизації.
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
Необхідно зробити можливим опрацювання рутиною як одного об'єкту user, так і масиву об'єктів user.
Для цього змінимо операції над вхідними даними наступним чином.
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

[Повернутись до змісту](../README.md#концепції)
