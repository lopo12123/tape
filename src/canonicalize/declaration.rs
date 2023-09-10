use serde::{Deserialize, Serialize};

// ========== ========== Keyboard/Mouse definitions

#[derive(Debug, Serialize, Deserialize)]
pub enum CanonicalKey {
    // Function keys  -- 16
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrtSc,
    ScrLk,
    Pause,
    // NumPad keys -- 17
    NumPad0,
    NumPad1,
    NumPad2,
    NumPad3,
    NumPad4,
    NumPad5,
    NumPad6,
    NumPad7,
    NumPad8,
    NumPad9,
    NumLock,
    NumPadDivide,
    NumPadMultiply,
    NumPadMinus,
    NumPadPlus,
    NumPadSeparator,
    NumPadDecimal,
    // Navigation & Arrow keys -- 10
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    // Modifier keys -- 13
    Tab,
    CapsLock,
    LShift,
    RShift,
    LCtrl,
    RCtrl,
    /// note: 'alt' on windows and linux, 'option' on mac
    LAlt,
    /// note: 'alt' on windows and linux, 'option' on mac
    RAlt,
    /// note: 'command' on mac, 'windows' on windows, 'super' on linux
    LMeta,
    /// note: 'command' on mac, 'windows' on windows, 'super' on linux
    RMeta,
    /// note: 'menu' on windows
    Menu,
    Enter,
    Backspace,
    // Printable keys -- 48
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    BackQuote,
    Minus,
    Equal,
    LBracket,
    RBracket,
    BackSlash,
    Semicolon,
    Quote,
    Comma,
    Period,
    Slash,
    Space,
    // Unknown keys -- 1
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CanonicalButton {
    // Mouse buttons -- 5
    Left,
    Right,
    Middle,
    Back,
    Forward,
    // Unknown buttons -- 1
    Unknown,
}

// ========== ========== Action definitions

#[derive(Debug, Serialize, Deserialize)]
pub enum KeyboardAction {
    Press,
    Release,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MouseAction {
    Press,
    Release,
    Move,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CanonicalAction {
    /// A keyboard action -- (type, key)
    Keyboard(KeyboardAction, CanonicalKey),
    /// A mouse action -- (type, button, (x, y))
    Mouse(MouseAction, CanonicalButton, (i32, i32)),
}