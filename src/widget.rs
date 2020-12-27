use crate::event;
use crate::event::Event;
use crate::key_code::KeyCode;
use crate::renderer::DrawImageOptions;
use crate::renderer::Message;
use crate::renderer::RenderInstruction;
use crate::renderer::RenderInstructionCollection;
use crate::util::Color;
use crate::util::IDMachine;
use crate::util::Queue;
use crate::util::Vector2D;

use std::cell::RefCell;
use std::rc::Weak;

/// Enum that classifies the type of constraints that
/// a parent imposes to its children
pub enum ConstraintType {
    Tight { size: Vector2D },
    Loose { min: Vector2D, max: Vector2D },
}

#[derive(Clone)]
pub enum Axis {
    Horizontal,
    Vertical,
}

/// TODO: DOCUMENTAR ISTO
/// D é Display
/// M é Message
pub trait Widget {
    /// @diogosemedo
    /// This function is needed to detect if the event is being done on this widget, update the state of
    /// the widget based on event and place a message in the message queue.
    ///
    /// # Returns
    /// An hyber Event
    ///
    /// # Arguments
    /// * `event` - an hyber event
    /// * `messages` - queue of messages
    fn on_event(&mut self, event: Event, messages: &mut Queue<Box<dyn Message>>);

    /// TODO: documentar
    fn set_id(&mut self, id: usize);
    fn id(&self) -> usize;

    /// @tofulynx
    /// this returns the "recipe" of the widget. In other words,
    /// it returns the collection of Instructions that tell the
    /// renderer how to draw this widget.
    fn recipe(&self) -> Vec<RenderInstruction>;

    /// @tofulynx
    /// For internal use only. Called by build(). marks widget as clean - no need to be rebuilt!
    fn set_dirty(&mut self, value: bool);

    /// TODO: documentar
    fn is_dirty(&self) -> bool;

    /// Adds a widget as a child of the current widget
    ///
    /// # Arguments
    /// `child` - widget to be added as a child
    ///
    /// # Examples
    ///
    /// Add a child to the parent widget
    ///
    /// ```no_run
    /// let parent = Widget::new();
    /// let child = Widget::new();
    ///
    /// parent.add_as_child(child);
    /// ```
    fn add_as_child(&mut self, child: Weak<RefCell<dyn Widget>>);

    /// Returns a collection of children of the current widget
    ///
    /// # Arguments
    ///
    /// # Examples
    ///
    /// List the children of the parent widget
    ///
    /// ```no_run
    /// let parent = Widget::new();
    /// let child = Widget::new();
    ///
    /// parent.add_as_child(child);
    ///
    /// let children = parent.children();
    /// ```
    fn get_children(&mut self) -> &mut Vec<Weak<RefCell<dyn Widget>>>;

    /// Sets a widget as the parent of the current widget (For internal use only)
    ///
    /// # Arguments
    /// `parent` - widget to be set as parent of the current widget
    ///
    /// # Examples
    ///
    /// Set a parent to the current widget
    ///
    /// ```no_run
    /// impl Widget<Display, MessageQueue> for ExampleWidget {
    ///     fn add_as_child(&mut self, child: &mut Self) {
    ///         child.set_as_parent(self);
    ///         ...
    ///     }
    /// }
    /// ```
    // fn set_as_parent(&mut self, parent: &mut Weak<RefCell<dyn Widget>>);

    /// Returns the position of the topleft corner of the current widget
    ///
    /// # Arguments
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let widget = Widget::new();
    ///
    /// let (x, y) = widget.position();
    /// ```
    fn position(&mut self) -> Vector2D;

    /// Returns the size of the current widget
    ///
    /// # Arguments
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let widget = Widget::new();
    ///
    /// let (width, height) = widget.size();
    /// ```
    fn size(&mut self) -> Vector2D;

    /// Returns the direction in which children of the current widget
    /// are placed (For internal use only)
    ///
    /// # Arguments
    ///
    /// # Examples
    ///
    /// ```no_run
    /// impl Widget<Display, MessageQueue> for ExampleWidget {
    ///     fn decompose_layout_to_children(&mut self, mut x: usize, mut y: usize, mut max_width: usize, mut max_height: usize) {
    ///         let axis = self.axis();
    ///         ...
    ///     }
    /// }
    /// ```
    fn axis(&mut self) -> &Axis;

    /// Returns the offset vector coordinates related with the margin in the current widget
    ///
    /// # Arguments
    ///
    /// # Examples
    ///
    /// ```no_run
    /// impl Widget<Display, MessageQueue> for ExampleWidget {
    ///     fn decompose_layout_to_children(&mut self, mut x: usize, mut y: usize, mut max_width: usize, mut max_height: usize) {
    ///         let (x_offset, y_offset) = self.offset();
    ///         ...
    ///     }
    /// }
    /// ```
    fn offset(&mut self) -> Vector2D;

    /// TODO: documentar
    fn get_fields(
        &mut self,
    ) -> (
        bool,
        &mut Vec<Weak<RefCell<dyn Widget>>>,
        Vector2D,
        Vector2D,
        &Axis,
        Vector2D,
    );

    /// Set the position of the topleft corner of the current widget
    ///
    /// # Arguments
    /// `x` - x-coordinate for the topleft corner
    /// `y` - y-coordinate for the topleft corner
    ///
    /// # Examples
    ///
    /// ```no_run
    /// impl Widget<Display, MessageQueue> for ExampleWidget {
    ///     fn decompose_layout_to_children(&mut self, mut x: usize, mut y: usize, mut max_width: usize, mut max_height: usize) {
    ///         self.set_position(x, y);
    ///         ...
    ///     }
    /// }
    /// ```
    fn set_position(&mut self, position: Vector2D);

    /// Sets the size of the current widget
    ///
    /// # Arguments
    /// `width` - width for the current widget
    /// `height` - height for the current widget
    ///
    /// # Examples
    ///
    /// ```no_run
    /// impl Widget<Display, MessageQueue> for ExampleWidget {
    ///     fn decompose_layout_to_children(&mut self, mut x: usize, mut y: usize, mut max_width: usize, mut max_height: usize) {
    ///         self.set_size(max_width, max_height);
    ///         ...
    ///     }
    /// }
    /// ```
    fn set_size(&mut self, size: Vector2D);

    /// TODO: documentar
    fn set_offset(&mut self, offset: Vector2D);

    /// Decomposes the layout constraints to the children of the current widget
    ///
    /// # Arguments
    /// `x` - x-coordinate for the topleft corner
    /// `y` - y-coordinate for the topleft corner
    /// `max_width` - maximum width that children can occupy. Should be equal to the width of the parent widget.
    /// `max_height` - maximum height that children can occupy. Should be equal to the height of the parent widget.
    ///
    /// # Examples
    ///
    /// Parent widget distributes its layout to the children
    ///
    /// ```no_run
    /// let parent = Widget::new();   // Parent Widget with dimensions 640x480
    /// let child_1 = Widget::new();  // Child Widget with dimensions 640x240
    /// let child_2 = Widget::new();  // Child Widget with dimensions 320x240
    ///
    /// parent.add_as_child(child_1);
    /// parent.add_as_child(child_2);
    ///
    /// (x, y) = parent.position();
    /// (width, height) = parent.size();
    ///
    /// decompose_layout_to_children(x, y, width, height);
    /// ```
    fn build(
        &mut self,
        mut position: Vector2D,
        mut max: Vector2D,
        id_machine: &mut IDMachine,
        instruction_collection: &mut RenderInstructionCollection,
    ) {
        if self.is_dirty() {
            // Assign position of widget
            self.set_position(position);
            // Assign size of widget
            self.set_size(max);

            instruction_collection.remove(self.id());
            self.set_id(id_machine.fetch_id());
            instruction_collection.replace_or_insert(self.id(), self.recipe().clone());
            self.set_dirty(false);
        }

        // Get children of widget
        // Get orientation to draw children in widget
        // Get offset vector
        let (_, children, _, _, axis, offset) = self.get_fields();

        // For children size
        let mut child_size: Vector2D;

        // Update maximum dimensions according to offset
        max -= offset * 2;

        // Update position of first child
        position += offset;

        let mut children_dirty = false;
        // Traverse each child and assign their constraints
        for value in children.iter_mut() {
            if let Some(child) = value.upgrade() {
                if children_dirty {
                    child.borrow_mut().set_dirty(true);
                } else if child.borrow_mut().is_dirty() {
                    children_dirty = true;
                }
                // Get child dimensions
                child_size = child.borrow_mut().size();

                // Do something to handle the dimensions assigned to the child
                child_size = child_size.min(max);

                // Pass the child the assigned dimensions
                child
                    .borrow_mut()
                    .build(position, child_size, id_machine, instruction_collection);

                // Update the constraints and position of next child
                match axis {
                    Axis::Horizontal => {
                        max.x -= child_size.x;
                        position.x += child_size.x;
                    }
                    Axis::Vertical => {
                        max.y -= child_size.y;
                        position.y += child_size.y;
                    }
                };
            }
        }
    }
}

#[derive(Clone)]
pub struct LabelWidget {
    id: usize,
    text: String,
    font_size: usize,
    background_color: Color,
    foreground_color: Color,
    dirty: bool,
    children: Vec<Weak<RefCell<dyn Widget>>>,
    position: Vector2D,
    size: Vector2D,
    axis: Axis,
    offset: Vector2D,
}

impl LabelWidget {
    pub fn new(
        text: String,
        size: Vector2D,
        font_size: usize,
        background_color: Color,
        foreground_color: Color,
        axis: Axis,
    ) -> LabelWidget {
        LabelWidget {
            id: 0,
            text: text,
            font_size: font_size,
            background_color: background_color,
            foreground_color: foreground_color,
            dirty: true,
            children: Vec::<Weak<RefCell<dyn Widget>>>::new(),
            position: Vector2D::new(0, 0),
            size: size,
            axis: axis,
            offset: Vector2D::new(0, 0),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.dirty = true;
    }
}

impl Widget for LabelWidget {
    fn on_event(&mut self, event: Event, messages: &mut Queue<Box<dyn Message>>) {}

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn id(&self) -> usize {
        self.id
    }

    fn recipe(&self) -> Vec<RenderInstruction> {
        vec![
            // Label rectangle.
            RenderInstruction::DrawRect {
                point: self.position,
                color: self.background_color.clone(),
                size: self.size,
            },
            // Label Text
            RenderInstruction::DrawText {
                point: Vector2D::new(self.position.x, self.position.y + self.size.y),
                color: self.foreground_color,
                font_size: self.font_size,
                string: self.text.clone(),
            },
        ]
    }

    fn set_dirty(&mut self, value: bool) {
        self.dirty = value;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn add_as_child(&mut self, child: Weak<RefCell<dyn Widget>>) {
        self.children.push(child);
    }

    fn get_children(&mut self) -> &mut Vec<Weak<RefCell<dyn Widget>>> {
        &mut self.children
    }

    fn position(&mut self) -> Vector2D {
        self.position
    }

    fn size(&mut self) -> Vector2D {
        self.size
    }

    fn axis(&mut self) -> &Axis {
        &self.axis
    }

    fn offset(&mut self) -> Vector2D {
        self.offset
    }

    fn get_fields(
        &mut self,
    ) -> (
        bool,
        &mut Vec<Weak<RefCell<dyn Widget>>>,
        Vector2D,
        Vector2D,
        &Axis,
        Vector2D,
    ) {
        (
            self.dirty,
            &mut self.children,
            self.position,
            self.size,
            &self.axis,
            self.offset,
        )
    }

    fn set_position(&mut self, position: Vector2D) {
        self.position = position;
    }

    fn set_size(&mut self, size: Vector2D) {
        self.dirty = true;
        self.size = size;
    }

    fn set_offset(&mut self, offset: Vector2D) {
        self.offset = offset;
    }
}

#[derive(Clone)]
pub struct RootWidget {
    id: usize,
    size: Vector2D,
    background_color: Color,
    message_incremented: Box<dyn Message>,
    message_decremented: Box<dyn Message>,
    message_resized: Box<dyn Message>,
    axis: Axis,
    dirty: bool,
    children: Vec<Weak<RefCell<dyn Widget>>>,
}

impl RootWidget {
    pub fn new(
        size: Vector2D,
        background_color: Color,
        axis: Axis,
        message_incremented: Box<dyn Message>,
        message_decremented: Box<dyn Message>,
        message_resized: Box<dyn Message>,
    ) -> RootWidget {
        RootWidget {
            id: 0,
            size: size,
            background_color: background_color,
            axis: axis,
            message_incremented: message_incremented,
            message_decremented: message_decremented,
            message_resized: message_resized,
            dirty: true,
            children: Vec::<Weak<RefCell<dyn Widget>>>::new(),
        }
    }
}

impl Widget for RootWidget {
    fn on_event(&mut self, event: Event, messages: &mut Queue<Box<dyn Message>>) {
        match event {
            event::Event::Keyboard(event::Keyboard::KeyPressed {
                key_code: KeyCode::I,
                modifiers:
                    event::ModifiersState {
                        shift: false,
                        control: false,
                        alt: false,
                        logo: false,
                    },
            }) => {
                println!("{:?} -> Entrou!", event);
                let mut message = self.message_incremented.clone();
                message.set_event(event);
                messages.enqueue(message);
            }
            event::Event::Keyboard(event::Keyboard::KeyPressed {
                key_code: KeyCode::D,
                modifiers:
                    event::ModifiersState {
                        shift: false,
                        control: false,
                        alt: false,
                        logo: false,
                    },
            }) => {
                println!("{:?} -> Entrou!", event);
                let mut message = self.message_decremented.clone();
                message.set_event(event);
                messages.enqueue(message);
            }
            event::Event::Mouse(event::Mouse::CursorMoved { x, y }) => {
                println!("{:?} -> Entrou!", event);
                let mut message = self.message_resized.clone();
                message.set_event(event);
                messages.enqueue(message);
            }
            _ => {
                println!("{:?} -> Passou root!", event);
                for value in self.children.iter_mut() {
                    if let Some(child) = value.upgrade() {
                        child.borrow_mut().on_event(event, messages);
                    }
                }
            }
        }
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn id(&self) -> usize {
        self.id
    }

    fn recipe(&self) -> Vec<RenderInstruction> {
        // TODO: Debater se isto deve ser usado como clear do ecrã.
        vec![RenderInstruction::Clear {
            color: self.background_color,
        }]
    }

    fn set_dirty(&mut self, value: bool) {
        self.dirty = value;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn add_as_child(&mut self, child: Weak<RefCell<dyn Widget>>) {
        self.children.push(child);
    }

    fn get_children(&mut self) -> &mut Vec<Weak<RefCell<dyn Widget>>> {
        &mut self.children
    }

    fn position(&mut self) -> Vector2D {
        Vector2D::new(0, 0)
    }

    fn size(&mut self) -> Vector2D {
        self.size
    }

    fn axis(&mut self) -> &Axis {
        // TODO: Ver se faz sentido ser só vertical
        &self.axis
    }

    fn offset(&mut self) -> Vector2D {
        Vector2D::new(0, 0)
    }

    fn get_fields(
        &mut self,
    ) -> (
        bool,
        &mut Vec<Weak<RefCell<dyn Widget>>>,
        Vector2D,
        Vector2D,
        &Axis,
        Vector2D,
    ) {
        (
            self.dirty,
            &mut self.children,
            Vector2D::new(0, 0),
            self.size,
            &self.axis,
            Vector2D::new(0, 0),
        )
    }

    fn set_position(&mut self, _position: Vector2D) {}

    fn set_size(&mut self, size: Vector2D) {
        self.dirty = true;
        self.size = size;
    }

    fn set_offset(&mut self, _offset: Vector2D) {}
}

#[derive(Clone)]
pub struct IconWidget {
    id: usize,
    path: String,
    options: DrawImageOptions,
    background_color: Color,
    dirty: bool,
    children: Vec<Weak<RefCell<dyn Widget>>>,
    position: Vector2D,
    size: Vector2D,
    axis: Axis,
    offset: Vector2D,
}

impl IconWidget {
    pub fn new(
        path: String,
        size: Vector2D,
        options: DrawImageOptions,
        background_color: Color,
        axis: Axis,
    ) -> IconWidget {
        IconWidget {
            id: 0,
            path: path,
            options: DrawImageOptions::OriginalSize,
            background_color: background_color,
            dirty: true,
            children: Vec::<Weak<RefCell<dyn Widget>>>::new(),
            position: Vector2D::new(0, 0),
            size: size,
            axis: axis,
            offset: Vector2D::new(0, 0),
        }
    }
}

impl Widget for IconWidget {
    fn on_event(&mut self, event: Event, messages: &mut Queue<Box<dyn Message>>) {}

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn id(&self) -> usize {
        self.id
    }

    fn recipe(&self) -> Vec<RenderInstruction> {
        vec![
            // Icon rectangle.
            RenderInstruction::DrawRect {
                point: self.position,
                color: self.background_color.clone(),
                size: self.size,
            },
            // Icon Image
            RenderInstruction::DrawImage {
                point: self.position, // todo: CHANGE after testing
                path: self.path.clone(),
                options: self.options.clone(),
            },
        ]
    }

    fn set_dirty(&mut self, value: bool) {
        self.dirty = value;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn add_as_child(&mut self, child: Weak<RefCell<dyn Widget>>) {
        self.children.push(child);
    }

    fn get_children(&mut self) -> &mut Vec<Weak<RefCell<dyn Widget>>> {
        &mut self.children
    }

    fn position(&mut self) -> Vector2D {
        self.position
    }

    fn size(&mut self) -> Vector2D {
        self.size
    }

    fn axis(&mut self) -> &Axis {
        &self.axis
    }

    fn offset(&mut self) -> Vector2D {
        self.offset
    }

    fn get_fields(
        &mut self,
    ) -> (
        bool,
        &mut Vec<Weak<RefCell<dyn Widget>>>,
        Vector2D,
        Vector2D,
        &Axis,
        Vector2D,
    ) {
        (
            self.dirty,
            &mut self.children,
            self.position,
            self.size,
            &self.axis,
            self.offset,
        )
    }

    fn set_position(&mut self, position: Vector2D) {
        self.position = position;
    }

    fn set_size(&mut self, size: Vector2D) {
        self.dirty = true;
        self.size = size;
    }

    fn set_offset(&mut self, offset: Vector2D) {
        self.offset = offset;
    }
}
