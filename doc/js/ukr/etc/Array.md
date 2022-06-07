# Призначення рутин array*

Рутини array* призначені для виконання операцій над масивами. Є п'ять операцій, що виконуються над масивами - Append, Prepend, Remove, Flatten, Replace, кожній з них відповідає одне сімейство рутин з відповідним позначенням.

Назва рутини складається з чотирьох частин:

- array - об'єкт над яким проводиться дія;
- action - дія, що виконується з об'єктами. Може бути Append, Prepend, Replace, Flatten i Replace, при цьому повертається масив. Або Appended, Prepended, Replaced, Flattened i Replaced - може повертатись індекс, кількість елементів або сам елемент;
- second - дає зрозуміти як розглядати другий аргумент. Може бути відсутнім, або ж Element, Array i Arrays.
- how many - дає зрозуміти скільки раз виконувати дію. Може бути відсутнім - рутина виконає дію для кожного елемента масиву, Once - рутина виконає дію принаймні один раз для кожного елемента масиву, OnceStrictly - рутина виконає дію тільки один раз або поверне помилку. Якщо другий елемент - масив або масиви, то дія виконується по одному разу для кожного елемента масиву.

Таким чином рутина

```js
arrayRemoveArrayOnceStrictly
```
Видалить з початкового масиву всі елементи, що є в другому масиві один раз. Якщо в першому масиві буде менше, або більше елементів, то рутина поверне помилку.

### Призначення рутин сімейства arrayPrepend

Рутини призначені для додавання елементів з другого аргумента в початок першого.

Прогляньте таблицю зі значеннями, що повертають рутини сімейства arrayPrepend. В першій колонці указано рутини, а в наступних приставки how many.

|                            | -                   | Once                | OnceStrictly               |
|----------------------------|---------------------|---------------------|----------------------------|
| arrayPrependUnrolling      | dstArray            | -                   | -                          |
| arrayPrepend               | dstArray            | dstArray            | dstArray                   |
| arrayPrependElement        | dstArray            | dstArray            | dstArray                   |
| arrayPrependArray          | dstArray            | dstArray            | dstArray                   |
| arrayPrependArrays         | dstArray            | dstArray            | dstArray                   |
| arrayPrepended             | index               | index <br> if not element was prepended <br> then -1 | index <br> if not element was prepended <br> then -1 |
| arrayPrependedElement      | index               | element <br> if not element was prepended <br> then undefined | element <br> if not element was prepended <br> then undefined; <br> if !Config.debug <br> then index |
| arrayPrependedArray        | number              | number              | number                     |
| arrayPrependedArrays       | number              | number              | number                     |


### Призначення рутин сімейства arrayAppend

Рутини призначені для додавання елементів з другого аргумента в кінець першого.

Прогляньте таблицю зі значеннями, що повертають рутини сімейства arrayAppend.

|                           | -                   | Once                | OnceStrictly               |
|---------------------------|---------------------|---------------------|----------------------------|
| arrayAppendUnrolling      | dstArray            | -                   | -                          |
| arrayAppend               | dstArray            | dstArray            | dstArray                   |
| arrayAppendElement        | dstArray            | dstArray            | dstArray                   |
| arrayAppendArray          | dstArray            | dstArray            | dstArray                   |
| arrayAppendArrays         | dstArray            | dstArray            | dstArray                   |
| arrayAppended             | index               | index               | index <br> if no element was appended <br> then -1 |
| arrayAppendedElement      | index               | element <br> if not element was appended <br> then false | element <br> if not element was appended <br> then false; <br> if !Config.debug <br> then index |
| arrayAppendedArray        | number              | number              | number                     |
| arrayAppendedArrays       | number              | number <br> if dstArray === undefined <br> then insArray  | number                     |


### Призначення рутин сімейства arrayRemove

Рутини призначені для видалення елементів з першого масиву, котрі присутні в другому аргументі.

Прогляньте таблицю зі значеннями, що повертають рутини сімейства arrayRemove.

|                           | -                   | Once                | OnceStrictly               |
|---------------------------|---------------------|---------------------|----------------------------|
| arrayRemove               | dstArray            | dstArray            | dstArray                   |
| arrayRemoveElement        | dstArray            | dstArray            | dstArray                   |
| arrayRemoveArray          | dstArray            | dstArray            | dstArray                   |
| arrayRemoveArrays         | dstArray            | dstArray            | dstArray                   |
| arrayRemoved              | number              | index               | index                      |
| arrayRemovedElement       | number              | index               | element                    |
| arrayRemovedArray         | number              | number              | number                     |
| arrayRemovedArrays        | number              | number              | number                     |
| arrayRemoveAll            | dstArray            | -                   | -                          |
| arrayRemovedAll           | number              | -                   | -                          |
| arrayRemoveDuplicates     | dstArray            | -                   | -                          |


### Призначення рутин сімейства arrayFlatten

Рутини призначені для розгортання масивів до масивів першого порядку. Якщо використовується два масиви, то вони об'єднуються.

Прогляньте таблицю зі значеннями, що повертають рутини сімейства arrayFlatten.

|                           | -                   | Once                | OnceStrictly               |
|---------------------------|---------------------|---------------------|----------------------------|
| arrayFlatten              | dstArray            | dstArray            | dstArray                   |
| arrayFlattenDefined       | dstArray            | dstArray            | dstArray                   |
| arrayFlattened            | number <br> if arguments.length === 1 <br> then dstArray | number <br> if arguments.length === 1 <br> then dstArray | number <br> if arguments.length === 1 <br> then dstArray |
| arrayFlattenedDefined     | number <br> if arguments.length === 1 <br> then dstArray | number <br> if arguments.length === 1 <br> then dstArray | number <br> if arguments.length === 1 <br> then dstArray |


### Призначення рутин сімейства arrayReplace

Рутини призначені для заміщення елементів з першого масиву, елементами другого аргумента.

Прогляньте таблицю зі значеннями, що повертають рутини сімейства arrayReplace.

|                           | -                   | Once                | OnceStrictly               |
|---------------------------|---------------------|---------------------|----------------------------|
| arrayReplace              | dstArray            | dstArray            | dstArray                   |
| arrayReplaceElement       | dstArray            | dstArray            | dstArray                   |
| arrayReplaceArray         | dstArray            | dstArray            | dstArray                   |
| arrayReplaceArrays        | dstArray            | dstArray            | dstArray                   |
| arrayReplaced             | number              | index               | index                      |
| arrayReplacedElement      | number              | ins <br> if not element was replaced <br> then undefined | ins <br> if not element was replaced <br> then undefined |
| arrayReplacedArray        | number              | number              | number                     |
| arrayReplacedArrays       | number              | number              | number                     |
| arrayReplaceAll           | dstArray            | -                   | -                          |
| arrayReplacedAll          | number              | -                   | -                          |
| arrayUpdate               | dstArray.length     | -                   | -                          |

# Особливості коду в рутинах array*

Розглядається код рутин array*, наявність однакового функціоналу і помилок.

В таблицях позначено:
- copy of - позначає, що рутина копіює код з указної далі рутини;
- return - якщо рутини повертають значення, що не відповідають функціоналу.

### Таблиця з особливостями коду рутин сімейства arrayRemove

|                            | -                   | Once                | OnceStrictly               |
|----------------------------|---------------------|---------------------|----------------------------|
| arrayRemove                | -                   | -                   | -                          |
| arrayRemoveElement         | - | - | - |
| arrayRemoveArray           | -                   | -                   | - |
| arrayRemoveArrays          | -                   | -                   | - |
| arrayRemoved               | -                   | -                   | -                          |
| arrayRemovedElement        | - | - | - |
| arrayRemovedArray          | -                   | -                   | - |
| arrayRemovedArrays         | -                   | -                   | - |
| arrayRemoveAll             | copy of arrayRemoveElement  | -                   | -                          |
| arrayRemoveDuplicates      | -                   | -                   | -                          |

### Таблиця з особливостями коду рутин сімейства arrayReplaced

|                            | -                   | Once                | OnceStrictly               |
|----------------------------|---------------------|---------------------|----------------------------|
| arrayReplace               | - | -                   | - |
| arrayReplaceElement        | - | - |  - |
| arrayReplaceArray          | -                   | -                   | - |
| arrayReplaceArrays         | -                   | -                   | - |
| arrayReplaced              | - | - | - |
| arrayReplacedElement       | - | routine return ins | routine return ins |
| arrayReplacedArray         | -                   | -                   | - |
| arrayReplacedArrays        | -                   | -                   | - |
| arrayReplaceAll            | copy of arrayReplacedElement | -                   | -                          |
| arrayReplaceUpdate         | -                   | -                   | -                          |

### Рутини, що мають однаковий функціонал

Рутина arrayRemoveAll повторює функціонал arrayRemovedElement. Це третя рутина з таким функціоналом.

Рутина arrayReplaceAll повторює функціонал arrayReplaceElement. Це третя рутина з таким функціоналом.

Рутини arrayReplacedElementOnce i arrayReplacedElementOnceStrictly мають некоректний повернений результат.

### Підсумок

- Рутина arrayRemoveAll копіює функціонал arrayRemoveElement. Краще її видалити.
- Рутина arrayReplceAll копіює функціонал arrayReplaceElement. Краще її видалити.
- Рутини arrayReplacedElementOnce i arrayReplacedElementOnceStrictly потрібно полагодити.
