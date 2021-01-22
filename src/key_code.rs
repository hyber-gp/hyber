//! Contains the key codes for all the keyboard events.
//!

/// Key codification that is being handled
///
/// The keys are being maped according to the keyboarding configuration
/// and base on the info at https://www.w3.org/TR/uievents-code/ and at
/// https://docs.microsoft.com/en-us/windows-hardware/customize/enterprise/keyboardfilter-key-names
/// 
/// _**Note:** Since the keys are being mapped according to the keyboard
/// configuration, there may be mapping problems. This solution was tested
/// with the default Portuguese keyboard settings
#[derive(Debug, Copy, Clone)]
pub enum KeyCode {
    /// The number 1 key.
    Key1,
    /// The number 2 key.
    Key2,
    /// The number 3 key.
    Key3,
    /// The number 4 key.
    Key4,
    /// The number 5 key.
    Key5,
    /// The number 6 key.
    Key6,
    /// The number 7 key.
    Key7,
    /// The number 8 key.
    Key8,
    /// The number 9 key.
    Key9,
    /// The number 0 key.
    Key0,
    /// The letter A key. 
    A,
    /// The letter B key. 
    B,
    /// The letter C key. 
    C,
    /// The letter D key. 
    D,
    /// The letter E key. 
    E,
    /// The letter F key. 
    F,
    /// The letter G key. 
    G,
    /// The letter H key. 
    H,
    /// The letter I key. 
    I,
    /// The letter J key. 
    J,
    /// The letter K key. 
    K,
    /// The letter L key. 
    L,
    /// The letter M key. 
    M,
    /// The letter N key. 
    N,
    /// The letter O key. 
    O,
    /// The letter P key. 
    P,
    /// The letter Q key. 
    Q,
    /// The letter R key. 
    R,
    /// The letter S key. 
    S,
    /// The letter T key. 
    T,
    /// The letter U key. 
    U,
    /// The letter V key. 
    V,
    /// The letter W key. 
    W,
    /// The letter X key. 
    X,
    /// The letter Y key. 
    Y,
    /// The letter Z key. 
    Z,
    /// The Escape key.
    Escape,
    /// The F1 key.     
    F1,
    /// The F2 key.
    F2,
    /// The F3 key.
    F3,
    /// The F4 key.
    F4,
    /// The F5 key.
    F5,
    /// The F6 key.
    F6,
    /// The F7 key.
    F7,
    /// The F8 key.
    F8,
    /// The F9 key.
    F9,
    /// The F10 key.
    F10,
    /// The F11 key.
    F11,
    /// The F12 key.
    F12,
    /// The F13 key.
    F13,
    /// The F14 key.
    F14,
    /// The F15 key.
    F15,
    /// The F16 key.
    F16,
    /// The F17 key.
    F17,
    /// The F18 key.
    F18,
    /// The F19 key.
    F19,
    /// The F20 key.
    F20,
    /// The F21 key.
    F21,
    /// The F22 key.
    F22,
    /// The F23 key.
    F23,
    /// The F24 key.
    F24,
    /// The PrtScr key, normally used to take screenshots.
    Snapshot,
    /// The Scroll Lock key.
    Scroll,
    /// The Pause/Break key.
    Pause,
    /// The Insert key.
    Insert,
    /// The Home key.
    Home,
    /// The Delete key.
    Delete,
    /// The End key.
    End,
    /// The Page Up key.
    PageDown,
    /// The Page Down key.
    PageUp,
    /// The Left arrow key.
    Left,
    /// The Up arrow key.
    Up,
    /// The Right arrow key.
    Right,
    /// The Down arrow key.
    Down,
    /// The Backspace key.
    Backspace,
    /// The Enter key.
    Enter,
    /// The Space bar key.
    Space,
    /// The Compose Character key on Linux.
    Compose,
    /// The caret (^) key (i.e., circumflex accent).
    Caret,
    /// The Num Lock key on numeric keypad.
    Numlock,
    /// The 0 key on numeric keypad.
    Numpad0,
    /// The 1 key on numeric keypad.
    Numpad1,
    /// The 2 key on numeric keypad.
    Numpad2,
    /// The 3 key on numeric keypad.
    Numpad3,
    /// The 4 key on numeric keypad.
    Numpad4,
    /// The 5 key on numeric keypad.
    Numpad5,
    /// The 6 key on numeric keypad.
    Numpad6,
    /// The 7 key on numeric keypad.
    Numpad7,
    /// The 8 key on numeric keypad.
    Numpad8,
    /// The 9 key on numeric keypad.
    Numpad9,
    /// The add (+) key, on numeric keypad.
    NumpadAdd,
    /// The divide (/) key, on numeric keypad.
    NumpadDivide,
    /// The . Del key, on numeric keypad.
    ///
    /// _**Note:** For locales where the decimal separator
    ///  is "," (e.g., Brazil), this key may generate a ,
    NumpadDecimal,
    /// The , key, on numeric keypad.
    ///
    /// _**Note:** For locales where the thousands separator
    /// is a "." (e.g., Brazil), this key may generate a .
    NumpadComma,
    /// The Enter key, on numeric keypad.
    NumpadEnter,
    /// The equal (=) key, on numeric keypad.
    NumpadEquals,
    /// The multiplication (*) key, on numeric keypad.
    NumpadMultiply,
    /// The subtract (-) key, on numeric keypad.
    NumpadSubtract,
    /// The Alt Gr left key.
    AbntC1,
    /// The Alt Gr right key.
    AbntC2,
    /// The apostrophe (') key.
    Apostrophe,
    /// The Apps key, usually on Windows keyboards.
    Apps,
    /// The asterisk (*) key.
    Asterisk,
    /// The at sign (@) key.
    At,
    /// Unknown key.
    Ax,
    /// The \, or backslash key.
    Backslash,
    /// The calculator shortcut key.
    Calculator,
    /// The capital key are enabled.
    Capital,
    /// The : key.
    Colon,
    /// The , key.
    Comma,
    /// Unknown key.
    Convert,
    /// The = key.
    Equals,
    /// The grave accent (`) key.
    Grave,
    /// The kana key (i.e., a key for japanese keyboard settings).
    Kana,
    /// The kanji key (i.e., a key for japanese keyboard settings).
    Kanji,
    /// The left Alt key.
    LAlt,
    /// The left ] key.
    LBracket,
    /// The left Ctrl key.
    LControl,
    /// The left shift key.
    LShift,
    /// The left Windows key, on Windows keyboards.
    LWin,
    /// The mail shortcut key.
    Mail,
    /// The media select shortcut key.
    MediaSelect,
    /// The media stop shortcut key.
    MediaStop,
    /// The - key.
    Minus,
    /// The mute key.
    Mute,
    /// The my computer shortcut key.
    MyComputer,
    /// The navigate forward, or next, key.
    NavigateForward,
    /// The navigate backwards, or prior, key.
    NavigateBackward,
    /// The next media shortcut key.
    NextTrack,
    /// Unknown key.
    NoConvert,
    /// Either the angle bracket key or the backslash 
    /// key on the RT 102-key keyboard
    OEM102,
    /// The period (.) key.
    Period,
    /// The play/pause shortcut key.
    PlayPause,
    /// The plus sign (+) key.
    Plus,
    /// The power key.
    Power,
    /// The previous track shortcut key.
    PrevTrack,
    /// The right Alt key.
    RAlt,
    /// The right bracket (]) key.
    RBracket,
    /// The right Ctrl key.
    RControl,
    /// The right shift key.
    RShift,
    /// The right Windows key, on Windows keyboards.
    RWin,
    /// The semicolon (;) key.
    Semicolon,
    /// The slash (/) key.
    Slash,
    /// The sleep shortcut key.
    Sleep,
    /// The stop key.
    Stop,
    /// The SysRq key, normally used to take screenshots.
    Sysrq,
    /// The Tab key.
    Tab,
    /// The Underline shortcut key.
    Underline,
    /// Unkown key.
    Unlabeled,
    /// The volume down key.
    VolumeDown,
    /// The volume down key.
    VolumeUp,
    /// The wake shortcut key.
    Wake,
    /// The shortcut key to get back to the previous web page, on browser.
    WebBack,
    /// The shortcut key to move to the favorites web page, on browser.
    WebFavorites,
    /// The shortcut key to move forward to the next web page, on browser.
    WebForward,
    /// The shortcut key to get back to the home web page, on browser.
    WebHome,
    /// The shortcut key to refresh the web page, on browser.
    WebRefresh,
    /// The shortcut key to search on that web page, on browser.
    WebSearch,
    /// The shortcut key to stop loading the web page, on browser.
    WebStop,
    /// The yen key (i.e., a key for japanese keyboard settings).
    Yen,
    /// The copy shortcut key.
    Copy,
    /// The paste shortcut key.
    Paste,
    /// The cut shortcut key.
    Cut,
}