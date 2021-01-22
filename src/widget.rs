//! Contains the foundational elements for widgets.
//! Widgets implement the [`Widget`] trait, containing a set of basic functions shared among all widgets.
//! [`hyber`](`crate`) has a set of basic widgets implemented, each with their own module.

use crate::event::Event;
use crate::renderer::Message;
use crate::renderer::RenderInstruction;
use crate::renderer::RenderInstructionCollection;
use crate::util::IDMachine;
use crate::util::Queue;
use crate::util::Vector2D;

use std::cell::RefCell;
use std::rc::Weak;

pub mod button_view;
pub mod checkbox;
pub mod grid_view;
pub mod icon;
pub mod label;
pub mod list_view;
pub mod panel;
pub mod progress_bar;
pub mod root;
pub mod slider;
pub mod tab;
pub mod textbox;
pub mod tooltip_view;

/// Constraints that a parent imposes to its children
///
/// _**Note:** Based on Flutter documentation about constraints at
/// https://flutter.dev/docs/development/ui/layout/constraints
pub enum ConstraintType {
    /// The widget tells its child that it must be of a certain size
    Tight {
        /// The exact widget's size (width and height)
        size: Vector2D,
    },
    /// The widget tells its child that it can be smaller than a certain size
    Loose {
        /// The minimum widget's size (width and height)
        min: Vector2D,
        /// The maximum widget's size (width and height)
        max: Vector2D,
    },
}

// TODO: Not implemented
/// <span style="color:red">NOT IMPLEMENTED.</span> Struct for flex properties (whether to fill the maximum possible area or have a specific size)
///
pub enum Num {
    Num(usize),
    Infinity,
}

// TODO: Not implemented
/// <span style="color:red">NOT IMPLEMENTED.</span> To be used for animation type. Part of the initial work of [`panel::PanelWidget`] widget.
pub enum Animation {
    Reveal,
    Push,
}

/// Type of widget's layout
///
/// _**Note:** Based on Flutter documentation about sliver layout at
/// https://flutter.dev/docs/development/ui/advanced/slivers and based
/// on Java documentation about layout components at
/// https://docs.oracle.com/javase/tutorial/uiswing/layout
#[derive(Clone)]
pub enum Layout {
    /// Box layout either stacks its components on top of each other
    /// or places them in a row
    Box(Axis),
    /// Grid layout places components in a grid of cells
    Grid(Axis, usize),
    /// Sliver layout is a portion of a scrollable area that can be
    /// defined to behave in a special way
    Sliver(Axis),
    /// Layout undefined
    None,
}

/// Direction in which widgets are aligned
///
/// _**Note:** Based on Flutter documentation about the axis enum at
/// https://api.flutter.dev/flutter/painting/Axis-class.html
#[derive(Clone)]
pub enum Axis {
    /// The widgets are aligned left and right
    Horizontal,
    /// The widgets are aligned up and down
    Vertical,
}

/// Widgets are part of a user interface. They can be rendered on the
/// display and they can contain as many childs as they need. The root
/// widget is at the top of the widget tree. He manages all the widgets
/// to be displayed since they are childs of him. Then, all widgets
/// have their own child tree.
pub trait Widget {
    /// Detect if the event is being done on this widget and then update the
    /// widget's state based on event. After that, a message is enqueded into
    /// the message queue.
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `event` - an hyber event
    /// * `messages` - queue of messages
    fn on_event(&mut self, event: Event, messages: &mut Queue<Box<dyn Message>>);

    /// Gets widget's identifier
    ///
    /// # Returns
    /// The widget's identifier
    ///
    /// # Arguments
    /// No arguments
    fn id(&self) -> usize;

    /// Sets widget's identifier
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `id` - the identifier of the widget
    fn set_id(&mut self, id: usize);

    /// Detect if the cursor is in the widget's area
    ///
    /// # Returns
    /// True, if the cursor is in the widget's area, false otherwise
    ///
    /// # Arguments
    /// * `cursor_pos` - the position of the cursor
    fn is_cursor_inside(&mut self, cursor_pos: Vector2D) -> bool;

    /// Gets the collection of renderer instructions needed to draw this widget
    ///
    /// # Returns
    /// The collection of renderer instructions needed to draw this widget
    ///
    /// # Arguments
    /// No arguments
    fn recipe(&self) -> Vec<RenderInstruction>;

    /// Mark the widget as dirty
    ///
    /// An internal method to know which widgets need to be rebuilt
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `value` - the status to be assigned to the widget
    fn set_dirty(&mut self, value: bool);

    /// Gets the widget dirty flag value
    ///
    /// # Returns
    /// True, if the widget is mark as dirty, false otherwise
    ///
    /// # Arguments
    /// No arguments
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

    /// Gets the collection of children of the current widget
    ///
    /// # Returns
    /// The collection of children of the current widget
    ///
    /// # Arguments
    /// No arguments
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

    /// Gets the position of the widget's top left corner
    ///
    /// # Returns
    /// The position of the widget's top left corner
    ///
    /// # Arguments
    /// No arguments
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let widget = Widget::new();
    ///
    /// let (x, y) = widget.position();
    /// ```
    fn position(&mut self) -> Vector2D;

    /// Gets the widget's current size (width and height)
    ///
    /// # Returns
    /// The widget's current size (width and height)
    ///
    /// # Arguments
    /// No arguments
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let widget = Widget::new();
    ///
    /// let (width, height) = widget.size();
    /// ```
    fn size(&mut self) -> Vector2D;

    /// Gets the widget's original size (width and height)
    ///
    /// # Returns
    /// The widget's original size (width and height)
    ///
    /// # Arguments
    /// No arguments
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let widget = Widget::new();
    ///
    /// let (width, height) = widget.original_size();
    /// ```
    fn original_size(&mut self) -> Vector2D;

    /// Gets the widget's layout
    ///
    /// # Returns
    /// The widget's layout
    ///
    /// # Arguments
    /// No arguments
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
    fn layout(&mut self) -> &Layout;

    /// Gets the offset vector coordinates related with the widget's margin
    ///
    /// # Returns
    /// The offset vector coordinates
    ///
    /// # Arguments
    /// No arguments
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

    /// Gets some widget's attributes values
    ///
    /// # Returns
    /// The widget's attributes values
    ///
    /// # Arguments
    /// No arguments
    fn get_fields(
        &mut self,
    ) -> (
        bool,
        &mut Vec<Weak<RefCell<dyn Widget>>>,
        Vector2D,
        Vector2D,
        Vector2D,
        &Layout,
        Vector2D,
    );

    /// Sets the position of the widget's top left corner (x-position and y-position)
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// `position` - the position to be assigned to the widget
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

    /// Sets the widget's current size (width and height)
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// `size` - the size to be assigned to the widget
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

    /// Sets the widget's original size (width and height)
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// `size` - the size to be assigned to the widget
    fn set_original_size(&mut self, size: Vector2D);

    /// Sets the widget's offset vector coordinates according to his margins
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// `offset` - the offset to be assigned to the widget
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

        // Get children, layout, and offset of widget
        let (_, children, _, size, _, layout, offset) = self.get_fields();

        match layout {
            Layout::Box(axis) => {
                // For children size
                let mut child_size: Vector2D;

                // Update maximum dimensions according to offset
                max -= offset * 2.;

                // Update position of first child
                position += offset;

                let mut children_dirty = false;

                for value in children.iter_mut() {
                    if let Some(child) = value.upgrade() {
                        if children_dirty {
                            child.borrow_mut().set_dirty(true);
                        } else if child.borrow_mut().is_dirty() {
                            children_dirty = true;
                        }

                        // Get original child dimensions and do something to handle
                        // the dimensions assigned to the child
                        child_size = child.borrow_mut().original_size().min(max);

                        // Pass the child the assigned dimensions
                        child.borrow_mut().build(
                            position,
                            child_size,
                            id_machine,
                            instruction_collection,
                        );
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
            Layout::Grid(axis, axis_length) => match axis {
                Axis::Vertical => {
                    let cell_size = size
                        / Vector2D::new(
                            *axis_length as f64,
                            children.len() as f64 / *axis_length as f64,
                        );

                    let mut i: usize = 0;
                    for value in children.iter_mut() {
                        if let Some(child) = value.upgrade() {
                            let child_size = child.borrow_mut().original_size().min(cell_size);

                            // Pass the child the assigned dimensions
                            child.borrow_mut().build(
                                position
                                    + cell_size
                                        * Vector2D::new(
                                            (i % *axis_length) as f64,
                                            (i / *axis_length) as f64,
                                        ),
                                child_size,
                                id_machine,
                                instruction_collection,
                            );
                            i += 1;
                        }
                    }
                }
                Axis::Horizontal => {
                    let cell_size = size
                        / Vector2D::new(
                            children.len() as f64 / *axis_length as f64,
                            *axis_length as f64,
                        );

                    let mut i: usize = 0;
                    for value in children.iter_mut() {
                        if let Some(child) = value.upgrade() {
                            let child_size = child.borrow_mut().original_size().min(cell_size);
                            // Pass the child the assigned dimensions
                            child.borrow_mut().build(
                                position
                                    + cell_size
                                        * Vector2D::new(
                                            (i / *axis_length) as f64,
                                            (i % *axis_length) as f64,
                                        ),
                                child_size,
                                id_machine,
                                instruction_collection,
                            );
                            i += 1;
                        }
                    }
                }
            },
            Layout::Sliver(_axis) => {
                unimplemented!();
            }
            Layout::None => {
                for value in children.iter_mut() {
                    if let Some(child) = value.upgrade() {
                        let child_size = child.borrow_mut().original_size().min(max);
                        // Pass the child the assigned dimensions
                        child.borrow_mut().build(
                            position,
                            child_size,
                            id_machine,
                            instruction_collection,
                        );
                    }
                }
            }
        }
    }
}
