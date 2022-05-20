# Type <code>BufferView</code>

 Standard untyped buffers that type data of `BufferRaw` dynamically.

[Type `BufferView`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView) has only one implementation, it is `BufferView` (`DataView`). Buffers of the type `BufferView` as well as` BufferTyped` do not have data but represent data that are in the associated buffer `BufferRaw`. Unlike `BufferTyped`, the type `BufferView` does not type elements statically but provides methods for dynamically typing of data into any numeric primitive.

To create an instance of type `BufferView`, the following arguments are passed to the class constructor: a buffer of type` BufferRaw`, offset by the number of bytes from the beginning and the number of bytes to read. If no buffer offset and size are specified, then a new `BufferView` creates from all content. The `BufferView` instance is a reference to the corresponding `BufferRaw` section, not a copy of it. It makes `BufferView` similar to` BufferTyped`. Therefore, changing one of the representations of the initial buffer `BufferRaw` changes the representation in all instances of the class` BufferView` that use this `BufferRaw`.

### Examples

```js
var raw = new BufferRaw( 20 );

var view1 = new BufferView( raw );
var view2 = new BufferView( raw, 10, 9 );

console.log( view1.getInt8( 17 ), view2.getInt8( 7 ) ); // log 0 0
console.log( view1.getUint16( 17 ), view2.getUint16( 7 ) ); // log 0 0

view1.setInt8( 17, -42 );
console.log( view1.getInt8( 17 ), view2.getInt8( 7 ) ); // log -42 -42
console.log( view1.getUint16( 17 ), view2.getUint16( 7 ) ); // log 54784 54784

view2.setUint16( 6, 100 );
console.log( view1.getInt8( 16 ), view2.getInt8( 6 ) ); // log 0 0
console.log( view1.getUint16( 16 ), view2.getUint16( 6 ) ); // log 100 100
```

In the example above, two `BufferView` views ( representations ) were created from buffer `raw`, it is `view1` and` view2`. The `view1` view uses the entire `raw` buffer. At the same time, the view `view2` uses nine bytes of the `raw` buffer, starting from the tenth byte from the beginning. The buffer `raw` allocated an initialized memory section. So initially, in each representation, each byte has a value of `0`.

Using the `setInt8` method, the view `view1` changes the value of the 17th byte of the buffer `raw` to the number` -42`. This change is also reflected in the buffer `view2`. Using the method `getUint16` shows that the same section of binary data can be interpreted differently. When reading two bytes from the 17th, the view `view1` returns the number `54784`. The value of the `view1` buffer is identical because the same section of the` raw` buffer is read.

The view `view2` using the method `setUint16` changed two bytes from 6th to number `100`. After these changes, reading one byte with index 6 returns the value `0`. It is explained by the fact that to write the number `100` is enough one lower byte of a two-byte number. When the method `getUint16` reads 2 bytes starting at 6th, the recorded number is returned, the number is `100`. The corresponding changes are reflected in the view `view1`.

### Summary

- Instances of type `BufferView` are representations of type `BufferRaw`.
- The `BufferView` instance is a reference to the corresponding `BufferRaw` section, not a copy of it.
- `BufferView` has no data.
- Data in `BufferView` is accessed using methods.
- Changes of the original buffer `BufferRaw` are reflected in all related views.

[Back to content](../README.md#Concepts)
