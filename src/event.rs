use crate::key_code::KeyCode;
#[derive(Debug)]
///The current state of the keyboard modifiers
pub struct ModifiersState {
    /// Whether a shift key is pressed
    pub shift: bool,

    /// Whether a control key is pressed
    pub control: bool,

    /// Whether an alt key is pressed
    pub alt: bool,

    /// Whether a logo key is pressed (e.g. windows key, command key...)
    pub logo: bool,
}

impl ModifiersState {
    /// Returns true if the current [`ModifiersState`] has at least the same
    /// modifiers enabled as the given value, and false otherwise.
    pub fn matches(&self, modifiers: ModifiersState) -> bool {
        let shift = !modifiers.shift || self.shift;
        let control = !modifiers.control || self.control;
        let alt = !modifiers.alt || self.alt;
        let logo = !modifiers.logo || self.logo;

        shift && control && alt && logo
    }
}
#[derive(Debug)]
///A keyboard event
pub enum Keyboard {
    ///A keyboard key was pressed
    KeyPressed {
        ///The key identifier
        key_code: KeyCode,

        ///The state of the modifiers keys
        modifiers: ModifiersState,
    },
    ///A keyboard key was released
    KeyReleased {
        ///The key identifier
        key_code: KeyCode,
        ///The state of the modifiers keys
        modifiers: ModifiersState,
    },
    ///The keyboard modifiers have changed
    ModifiersChanged(ModifiersState),
}
#[derive(Debug)]
///A mouse event
pub enum Mouse {
    ///A mouse button was pressed
    ButtonPressed(MouseButton),
    ///A mouse button was released
    ButtonReleased(MouseButton),

    ///The mouse cursor entered the window
    CursorEntered,

    ///The mouse cursor left the window
    CursorLeft,

    ///The mouse cursor moved
    CursorMoved {
        ///The X coordinate of the mouse position
        x: usize,

        ///The Y coordinate of the mouse position
        y: usize,
    },

    ///The mouse wheel was scrolled
    WheelScrolled {
        ///The scroll movement
        delta: ScrollDelta,
    },
}
#[derive(Debug)]
///The button of a mouse
pub enum MouseButton {
    /// The left mouse button.
    Left,

    /// The right mouse button.
    Right,

    /// The middle (wheel) button.
    Middle,

    /// Some other button.
    Other(u8),
}
#[derive(Debug)]
pub enum ScrollDelta {
    /// A pixel-based scroll movement
    Pixels {
        /// The number of horizontal pixels scrolled
        x: usize,

        /// The number of vertical pixels scrolled
        y: usize,
    },
}
#[derive(Debug)]
///A window event
pub enum Window {
    ///The window was rezised
    Resized {
        ///The new width of the window
        width: u32,

        ///The new height of the window
        height: u32,
    },
}
#[derive(Debug)]
///Representation of an user interface event
pub enum Event {
    /// A keyboard event (eg. KeyPressed, KeyRelease...)
    Keyboard(Keyboard),

    ///A mouse event (eg. LeftClick, MouseMove,...)
    Mouse(Mouse),

    ///A windown event (eg. Resize, ...)
    Window(Window),
}
