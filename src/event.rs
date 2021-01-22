use crate::key_code::KeyCode;

/// The current state of the keyboard modifiers
#[derive(Debug, Copy, Clone)]
pub struct ModifiersState {
    /// Whether a shift key is pressed 
    ///
    /// [default: false]
    pub shift: bool,

    /// Whether a control key is pressed
    ///
    /// [default: false]
    pub control: bool,

    /// Whether an alt key is pressed
    ///
    /// [default: false]
    pub alt: bool,

    /// Whether a logo key is pressed (e.g. windows key, command key...)
    ///
    /// [default: false]
    pub logo: bool,
}

impl ModifiersState {
    /// Compares two [`ModifiersState`] to see if they match
    ///
    /// # Returns
    /// True, if the current [`ModifiersState`] has at least the same
    /// modifiers enabled as the given value, and false otherwise
    ///
    /// # Arguments
    /// * `modifiers` - the [`ModifiersState`] to be compared
    pub fn matches(&self, modifiers: ModifiersState) -> bool {
        let shift = !modifiers.shift || self.shift;
        let control = !modifiers.control || self.control;
        let alt = !modifiers.alt || self.alt;
        let logo = !modifiers.logo || self.logo;

        shift && control && alt && logo
    }
}

/// A keyboard event
#[derive(Debug, Copy, Clone)]
pub enum Keyboard {
    /// A keyboard key was pressed
    KeyPressed {
        /// The key identifier
        key_code: KeyCode,

        /// The state of the modifiers keys
        modifiers: ModifiersState,
    },

    /// A keyboard key was released
    KeyReleased {
        /// The key identifier
        key_code: KeyCode,

        /// The state of the modifiers keys
        modifiers: ModifiersState,
    },

    /// The keyboard modifiers have changed
    ModifiersChanged(ModifiersState),
}

/// A mouse event
#[derive(Debug, Copy, Clone)]
pub enum Mouse {
    /// A mouse button was pressed
    ButtonPressed(MouseButton),

    /// A mouse button was released
    ButtonReleased(MouseButton),

    /// The mouse cursor entered the window
    CursorEntered,

    /// The mouse cursor left the window
    CursorLeft,

    /// The mouse cursor moved
    CursorMoved {
        /// The X coordinate of the mouse position
        x: usize,

        /// The Y coordinate of the mouse position
        y: usize,
    },

    /// The mouse wheel was scrolled
    WheelScrolled {
        /// The scroll movement
        delta: ScrollDelta,
    },
}

/// A mouse button
#[derive(Debug, Copy, Clone)]
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

/// A scroll event
///
/// The number of units moved when the user scrolls
#[derive(Debug, Copy, Clone)]
pub enum ScrollDelta {
    /// A pixel-based scroll movement
    Pixels {
        /// The number of horizontal pixels scrolled
        x: f64,

        /// The number of vertical pixels scrolled
        y: f64,
    },
}

/// A window event
#[derive(Debug, Copy, Clone)]
pub enum Window {
    /// The window was rezised
    Resized {
        /// The new width of the window
        width: u32,

        /// The new height of the window
        height: u32,
    },
}

/// An user interface event
#[derive(Debug, Copy, Clone)]
pub enum Event {
    /// A keyboard event (eg. KeyPressed, KeyRelease...)
    Keyboard(Keyboard),

    ///A mouse event (eg. LeftClick, MouseMove,...)
    Mouse(Mouse),

    ///A windown event (eg. Resize, ...)
    Window(Window),
}
