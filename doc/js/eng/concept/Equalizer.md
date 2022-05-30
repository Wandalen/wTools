# Equalizer

This is a routine that compares two elements of an array and returns <code>true</code> if the comparison condition is met,
otherwise - <code>false</code>. The `equalizer` as a callback function is passed to the routines intended for working with [Long](./Long.md) types.

<!-- xxx : check consistancy -->

The `equalizer` takes two arguments, as opposed to the [evaluator](./Evaluator.md#evaluator). An `equalizer` is necessary
if you want to specify an arbitrary algorithm for comparing the elements of the `Long` container, for example for sorting.

### Example

```js
var dst = [ { val : 3 }, { val : 5 }, { val : 0 } ];

function equalize( e1, e2 )
{
  return e1.val === e2.val;
}

var result = _.arrayRemoveElement( dst, { val : 5 }, equalize );
console.log( result );
/* log : [ { val : 3 }, { val : 0 } ] */
```

The `arrayRemoveElement` routine will remove from the `dst` array those elements, which when comparing with `compareWithThis`
will satisfy the comparison condition, i.e. the `equalize` will return <code>true</code>.
At each iteration, `equalize` is called with two parameters - `dst[ i ]` and `compareWithThis`.
In this case, the comparison condition is such that <code>true</code> will be returned for the pair of elements in which
the values of the `val` property are equal.
Therefore, one element - `{val: 5}` will be removed from the `dst`.

[Back to content](../README.md#concepts)

