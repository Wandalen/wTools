# Module :: `variadic_from`

<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_variadic_from_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_variadic_from_push.yml) [![docs.rs](https://img.shields.io/docsrs/variadic_from?color=e3e8f0&logo=docs.rs)](https://docs.rs/variadic_from) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fvariadic_from%2Fexamples%2Fvariadic_from_trivial.rs,RUN_POSTFIX=--example%20module%2Fcore%2Fvariadic_from%2Fexamples%2Fvariadic_from_trivial.rs/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

The `variadic_from` crate provides a powerful procedural macro and helper traits to simplify the creation of flexible constructors for Rust structs. It automates the implementation of `From`-like traits, allowing structs to be instantiated from a variable number of arguments or tuples, reducing boilerplate and enhancing code readability.

### Features

*   **Variadic Constructors:** Easily create instances of structs from 0 to 3 arguments using the `from!` macro.
*   **Derive Macro (`VariadicFrom`):** Automatically implements `FromN` traits and standard `From<T>`/`From<tuple>` for structs with 1, 2, or 3 fields.
*   **Tuple Conversion:** Seamlessly convert tuples into struct instances using the standard `From` and `Into` traits.
*   **Compile-time Safety:** The `from!` macro provides compile-time errors for invalid argument counts (e.g., more than 3 arguments).
*   **No Code Generation for >3 Fields:** The derive macro intelligently generates no code for structs with 0 or more than 3 fields, preventing unexpected behavior.

### Quick Start

To get started with `variadic_from`, follow these simple steps:

1.  **Add to your `Cargo.toml`:**

    ```toml
    [dependencies]
    variadic_from = "0.1" # Or the latest version
    variadic_from_meta = { path = "../variadic_from_meta" } # If using from workspace
    ```

2.  **Basic Usage Example:**

    This example demonstrates the use of the `variadic_from` macro to implement flexible constructors for a struct, allowing it to be instantiated from different numbers of arguments or tuples. It also showcases how to derive common traits like `Debug`, `PartialEq`, `Default`, and `VariadicFrom` for the struct.

    ```rust
    #[test]
    fn readme_example_basic()
    {
      use variadic_from::exposed::*;

      #[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
      struct MyStruct
      {
        a : i32,
        b : i32,
      }

      let got : MyStruct = from!();
      let exp = MyStruct { a : 0, b : 0 };
      assert_eq!( got, exp );

      let got : MyStruct = from!( 13 );
      let exp = MyStruct { a : 13, b : 13 };
      assert_eq!( got, exp );

      let got : MyStruct = from!( 13, 14 );
      let exp = MyStruct { a : 13, b : 14 };
      assert_eq!( got, exp );
    }
    ```

3.  **Expanded Code Example (What the macro generates):**

    This section shows the code that the `VariadicFrom` derive macro generates for `MyStruct` (a two-field struct), including the `From2` trait implementation and the standard `From<(T1, T2)>` implementation.

    ```rust
    #[test]
    fn readme_example_expanded()
    {
      use variadic_from::exposed::*;

      #[ derive( Debug, PartialEq, Default ) ]
      struct MyStruct
      {
        a : i32,
        b : i32,
      }

      impl From2< i32, i32 > for MyStruct
      {
        fn from2( a : i32, b : i32 ) -> Self { Self{ a : a, b : b } }
      }

      impl From< ( i32, i32 ) > for MyStruct
      {
        #[ inline( always ) ]
        fn from( ( a, b ) : ( i32, i32 ) ) -> Self
        {
          Self::from2( a, b )
        }
      }

      let got : MyStruct = from!();
      let exp = MyStruct { a : 0, b : 0 };
      assert_eq!( got, exp );

      let got : MyStruct = from!( 13 );
      let exp = MyStruct { a : 13, b : 13 };
      assert_eq!( got, exp );

      let got : MyStruct = from!( 13, 14 );
      let exp = MyStruct { a : 13, b : 14 };
      assert_eq!( got, exp );
    }
    ```

### Macro Behavior Details

*   **`#[derive(VariadicFrom)]`:**
    *   For a struct with **1 field** (e.g., `struct MyStruct(i32)` or `struct MyStruct { field: i32 }`), it generates:
        *   `impl From1<FieldType> for MyStruct`
        *   `impl From<FieldType> for MyStruct` (delegating to `From1`)
    *   For a struct with **2 fields** (e.g., `struct MyStruct(i32, i32)` or `struct MyStruct { a: i32, b: i32 }`), it generates:
        *   `impl From2<Field1Type, Field2Type> for MyStruct`
        *   `impl From<(Field1Type, Field2Type)> for MyStruct` (delegating to `From2`)
        *   Additionally, it generates `impl From1<Field1Type> for MyStruct` (where `Field1Type` is used for all fields, for convenience).
    *   For a struct with **3 fields**, similar `From3` and `From<(T1, T2, T3)>` implementations are generated, along with `From1` and `From2` convenience implementations.
    *   For structs with **0 fields or more than 3 fields**, the derive macro generates **no code**. This means you cannot use `from!` or `FromN` traits with such structs unless you implement them manually.

*   **`from!` Macro:**
    *   `from!()` -> `Default::default()`
    *   `from!(arg1)` -> `From1::from1(arg1)`
    *   `from!(arg1, arg2)` -> `From2::from2(arg1, arg2)`
    *   `from!(arg1, arg2, arg3)` -> `From3::from3(arg1, arg2, arg3)`
    *   `from!(...)` with more than 3 arguments will result in a **compile-time error**.

### API Documentation

For detailed API documentation, visit [docs.rs/variadic_from](https://docs.rs/variadic_from).

### Contributing

We welcome contributions! Please see our [CONTRIBUTING.md](../../../CONTRIBUTING.md) for guidelines on how to contribute.

### License

This project is licensed under the [License](./License) file.

### Troubleshooting

*   **`Too many arguments` compile error with `from!` macro:** This means you are trying to use `from!` with more than 3 arguments. The macro currently only supports up to 3 arguments. Consider using a regular struct constructor or manually implementing `FromN` for more fields.
*   **`FromN` trait not implemented:** Ensure your struct has `#[derive(VariadicFrom)]` and the number of fields is between 1 and 3 (inclusive). If it's a 0-field or >3-field struct, the derive macro will not generate `FromN` implementations.
*   **Conflicting `From` implementations:** If you manually implement `From<T>` or `From<(T1, ...)>` for a struct that also derives `VariadicFrom`, you might encounter conflicts. Prefer using the derive macro for automatic implementations, or manually implement `FromN` traits and use the `from!` macro.

### Project Structure

The `variadic_from` project consists of two main crates:

*   `variadic_from`: The main library crate, containing the `FromN` traits, the `from!` declarative macro, and blanket implementations.
*   `variadic_from_meta`: A procedural macro crate that implements the `#[derive(VariadicFrom)]` macro.

### Testing

To run all tests for the project, including unit tests, integration tests, and doc tests:

```sh
cargo test --workspace
```

To run tests for a specific crate:

```sh
cargo test -p variadic_from --all-targets
cargo test -p variadic_from_meta --all-targets
```

To run only the doc tests:

```sh
cargo test -p variadic_from --doc
```

### Debugging

For debugging procedural macros, you can use `cargo expand` to see the code generated by the macro. Add `#[debug]` attribute to your struct to see the expanded code.

```sh
cargo expand --example variadic_from_trivial
```

You can also use a debugger attached to your test runner.

```sh
# Example for VS Code with CodeLLDB
# In .vscode/launch.json:
# {
#     "type": "lldb",
#     "request": "launch",
#     "name": "Debug variadic_from_tests",
#     "cargo": {
#         "args": [
#             "test",
#             "--package=variadic_from",
#             "--test=variadic_from_tests",
#             "--no-run",
#             "--message-format=json-render-diagnostics"
#         ],
#         "filter": {
#             "name": "variadic_from_tests",
#             "kind": "test"
#         }
#     },
#     "args": [],
#     "cwd": "${workspaceFolder}"
# }
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools/module/core/variadic_from # Navigate to the crate directory
cargo run --example variadic_from_trivial
