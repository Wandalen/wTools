# Evaluator

This is a routine or pair of routines that allow you to arbitrarily convert array elements before comparing them. If the obtained values match - the evaluator returns <code>true</code>, otherwise - <code>false</code>. The evaluator as a callback function is passed to the routines intended for working with [Long](./Long.md) types.

The evaluator accepts one argument, as opposed to [equalizer](./Equalizer.md).
If the evaluator consists of one routine, it applies to both the container elements passed in the first argument and the container elements passed in the second argument.

### Example: an evaluator consisting of one routine

```js
var dst = [ { val : 3 }, { val : 0 }, { val : 5 } ];
var result = _.arrayRemoveElement( dst, { val : 0 }, ( e ) => e.val );
console.log( result );
/* log : [ { val : 3 }, { val : 5 } ] */
```

The `arrayRemoveElement` routine contains the following arguments:

- array with maps `dst = [ { val : 3 }, { val : 0 }, { val : 5 } ]`;
- comparison element `{ val : 0 }`;
- evaluator `( e ) => e.val`.

To remove the `{ val: 0 }` element from the `dst` container, you cannot simply use the comparison  `===`. You must first convert elements to their equivalent forms. The `arrayRemoveElement` routine iterates over the elements of the array, applying the evaluator at each step to both the container element and the comparison element. And only then compares the returned values using the operator `===`.

In this case, the second element of the `dst` container will be removed.

The second argument can be a scalar, as in the example below.

```js
var dst = [ { val : 3 }, { val : 0 }, { val : 5 } ];
var result = _.arrayRemoveElement( dst, 0, ( e ) => e.val );
console.log( result );
/* log : [ { val : 3 }, { val : 0 }, { val : 5 } ] */
```

The evaluator did not find any matches, because the second argument `0` after the conversion will have the value` undefined`. No element in the `dst` container will be converted to this value after using the evaluator. This problem is solved in the example below.

If the evaluator consists of two routines, the first applies to the elements of the container passed in the first argument, and the second applies to the elements of the container passed in the second argument.

### Example: an evaluator consisting of two routines

We can specify two evaluators, separately for the first and for the second argument.

```js
var dst = [ { val : 3 }, { val : 0 }, { val : 5 } ];
var evalutor1 = ( e ) => e.val;
var evalutor2 = ( e ) => e;
var result = _.arrayRemoveElement( dst, 0, evalutor1, evalutor2 );
console.log( result );
/* log : [ { val : 3 }, { val : 5 } ] */
```

After executing the program in the `dst` container there will be no element `{val: 0}` despite the fact that the second argument is `0`.

The `evalutor1` applies only to the elements of the` dst` container, and the `evaluator2` applies only to the second argument, which is the` 0` scalar.

[Back to content](../README.md#Concepts)

