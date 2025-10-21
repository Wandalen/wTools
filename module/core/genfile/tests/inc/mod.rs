use genfile_core :: *;

mod basic_test;
mod value_test;
mod parameter_test;
mod values_test;

#[ cfg( feature = "renderer" ) ]
mod renderer_test;

mod file_descriptor_test;

#[ cfg( feature = "filesystem" ) ]
mod filesystem_test;

#[ cfg( feature = "template" ) ]
mod template_test;

#[ cfg( feature = "template" ) ]
mod template_error_test;

#[ cfg( feature = "archive" ) ]
mod integration_test;

// mod builder_test; // Disabled: FR21 deferred until Former UX improves

#[ cfg( feature = "archive" ) ]
mod archive_test;

#[ cfg( feature = "external_content" ) ]
mod content_source_test;

#[ cfg( feature = "external_content" ) ]
mod content_source_example;

#[ cfg( feature = "archive" ) ]
mod archive_advanced_test;

#[ cfg( feature = "archive" ) ]
mod workflow_example;
