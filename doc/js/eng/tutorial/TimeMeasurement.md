# How to measure time

Measurement of algorithms performance to choose the best solution.

A problem can have more than one solution, and measurement of the running time is a tool for choosing the best solution.

### Time measurement

Measure the current time with the routine `_.time.now()` or alternatively `Date.now()` before running the algorithm.

``` js
var time = _.time.now();
```

Save the result to a variable or a constant. Get the time difference with the routine `_.time.spent()` after running the algorithm.

```js
_.time.spent( time );
```

Alternatively, it is possible to call `_.time.now()` again and get the difference between the two measurements, but then it requires the manual transformation of the units.

### Example

To demonstrate the time measurement technique, create a file `Performance.js` with the following content.

<details>
<summary><u>Code of the file <code>Performance.js</code></u></summary>

``` js
let _ = require( 'wTools' );
let times = 10;
let size = 50000000;
let array = new U8x( size );

var counter = 0;
var time = _.time.now();
for( let i = times ; i > 0; i-- )
var result = forLoop( array, () => counter += 1 );
console.log( `For loop took ${_.time.spent( time )} on Njs ${process.version}` );
console.info( `Output ${counter} to avoid unwanted optimization` );

var counter = 0;
var time = _.time.now();
for( let i = times ; i > 0; i-- )
var result = forEach( array, () => counter += 1 );
console.log( `For each took ${_.time.spent( time )} on Njs ${process.version}` );
console.info( `Output ${counter} to avoid unwanted optimization` );

function forLoop( src, onEach )
{
  for( let k = 0 ; k < src.length ; k++ )
  onEach( src[ k ], k, src );
  return src
}

function forEach( src, onEach )
{
  src.forEach( ( e, k, src ) => onEach( e, k, src ) );
  return src;
}
```

</details>
<br>
The above code has 2 implementations of algorithms iterating elements of an array. It is `forLoop` and` forEach`.
The testing program should consist of 2 cases ( can be written as 1 case, then by changing size of entity and number of iterations - run code to produce results for the second case ):

- Large entity, small amount of iterations ( measure cyclic, mostly code that runs on entity );
- Small entity, large amount of iterations ( measure non-cyclic, mostly code that runs outside of entity );

The size of entity and iterations should be choosen to always produce similar results .

The program measures the time it takes to iterate elements of an array of length 50 000 000, run both algorithms 10 times.<br>
Also the program measures the time it takes to iterate elements of an array of length 500, run both algorithms 100000 times.<br>
The program outputs the variable `counter` to avoid unwanted optimization.
After completion, time that the test routine took should be analyzed to make sure that it runs nearly times of algorithms checked combined.
### Measurement results

The measurements results should be compiled into a table indicating the interpreter and its version. The results may differ significantly depending on the interpreter and its version. Also test case should be specified.
Can use abbreviations such as :

- BASI = big array( length : \<length> ), small amount of iterations ( size : \<size> )
- SABI = small array ( length : \<length> ), big amount of iterations ( size : \<size> )

| Interpretor  | forLoop( BASI ) | forEach(BASI) | forLoop( SABI ) | forEach(SABI) |
| ------------ | --------------- | ------------- | --------------- | ------------- |
| Njs v10.16.0 | 9.940 s         | 17.410 s      | 0.134 s         | 0.728 s       |
| Njs v11.3.0  | 10.182 s        | 17.403 s      | 0.137 s         | 0.758 s       |
| Njs v12.7.0  | 11.533 s        | 21.673 s      | 0.134 s         | 0.801 s       |
| Njs v13.3.0  | 11.448 s        | 21.324 s      | 0.135 s         | 0.811 s       |

The report shows that the `for` loop works at least twice as fast as the` forEach` loop. On this basis, the researcher should recommend using the 1st implementation of the algorithm.

### Validity and pitfalls

It is difficult to guarantee the validity of the measurements results, but it is possible to avoid some common mistakes. The obtained data of time measurement results are reliable if it is possible to make predictions about the performance of the algorithm.

The impact of an environment must be minimized. Weighty tasks that are performing in the background on your machine can significantly distort the result. The results cannot be compared if they were obtained under different conditions. If the environment changes, then the result changes. The results measured on the different machines cannot be compared. And no matter how hard you try, there is still the impact of uncontrollable factors. So, you need to do the measurement several times and take the average or total value.

The measurement cannot be valid if the time interval is a millisecond. Finger rule: The interval should be at least one second.

Another mistake: including test data generation in the total time. Input data generation takes time. The data should be prepared before the start of time measurement and used when needed.

Interpreter optimization is another pitfall. The interpreter may throw away some code if it notices that it is not being used. So the tested algorithm should have valid arguments, and the result of the execution should be used somehow.

### Summary

- Time can be measured with a couple of routines `_.time.now` and `_.time.spent`.
- Minimize the influence of the environment to ensure the validity of the results.
- To minimize the inaccuracy, run the experiment multiple times and use the average or total time.
- The duration of an experiment should be long enough to eliminate the possibility of significant inaccuracy.
- Data generation must be separated from the algorithm which you measure.
- The runtime dynamic introduces an inaccuracy into the measurement, so the algorithm should be executed several times to obtain an average value.
- Dynamic environment impact should be minimized: equal load, same versions, one machine.
- The performance of an algorithm by one interpreter may differ from the performance of the algorithm by another interpreter. The report should include information about the interpreter and its version.
- To avoid unwanted optimization, use the result of the algorithm being tested, and pass the correct input arguments.

[Back to content](../README.md#Tutorials)

