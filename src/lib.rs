use std::collections::BTreeMap;

/// Enumeration with the Render Instructions @joaosantos
pub enum RenderInstruction {
    /// Instruction to the Render that a point needs to be drawn on the next Clipping
    /// The point should be rendered on absolute coordinates (x,y)
	DrawPoint,
    /// Instruction to the Render that a line needs to be drawn on the next Clipping
    /// The line should be rendered on absolute coordinates from (x1, y1) to (x2, y2)
	DrawLine,
    /// Instruction to the Render that an arc needs to be drawn on the next Clipping
    /// The arc should be rendered with center on absolute coordinates (x, y), 'r'
    /// radius, 'sang' start angle and 'eang' end angle
    DrawArc,
    /// Instruction to the Render that a circle needs to be drawn on the next Clipping
    /// The circle should be rendered with center on absolute coordinates (x, y) and 'r'
    /// radius
    DrawCircle,
    /// Instruction to the Render that a rectangle needs to be drawn on the next Clipping
    /// The rectangle should be rendered on absolute coordinates (x, y) with 'l' length
    /// and 'w' width
	DrawRect,
    /// Instruction to the Render that a triangle needs to be drawn on the next Clipping
    /// The triangle should be rendered between the absolute coordinates (x1, y1),
    /// (x2, y2) and (x3, y3)
    DrawTriangle,
    /// Instruction to the Render that an image needs to be drawn on the next Clipping
    /// [Doubt] The image should be rendered with center on the absolute coordinates (x, y)
    /// and with 'w' width and 'l' length
    DrawImage,
    /// Instruction to the Render that some text needs to be drawn on the next Clipping
    /// [Doubt] The text should be rendered according to the text_alignment
    DrawText,
}
/// Assumptions:
///     - 2D Meshes are compounded by a list of triangles so the instructions are gonna be
///     multiple DrawTriangleAbs intructions
///     Reference: https://github.com/hecrj/iced/blob/master/graphics/src/triangle.rs
///     - Based on: https://en.wikipedia.org/wiki/Geometric_primitive
///     - And on:   https://www.freepascal.org/docs-html/current/rtl/graph/funcdrawing.html

// Structure of an Instruction to be on the Render Instrucstions Collection
pub struct Instruction {
    pub id: u32,
    pub instruction: RenderInstruction,
}

/// Implements the method for a new Instruction
impl Instruction {
    pub fn new(id: u32, instruction: RenderInstruction) -> Instruction {
        Instruction {
            id,
            instruction,
        }
    }
}

/// Example:
///
/// Criar:
///     (-> BTreeMap<K, V>)
///     - makes a new empty BTreeMap.
///
///     let mut map = BTreeMap::new();
///
/// Limpar:
///     - clears the map, removing all elements
///
///     map.clear();
///
/// Get Value:
///     (-> Option<&V>)
///     - returns a reference to the value corresponding to the key
///
///     map.get(&1);
///
/// Get Key-Value:
///     (-> Option<(&K, &V)>)
///     - returns the key-value pair corresponding to the supplied key
///
///     map.get_key_value(&1);
///
/// Get Mutable Value:
///     (-> Option<&mut V>)
///     - returns a mutable reference to the value corresponding to the key.
///
///     map.get_mut(&1);
///
/// Contains Key:
///     (bool)
///     - returns true if the map contains a value for the specified key.
///
///     map.contains_key(&1);
///
/// First Key-Value:
///     (-> Option<(&K, &V)>)
///     - to obtain the first key-value pair in the map
///
///     map.first_key_value();
///
/// Insertion:
///     (-> Option<V>)
///     - inserts a key-value pair into the map
///
///     map.insert(1, RenderInstruction::DrawPoint);
///
/// Remove:
///     (-> Option<V>)
///     - removes a key from the map, returning the value at the key
///     - if the key was previously in the map
///
///     map.remove(&1);
///
/// Remove Entry:
///     (-> Option<(K, V)>)
///     - removes a key from the map, returning the stored key and value
///     - if the key was previously in the map
///
///     map.remove_entry(&1);

let mut instruction = BTreeMap::new();

/// Assumptions for the map:
///  - Need to have a key-value pair of <u32, RenderInstruction>/<id, RenderInstruction>
/// Requirements:
///  - Fast iterator, due to client requirements of rendering
/// 
/// BTreeMap is the choice because of our use case:
///     - You want a map sorted by its keys.
///
/// References: 
///     - https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.remove
///     - https://doc.rust-lang.org/std/collections/index.html#use-a-btreemap-when

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

/// Structure that contains optional settings for a display window.
/// It should be used when creating a new Display.
/// It also provides default settings for a new Display.
pub struct DisplayDescritor {
    /// Indicates if display has a border (default: true)
    pub border: bool,
    /// Indicates if display is titled (default: true)
    pub titled: bool,
    /// Indicates if display is resizable (default: false)
    pub resizable: bool,
    /// Indicates if display always appears on top of all displays which are not topmost (default: false)
    pub topmost: bool,
    /// Indicates if display is minimizable (default: true)
    pub minimizable: bool,
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
    pub fn default() -> DisplayDescritor {
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
    ///     fn new(title: &str, width: usize, height: usize, display_descriptor: DisplayDescritor) -> Self {
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
    fn new(title: &str, width: usize, height: usize, display_descriptor: DisplayDescritor) -> Self;

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
    fn update_with_buffer(&mut self, buffer: &Self::Buffer, width: usize, height: usize);

    /// Check if the display is open. The user may want to take some action depending on this state.
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// impl Display for Implementor {
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
    ///     fn set_position(&mut self, x: usize, y: usize) {
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
    fn set_position(&mut self, x: usize, y: usize);

    /// Toggles a border in the display
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
    ///     fn set_background_color(&mut self, red: usize, green: usize, blue: usize) {
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
    fn set_background_color(&mut self, red: usize, green: usize, blue: usize);

    /// Returns the current size of the display
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ``no_run
    /// impl Display for Implementor {
    ///     fn get_size(&self) -> (usize, usize) {
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
    fn get_size(&self) -> (usize, usize);

    /// Checks if the display is the current active one
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ``no_run
    /// impl Display for Implementor {
    ///     fn is_active(&mut self) -> bool {
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
    fn is_active(&mut self) -> bool;

    // Tudo menos Cursor, Unscaled, Menus e limit_update_rate
}

struct BoxLayout {
    // min_x: unimplemented!(),
    // max_x: unimplemented!(),
    // min_y: unimplemented!(),
    // max_y: unimplemented!()
}

pub trait Display {
    
}

struct BoxLayout {
    min_x: unimplemented!(),
    max_x: unimplemented!(),
    min_y: unimplemented!(),
    max_y: unimplemented!()
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
    
    fn event_loop(&mut self, mut events: Queue<Event>, mut messages: Queue<Self::Message>, system: &mut T) {
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
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        
    }
}


