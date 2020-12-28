use crate::event::Event;
use crate::renderer::Message;
use crate::renderer::RenderInstruction;
use crate::renderer::RenderInstructionCollection;
use crate::util::IDMachine;
use crate::util::Queue;
use crate::util::Vector2D;

use std::cell::RefCell;
use std::rc::Weak;

pub mod icon;
pub mod label;
pub mod root;

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
