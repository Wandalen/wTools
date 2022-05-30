# Array Families

## Abstract

  wTools provides a collection of functions to work with arrays.
  Among such routines you may find several families: append, prepend, remove, replace.
  In this file, we are going to see the families naming patterns work, and see the differences between the functions of a same family.

## Naming patterns

  Names of routines of this families have patterns for better understand the behavior of each routine, here comes a quick summary of the information a function name can give us:

  `array{ Op }{ Tense }{ Second }{ How }`

### Example

```
  array{Append}{ed}{Element}{Once}
```

### Where

    { Op } can be one of [ `Append`, `Prepend`, `Remove`, `Flatten`, `Replace` ].

    { Tense } can be one of [ `-`, `ed` ].

    { Second } can be one of [ `-` , `element`, `array`, `arrays` ].

    { How } can be one of [ `-` , `Once`, `OnceStrictly` ].

### So

  * { Op } corresponds to the action to perform.  

  * { Tense } gives information on what to return ( the output value ).  
    - `-` for dst array,
    - `ed` for index, number of elements or element itself.

  * { Second } gives a hint on how the routine treats the second argument.  
    - `-` treat the second argument
    - `element` treat the second argument as a scalar
    - `array` treat the second argument as an array
    - `arrays` treat the second argument as an array of Arrays

  * { How } tells us how the routine treats the repeats.  
    - `-` the routine performs the action for every match
    - `Once` the routine performs the action on the first matching element
    - `OnceStrictly` the routine will perform the action once, and only once

  For more information on these patters please see [ ArrayRoutines.md ](ArrayRoutines.md).

## Methods of arrayAppend

  Let´s see now all the different functions that cover the functionality of appending elements to an the array
  ( the small family of arrayFlatten methods has been included in the last row - it is not related with arrayAppend ):

  | **arrayAppend** | **-** | **Once** | **OnceStrictly** | **ed** | **edOnce** | **edOnceStrictly** |
  | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
  | **-** | dst + | dst | dst | index + | index | index + |
  | **Element** | dst | dst + | dst + | index \| appended element | appended element + | appended element + |
  | **Array** | dst | dst | dst | number | number | number + |
  | **Arrays** | dst | dst | dst | number | number | number + |
  | **arrayFlatten** | dst | dst | dst | number | number | number + |

### Where

    `-` means that the corresponding function hasn´t been developped.

    `+` means that the corresponding function has been recently developped.

    `dst` corresponds to the input array with the appended element( s ) included at its end.

    `index` corresponds to the position of the appended element in the destination array ( -1 if not appended ).

    `number` corresponds to the number of appended elements.

    `index | appended element` means that the corresponding function currently returns `index`,
    but should return `appended element`.

### Example

  The table rows and columns correspond to naming patterns, and by combining them we get the full function names. If we take the cell of the row `Array` and the column `edOnce`, it corresponds to the function `arrayAppendedArrayOnce`.  
  The values inside the cells are the outputs of these functions, in our case `number`.  
  Therefore, `arrayAppendedArrayOnce` returns the number of appended elements.

## Methods of arrayRemove

  Let´s see now all the different functions that cover the functionality of removing elements from the array:

  | **arrayRemove** | **-** | **Once** | **OnceStrictly** | **ed** | **edOnce** | **edOnceStrictly** |
  | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
  | **-** | dst | dst | dst | number | index | index |
  | **Element** | dst | dst | dst | number | index \| removed element | removed element |
  | **Array** | dst | dst | dst | number | number | number |
  | **Arrays** | dst | dst | dst | number | number | number |
  | **All** | dst \| - | - | - | number \| - | - | - |

## Methods of arrayReplace

  Let´s see now all the different functions that cover the functionality of replacing elements in the array:

  | **arrayReplace** | **-** | **Once** | **OnceStrictly** | **ed** | **edOnce** | **edOnceStrictly** |
  | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
  | **-** | dst + | dst | dst | number + | index | index |
  | **Element** | dst + | dst + | dst + | number + | replaced element + | replaced element + |
  | **Array** | dst + | dst | dst | number + | number | number + |
  | **Arrays** | dst + | dst + | dst + | number + | number + | number + |
  | **All** | dst \| - | - | - | number \| - | - | - |

## Methods of arrayPrepend

  Let´s see now all the different functions that cover the functionality of prepending elements to an the array:

  | **arraPrepend** | **-** | **Once** | **OnceStrictly** | **ed** | **edOnce** | **edOnceStrictly** |
  | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
  | **-** | dst + | dst | dst | index + | index | index + |
  | **Element** | dst | dst + | dst + | index \| prepended element | prepended element + | prepended element + |
  | **Array** | dst | dst | dst | number | number |  number + |
  | **Arrays** | dst | dst | dst | number | number | number + |
