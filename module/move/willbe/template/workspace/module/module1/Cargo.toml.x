[package]
name = "example_module"
version = "0.2.0"
edition = "2021"
license = "MIT"
readme = "Readme.md"
repository = "{{repository url}}"

[lints]
workspace = true

[package.metadata.docs.rs]
features = [ "full" ]
all-features = false

exclude = [ "/tests", "/examples", "-*" ]
