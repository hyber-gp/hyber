use crate::event::Event;
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
pub trait Widget<D, M> {
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

    /// @joaosantos
    /// this returns the "recipe" of the widget. In other words,
    /// it returns the collection of Instructions that tell the
    /// renderer how to draw this widget.
    fn recipe(&mut self);

    /// @joaosantos
    /// recursive function
    /// 3 steps:
    /// - add recipe() instructions to display's tree view
    /// - mark_as_clean()
    /// - call build() on children
    fn build(&mut self);

    /// @joaosantos
    /// marks widget and its children as dirty - they need to be rebuilt!
    fn mark_as_dirty(&mut self);

    /// @joaosantos
    /// For internal use only. Called by build(). marks widget as clean - no need to be rebuilt!
    fn mark_as_clean(&mut self);

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
    fn add_as_child(&mut self, child: &mut Box<dyn Widget<D, M>>);

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
    fn children(&self) -> &mut Vec<&mut Box<dyn Widget<D, M>>>;

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
    fn set_as_parent(&mut self, parent: &mut Box<dyn Widget<D, M>>);

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
    fn size(&self) -> (usize, usize);

    /// Returns the direction in which children of the current widget
    /// are placed (For internal use only)
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// impl Widget<Display, MessageQueue> for ExampleWidget {
    ///     fn decompose_layout_to_children(&mut self, mut max_width: usize, mut max_height: usize) {
    ///         let axis = self.axis();
    ///         ...
    ///     }
    /// }
    /// ```
    fn axis(&self) -> &Axis;

    /// Decomposes the layout constraints to the children of the current widget
    ///
    /// # Arguments
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
    /// decompose_layout_to_children()
    /// ```
    fn decompose_layout_to_children(&mut self, mut max_width: usize, mut max_height: usize) {
        // Get children of widget
        let children = self.children();
        let axis = self.axis();

        // Traverse each child and assign their constraints
        for child in children.iter_mut() {
            // Get child dimensions
            let (mut child_width, mut child_height) = child.size();

            // Do something to handle the dimensions assigned to the child
            if child_width >= max_width {
                child_width = max_width;
            }
            if child_height >= max_height {
                child_height = max_height;
            }

            // Update the constraints
            match axis {
                Axis::Vertical => max_height -= child_height,
                Axis::Horizontal => max_width -= child_width,
            };

            // Pass the child the assigned dimensions
            child.decompose_layout_to_children(child_width, child_height);
        }
    }
}

pub struct DebugWidget {   
}
