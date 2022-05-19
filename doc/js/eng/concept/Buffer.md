# Buffer types

Entities for storing and processing binary data.

A buffer can have size and length. The size measures in bytes, and length measures in a number of elements. If the buffer element size is one byte, then the size is equal to the length. The buffer may consist of the same type of elements or be untyped. An untyped buffer is simply a byte sequence. A typed buffer is a sequence of elements of some primitive type of a fixed size.

### Buffers `BufferRaw`

[Type `BufferRaw`](BufferRaw.md) ( `ArrayBuffer` ) is a standard untyped buffer, a raw data buffer that is not intended to access data by this object directly but to be used by other buffers as a data container. This type allows allocating memory for the data, but it does not provide the instruments to access the data. For processing data contained in an instance of type `BufferRaw`, the mechanism of data representation is used. Representation could be done by [typed buffers](./ BufferTyped.md) or by [`BufferView` buffer](./ BufferView.md).

### Buffers `BufferTyped`

[Type `BufferTyped`](BufferTyped.md) ( `TypedArray` ) presents standard typed buffers. This type of buffer represents `BufferRaw` binary data in the form of numerical primitives such as` Int8`, `Uint32`,` Float64` and others. The typed buffers do not contain real data but are a representation of the data in `BufferRaw`. Typed buffers implement strategies for accessing to `BufferRaw` data.

### Buffers `BufferView`

[Type `BufferView`](BufferView.md) ( `DataView` ) is a standard untyped buffers that type data of `BufferRaw` dynamically. The type `BufferView` has only one implementation, it is `BufferView` (`DataView`). Buffers of the type `BufferView` as well as` BufferTyped` do not have data but represent data that are in the associated buffer `BufferRaw`. Unlike `BufferTyped`, the type `BufferView` does not type elements statically but provides methods for dynamically typing of data into any numeric primitive.

### Buffers `BufferNode`

[Type `BufferNode`](BufferNode.md) ( `Buffer` ) is a `NodeJS` non-standard implementation of untyped buffer with built-in access to buffer data. Presented by one class `BufferNode` ( `Buffer` ). The `BufferNode` class exists only in the `NodeJS` interpreter environment.

### Summary

- Buffers used for storing and processing binary data.
- Four families of buffers exist: `BufferRaw`, `BufferTyped`, `BufferView` i `BufferNode`.
- Type `BufferRaw` allocates a memory for storing and processing binary data.
- Type `BufferTyped` represents binary data in the form of numerical primitives.
- Type `BufferView` implements a set of methods for dynamically representing binary data in the form of numerical primitives.
- Type `BufferNode` is non-standard and uses by `NodeJS` interpreter.

[Back to content](../README.md#Concepts)
