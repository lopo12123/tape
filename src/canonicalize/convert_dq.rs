use device_query::{Keycode};
use crate::canonicalize::declare::CanonicalKey;

// TODO: pr for missing keys in device_query
impl From<Keycode> for CanonicalKey {
    fn from(value: Keycode) -> Self {
        match value {
            // Function keys  -- 16
            Keycode::Escape => CanonicalKey::Escape,
            Keycode::F1 => CanonicalKey::F1,
            Keycode::F2 => CanonicalKey::F2,
            Keycode::F3 => CanonicalKey::F3,
            Keycode::F4 => CanonicalKey::F4,
            Keycode::F5 => CanonicalKey::F5,
            Keycode::F6 => CanonicalKey::F6,
            Keycode::F7 => CanonicalKey::F7,
            Keycode::F8 => CanonicalKey::F8,
            Keycode::F9 => CanonicalKey::F9,
            Keycode::F10 => CanonicalKey::F10,
            Keycode::F11 => CanonicalKey::F11,
            Keycode::F12 => CanonicalKey::F12,
            // Keycode::PrtSc => CanonicalKey::PrtSc,
            // Keycode::ScrLk => CanonicalKey::ScrLk,
            // Keycode::Pause => CanonicalKey::Pause,
            // NumPad keys -- 17
            Keycode::Numpad0 => CanonicalKey::NumPad0,
            Keycode::Numpad1 => CanonicalKey::NumPad1,
            Keycode::Numpad2 => CanonicalKey::NumPad2,
            Keycode::Numpad3 => CanonicalKey::NumPad3,
            Keycode::Numpad4 => CanonicalKey::NumPad4,
            Keycode::Numpad5 => CanonicalKey::NumPad5,
            Keycode::Numpad6 => CanonicalKey::NumPad6,
            Keycode::Numpad7 => CanonicalKey::NumPad7,
            Keycode::Numpad8 => CanonicalKey::NumPad8,
            Keycode::Numpad9 => CanonicalKey::NumPad9,
            // Keycode::Numlock => CanonicalKey::NumLock,
            Keycode::NumpadDivide => CanonicalKey::NumPadDivide,
            Keycode::NumpadMultiply => CanonicalKey::NumPadMultiply,
            Keycode::NumpadSubtract => CanonicalKey::NumPadMinus,
            Keycode::NumpadAdd => CanonicalKey::NumPadPlus,
            // Keycode::NumpadSeparator => CanonicalKey::NumPadSeparator,
            // Keycode::Decimal => CanonicalKey::NumPadDecimal,
            // Navigation & Arrow keys -- 10
            Keycode::Insert => CanonicalKey::Insert,
            Keycode::Delete => CanonicalKey::Delete,
            Keycode::Home => CanonicalKey::Home,
            Keycode::End => CanonicalKey::End,
            Keycode::PageUp => CanonicalKey::PageUp,
            Keycode::PageDown => CanonicalKey::PageDown,
            Keycode::Up => CanonicalKey::ArrowUp,
            Keycode::Down => CanonicalKey::ArrowDown,
            Keycode::Left => CanonicalKey::ArrowLeft,
            Keycode::Right => CanonicalKey::ArrowRight,
            // Modifier keys -- 13
            Keycode::Tab => CanonicalKey::Tab,
            Keycode::CapsLock => CanonicalKey::CapsLock,
            Keycode::LShift => CanonicalKey::LShift,
            Keycode::RShift => CanonicalKey::RShift,
            Keycode::LControl => CanonicalKey::LCtrl,
            Keycode::RControl => CanonicalKey::RCtrl,
            Keycode::LAlt => CanonicalKey::LAlt,
            Keycode::RAlt => CanonicalKey::RAlt,
            Keycode::Meta => CanonicalKey::LMeta,
            // Keycode::LMeta => CanonicalKey::RMeta,
            // Keycode::RMeta => CanonicalKey::RMeta,
            // Keycode::Menu => CanonicalKey::Menu,
            Keycode::Enter => CanonicalKey::Enter,
            Keycode::Backspace => CanonicalKey::Backspace,
            // Printable keys -- 48
            Keycode::A => CanonicalKey::KeyA,
            Keycode::B => CanonicalKey::KeyB,
            Keycode::C => CanonicalKey::KeyC,
            Keycode::D => CanonicalKey::KeyD,
            Keycode::E => CanonicalKey::KeyE,
            Keycode::F => CanonicalKey::KeyF,
            Keycode::G => CanonicalKey::KeyG,
            Keycode::H => CanonicalKey::KeyH,
            Keycode::I => CanonicalKey::KeyI,
            Keycode::J => CanonicalKey::KeyJ,
            Keycode::K => CanonicalKey::KeyK,
            Keycode::L => CanonicalKey::KeyL,
            Keycode::M => CanonicalKey::KeyM,
            Keycode::N => CanonicalKey::KeyN,
            Keycode::O => CanonicalKey::KeyO,
            Keycode::P => CanonicalKey::KeyP,
            Keycode::Q => CanonicalKey::KeyQ,
            Keycode::R => CanonicalKey::KeyR,
            Keycode::S => CanonicalKey::KeyS,
            Keycode::T => CanonicalKey::KeyT,
            Keycode::U => CanonicalKey::KeyU,
            Keycode::V => CanonicalKey::KeyV,
            Keycode::W => CanonicalKey::KeyW,
            Keycode::X => CanonicalKey::KeyX,
            Keycode::Y => CanonicalKey::KeyY,
            Keycode::Z => CanonicalKey::KeyZ,
            Keycode::Key0 => CanonicalKey::Num0,
            Keycode::Key1 => CanonicalKey::Num1,
            Keycode::Key2 => CanonicalKey::Num2,
            Keycode::Key3 => CanonicalKey::Num3,
            Keycode::Key4 => CanonicalKey::Num4,
            Keycode::Key5 => CanonicalKey::Num5,
            Keycode::Key6 => CanonicalKey::Num6,
            Keycode::Key7 => CanonicalKey::Num7,
            Keycode::Key8 => CanonicalKey::Num8,
            Keycode::Key9 => CanonicalKey::Num9,
            Keycode::Grave => CanonicalKey::BackQuote,
            Keycode::Minus => CanonicalKey::Minus,
            Keycode::Equal => CanonicalKey::Equal,
            Keycode::LeftBracket => CanonicalKey::LBracket,
            Keycode::RightBracket => CanonicalKey::RBracket,
            Keycode::BackSlash => CanonicalKey::BackSlash,
            Keycode::Semicolon => CanonicalKey::Semicolon,
            Keycode::Apostrophe => CanonicalKey::Quote,
            Keycode::Comma => CanonicalKey::Comma,
            Keycode::Dot => CanonicalKey::Period,
            Keycode::Slash => CanonicalKey::Slash,
            Keycode::Space => CanonicalKey::Space,
        }
    }
}

impl TryFrom<CanonicalKey> for Keycode {
    type Error = &'static str;

    fn try_from(value: CanonicalKey) -> Result<Self, Self::Error> {
        todo!()
    }
}