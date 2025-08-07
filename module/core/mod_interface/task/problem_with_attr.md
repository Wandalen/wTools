# Fix issue with outer attribute

● Update(src/lib.rs)
  ⎿  Updated src/lib.rs with 1 removal
       33
       34    crate::mod_interface!
       35    {
       36 -    #![ doc = "Public module interface exposing all API functionality." ]
       36
       37      layer client;
       38      layer environment;


● Bash(cargo clippy -p api_openai -- -D warnings)
  ⎿  Error: Checking api_openai v0.2.0 (/home/user1/pro/lib/llm_tools/module/api_openai)
     error: Unknown inner attribute:
            attr : # ! [doc = "Public module interface exposing all API functionality."] :
            Attribute {
                pound_token: Pound,
                style: AttrStyle::Inner(
                    Not,
                ),
                bracket_token: Bracket,
                meta: Meta::NameValue {
