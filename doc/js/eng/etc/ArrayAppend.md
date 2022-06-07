# ArrayAppend

## Schema

  The following table shows what the outputs of the functions should be, not what they currently are ( for that see
  [ ArrayFamily.md ](ArrayFamily.md) ).  
  Let´s see now all the different functions that cover the functionality of appending elements to an the array
  ( the small family of `arrayFlatten` methods has been included in the last row `Flatten` - it is not related with `arrayAppend` ):

  | **arrayAppend** | **-**  | **Once** | **OnceStrictly** | **ed**           | **edOnce**       | **edOnceStrictly** |
  | :-------------: | :----: | :------: | :--------------: | :--------------: | :--------------: | :----------------: |
  | **-**           | dst    | dst      | dst              | index            | index            | index              |
  | **Element**     | dst    | dst      | dst              | appended element | appended element | appended element   |
  | **Array**       | dst    | dst      | dst              | number           | number           | number             |
  | **Arrays**      | dst    | dst      | dst              | number           | number           | number             |
  | **Flatten**     | dst    | dst      | dst              | number           | number           | number             |

### Where:

    `dst` corresponds to the input array with the appended element( s ) included at its end.

    `index` corresponds to the position of the appended element in the destination array ( -1 if not appended ).

    `number` corresponds to the number of appended ( flattened ) elements.

## Naming patterns

Many function names contain patterns to help us understand the behaviour of each routine, here comes a quick summary of the information a function name can give us:

The format of a function name is:  

`array{ Op }{ Tense }{ Second }{ How }`

### Where

    { Op } can be one of [ `Append`, `Prepend`, `Remove`, `Flatten`, `Replace` ],
    corresponding to the action to perform.

    { Tense } can be one of [ `-`, `ed` ], giving information on what to return ( the output value ).

    { Second } can be one of [ `-` , `element`, `array`, `arrays` ], refering to how to treat the src argument.

    { How } can be one of [ `-` , `Once`, `OnceStrictly` ], and tell us how to treat the repeats.

### Example

```
array{Append}{ed}{Array}{Once}
```

  The table rows and columns correspond to naming patterns, and by combining them we get the full function names.

### Example

  The cell of the row `Array` and the column `edOnce` corresponds to the function `arrayAppendedArrayOnce`.  
  The values inside the cells are the expected outputs of these functions:  

  ```

  `arrayAppendedArrayOnce` returns the number of appended elements.

  ```

## Examples

  Let´s see now a couple of examples of how to get all the information out of a function name containing
  the previous patterns:

### Example1:

  ```
  arrayPrependArrayOnceStrictly ->

  { op } = Prepend -> Includes the arguments to the beggining of the destination array.   
  { Tense } = '-'  -> The output is the destination array.
  { Second } = 'Array' -> Arguments are treated as arrays ( function will iterate over its elements ).
  { How } = 'OnceStrictly' -> The action is done, and only once ( repetead elements will not be included ).
  ```
  In conclusion, arrayPrependArrayOnceStrictly includes to the beggining of the destination array all elements
  of the arguments ( treated as arrays ), as long as they are not repeated. It returns the destination array.

  ```
  _.arrayPrependArrayOnceStrictly( [ 1, 2, 3 ], [ 4, 5 ] ) -> [ 4, 5, 1, 2, 3 ]
  ```

### Example2:

  ```
  arrayRemovedElement ->

  { op } = Remove -> Removes the arguments from the destination array.   
  { Tense } = 'ed'  -> The output is the number of removed elements.
  { Second } = 'Element' -> Second argument is treated as a scalar.
  { How } = '-' -> There are no limitations on how the action is done.
  ```
  In conclusion, arrayRemovedElement removes, from the destination array, the element
  in the second argument ( treated as a scalar ). It returns the number of removed elements.

  ```
  var dstArray = [ 1, 2, 2, 2, 3, 4 ];  
  _.arrayRemovedElement( dstArray, 2 ) -> 3   ( dstArray = [ 1, 3, 4 ] )

  ```

  For more information on these patterns, please see [ ArrayRoutines.md ](ArrayRoutines.md).
