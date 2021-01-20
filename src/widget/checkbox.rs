use crate::event;
use crate::event::Event;
use crate::renderer::{Message, RenderInstruction};
use crate::util::{Color, Queue, Vector2D};
use crate::widget::{Layout, Widget};

use std::cell::RefCell;
use std::rc::Weak;

/// Checkbox is a stateful widget that when state changes calls
/// the `on_change` callback. When this changes are made, the 
/// checkbox is rebuilt and his visual appearance is updated.
#[derive(Clone)]
pub struct CheckBoxWidget {
    /// The checkbox's identifier
    id: usize,

    /// The checkbox's background color
    background_color: Color,

    /// The checkbox's border color when checked
    selected_color: Color,

    /// The message to be handled when a user change the 
    /// checkbox's checked flag (i.e., when the `is_checked` 
    /// flag changes its value)
    on_change: Option<Box<dyn Message>>,

    /// The checkbox's border size when not checked
    border_size: f64,

    /// The checkbox's border size when checked
    selected_relative_size: f64,
    
    /// The cursor's position
    cursor_pos: Vector2D,
    
    /// Whether the checkbox is checked
    is_checked: bool,
    
    /// The dirty flag (i.e., flag used to mark the widgets needed to be rebuilt)
    dirty: bool,
    
    /// The checkbox's children (i.e., his widgets tree)
    children: Vec<Weak<RefCell<dyn Widget>>>,
    
    /// The checkbox's position, on a two-dimensional space (x-coordinate and y-coordinate) 
    /// relative to the top left corner
    position: Vector2D,
    
    /// The checkbox's current size (width and height)
    size: Vector2D,
    
    /// The checkbox's original size (width and height)
    original_size: Vector2D,
    
    /// The checkbox's layout
    layout: Layout,

    /// The checkbox's offset vector coordinates
    offset: Vector2D,
}

impl CheckBoxWidget {
    /// Creates a new `CheckBoxWidget`
    ///
    /// # Returns
    /// The checkbox created
    ///
    /// # Arguments
    /// * `size` - the size (width and height) to be assigned to the checkbox
    /// * `background_color` - the color to be assigned to the checkbox's background
    /// * `selected_color` - the color to be assigned to the checkbox border when checked
    /// * `on_change` - the message to be handled when the checkbox's `is_checked` value change
    /// * `is_checked` - boolean indicating if checkbox is checked
    /// * `border_size` - the size to be assigned to the checkbox border when not checked
    /// * `selected_relative_size` - the size to be assigned to the checkbox border when checked
    pub fn new(
        size: Vector2D,
        background_color: Color,
        selected_color: Color,
        on_change: Option<Box<dyn Message>>,
        is_checked: bool,
        border_size: f64,
        selected_relative_size: f64,
    ) -> CheckBoxWidget {
        CheckBoxWidget {
            id: 0,
            background_color: background_color,
            selected_color: selected_color,
            on_change: on_change,
            is_checked: is_checked,
            border_size: border_size,
            selected_relative_size: selected_relative_size,
            cursor_pos: Vector2D::new(-1., -1.),
            dirty: true,
            children: Vec::<Weak<RefCell<dyn Widget>>>::new(),
            position: Vector2D::new(0., 0.),
            size: size,
            original_size: size,
            layout: Layout::None,
            offset: Vector2D::new(0., 0.),
        }
    }

    /// Not documented, check Drive.
    fn is_mouse_inside(&mut self) -> bool {
        if self.cursor_pos.x >= self.position().x
            && self.cursor_pos.x <= (self.position().x + self.size().x)
            && self.cursor_pos.y >= self.position().y
            && self.cursor_pos.y <= (self.position().y + self.size().y)
        {
            true
        } else {
            false
        }
    }

    /// Sets the message to be handled when the checkbox's `is_checked` value change
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `on_change` - the message to be handled when the checkbox's `is_checked` value change
    pub fn set_message(&mut self, on_change: Option<Box<dyn Message>>) {
        self.on_change = on_change;
    }
    
    /// Checks if the checkbox is checked
    ///
    /// # Returns
    /// True, if the checkbox is checked, false otherwise
    ///
    /// # Arguments
    /// No arguments
    pub fn get_is_checked(&self) -> bool {
        self.is_checked
    }
}

impl Widget for CheckBoxWidget {
    fn on_event(&mut self, event: Event, messages: &mut Queue<Box<dyn Message>>) {
        match event {
            event::Event::Mouse(event::Mouse::CursorMoved { x: x_pos, y: y_pos }) => {
                self.cursor_pos = Vector2D::new(x_pos as f64, y_pos as f64);
                for value in self.children.iter_mut() {
                    if let Some(child) = value.upgrade() {
                        child.borrow_mut().on_event(event, messages);
                    }
                }
            }
            event::Event::Mouse(event::Mouse::ButtonPressed(event::MouseButton::Left)) => {
                if self.is_mouse_inside() {
                    if let Some(mut message) = self.on_change.clone() {
                        message.set_event(event);
                        messages.enqueue(message);
                    }
                    self.is_checked = !self.is_checked;
                    self.set_dirty(true);
                }
            }
            _ => {
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
        if self.is_checked {
            vec![
                RenderInstruction::DrawRect {
                    point: self.position,
                    color: self.selected_color,
                    size: self.size,
                    clip_point: self.position,
                    clip_size: self.size,
                },
                RenderInstruction::DrawRect {
                    point: Vector2D::new(
                        self.position.x + self.size.x * self.selected_relative_size,
                        self.position.y + self.size.y * self.selected_relative_size,
                    ),
                    color: self.background_color,
                    size: Vector2D::new(
                        self.size.x - (2. * (self.size.x * self.selected_relative_size)),
                        self.size.y - (2. * (self.size.y * self.selected_relative_size)),
                    ),
                    clip_point: self.position,
                    clip_size: self.size,
                },
            ]
        } else {
            vec![
                RenderInstruction::DrawRect {
                    point: self.position,
                    color: Color::from_hex(0xFF000000),
                    size: self.size,
                    clip_point: self.position,
                    clip_size: self.size,
                },
                RenderInstruction::DrawRect {
                    point: Vector2D::new(
                        self.position.x + self.border_size,
                        self.position.y + self.border_size,
                    ),
                    color: self.background_color,
                    size: Vector2D::new(
                        self.size.x - (2. * self.border_size),
                        self.size.y - (2. * self.border_size),
                    ),
                    clip_point: self.position,
                    clip_size: self.size,
                },
            ]
        }
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
    fn original_size(&mut self) -> Vector2D {
        self.original_size
    }

    fn layout(&mut self) -> &Layout {
        &self.layout
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
        Vector2D,
        &Layout,
        Vector2D,
    ) {
        (
            self.dirty,
            &mut self.children,
            self.position,
            self.size,
            self.original_size,
            &self.layout,
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

    fn set_original_size(&mut self, size: Vector2D) {
        self.dirty = true;
        self.original_size = size;
    }

    fn set_offset(&mut self, offset: Vector2D) {
        self.offset = offset;
    }

    fn is_cursor_inside(&mut self, cursor_pos: Vector2D) -> bool {
        if (self.position.x + self.size.x) >= cursor_pos.x
            && (self.position.y + self.size.y) >= cursor_pos.y
            && self.position.x <= cursor_pos.x
            && self.position.y <= cursor_pos.y
        {
            true
        } else {
            false
        }
    }
}
