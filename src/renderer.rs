use crate::event::Event;
use crate::util::Color;
use crate::util::IDMachine;
use crate::util::Queue;
use crate::util::Vector2D;
use crate::widget::Widget;

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::rc::Weak;

#[derive(Clone)]
pub enum DrawImageOptions {
    OriginalSize,
    Resize { width: usize, height: usize },
    ResizeMultiplyer { mult: usize },
}

/// Enumeration with the Render Instructions
#[derive(Clone)]
pub enum RenderInstruction {
    /// Instruction to the Render that the buffer needs to be cleared
    /// Uses a Color struct using hexadecimal alpha and rgb for coloring
    Clear { color: Color },

    /// Instruction to the Render that a point needs to be drawn on the next Clipping
    /// The point should be rendered on absolute coordinates (x,y)
    /// Uses a Color struct using hexadecimal alpha and rgb for coloring
    DrawPoint { point: Vector2D, color: Color },

    /// Instruction to the Render that a line needs to be drawn on the next Clipping
    /// The line should be rendered on absolute coordinates from (x1, y1) to (x2, y2)
    /// Uses a Color struct using hexadecimal alpha and rgb for coloring
    DrawLine {
        point_a: Vector2D,
        point_b: Vector2D,
        color: Color,
    },

    /// Instruction to the Render that an arc needs to be drawn on the next Clipping
    /// The arc should be rendered with center on absolute coordinates (x, y), 'r'
    /// radius, 'sang' start angle and 'eang' end angle.
    /// Uses a Color struct using hexadecimal alpha and rgb for coloring
    DrawArc {
        point: Vector2D,
        r: usize,
        s_ang: usize,
        e_ang: usize,
        color: Color,
    },

    /// Instruction to the Render that a circle needs to be drawn on the next Clipping
    /// The circle should be rendered with center on absolute coordinates (x, y) and 'r'
    /// radius
    /// Uses a Color struct using hexadecimal alpha and rgb for coloring
    DrawCircle {
        point: Vector2D,
        r: usize,
        color: Color,
    },
    /// Instruction to the Render that a rectangle needs to be drawn on the next Clipping
    /// The rectangle should be rendered on absolute coordinates (x, y) with 'l' length
    /// and 'w' width
    /// Uses a Color struct using hexadecimal alpha and rgb for coloring
    /// Uses a Color struct using hexadecimal alpha and rgb for coloring
    DrawRect {
        point: Vector2D,
        size: Vector2D,
        color: Color,
    },

    /// Instruction to the Render that a triangle needs to be drawn on the next Clipping
    /// The triangle should be rendered between the absolute coordinates (x1, y1),
    /// (x2, y2) and (x3, y3)
    /// Uses a Color struct using hexadecimal alpha and rgb for coloring
    DrawTriangle {
        point_a: Vector2D,
        point_b: Vector2D,
        point_c: Vector2D,
        color: Color,
    },

    /// Instruction to the Render that an image needs to be drawn on the next Clipping
    /// [Doubt] The image should be rendered with center on the absolute coordinates (x, y)
    /// and with 'w' width and 'l' length
    DrawImage {
        point: Vector2D,
        path: String,
        options: DrawImageOptions,
    },

    /// Instruction to the Render that some text needs to be drawn on the next Clipping
    /// [Doubt] The text should be rendered according to the text_alignment
    DrawText {
        point: Vector2D,
        font_size: usize,
        string: String,
        color: Color,
    },
}

pub trait Message: MessageClone {
    fn update(&self);
    fn set_event(&mut self, event: Event);
}

// Splitting AnimalClone into its own trait allows us to provide a blanket
// implementation for all compatible types, without having to implement the
// rest of Animal.  In this case, we implement it for all types that have
// 'static lifetime (*i.e.* they don't contain non-'static pointers), and
// implement both Animal and Clone.  Don't ask me how the compiler resolves
// implementing AnimalClone for Animal when Animal requires AnimalClone; I
// have *no* idea why this works.
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

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Message> {
    fn clone(&self) -> Box<dyn Message> {
        self.clone_box()
    }
}

// Assumptions:
//     - 2D Meshes are compounded by a list of triangles so the instructions are gonna be
//     multiple DrawTriangleAbs intructions
//     Reference: https://github.com/hecrj/iced/blob/master/graphics/src/triangle.rs
//     - Based on: https://en.wikipedia.org/wiki/Geometric_primitive
//     - And on:   https://www.freepascal.org/docs-html/current/rtl/graph/funcdrawing.html
pub trait Renderer<D, E> {
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
    fn create_message_queue(&mut self) -> Queue<Box<dyn Message>> {
        let queue: Queue<Box<dyn Message>> = Queue::new();
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
    /// * `display` - a generic type to access display events eg. in minifb crate its accessed via window
    //fn detect_display_events(events: &Queue<Event>);
    fn detect_display_events(events: &mut Queue<Event>, display: &mut D);

    /// This function has the event loop of hyber. It can be described in 4 steps:
    /// * 1st - To recall the display events.
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
    /// * `display` - a generic type to access display events eg. in minifb crate its accessed via window
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
            // 1º RECOLHER -> MAPEAR -> METER NA QUEUE
            Self::detect_display_events(&mut events, display);

            // 2º chamar on event na arvore de widgets
            // estes eventos alterarão a collection.
            if let Some(root) = root_ptr.upgrade() {
                if let Some(render_instruction_collection) =
                    render_instruction_collection_ptr.upgrade()
                {
                    for event in events.queue.drain(..) {
                        root.borrow_mut().on_event(event, &mut messages);
                    }

                    root.borrow_mut().build(
                        Vector2D::new(0., 0.),
                        display_size,
                        id_machine,
                        &mut render_instruction_collection.borrow_mut(),
                    );

                    // Chamar build do absolute
                    if let Some(absolute_widgets) = absolute_widget_collection_ptr.upgrade() {
                        println!("tamanho: {}", absolute_widgets.borrow_mut().widgets.len());
                        for (id, (value, position, size)) in
                            absolute_widgets.borrow_mut().widgets.iter()
                        {
                            if let Some(widget) = value.upgrade() {
                                if widget.borrow_mut().is_dirty() {
                                    // Assign position of widget
                                    widget.borrow_mut().set_position(*position);
                                    // Assign size of widget
                                    widget.borrow_mut().set_size(*size);

                                    render_instruction_collection.borrow_mut().remove(*id);
                                    render_instruction_collection
                                        .borrow_mut()
                                        .replace_or_insert(
                                            *id,
                                            widget.borrow_mut().recipe().clone(),
                                        );
                                    widget.borrow_mut().set_dirty(false);
                                }
                            }
                        }
                    }

                    // 3º desenhar
                    self.draw_collection(&mut render_instruction_collection.borrow_mut(), display);
                    // 4º percorrer as mensagens e fazer update
                    for message in messages.queue.drain(..) {
                        message.update();
                    }
                }
            }
        }
    }

    /// This function is used to draw the given RenderInstruction in a Display
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `instruction` - RenderInstruction to draw a primitive
    fn draw_collection(&mut self, collection: &RenderInstructionCollection, display: &mut D);
}

/// Structure that represents the collection of Render Instructions to be
/// rendered each frame
pub struct RenderInstructionCollection {
    pub pairs: BTreeMap<usize, Vec<RenderInstruction>>,
}

impl RenderInstructionCollection {
    pub fn new() -> RenderInstructionCollection {
        RenderInstructionCollection {
            pairs: BTreeMap::<usize, Vec<RenderInstruction>>::new(),
        }
    }

    pub fn replace_or_insert(&mut self, id: usize, instructions: Vec<RenderInstruction>) {
        self.pairs.insert(id, instructions);
    }

    pub fn remove(&mut self, id: usize) {
        self.pairs.remove(&id);
    }
}

pub struct AbsoluteWidgetCollection {
    counter: usize,
    pub widgets: HashMap<usize, (Weak<RefCell<dyn Widget>>, Vector2D, Vector2D)>,
}

impl AbsoluteWidgetCollection {
    pub fn new() -> AbsoluteWidgetCollection {
        AbsoluteWidgetCollection {
            counter: usize::MAX,
            widgets: HashMap::<usize, (Weak<RefCell<dyn Widget>>, Vector2D, Vector2D)>::new(),
        }
    }

    pub fn insert(
        &mut self,
        widget_ptr: Weak<RefCell<dyn Widget>>,
        position: Vector2D,
        size: Vector2D,
    ) {
        if let Some(widget) = widget_ptr.upgrade() {
            widget.borrow_mut().set_id(self.counter);
            self.widgets
                .insert(self.counter, (widget_ptr, position, size));
            self.counter -= 1;
        }
    }

    pub fn remove(&mut self, id: usize) {
        self.widgets.remove(&id);
    }
}
