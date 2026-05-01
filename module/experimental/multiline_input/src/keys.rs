//! Key event parsing and handling

use crossterm::event::{ KeyCode, KeyModifiers, KeyEvent };
use crate::buffer::TextBuffer;

/// Action to take based on key event
#[derive( Debug, PartialEq, Eq )]
pub enum KeyAction
{
  /// Submit input and return text
  Submit,

  /// Cancel input and return None
  Cancel,

  /// Continue editing (no special action)
  Continue,
}

/// Handle key event and update buffer
///
/// Returns KeyAction to indicate what should happen next
pub fn handle_key( key: KeyEvent, buffer: &mut TextBuffer ) -> KeyAction
{
  match ( key.code, key.modifiers )
  {
    // ENTER → Submit
    ( KeyCode::Enter, KeyModifiers::NONE ) => KeyAction::Submit,

    // CTRL+ENTER or SHIFT+ENTER → Insert newline
    ( KeyCode::Enter, mods ) if mods.contains( KeyModifiers::CONTROL ) || mods.contains( KeyModifiers::SHIFT ) =>
    {
      buffer.insert_newline();
      KeyAction::Continue
    }

    // CTRL+D → Submit (alternative)
    ( KeyCode::Char( 'd' ), mods ) if mods.contains( KeyModifiers::CONTROL ) =>
    {
      KeyAction::Submit
    }

    // ESC → Cancel
    ( KeyCode::Esc, _ ) => KeyAction::Cancel,

    // CTRL+C → Cancel
    ( KeyCode::Char( 'c' ), mods ) if mods.contains( KeyModifiers::CONTROL ) =>
    {
      KeyAction::Cancel
    }

    // Backspace → Delete before cursor
    ( KeyCode::Backspace, _ ) =>
    {
      buffer.delete_char_before();
      KeyAction::Continue
    }

    // Delete → Delete at cursor
    ( KeyCode::Delete, _ ) =>
    {
      buffer.delete_char_at();
      KeyAction::Continue
    }

    // Arrow Left → Move left
    ( KeyCode::Left, _ ) =>
    {
      buffer.move_left();
      KeyAction::Continue
    }

    // Arrow Right → Move right
    ( KeyCode::Right, _ ) =>
    {
      buffer.move_right();
      KeyAction::Continue
    }

    // Arrow Up → Move up
    ( KeyCode::Up, _ ) =>
    {
      buffer.move_up();
      KeyAction::Continue
    }

    // Arrow Down → Move down
    ( KeyCode::Down, _ ) =>
    {
      buffer.move_down();
      KeyAction::Continue
    }

    // Home → Line start
    ( KeyCode::Home, KeyModifiers::NONE ) =>
    {
      buffer.move_to_line_start();
      KeyAction::Continue
    }

    // End → Line end
    ( KeyCode::End, KeyModifiers::NONE ) =>
    {
      buffer.move_to_line_end();
      KeyAction::Continue
    }

    // CTRL+Home → Text start
    ( KeyCode::Home, mods ) if mods.contains( KeyModifiers::CONTROL ) =>
    {
      buffer.move_to_text_start();
      KeyAction::Continue
    }

    // CTRL+End → Text end
    ( KeyCode::End, mods ) if mods.contains( KeyModifiers::CONTROL ) =>
    {
      buffer.move_to_text_end();
      KeyAction::Continue
    }

    // Regular character → Insert
    ( KeyCode::Char( ch ), mods )
      if !mods.contains( KeyModifiers::CONTROL ) || mods == KeyModifiers::SHIFT =>
    {
      buffer.insert_char( ch );
      KeyAction::Continue
    }

    // Tab → Insert as spaces (4 spaces)
    ( KeyCode::Tab, _ ) =>
    {
      buffer.insert_char( ' ' );
      buffer.insert_char( ' ' );
      KeyAction::Continue
    }

    // Ignore all other key combinations
    _ => KeyAction::Continue,
  }
}
