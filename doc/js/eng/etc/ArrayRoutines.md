# Array Methods Usage

## Definition

  wTools provides a collection of functions to work with arrays.

## Methods

  * `arrayIs( src )` - Checks if `src` is an array.
  * `arrayleftIndex/arrayRightIndex( arr, ins, evaluator1, evaluator2 )` - Returns the index of the first/last matching `ins` element of an array `arr`.  
  ( see evaluators on `EVALUATORS` )
  * `arrayRight/arrayLeft( arr, ins, evaluator1, evaluator2 )` - Returns a new object containing the properties, `index, element`, corresponding to
    a found value `ins` of an array `arr`.
  * `arrayCount( src, instance )` - Returns the count of matched elements with `instance` of the array `src`.  
  * `arrayCountUnique( src, onEvaluate )` - Returns the count of matched pairs of the array `src`.
  * `arrayPrepend*` - Adds a value of the argument `ins` to the beginning of the array `dstArray`.
    ( see options for `*` on `Patterns` )
  * `arrayAppend*` - Adds the value of the argument `ins` at the end of the array `dstArray`.
  * `arrayRemove*` - Removes the matching elements `ins` of the array `dstArray`.
  * `arrayRemoveDuplicates( dstArray, onEvaluate )` - Removes the duplicated elements of the array `dstArray`.
  * `arrayFlatten*` - Returns an array that contains all the passed arguments.
  * `arrayReplace*` - Returns an array where the matching elememts of `ins` are replaced by the ones in `sub`.
  * `arrayUpdate( dstArray, ins, sub, evaluator1, evaluator2 )` - Adds a value `sub` to the array `dstArray` or replaces a value `ins` of the array `dstArray` by `sub`.


## Patterns

  Many function names contain patterns to help us understand the behaviour of each routine, here comes a quick summary of the information a function name can give us:

    The format of a function name will be: `array { Op } { Tense } { Second } { How }`

### Where:

    { Op } can be one of [ `Append`, `Prepend`, `Remove`, `Flatten`, `Replace` ],
    corresponding to the action to perform.

    { Tense } can be `-`, or `ed`, giving information on what to return ( the output value ).

    { Second } can be one of [ `-` , `element`, `array`, `arrays` ], refering to how to treat the source argument.

    { How } can be one of [ `-` , `Once`, `OnceStrictly` ], and tell us how to treat the repeats.

### 1: Function contains `*ed`

  If the function contains `*ed`, instead of returning the destination array `dstArray`, the returned value will be
  the index, count or element ( depending on the type of the function ).

#### Example

  ```
  arrayFlatten( [ 0 ], [ 1, 2, 3 ] )   
  returns ->   [ 0, 1, 2, 3 ]  ( the destination array )

  arrayFlattened( [ 0 ], [ 1, 2, 3 ] )   
  returns ->   3 ( the count of the added elements )
  ```

### 2: Function contains `*Once`

  If the function contains `*Once`, the action will be performed only on the first matching element, or when
  the destination array `dstArray` doesn't have the value of `ins`.

#### Example

  ```
  arrayRemoveElement( [ 1, 2, 1 ], 1 )  
  returns ->    [ 2 ] ( the destination array with the removed elements )

  arrayRemoveElementOnce( [ 1, 2, 1 ], 1 )  
  returns ->    [ 2, 1 ] ( the destination array with only one removed element )

  arrayPrependElement( [ 1, 2, 3, 4, 5 ], 5 )  
  returns ->    [ 5, 1, 2, 3, 4, 5 ] ( the destination array with the prepended element )

  arrayPrependOnce( [ 1, 2, 3, 4, 5 ], 5 )  
  returns ->    [ 1, 2, 3, 4, 5 ] ( Element is not prepended because it already is in the array )
  ```

### 3: Function contains `*OnceStrictly`

  If the function contains `*OnceStrictly`, it expects that the action will be done, and only once: an error will be thrown if there are no matching elements or elements are not unique ( they are both in `src` and `ins` ).

#### Example

  ```
  arrayAppendOnceStrictly( [ 1, 2, 3 ], 3 )  
  returns ->    'array should have only unique element, but has several 3'

  arrayReplaceOnceStrictly( [ 1, 2, 3 ], 4, 5 )  
  returns ->    'array does not have element 4'
  ```

  Without `*Strictly` it would return:

#### Example

  ```
  arrayAppendOnce( [ 1, 2, 3 ], 3 )    
  returns ->    [ 1, 2, 3 ]  ( The ins element is already in the array )

  arrayReplaceOnce( [ 1, 2, 3 ], 4, 5 );    
  returns ->    [ 1, 2, 3 ]  ( No match for the ins element )
  ```

### 4: Function contains `*Element`, `*Array`, `*Arrays`

  If the function contains `*Element`, the argument `ins` is treated as a scalar.

#### Example

  ```
  - arrayRemoveElement( [ 1, 2, 3, 4 ], 3 )    
  returns ->    [ 1, 2, 4 ]

  - arrayRemoveElement( [ 1, 2, 3, 4 ], [ 3 ] )    
  returns ->    [ 1, 2, 3, 4 ]  ( 3 !== [ 3 ] - no match )
  ```

  If the function contains `*Array`, the argument `ins` is expected to be an array, and it will iterate over its elements.

#### Example

  ```
  arrayRemoveArray( [ 1, 2, 3, 4 ], [ 3, 4 ] )    
  returns ->    [ 1, 2 ]

  arrayRemoveArray( [ 1, 2, 3, 4 ], [ [ 3 ], [ 4 ] ] )
  returns   ->    [ 1, 2, 3, 4 ]
  ```

  If the function contains `*Arrays`, it will iterate over the elements of elements of `ins` recursively ( it is expected to be an array of arrays).

#### Example

  ```
  arrayRemoveArrays( [ 1, 2, 3, 4 ], [ [ 3 ], [ 4, 5 ] ] )    
  returns ->    [ 1, 2 ]
  ```


## EVALUATORS

  Most of the routines can take additional routine parameters to evaluate and compare the input elements, there are four options:

  | **Type of Evaluator** | **Applies to** | **Possible Outputs** |
  | :---: | :---: | :---: |
  | Evaluator | One value | true/false |
  | Tandem of Evaluators| Two values | true/false - true/false |
  | Equalizers | Two values | true/false |
  | Comparator |mm  Two values | - 1 / 0 / + 1 |

### 1: Evaluators

  Check `src` and `ins` values with the evaluator function ( the third argument ).  

#### Example

  ```
  arrayRemoveElement( [ [ 1 ], [ 2 ] ], [ 1 ], ( e ) => e[ 0 ] )    
  returns ->    [ [ 2 ] ]

  arrayRemoveElement( [ [ 1 ], [ 2 ] ], 1, ( e ) => e[ 0 ] )    
  returns ->    [ [ 1 ], [ 2 ] ]
  ```

### 2: Tandem of evaluators

  One evaluator to check `src` ( the third argument ) and a different one to check `ins` ( the fourth argument ).

#### Example

  ```
  arrayRemoveElement( [ [ 1 ], [ 2 ] ], 1, ( e ) => e[ 0 ], ( e ) => e )    
  returns ->    [ [ 2 ] ]
  ```

### 3: Equalizers

  Compare `src` and `ins` values with the equalizer function: returns `true` or `false` for a two values comparison.

#### Example

  ```
  var onEqualize = function( a, b )
  {
    return a.num === b.num;
  }

  arrayRemoveElement( [ { num : 1 },{ num : 2 },{ num : 3 } ], { num : 2 }, onEqualize )    
  returns ->    [ { num : 1 }, { num : 3 } ]

  Without equalizer:  
  arrayRemoveElement( [ { num : 1 },{ num : 2 },{ num : 3 } ], { num : 2 } )    
  returns ->    [ { num : 1 }, { num : 2 }, { num : 3 } ]
  ```

### 4: Comparator

  Compare `src` and `ins` values with the comparator function: returns `+ 1`, `0` or `- 1` for a two values comparison.

#### Example

  ```
  function compare(a, b)
  {
    if ( a % 2 === 0 )
    {
      return -1;
    }
    if ( b % 2 === 0 )
    {
      return 1;
    }
    return 0;
  }

  var arr = [ 0, 1, 2, 3, 4, 5 ]
  arr.sort( compare )
  returns ->    [ 0, 2, 4, 1, 3, 5 ]
  ```


## Try out  

```
npm install
node sample/SampleArrayPrepend.js
node sample/SampleArrayRemove.s
```
