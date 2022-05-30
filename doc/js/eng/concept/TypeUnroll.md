# Type <code>unroll</code>

Data type <code>unroll</code> - a special type of array that can be unrolled in another array when performing an operation on it.

When performing operations on the `unroll-array` that are intended for a regular array ( `arrayPrepend`, `arrayAppend`, `arrayRemove`, `arrayReplace`, `arrayFlatten` and any of Array.prototype ), it behaves like a regular array and does not change its type.

```js
// creating unroll array
var unroll = _.unroll.make([ 2, 3, 4 ]);

var result = _.arrayAppend( unroll, 5 );
/* returns : [ 2, 3, 4, 5 ] */
console.log( _.unroll.is( result ) );
/* log : true */

unroll.push( 'str' );
console.log( _.unroll.is( unroll ) );
/* log : true */
```

The routine `make` of namespace `unroll` is used to create an` unroll-array`. After performing the routine `arrayAppend` and the method `push` which work with regular arrays, the `unroll-array` `unroll` retained its type.

Routines designed to work with `unroll-arrays` can be applied to regular arrays, it does not change their type.

```js
var arr = [ 0, 1, 2, 3 ];

var result = _.unrollPrepend( arr, 4 );
/* returns : [ 4, 0, 1, 2, 3 ] */
console.log( _.unroll.is( result ) );
/* log : false */
```

The routine `unrollPrepend` added the element to the array `arr`. The array `arr` has retained the type `Array`.

When using routines whose name has the prefix `unroll` - the contents of `unroll-arrays` will be unrolled.
As a result of unrolling of `unroll-arrays`, each element of `unroll-array` will become an element of an array in which there was an `unroll-array`, and the `unroll-array` will cease to be an element of its container.

```js
var unroll1 = _.unroll.make([ 7, [ 2 ] ]);
var unroll2 = _.unroll.make([ 0, 1, 'str' ]);

var result = _.unrollAppend( unroll1, unroll2 );
/* returns : [ 7, [ 2 ], 0, 1, 'str' ] */
console.log( _.unroll.is( result ) );
/* log : true */
```

Elements of two `unroll-arrays` `unroll1` and `unroll2` became elements of a single `unroll-array` `result`. The `unroll-array` `unroll2` was unrolled and its elements were added to `unroll1`.

```js
var unroll1 = _.unroll.make([ '5' ]);
var unroll2 = _.unroll.make([ 's', 3, [ 4 ] ]);

// creating an unroll-array from a given array
var result = _.unroll.from([ 1, 2, unroll1, unroll2 ]);
/* returns : [ 1, 2, '5', 's', 3, [ 4 ] ] */
console.log( _.unroll.is( result ) );
/* log : true */
```

When a new `unroll-array` is created, then nested` unroll-arrays` unrolled. The new `unroll-array` `result` contains elements of `unroll-arrays` `unroll1` and `unroll2`.

```js
var unroll1 = _.unroll.make([ '5' ]);
var unroll2 = _.unroll.make([ 'str', [ 3 ] ]);

// unrolling elements of the given array that are unroll-array
var result = _.unroll.normalize([ 0, 7, unroll1, [ unroll2, unroll1 ] ]);
/* returns : [ 0, 7, '5', [ 'str', [ 3 ],  '5' ] ] */
console.log( _.unroll.is( result ) );
/* log : false */
```

The routine `unrollNormalize` normalizes ( unrolls ) nested `unroll-arrays` and retains the type of container. The normalized array `result` retained the type `Array`.

When performing operations on a multidimensional `unroll-array` that contains other `unroll-arrays`, only the first level of each element of this array will be unrolled ( if the element is an `unroll-array` ).

```js
var unroll1 = _.unroll.make([ 1, 2 ]);
var unroll2 = _.unroll.make([ 3, 4 ]);
unroll1[ 2 ] = unroll2;

console.log( unroll1 );
/* log : [ 1, 2, [ 3, 4 ] ] */
console.log( unroll2 );
/* log : [ 3, 4 ] */

var result = _.unroll.normalize( [ unroll1, unroll2 ] );
console.log( result );
/* log :
[
  1, 2, [ 3, 4 ],
  3, 4
]
*/
```

The elements of `unroll-arrays` `unroll1` and `unroll2` are no longer elements of its containers. At the same time, the nested `unroll-array` `unroll2` remained unrolled.

### Summary

- `Unroll-arrays`, which are elements of other `unroll-arrays` and ordinary arrays - are able to unfold in them;
- When performing an operation for `unroll-arrays` on a nested `unroll-array` - such an array is unrolled;
- When performing an operation for regular arrays on a `unroll-array` it behaves like a regular array;
- When performing an operation for `unroll-arrays` on a multidimensional `unroll-array`, only the first level of each element of this array will be unrolled ( if the element is an `unroll-array` ).

[Back to content](../README.md#Concepts)
