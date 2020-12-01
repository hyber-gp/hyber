mod key_code;
pub use key_code::KeyCode;

mod queue;
pub use queue::Queue;

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
    /// # Arguments
    /// 
    /// # Examples
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
    /// # Arguments
    /// * `title` - Title of the display
    /// * `width` - Width of the display
    /// * `height` - Height of the display
    /// * `display_descriptor` - Holds a reference to a DisplayDescriptor, which contains optional stylings for the display
    /// 
    /// # Examples
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
    /// # Arguments
    /// 
    /// * `title` - Title of the display
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn new(title: &str, width: u32, height: u32, display_descriptor: DisplayDescritor) -> Self {
    ///         ...
    ///     }
    /// 
    ///     fn set_title(&mut self, title: &str) {
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
    
    /// Returns the pixel buffer associated to the display
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn new(title: &str, width: u32, height: u32, display_descriptor: DisplayDescritor) -> Self {
    ///         ...
    ///     }
    /// 
    ///     fn get_buffer(&mut self) {
    ///         &mut self::Buffer
    ///     }
    /// }
    /// ```
    /// 
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    /// 
    /// let &mut buffer = display.get_buffer();
    /// ```
    fn get_buffer(&mut self) -> &mut Self::Buffer;

    /// Updates the display
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// Updates the display after setting a new title for it
    /// 
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn new(title: &str, width: u32, height: u32, display_descriptor: DisplayDescritor) -> Self {
    ///         ...
    ///     }
    /// 
    ///     fn set_title(&mut self, title: &str) {
    ///         ...
    ///     }
    /// 
    ///     fn update(&mut self) {
    ///         ...
    ///     }
    /// }
    /// ```
    /// 
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    /// 
    /// display.set_title("Other Example");
    /// 
    /// display.update();
    /// ```
    fn update(&mut self);

    /// Updates the display along with its buffer
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// 
    fn update_with_buffer(&mut self);

    /// Check if the display is open. The user may want to take some action depending on this state.
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn new(title: &str, width: u32, height: u32, display_descriptor: DisplayDescritor) -> Self {
    ///         ...
    ///     }
    /// 
    ///     fn is_open(&self) -> bool {
    ///         ...
    ///     }
    /// }
    /// ```
    /// 
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    /// 
    /// let display_opened = display.is_open();
    /// ```
    fn is_open(&self) -> bool;

    /// Sets the position of the window, relative to the topleft corner of the display.
    /// 
    /// # Arguments
    /// * `x` - x-coordinate for the topleft corner
    /// * `y` - y-coordinate for the topleft corner
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn new(title: &str, width: u32, height: u32, display_descriptor: DisplayDescritor) -> Self {
    ///         ...
    ///     }
    /// 
    ///     fn set_position(&mut self, x: u32, y: u32) {
    ///         ...
    ///     }
    /// }
    /// ```
    /// 
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    /// 
    /// display.set_position(0, 0);
    /// ```
    fn set_position(&mut self, x: u32, y: u32);

    /// Adds a border to the display
    /// 
    /// # Arguments
    /// * `border` - boolean indicating if display has a border
    /// 
    /// # Examples
    /// 
    /// Make the display borderless
    /// 
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn new(title: &str, width: u32, height: u32, display_descriptor: DisplayDescritor) -> Self {
    ///         ...
    ///     }
    /// 
    ///     fn border(&mut self, border: bool) {
    ///         ...
    ///     }
    /// }
    /// ```
    /// 
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    /// 
    /// display.border(false);
    /// ```
    fn border(&mut self, border: bool);

    /// Makes the display resizable
    /// 
    /// # Arguments
    /// * `resizable` - boolean indicating if display is resizable
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn new(title: &str, width: u32, height: u32, display_descriptor: DisplayDescritor) -> Self {
    ///         ...
    ///     }
    /// 
    ///     fn resizable(&mut self, resizable: bool) {
    ///         ...
    ///     }
    /// }
    /// ```
    /// 
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    /// 
    /// display.resizable(true);
    /// ```
    fn resizable(&mut self, resizable: bool);

    /// Makes the display always appear on top of those that are not topmost
    /// 
    /// # Arguments
    /// * `topmost` - boolean indicating if display is topmost
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn new(title: &str, width: u32, height: u32, display_descriptor: DisplayDescritor) -> Self {
    ///         ...
    ///     }
    /// 
    ///     fn topmost(&mut self, topmost: bool) {
    ///         ...
    ///     }
    /// }
    /// ```
    /// 
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    /// 
    /// display.topmost(true);
    /// ```
    fn topmost(&mut self, topmost: bool);

    /// Makes the display minimizable
    /// 
    /// # Arguments
    /// * `minimizable` - boolean indicating if display is minimizable
    /// 
    /// # Examples
    /// 
    /// Makes the display non-minimizable
    /// 
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn new(title: &str, width: u32, height: u32, display_descriptor: DisplayDescritor) -> Self {
    ///         ...
    ///     }
    /// 
    ///     fn minimizable(&mut self, minimizable: bool) {
    ///         ...
    ///     }
    /// }
    /// ```
    /// 
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    /// 
    /// display.minimizable(false);
    /// ```
    fn minimizable(&mut self, minimizable: bool);

    /// Sets background color that is updated in the buffer.
    /// 
    /// # Arguments
    /// * `red` - value for red color
    /// * `green` - value value for green color
    /// * `blue` - value for blue color
    /// 
    /// # Examples
    /// 
    /// Set background color to yellow
    /// 
    /// ```no_run
    /// impl Display for Implementor {
    ///     fn new(title: &str, width: u32, height: u32, display_descriptor: DisplayDescritor) -> Self {
    ///         ...
    ///     }
    /// 
    ///     fn set_background_color(&mut self, red: u32, green: u32, blue: u32) {
    ///         ...
    ///     }
    /// }
    /// ```
    /// 
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    /// 
    /// display.set_background_color(255, 255, 0);
    /// ```
    fn set_background_color(&mut self, red: u32, green: u32, blue: u32);

    /// Returns the current size of the display
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ``no_run
    /// impl Display for Implementor {
    ///     fn new(title: &str, width: u32, height: u32, display_descriptor: DisplayDescritor) -> Self {
    ///         ...
    ///     }
    /// 
    ///     fn get_size(&self) -> (u32, u32) {
    ///         ...
    ///     }
    /// }
    /// ```
    /// 
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    /// 
    /// let size = display.get_size();
    /// ```
    fn get_size(&self) -> (u32, u32);

    /// Checks if the display is the current active one
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ``no_run
    /// impl Display for Implementor {
    ///     fn new(title: &str, width: u32, height: u32, display_descriptor: DisplayDescritor) -> Self {
    ///         ...
    ///     }
    /// 
    ///     fn is_active(&self) -> bool {
    ///         ...
    ///     }
    /// }
    /// ```
    /// 
    /// ```no_run
    /// let mut display = Implementor::new("Example", 640, 400, DisplayDescriptor::default());
    /// 
    /// let display_active = display.is_active();
    /// ```
    fn is_active(&self) -> bool;

    // Tudo menos Cursor, Unscaled, Menus e limit_update_rate
}

struct BoxLayout {
    // min_x: unimplemented!(),
    // max_x: unimplemented!(),
    // min_y: unimplemented!(),
    // max_y: unimplemented!()
}

struct SliverLayout {
}

pub trait Widget<Message> {
    /// This function is needed to detect if the event is being done on this widget, update the state of 
    /// the widget based on event and place a message in the message queue.
    /// 
    /// # Returns 
    /// An hyber Event
    ///
    /// # Arguments
    /// * `event` - an hyber event
    /// * `messages` - queue of messages 
    fn on_event(event: Event, messages: &Queue<Message>);
}

pub trait Renderer<T,X>{

    type Message;
    
    /// This function is needed to map the events detected (Window, Keyboard, Mouse) into hyber events.
    /// We recommend user to define T as an enum.
    /// 
    /// # Returns 
    /// An hyber Event
    ///
    /// # Arguments
    /// `event` - a generic event 
    ///  
    /// # Examples
    /// fn map_events<T>(event: T) -> Event {
    ///     ...
    ///     match event {
    ///         leftclick => {
    ///             Mouse(Mouse::ButtonPressed(MouseButton::Left))
    ///         }
    ///         ...
    ///     }
    /// }
    fn map_events(event: X) -> Event;
    
    ///This function creates a queue of events
    /// 
    /// # Returns 
    /// An empty vector for events
    /// 
    /// # Arguments
    /// No args
    fn create_events_queue(&mut self) -> Queue<Event> {
        let queue: Queue<Event> = Queue::<Event>::new();
        queue
    }
    
    /// This function creates a queue of messages.
    /// Message should be an enum.
    /// # Returns 
    /// An empty vector for messages
    /// 
    /// # Arguments
    /// No args
    fn create_message_queue(&mut self) -> Queue<Self::Message> {
        let queue: Queue<Self::Message> = Queue::new();
        queue
    }
    
    /// This function is used to detect the system events and map them into hyber events using map_events function.
    /// The user should implement this function and put the events on the queue events, using events.enqueue .
    /// 
    /// # Returns 
    /// No returns
    ///
    /// # Arguments
    /// * `events` - queue of events
    /// * `system` - a generic type to access system events eg. in minifb crate its accessed via window 
    //fn detect_sys_events(events: &Queue<Event>);
    fn detect_sys_events(events: &mut Queue<Event>, system: &mut T);


    /// This function has the event loop of hyber. It can be described in 4 steps:
    /// * 1st - To recall the system events.
    /// * 2nd - Call the on_event in our widget tree, regarding the queue of events.
    /// * 3rd - Draw.
    /// * 4th - Iterate over message queue and update the state.
    /// 
    /// # Returns 
    /// No returns
    ///
    /// # Arguments
    /// * `events` - queue of events
    /// * `messages` - queue of messages
    /// * `system` - a generic type to access system events eg. in minifb crate its accessed via window
    
    fn event_loop(&mut self, mut events: Queue<Event>,mut messages: Queue<Self::Message>, system: &mut T) {
        loop{
            // 1º RECOLHER -> MAPEAR -> METER NA QUEUE
            Self::detect_sys_events(&mut events, system);
            if events.lenght() != 0{
                let _event = events.dequeue();
                println!("novo evento");
            }
            // 2º chamar on event na arvore de widgets
            // 3º desenhar
            // 4º percorrer as mensagens e fazer update
            /*for _message in messages.queue.drain(..){

            }*/
            
        }
    }/*
    fn event_loop(&mut self, mut events: Queue<Event>,mut messages: Queue<Self::Message>) {
        loop{
            // 1º RECOLHER -> MAPEAR -> METER NA QUEUE
            Self::detect_sys_events(&events);
            if events.lenght() != 0{
                let _event = events.dequeue();
                println!("novo evento");
            }
            // 2º chamar on event na arvore de widgets
            // 3º desenhar
            // 4º percorrer as mensagens e fazer update
            for _message in messages.queue.drain(..){

            }
            
        }
    }*/
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        
    }
}


