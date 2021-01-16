use crate::event::Event;
use crate::util::Color;
use crate::util::IDMachine;
use crate::util::Queue;
use crate::util::Vector2D;
use crate::widget::Widget;

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::rc::Weak;

/// Images resize configuration type
/// 
/// The image resize configuration is used to allow the renderer to know
/// how the image scales to fit the container dimensions.
#[derive(Clone)]
pub enum DrawImageOptions {
    /// Image is rendered with its default size
    OriginalSize,
    /// Image is resized by specific width and height values before being render
    Resize { 
        /// The new image's width
        width: usize, 
        /// The new image's height
        height: usize 
    },
    /// Image's dimensions are resized by a multiplier before image being render
    ResizeMultiplyer { 
        /// The image's dimensions multiplier
        mult: usize 
    },
}

/// Instructions to be executed by the renderer on the next clipping frame
/// 
/// This instructions are responsible for invoking primitive methods in the renderer
#[derive(Clone)]
pub enum RenderInstruction {
    /// Clear the render's buffer
    Clear { 
        /// Color to fill the window's background
        color: Color 
    },

    /// Draw a colored point on a specific point
    DrawPoint {
        /// The point's render position, on a two-dimensional space
        point: Vector2D,
        /// The point's color
        color: Color,
        /// The clipping area start point, on a two-dimensional space - window's 
        /// upper left corner
        clip_point: Vector2D,
        /// The clipping area end point, on a two-dimensional space - window's 
        /// bottom right corner
        clip_size: Vector2D,
    },

    /// Draw a colored line between two specific points
    DrawLine {
        /// The line start point, on a two-dimensional space
        point_a: Vector2D,
        /// The line end point, on a two-dimensional space
        point_b: Vector2D,
        /// The line's color
        color: Color,
        /// The clipping area start point, on a two-dimensional space - window's 
        /// upper left corner
        clip_point: Vector2D,
        /// The clipping area end point, on a two-dimensional space - window's 
        /// bottom right corner
        clip_size: Vector2D,
    },

    /// Draw an arc from a specific center point
    DrawArc {
        /// The arc center point, on a two-dimensional space
        point: Vector2D,
        /// The arc radius
        r: usize,
        /// The arc startangle
        s_ang: usize,
        /// The arc endangle
        e_ang: usize,
        /// The arc's fill color
        color: Color,
        /// The clipping area start point, on a two-dimensional space - window's 
        /// upper left corner
        clip_point: Vector2D,
        /// The clipping area end point, on a two-dimensional space - window's 
        /// bottom right corner
        clip_size: Vector2D,
    },

    /// Draw a circle centered on a specific point
    DrawCircle {
        /// The circle center point, on a two-dimensional space
        point: Vector2D,
        /// The circle radius
        r: usize,
        /// The circle fill color
        color: Color,
        /// The clipping area start point, on a two-dimensional space - window's 
        /// upper left corner
        clip_point: Vector2D,
        /// The clipping area end point, on a two-dimensional space - window's 
        /// bottom right corner
        clip_size: Vector2D,
    },

    /// Draw a rectangle based on the upper left and bottom right corners
    DrawRect {
        /// The rectangle start point, on a two-dimensional space - upper left corner
        point: Vector2D,
        /// The rectangle end point, on a two-dimensional space - bottom right corner
        size: Vector2D,
        /// The rectangle fill color
        color: Color,
        /// The clipping area start point, on a two-dimensional space - window's 
        /// upper left corner
        clip_point: Vector2D,
        /// The clipping area end point, on a two-dimensional space - window's 
        /// bottom right corner
        clip_size: Vector2D,
    },

    /// Draw a triangle based on his vertices locations
    DrawTriangle {
        /// The first vertice location of the triangle, on a two-dimensional space
        point_a: Vector2D,
        /// The second vertice location of the triangle, on a two-dimensional space
        point_b: Vector2D,
        /// The third vertice location of the triangle, on a two-dimensional space
        point_c: Vector2D,
        /// The triangle's fill color
        color: Color,
        /// The clipping area start point, on a two-dimensional space - window's 
        /// upper left corner
        clip_point: Vector2D,
        /// The clipping area end point, on a two-dimensional space - window's 
        /// bottom right corner
        clip_size: Vector2D,
    },

    /// Draw an image centered on a speciic point
    DrawImage {
        /// The image's center point, on a two-dimensional space
        point: Vector2D,
        /// The image's relative path
        path: String,
        /// The image's resize configuration
        options: DrawImageOptions,
        /// The clipping area start point, on a two-dimensional space - window's 
        /// upper left corner
        clip_point: Vector2D,
        /// The clipping area end point, on a two-dimensional space - window's 
        /// bottom right corner
        clip_size: Vector2D,
    },

    /// Draw text from a specific point forward
    ///
    /// _**Note:** The text alignment property is not considered here
    DrawText {
        /// The text starting point, on a two-dimensional space
        point: Vector2D,
        /// The text font size
        font_size: usize,
        /// The vector of characters (i.e., text) to be render
        string: String,
        /// The text font color
        color: Color,
        /// The clipping area start point, on a two-dimensional space - window's 
        /// upper left corner
        clip_point: Vector2D,
        /// The clipping area end point, on a two-dimensional space - window's 
        /// bottom right corner
        clip_size: Vector2D,
    },
}

/// Event messages
/// 
/// This messages are used to inform the application of an event that have occured
pub trait Message: MessageClone {
    /// Updates the widget according to the message's event
    fn update(&self);

    /// Sets the event to the message
    fn set_event(&mut self, event: Event);
}

/// Trait that allows to provide a blanket implementation for all compatible 
/// types, without having to implement the rest of Message.
///
/// This Clone is used to solve problems from cloning vector or boxes of messages
pub trait MessageClone {
    fn clone_box(&self) -> Box<dyn Message>;
}

impl<T> MessageClone for T
where
    T: 'static + Message + Clone,
{
    fn clone_box(&self) -> Box<dyn Message> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Message> {
    fn clone(&self) -> Box<dyn Message> {
        self.clone_box()
    }
}

/// Agnostic Renderer to handle all agnostic methods to the client's renderer
pub trait Renderer<D, E> {
    /// Map the events detected (i.e., Window, Keyboard, Mouse) into hyber events
    /// 
    /// _**Note:** It's recommended to define T as an enum
    ///
    /// # Returns
    /// An hyber event
    ///
    /// # Arguments
    /// * `event` - a generic event
    ///  
    /// # Examples
    /// fn map_events<T>(event: E) -> Event {
    ///     ...
    ///     match event {
    ///         leftclick => {
    ///             Mouse(Mouse::ButtonPressed(MouseButton::Left))
    ///         }
    ///         ...
    ///     }
    /// }
    fn map_events(event: E) -> Event;

    /// Creates an empty queue of events
    ///
    /// # Returns
    /// An empty queue of events
    ///
    /// # Arguments
    /// No arguments
    fn create_events_queue(&mut self) -> Queue<Event> {
        let queue: Queue<Event> = Queue::<Event>::new();
        queue
    }

    /// Creates an empty queue of messages
    ///
    /// _**Note:** Message should be an enum
    ///
    /// # Returns
    /// An empty queue of messages
    ///
    /// # Arguments
    /// No arguments
    fn create_message_queue(&mut self) -> Queue<Box<dyn Message>> {
        let queue: Queue<Box<dyn Message>> = Queue::new();
        queue
    }

    /// Detects the system events and map them into hyber events using map_events function
    ///
    /// _**Note:** The user should implement this function and enqueue the events on the events queue
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `events` - queue of events
    /// * `display` - generic type to access display events
    fn detect_display_events(events: &mut Queue<Event>, display: &mut D);

    /// Event loop that handles the events within hyber
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `events` - queue of events
    /// * `messages` - queue of messages
    /// * `root_ptr` - reference to the root widget
    /// * `display` - generic type to access display events
    /// * `display_size` - the display bottom right corner (i.e., the maximum window width and height)
    /// * `id_machine` - identifier of the machine
    /// * `render_instruction_collection_ptr` - collection of instructions to be rendered
    /// * `absolute_widget_collection_ptr` - collection of widgets to be rendered on absolute positions
    fn event_loop(
        &mut self,
        mut events: Queue<Event>,
        mut messages: Queue<Box<dyn Message>>,
        root_ptr: Weak<RefCell<dyn Widget>>,
        display: &mut D,
        display_size: Vector2D,
        id_machine: &mut IDMachine,
        render_instruction_collection_ptr: Weak<RefCell<RenderInstructionCollection>>,
        absolute_widget_collection_ptr: Weak<RefCell<AbsoluteWidgetCollection>>,
    ) {
        loop {
            // Detects and map the system events into hyber events
            Self::detect_display_events(&mut events, display);

            // Get the root "object" - allocation 
            if let Some(root) = root_ptr.upgrade() {
                // Get the render instructions collection "object" - allocation 
                if let Some(render_instruction_collection) =
                    render_instruction_collection_ptr.upgrade()
                {
                    // Iterate over the events queue
                    for event in events.queue.drain(..) {
                        // Call on_event method to detect if the event is being done on this 
                        // widget, update the state of the widget based on event and place a 
                        // message in the message queue.
                        root.borrow_mut().on_event(event, &mut messages);
                    }

                    // Iterate over all elements of the widget tree (i.e., starting from the
                    // root widget through all is childrens) to build them, if needed, and 
                    // decomposes the layout constraints to the children
                    root.borrow_mut().build(
                        Vector2D::new(0., 0.),
                        display_size,
                        id_machine,
                        &mut render_instruction_collection.borrow_mut(),
                    );

                    // Iterate over all elements of the absolute widgets collection to build 
                    // them, if needed, and decomposes the layout constraints to the children
                    if let Some(absolute_widgets) = absolute_widget_collection_ptr.upgrade() {
                        for (id, (value, position, size)) in
                            absolute_widgets.borrow_mut().widgets.iter()
                        {
                            // Get the widget "object" - allocation 
                            if let Some(widget) = value.upgrade() {
                                // If the widget needs to be rebuilt
                                if widget.borrow_mut().is_dirty() {
                                    // Assign position of widget
                                    widget.borrow_mut().set_position(*position);
                                    // Assign size of widget
                                    widget.borrow_mut().set_size(*size);

                                    render_instruction_collection.borrow_mut().remove(*id);
                                    // Add the render instructions of the widget to the render
                                    // instructions collection so that the widget is drawn
                                    render_instruction_collection
                                        .borrow_mut()
                                        .replace_or_insert(
                                            *id,
                                            widget.borrow_mut().recipe().clone(),
                                        );
                                
                                    // Update the dirty flag, set the widget as clean now
                                    widget.borrow_mut().set_dirty(false);
                                }
                            }
                        }
                    }

                    // Draws the collection of render instructions on the display
                    self.draw_collection(&mut render_instruction_collection.borrow_mut(), display);
                    
                    // Update messages
                    for message in messages.queue.drain(..) {
                        message.update();
                    }
                }
            }
        }
    }

    /// Draws the collection of render instructions on the display
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `collection` - collection of instructions to render
    /// * `display` - generic type to access display events
    fn draw_collection(&mut self, collection: &RenderInstructionCollection, display: &mut D);
}

/// Collection of render instructions to be rendered each frame
///
/// An ordered key-value collection which contains as key the identifier of the widget and as
/// value a vector of all his render instructions
pub struct RenderInstructionCollection {
    /// TreeMap of render instructions having the widget's identifier as key and the 
    /// vector of instructions to be rendered as value. The vector type was developed 
    /// by us and is available at util.rs
    ///
    /// _**Note:** Could be a crucial point on performance because it is used on the 
    /// renderization of every frame and his search method within the collection is 
    /// fundamental. Based on [`BTreeMap`] and used because of the ordination requisite
    ///
    /// [`BTreeMap`]: https://doc.rust-lang.org/beta/std/collections/struct.BTreeMap.html
    pub pairs: BTreeMap<usize, Vec<RenderInstruction>>,
}

impl RenderInstructionCollection {
    /// Creates a new `RenderInstructionCollection`
    ///
    /// # Returns
    /// An empty collection of render instructions
    ///
    /// # Arguments
    /// No arguments
    pub fn new() -> RenderInstructionCollection {
        RenderInstructionCollection {
            // Instantiates a new empty BTreeMap
            pairs: BTreeMap::<usize, Vec<RenderInstruction>>::new(),
        }
    }

    /// Replace/Insert the value of/to a given key 
    /// 
    /// # Returns
    /// No returns
    /// 
    /// # Arguments
    /// * `id` - the identifier of the widget that needs to be rendered
    /// * `instructions` - the widget's instructions to the renderer knows how to draw it
    pub fn replace_or_insert(&mut self, id: usize, instructions: Vec<RenderInstruction>) {
        // The BTreeMap replaces the value if the key already exists, otherwise insert a 
        // new map entry
        self.pairs.insert(id, instructions);
    }

    /// Remove the pair key-value from the render instructions collection
    /// 
    /// # Returns
    /// No returns
    /// 
    /// # Arguments
    /// * `id` - the key of the entry to be removed
    pub fn remove(&mut self, id: usize) {
        // The BTreeMap removes the map entry if the key exists there
        self.pairs.remove(&id);
    }
}

/// Collection of absolute widgets
///
/// This collection is iterated after the normal widget tree to ensure that widgets with
/// absolute positions are drawed over the relatives. The library ensures this by insert
/// this widgets render instructions after the render instructions of the widgets whithin
/// the widget tree.
pub struct AbsoluteWidgetCollection {
    /// The number of ids that is possible to generate
    counter: usize,
    /// HashMap of widgets with the corresponding value of the widget on the collection's 
    /// counter as key and with the widget itself and is location, on a two dimensional 
    /// space, as value
    pub widgets: HashMap<usize, (Weak<RefCell<dyn Widget>>, Vector2D, Vector2D)>,
}

impl AbsoluteWidgetCollection {
    /// Creates a new `AbsoluteWidgetCollection`
    ///
    /// # Returns
    /// An empty collection of absolute widgets with counter set to 0
    ///
    /// # Arguments
    /// No arguments
    pub fn new() -> AbsoluteWidgetCollection {
        AbsoluteWidgetCollection {
            counter: usize::MAX,
            widgets: HashMap::<usize, (Weak<RefCell<dyn Widget>>, Vector2D, Vector2D)>::new(),
        }
    }

    /// Inserts a new widget on the `AbsoluteWidgetCollection`
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `widget_ptr` - the widget to be inserted on the collection
    /// * `position` - the widget's upper left corner
    /// * `size` - the widget's bottom right corner
    pub fn insert(
        &mut self,
        widget_ptr: Weak<RefCell<dyn Widget>>,
        position: Vector2D,
        size: Vector2D,
    ) {
        // Since widget is a weak version of Rc that holds a non-owning reference 
        // to the managed allocation, we can access the allocation by calling upgrade
        // on the Weak pointer, which returns an Option<Rc<T>>
        // Returns None if the inner value has since been dropped.
        if let Some(widget) = widget_ptr.upgrade() {
            widget.borrow_mut().set_id(self.counter);
            // Insert the widget on the `AbsoluteWidgetCollection` with the value
            // of the counter at the moment as key and with the pointer to the widget
            // and his dimensions as value
            self.widgets
                .insert(self.counter, (widget_ptr, position, size));
            // Update the counter value
            self.counter -= 1;
        }
    }

    /// Removes a widget from the `AbsoluteWidgetCollection`
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `id` - the identifier of the widget that needs to be rendered
    pub fn remove(&mut self, id: usize) {
        // Remove the entry with key `id` from the `AbsoluteWidgetCollection`
        self.widgets.remove(&id);
    }
}
