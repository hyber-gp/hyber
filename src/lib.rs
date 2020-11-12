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

///A keyboard event
pub enum Keyboard{
    ///A keyboard key was pressed 
    KeyPressed {
        ///The key identifier
        key_code: i16,

        ///The state of the modifiers keys
        modifiers: ModifiersState,
    },
    ///A keyboard key was released
    KeyReleased {
        ///The key identifier
        key_code: i16,
        ///The state of the modifiers keys
        modifiers: ModifiersState,
    },
    ///The keyboard modifiers have changed
    ModifiersChanged(ModifiersState),

}

///A mouse event
pub enum Mouse{
    ///A mouse button was pressed
    ButtonPressed(MouseButton),
    
    ///A mouse button was released
    ButtonReleased(MouseButton),

    ///The mouse cursor entered the window
    CursorEntered,

    ///The mouse cursor left the window
    CursorLeft,

    ///The mouse cursor moved
    CursorMoved{
        ///The X coordinate of the mouse position
        x: f32,

        ///The Y coordinate of the mouse position
        y: f32
    },

    ///The mouse wheel was scrolled
    WheelScrolled{
        ///The scroll movement
        delta: ScrollDelta,
    },
}

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

pub enum ScrollDelta {
    /// A pixel-based scroll movement
    Pixels {
        /// The number of horizontal pixels scrolled
        x: f32,

        /// The number of vertical pixels scrolled
        y: f32,
    },
}

///A window event
pub enum Window{
    ///The window was rezised
    Resized{
        ///The new width of the window
        width: u32,

        ///The new height of the window
        height: u32
    }
}

///Representation of an user interface event
pub enum Event {
    /// A keyboard event (eg. KeyPressed, KeyRelease...)
    Keyboard(Keyboard),

    ///A mouse event (eg. LeftClick, MouseMove,...)
    Mouse(Mouse),

    ///A windown event (eg. Resize, ...)
    Window(Window)
}


impl Event {
    // TODO: funcoes
    fn on_event(event: Event) { 
        unimplemented!()
    }
    
}

/// Structure that contains optional settings for a display window.
/// It should be used when creating a new Display.
/// It also provides default settings for a new Display.
pub struct DisplayDescritor {
    /// Indicates if display has a border (default: true)
    border: bool,
    /// Indicates if display is titled (default: true)
    titled: bool,
    /// Indicates if display is resizable (default: false)
    resizable: bool,
    /// Indicates if display always appears on top of all displays which are not topmost (default: false)
    topmost: bool,
    /// Indicates if display is minimizable (default: true)
    minimizable: bool,
}

impl DisplayDescritor {
    /// Returns a DisplayDescriptor with default values, allowing to override each attribute
    ///
    /// #Arguments
    /// 
    /// #Examples
    /// 
    /// ```no_run
    /// let mut display_descriptor = DisplayDescriptor { border: false, titled: false, ..Default::default() }
    /// ```
    /// 
    /// ```no_run
    /// let mut display_descriptor = DisplayDescriptor { ..Default::default() }
    /// ```
    fn default() -> DisplayDescritor {
        DisplayDescritor {
            border: true,
            titled: true,
            resizable: false,
            topmost: false,
            minimizable: true,
        }
    }
}

/// Implements the necessary methods to a complete Display/Window system
pub trait Display {
    type Buffer;

    /// Creates and opens up a new display
    /// 
    /// #Arguments
    /// 
    /// #Examples
    /// 
    /// Create and open up a display with popup behaviour
    /// 
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn new(title: &str, width: u32, height: u32, display_descriptor: DisplayDescritor) -> Self {
    ///         ...
    ///     }
    /// }
    /// ```
    /// 
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400,
    ///     DisplayDescriptor {
    ///         topmost: true,
    ///         minimizable: false,
    ///         ..DisplayDescriptor::default()
    /// });
    /// ```
    fn new(title: &str, width: u32, height: u32, display_descriptor: DisplayDescritor) -> Self;

    /// Sets a new title for the display
    /// 
    /// #Arguments
    /// 
    /// #Examples
    /// 
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn new(title: &str, width: u32, height: u32, display_descriptor: DisplayDescritor) -> Self {
    ///         ...
    ///     }
    /// 
    ///     fn set_title(&mut self, title: &str) -> Self {
    ///         ...
    ///     }
    /// }
    /// ```
    /// 
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    /// 
    /// display.set_title("Other Example");
    /// ```
    fn set_title(&mut self, title: &str);
    
    /// 
    /// 
    fn buffer(&self) -> Self::Buffer;
}

struct BoxLayout {
    // min_x: unimplemented!(),
    // max_x: unimplemented!(),
    // min_y: unimplemented!(),
    // max_y: unimplemented!()
}

struct SliverLayout {
}

pub trait Widget {

}

pub trait Renderer {
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


