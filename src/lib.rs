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
    
}


pub trait Display {
    
}

struct BoxLayout {
    min_x: unimplemented!(),
    max_x: unimplemented!(),
    min_y: unimplemented!(),
    max_y: unimplemented!()
}

struct SliverLayout {
}

pub trait Widget<Message> {
    fn on_event(event: Event, messages: Queue<Message>) { 
        unimplemented!()
    }
}

pub trait Renderer {

    type Message;
    
    /// This function is needed to map the events detected (Window, Keyboard, Mouse) into hyber events.
    /// We recommend user to define T has an enum.
    /// 
    /// # Returns 
    /// An hyber Event
    ///
    /// # Arguments
    /// It receives a generic event 
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
    fn map_events<T>(event: T) -> Event;
    
    ///This function creates a queue of events
    /// 
    /// # Returns 
    /// An empty vector for events
    /// 
    /// # Arguments
    /// No args
    fn create_events_queue() -> Queue<Event> {
        let mut queue: Queue<Event> = Queue::new();
        queue
    }
    
    /// This function creates a queue of messages.
    /// Message should be an enum.
    /// # Returns 
    /// An empty vector for messages
    /// 
    /// # Arguments
    /// No args
    fn create_message_queue<Message>() -> Queue<Message> {
        let mut queue: Queue<Message> = Queue::new();
        queue
    }
    
    /// This function is used to detect the system events and map them into hyber events using map_events function.
    /// The user should implement this function and put the events on the queue, using queue.enqueue .
    /// 
    /// # Returns 
    /// No returns
    ///
    /// # Arguments
    /// Receives the queue and system (generic type to access system events eg. in minifb crate its accessed via window). 
    fn detect_sys_events<T>(queue: &Queue<Event>, system: T);

    ///Este loop é responsável por:
    /// -> recolher os eventos do sistema
    /// -> dar update da user interface fazendo iteração sobre os eventos
    /// -> desenhar
    /// -> percorrer as mensagens e fazer o update
    fn event_loop<Message,T>(queue: Queue<Event>, message: Queue<Message>, system: T) {
        loop{
            // 1º RECOLHER -> MAPEAR -> METER NA QUEUE
            Renderer::detect_sys_events(&queue, system);
            /*if queue.lenght() != 0{
                let event = queue.dequeue();
                println!("novo evento");
            }*/
            // 2º chamar on event na arvore de widgets
            // 3º desenhar
            // 4º percorrer as mensagens e fazer update
            
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


