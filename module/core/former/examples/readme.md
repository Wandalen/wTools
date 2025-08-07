# Former Crate Examples

This directory contains runnable examples demonstrating various features and use cases of the `former` crate and its associated derive macros (`#[ derive( Former ) ]`, `#[ derive( Assign ) ]`, etc.).

Each file focuses on a specific aspect, from basic usage to advanced customization and subforming patterns.

## How to Run Examples

To run any of the examples listed below, navigate to the `former` crate's root directory (`module/core/former`) in your terminal and use the `cargo run --example` command, replacing `<example_name>` with the name of the file (without the `.rs` extension).

**Command:**

```sh
# Replace <example_name> with the desired example file name
cargo run --example <example_name>
```

**Example:**

```sh
# From the module/core/former directory:
cargo run --example former_trivial
```

**Note:** Some examples might require specific features to be enabled if you are running them outside the default configuration, although most rely on the default features. Check the top of the example file for any `#[ cfg(...) ]` attributes if you encounter issues.

## Example Index

| Group                | Example File                                                                 | Description                                                                                          |
|----------------------|------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------|
| **Basic Usage**      | [former_trivial.rs](./former_trivial.rs)                                     | Basic derive usage with required/optional fields.                                                    |
|                      | [former_many_fields.rs](./former_many_fields.rs)                             | Derive usage with various field types (primitives, String, Option, Vec, HashMap) using scalar setters. |
| **Collections**      | [former_collection_vector.rs](./former_collection_vector.rs)                 | Building a `Vec` using `#[ subform_collection ]` and `.add()`.                                       |
|                      | [former_collection_hashmap.rs](./former_collection_hashmap.rs)               | Building a `HashMap` using `#[ subform_collection ]` and `.add( ( k, v ) )`.                          |
|                      | [former_collection_hashset.rs](./former_collection_hashset.rs)               | Building a `HashSet` using `#[ subform_collection ]` and `.add( value )`.                            |
| **Customization**    | [former_custom_defaults.rs](./former_custom_defaults.rs)                     | Specifying custom default values with `#[ former( default = ... ) ]`.                                |
|                      | [former_custom_setter.rs](./former_custom_setter.rs)                         | Defining an alternative custom setter method on the Former struct.                                   |
|                      | [former_custom_setter_overriden.rs](./former_custom_setter_overriden.rs)     | Overriding a default setter using `#[ scalar( setter = false ) ]`.                                   |
|                      | [former_custom_scalar_setter.rs](./former_custom_scalar_setter.rs)           | Defining a custom *scalar* setter manually (contrasting subform approach).                           |
| **Subformers**       | [former_custom_subform_scalar.rs](./former_custom_subform_scalar.rs)         | Building a nested struct using `#[ subform_scalar ]`.                                                |
|                      | [former_custom_subform_collection.rs](./former_custom_subform_collection.rs) | Implementing a custom *collection* subformer setter manually.                                        |
|                      | [former_custom_subform_entry.rs](./former_custom_subform_entry.rs)           | Building collection entries individually using `#[ subform_entry ]` and a custom setter helper.      |
|                      | [former_custom_subform_entry2.rs](./former_custom_subform_entry2.rs)         | Building collection entries individually using `#[ subform_entry ]` with fully manual closure logic. |
| **Advanced**         | [former_custom_mutator.rs](./former_custom_mutator.rs)                       | Using `#[ storage_fields ]` and `#[ mutator( custom ) ]` with `impl FormerMutator`.                  |
|                      | [former_custom_definition.rs](./former_custom_definition.rs)                 | Defining a custom `FormerDefinition` and `FormingEnd` to change the formed type.                   |
|                      | [former_custom_collection.rs](./former_custom_collection.rs)                 | Implementing `Collection` traits for a custom collection type.                                       |
| **Component Model**  | [former_component_from.rs](./former_component_from.rs)                       | Using `#[ derive( ComponentFrom ) ]` for type-based field extraction.                                |
| **Debugging**        | [former_debug.rs](./former_debug.rs)                                         | Using the struct-level `#[ debug ]` attribute to view generated code.                                |
