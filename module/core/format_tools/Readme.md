# Module :: format_tools

<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_reflect_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_reflect_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/format_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/format_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Freflect_tools%2Fexamples%2Freflect_tools_trivial.rs,RUN_POSTFIX=--example%20reflect_tools_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Collection of mechanisms for formatting and serialization into string.

### Basic use-case

<!-- {{# generate.module{} #}} -->

Using the `to_string_with_fallback` macro to convert values to strings with a primary and fallback formatting method.

```rust
fn main()
{
  #[ cfg( feature = "enabled" ) ]
  {

    // Import necessary traits and the macro from the `format_tools` crate.
    use core::fmt;
    use format_tools::
    {
      WithDebug,
      WithDisplay,
      to_string_with_fallback,
    };

    // Define a struct that implements both Debug and Display traits.
    struct Both;

    // Implement the Debug trait for the Both struct.
    impl fmt::Debug for Both
    {
      fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
      {
        write!( f, "This is debug" )
      }
    }

    // Implement the Display trait for the Both struct.
    impl fmt::Display for Both
    {
      fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
      {
        write!( f, "This is display" )
      }
    }

    // Define a struct that implements only the Debug trait.
    struct OnlyDebug;

    // Implement the Debug trait for the OnlyDebug struct.
    impl fmt::Debug for OnlyDebug
    {
      fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
      {
        write!( f, "This is debug" )
      }
    }

    // Example usage: Using Both which implements both Debug and Display.
    let src = Both;
    // Convert the struct to a string using `to_string_with_fallback` macro.
    // The primary formatting method WithDisplay is used.
    let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
    let exp = "This is display".to_string();
    // Assert that the result matches the expected value.
    assert_eq!( got, exp );

    // Example usage: Using OnlyDebug which implements only Debug.
    let src = OnlyDebug;
    // Convert the struct to a string using `to_string_with_fallback` macro.
    // The primary formatting method WithDisplay is not available, so the fallback WithDebug is used.
    let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
    let exp = "This is debug".to_string();
    // Assert that the result matches the expected value.
    assert_eq!( got, exp );
    
  }
}
```

### To add to your project

```sh
cargo add format_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/foramt_tools_trivial
cargo run
```
