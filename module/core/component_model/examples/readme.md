# Former Crate Examples

This directory contains runnable examples demonstrating various features and use cases of the `component_model` crate and its associated derive macros (`#[ derive( Former ) ]`, `#[ derive( Assign ) ]`, etc.).

Each file focuses on a specific aspect, from basic usage to advanced customization and subforming patterns.

## How to Run Examples

To run any of the examples listed below, navigate to the `component_model` crate's root directory (`module/core/component_model`) in your terminal and use the `cargo run --example` command, replacing `<example_name>` with the name of the file (without the `.rs` extension).

**Command:**

```sh
# Replace <example_name> with the desired example file name
cargo run --example <example_name>
```

**Example:**

```sh
# From the module/core/component_model directory:
cargo run --example component_model_trivial
```

**Note:** Some examples might require specific features to be enabled if you are running them outside the default configuration, although most rely on the default features. Check the top of the example file for any `#[ cfg(...) ]` attributes if you encounter issues.

## Example Index

| Group                | Example File                                                                 | Description                                                                                          |
|----------------------|------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------|
| **Basic Usage**      | [component_model_trivial.rs](./component_model_trivial.rs)                                     | Basic derive usage with required/optional fields.                                                    |
|                      | [component_model_many_fields.rs](./component_model_many_fields.rs)                             | Derive usage with various field types (primitives, String, Option, Vec, HashMap) using scalar setters. |
| **Collections**      | [component_model_collection_vector.rs](./component_model_collection_vector.rs)                 | Building a `Vec` using `#[ subform_collection ]` and `.add()`.                                       |
|                      | [component_model_collection_hashmap.rs](./component_model_collection_hashmap.rs)               | Building a `HashMap` using `#[ subform_collection ]` and `.add( ( k, v ) )`.                          |
|                      | [component_model_collection_hashset.rs](./component_model_collection_hashset.rs)               | Building a `HashSet` using `#[ subform_collection ]` and `.add( value )`.                            |
| **Customization**    | [component_model_custom_defaults.rs](./component_model_custom_defaults.rs)                     | Specifying custom default values with `#[ component_model( default = ... ) ]`.                                |
|                      | [component_model_custom_setter.rs](./component_model_custom_setter.rs)                         | Defining an alternative custom setter method on the Former struct.                                   |
|                      | [component_model_custom_setter_overriden.rs](./component_model_custom_setter_overriden.rs)     | Overriding a default setter using `#[ scalar( setter = false ) ]`.                                   |
|                      | [component_model_custom_scalar_setter.rs](./component_model_custom_scalar_setter.rs)           | Defining a custom *scalar* setter manually (contrasting subform approach).                           |
| **Subcomponent_models**       | [component_model_custom_subform_scalar.rs](./component_model_custom_subform_scalar.rs)         | Building a nested struct using `#[ subform_scalar ]`.                                                |
|                      | [component_model_custom_subform_collection.rs](./component_model_custom_subform_collection.rs) | Implementing a custom *collection* subcomponent_model setter manually.                                        |
|                      | [component_model_custom_subform_entry.rs](./component_model_custom_subform_entry.rs)           | Building collection entries individually using `#[ subform_entry ]` and a custom setter helper.      |
|                      | [component_model_custom_subform_entry2.rs](./component_model_custom_subform_entry2.rs)         | Building collection entries individually using `#[ subform_entry ]` with fully manual closure logic. |
| **Advanced**         | [component_model_custom_mutator.rs](./component_model_custom_mutator.rs)                       | Using `#[ storage_fields ]` and `#[ mutator( custom ) ]` with `impl FormerMutator`.                  |
|                      | [component_model_custom_definition.rs](./component_model_custom_definition.rs)                 | Defining a custom `FormerDefinition` and `FormingEnd` to change the formed type.                   |
|                      | [component_model_custom_collection.rs](./component_model_custom_collection.rs)                 | Implementing `Collection` traits for a custom collection type.                                       |
| **Component Model**  | [component_model_component_from.rs](./component_model_component_from.rs)                       | Using `#[ derive( ComponentFrom ) ]` for type-based field extraction.                                |
| **Debugging**        | [component_model_debug.rs](./component_model_debug.rs)                                         | Using the struct-level `#[ debug ]` attribute to view generated code.                                |
