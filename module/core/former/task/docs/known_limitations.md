# Known Limitations

## Lifetime-only Structs

Currently, the `Former` derive macro does not support structs that have only lifetime parameters without any type parameters.

### Example of unsupported code:
```rust
#[derive(Former)]
struct MyStruct<'a> {
    data: &'a str,
}
```

### Workaround

Add a phantom type parameter:

```rust
use std::marker::PhantomData;

#[derive(Former)]
struct MyStruct<'a, T = ()> {
    data: &'a str,
    _phantom: PhantomData<T>,
}
```

### Why this limitation exists

The Former macro generates code that expects at least one non-lifetime generic parameter. When a struct has only lifetime parameters, the generated code produces invalid syntax like `Former<'a, Definition>` where the lifetime appears in a position that requires a type parameter.

Fixing this would require significant refactoring of how the macro handles generics, distinguishing between:
- Structs with no generics
- Structs with only lifetimes
- Structs with only type parameters
- Structs with both lifetimes and type parameters

This is planned for a future release.