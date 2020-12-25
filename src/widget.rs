use crate::event::Event;
use crate::renderer::DrawImageOptions;
use crate::renderer::RenderInstruction;
use crate::renderer::RenderInstructionCollection;
use crate::util::Color;
use crate::util::IDMachine;
use crate::util::Point;
use crate::util::Queue;

/// Enum that classifies the type of constraints that
/// a parent imposes to its children
pub enum ConstraintType {
    Tight {
        x: usize,
        y: usize,
    },
    Loose {
        min_x: usize,
        max_x: usize,
        min_y: usize,
        max_y: usize,
    },
}

pub enum Axis {
    Horizontal,
    Vertical,
}

/// TODO: DOCUMENTAR ISTO
/// D é Display
/// M é Message
pub trait Widget<M> {
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
    fn on_event(&mut self, event: Event, messages: &Queue<M>);

    /// TODO: documentar
    fn set_id(&mut self, id: usize);
    fn id(&self) -> usize;

    /// @tofulynx
    /// this returns the "recipe" of the widget. In other words,
    /// it returns the collection of Instructions that tell the
    /// renderer how to draw this widget.
    fn recipe(&self) -> Vec<RenderInstruction>;

    /// @tofulynx
    /// recursive function
    /// 3 steps:
    /// - add recipe() instructions to display's tree view
    /// - mark_as_clean()
    /// - call build() on children
    /// @tofulynx
    /// marks widget and its children as dirty - they need to be rebuilt!
    fn mark_as_dirty(&mut self, instruction_collection: &mut RenderInstructionCollection) {
        self.set_dirty(true);

        for child in self.get_children().iter_mut() {
            child.mark_as_dirty(instruction_collection);
        }
    }

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
    fn add_as_child(&mut self, child: Box<dyn Widget<M>>);

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
    fn get_children(&mut self) -> &mut Vec<Box<dyn Widget<M>>>;

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
    // fn set_as_parent(&mut self, parent: &mut Box<dyn Widget<M>>);

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
    fn position(&mut self) -> (usize, usize);

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
    fn size(&mut self) -> (usize, usize);

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
    fn offset(&mut self) -> (usize, usize);

    /// TODO: documentar
    fn get_fields(
        &mut self,
    ) -> (
        bool,
        &mut Vec<Box<dyn Widget<M>>>,
        (usize, usize),
        (usize, usize),
        &Axis,
        (usize, usize),
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
    fn set_position(&mut self, x: usize, y: usize);

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
    fn set_size(&mut self, width: usize, height: usize);

    /// TODO: documentar
    fn set_offset(&mut self, x: usize, y: usize);

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
        mut x: usize,
        mut y: usize,
        mut max_width: usize,
        mut max_height: usize,
        id_machine: &mut IDMachine,
        instruction_collection: &mut RenderInstructionCollection,
    ) {
        if self.is_dirty() {
            // Assign position of widget
            self.set_position(x, y);
            // Assign size of widget
            self.set_size(max_width, max_height);

            self.set_id(id_machine.fetch_id());
            instruction_collection.replace_or_insert(self.id(), self.recipe().clone());
            self.set_dirty(false);

            // Get children of widget
            // Get orientation to draw children in widget
            // Get offset vector
            let (_, children, _, _, axis, (x_offset, y_offset)) = self.get_fields();

            // For children size
            let mut child_size: (usize, usize);

            // Update maximum dimensions according to offset
            max_width -= 2 * x_offset;
            max_height -= 2 * y_offset;

            // Update position of first child
            x += x_offset;
            y += y_offset;

            // Traverse each child and assign their constraints
            for child in children.iter_mut() {
                // Get child dimensions
                child_size = child.size();

                // Do something to handle the dimensions assigned to the child
                child_size.0 = child_size.0.min(max_width);
                child_size.1 = child_size.1.min(max_height);

                // Pass the child the assigned dimensions
                child.build(
                    x,
                    y,
                    child_size.0,
                    child_size.1,
                    id_machine,
                    instruction_collection,
                );

                // Update the constraints and position of next child
                match axis {
                    Axis::Horizontal => {
                        max_width -= child_size.0;
                        x += child_size.0
                    }
                    Axis::Vertical => {
                        max_height -= child_size.1;
                        y += child_size.1
                    }
                };
            }
        }
    }
}

pub struct LabelWidget<M> {
    id: usize,
    text: String,
    color: Color,
    dirty: bool,
    children: Vec<Box<dyn Widget<M>>>,
    position: (usize, usize),
    size: (usize, usize),
    axis: Axis,
    offset: (usize, usize),
}

impl<M> LabelWidget<M> {
    pub fn new(text: String, size: (usize, usize), color: Color, axis: Axis) -> LabelWidget<M> {
        LabelWidget {
            id: 0,
            text: text,
            color: color,
            dirty: true,
            children: Vec::<Box<dyn Widget<M>>>::new(),
            position: (0, 0),
            size: size,
            axis: axis,
            offset: (0, 0),
        }
    }
}

impl<M> Widget<M> for LabelWidget<M> {
    fn on_event(&mut self, event: Event, messages: &Queue<M>) {
        unimplemented!();
    }

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
                point: Point {
                    x: self.position.0 as f32,
                    y: self.position.1 as f32,
                },
                color: self.color.clone(),
                height: self.size.0 as u32,
                width: self.size.1 as u32,
            },
            // Label Text
            RenderInstruction::DrawText {
                point: Point {
                    x: self.position.0 as f32,
                    y: (self.position.1 + self.size.1) as f32,
                },
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

    fn add_as_child(&mut self, child: Box<dyn Widget<M>>) {
        self.children.push(child);
    }

    fn get_children(&mut self) -> &mut Vec<Box<dyn Widget<M>>> {
        &mut self.children
    }

    fn position(&mut self) -> (usize, usize) {
        self.position
    }

    fn size(&mut self) -> (usize, usize) {
        self.size
    }

    fn axis(&mut self) -> &Axis {
        &self.axis
    }

    fn offset(&mut self) -> (usize, usize) {
        self.offset
    }

    fn get_fields(
        &mut self,
    ) -> (
        bool,
        &mut Vec<Box<dyn Widget<M>>>,
        (usize, usize),
        (usize, usize),
        &Axis,
        (usize, usize),
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

    fn set_position(&mut self, x: usize, y: usize) {
        self.position = (x, y);
    }

    fn set_size(&mut self, x: usize, y: usize) {
        self.size = (x, y);
    }

    fn set_offset(&mut self, x: usize, y: usize) {
        self.offset = (x, y);
    }
}

pub struct RootWidget<M> {
    id: usize,
    size: (usize, usize),
    axis: Axis,
    dirty: bool,
    children: Vec<Box<dyn Widget<M>>>,
}

impl<M> RootWidget<M> {
    pub fn new(size: (usize, usize), axis: Axis) -> RootWidget<M> {
        RootWidget {
            id: 0,
            size: size,
            axis: axis,
            dirty: true,
            children: Vec::<Box<dyn Widget<M>>>::new(),
        }
    }
}

impl<M> Widget<M> for RootWidget<M> {
    fn on_event(&mut self, event: Event, messages: &Queue<M>) {
        unimplemented!();
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn id(&self) -> usize {
        self.id
    }

    fn recipe(&self) -> Vec<RenderInstruction> {
        // TODO: Debater se isto deve ser usado como clear do ecrã.
        Vec::<RenderInstruction>::new()
    }

    fn set_dirty(&mut self, value: bool) {
        self.dirty = value;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn add_as_child(&mut self, child: Box<dyn Widget<M>>) {
        self.children.push(child);
    }

    fn get_children(&mut self) -> &mut Vec<Box<dyn Widget<M>>> {
        &mut self.children
    }

    fn position(&mut self) -> (usize, usize) {
        (0, 0)
    }

    fn size(&mut self) -> (usize, usize) {
        self.size
    }

    fn axis(&mut self) -> &Axis {
        // TODO: Ver se faz sentido ser só vertical
        &self.axis
    }

    fn offset(&mut self) -> (usize, usize) {
        (0, 0)
    }

    fn get_fields(
        &mut self,
    ) -> (
        bool,
        &mut Vec<Box<dyn Widget<M>>>,
        (usize, usize),
        (usize, usize),
        &Axis,
        (usize, usize),
    ) {
        (
            self.dirty,
            &mut self.children,
            (0, 0),
            self.size,
            &self.axis,
            (0, 0),
        )
    }

    fn set_position(&mut self, _x: usize, _y: usize) {}

    fn set_size(&mut self, x: usize, y: usize) {
        self.size = (x, y);
    }

    fn set_offset(&mut self, _x: usize, _y: usize) {}
}
