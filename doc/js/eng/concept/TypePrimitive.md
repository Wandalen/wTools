# Type <code>primitive</code>

In <code>JavaScript</code>, all data types that are integral and have no changeable elements are called primitives.

The primitive data types in JavaScript include:

- `boolean`
- `undefined`
- `null`
- `number`
- `symbol`
- `string`

### Type `boolean`

The primitive data type for carrying two logical values: `true` and` false`.

```js
let a = true;
let b = false;

console.log( typeof a, typeof b ); // boolean boolean
```

### Type `undefined`

A primitive quasi-type denoting the absence of a value.

```js
let a;
console.log( typeof a );
// undefined
```

In `JavaScript` `undefined` type is handled in a special way. For example, assigning to key a value `undefined` does not delete the `key-value` pair, but replaces it with a new pair that has value `undefined`.


```js
let m = {};
m.x = undefined;
console.log( m );
// { x : undefined }
```

### Type `null`

A primitive data type with a single special value. `null` as` undefined` returns `false` when it is converted to a logical value. In opposite to `undefined`, variables of type `null` exist. `null` is often used to indicate that the variable should get the default value.

```js
let a = null;
console.log( typeof a );
// object
if( !a )
console.log( a );
// null
```

The console output shows that the type of variable is `object` and this is a feature of JavaScript.

### Type `number`

A primitive data type for numeric values. According to the ECMAScript standard, there is only one type of numbers: a 64-bit double-precision floating-point number in IEEE 754 format (takes values from -((2 ^ 53) - 1) to ((2 ^ 53) - 1) ).

For integers there is no particular data type.

The IEEE 754 format has two zero values `+0` and `-0`. If you do not specify a sign for zero, the interpreter will read it as `+0`.

In addition to floating-point numbers, this format supports three special values:
- `+Infinity` when number divided by zero;
- `-Infinity` when number divided `-0`;
- `NaN` if the value of variable is not a number.

```js
let a = 1;
let b = -1.00001;
let c = Infinity;
let d = -Infinity;
let e = NaN;

console.log( typeof a, typeof b, typeof c, typeof d, typeof e );
// number number number number number
```

### Type `symbol`

The symbol is a primitive data type, a unique and unchangeable entity, which can be used as a key in a `key-value` pair of maps. If the key is a `symbol`, the field values are not listed, so this mechanism can be used to add "invisible" fields to the map.

```js
let a = Symbol.for( 'field1' );

console.log( typeof a );
// symbol
console.log( Symbol.for( 'field1' ) === Symbol.for( 'field1' ) );
// true
console.log( Symbol.for() === Symbol() );
// false
```

The output of the line `false` indicates that the `symbol` always has a unique value.

### Type <code>string</code>

Primitive data type to represent text data.

```js
let a = 'str';
let b = "string";
let c = String( true );

console.log( typeof a, typeof b, typeof c );
// string string string
```

String types are indicated as above, that is, in single quotes, in double quotes, or the data type is created using the `String` constructor.

The strings in `JavaScript` after the declaration remain unchanged.

[Back to content](../README.md#Concepts)
