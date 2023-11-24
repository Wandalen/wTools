mod private
{
    use error_tools::for_app::Result;

    /// generate workflow
    pub fn workflow_generate() -> Result< () >
    {
        todo!()
    }
}

crate::mod_interface!
{
    prelude use workflow_generate;
}