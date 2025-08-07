
mod private
{
  pub const DEFAULT_TOKEN_URI: &'static str = "https://oauth2.googleapis.com/token";
  pub const DEFAULT_AUTH_URI: &'static str = "https://accounts.google.com/o/oauth2/auth";
  pub const GOOGLE_API_URL: &'static str = "https://sheets.googleapis.com/v4/spreadsheets";
  pub const GOOGLE_SPREADSHEET_SCOPE: &'static str = "https://www.googleapis.com/auth/spreadsheets";
}

crate::mod_interface!
{
  prelude use
  {
    DEFAULT_AUTH_URI,
    DEFAULT_TOKEN_URI,
    GOOGLE_API_URL,
    GOOGLE_SPREADSHEET_SCOPE
  };
}