use crate::event;
use crate::event::Event;
use crate::renderer::{Message, RenderInstruction};
use crate::util::{Color, Queue, Vector2D};
use crate::widget::{Layout, Widget};

use std::cell::RefCell;
use std::rc::Weak;

#[derive(Clone)]
pub struct TextBoxWidget {
    /// The textbox's identifier
    id: usize,
    
    /// The textbox's background color
    background_color: Color,
    
    /// The textbox's text color
    text_color: Color,
    
    /// The textbox's border thickness
    border_thickness: f64,
    
    /// The textbox's text
    text: String,
    
    /// The message to be handled when the text changes
    on_text_change: Option<Box<dyn Message>>,
    
    /// Whether the textbox is focused
    is_focused: bool,
    
    /// The cursor's position
    cursor_pos: Vector2D,

    /// The dirty flag (i.e., flag used to mark the widgets needed to be rebuilt)
    dirty: bool,

    /// The textbox's children (i.e., his widgets tree)
    children: Vec<Weak<RefCell<dyn Widget>>>,

    /// The textbox's position, on a two-dimensional space (x-coordinate and y-coordinate) 
    /// relative to the top left corner
    position: Vector2D,

    /// The textbox's current size (width and height)
    size: Vector2D,

    /// The textbox's original size (width and height)
    original_size: Vector2D,

    /// The textbox's layout
    layout: Layout,

    /// The textbox's offset vector coordinates
    offset: Vector2D,
}

impl TextBoxWidget {
    /// Creates a new `TextBoxWidget`
    ///
    /// # Returns
    /// The textbox created
    ///
    /// # Arguments
    /// * `size` - the size (width and height) to be assigned to the textbox
    /// * `background_color` - the color to be assigned to the textbox's background
    /// * `text_color` - the color to be assigned to the textbox's text
    /// * `border_thickness` - the thickness to be assigned to the textbox's border
    /// * `text` - the text to be assigned to the textbox
    /// * `on_text_change` - the message to be handled when the tetxbox is focused 
    pub fn new(
        size: Vector2D,
        background_color: Color,
        text_color: Color,
        border_thickness: f64,
        text: String,
        on_text_change: Option<Box<dyn Message>>,
    ) -> TextBoxWidget {
        TextBoxWidget {
            id: 0,
            background_color: background_color,
            text_color: text_color,
            border_thickness: border_thickness,
            text: text,
            on_text_change: on_text_change,
            is_focused: false,
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

    /// Sets the message to be handled when the textbox is focused
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `on_text_change` - the new message to be handled when the textbox is focused
    pub fn set_message(&mut self, on_text_change: Option<Box<dyn Message>>) {
        self.on_text_change = on_text_change;
    }
}

impl Widget for TextBoxWidget {
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
                if self.is_cursor_inside(self.cursor_pos) {
                    self.is_focused = true;
                } else {
                    self.is_focused = false;
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
        vec![
            RenderInstruction::DrawRect {
                point: self.position,
                size: self.size,
                color: Color::from_hex(0xFF000000),
                clip_point: self.position,
                clip_size: self.size,
            },
            RenderInstruction::DrawRect {
                point: Vector2D::new(
                    self.position.x + self.border_thickness,
                    self.position.y + self.border_thickness,
                ),
                size: Vector2D::new(
                    self.size.x - (2. * self.border_thickness),
                    self.size.y - (2. * self.border_thickness),
                ),
                color: self.background_color,
                clip_point: self.position,
                clip_size: self.size,
            },
            RenderInstruction::DrawText {
                point: Vector2D::new(self.position.x + 10., self.position.y + 20.),
                font_size: 22,
                string: self.text.clone(),
                color: self.text_color,
                clip_point: self.position,
                clip_size: self.size,
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

    fn set_clip_point(&mut self, _clip_point: Option<Vector2D>) {
        unimplemented!();
    }

    fn set_clip_size(&mut self, _clip_size: Option<Vector2D>) {
        unimplemented!();
    }

    fn is_cursor_inside(&mut self, cursor_pos: Vector2D) -> bool {
        if cursor_pos.x >= self.position.x
            && cursor_pos.x <= (self.position.x + self.size.x)
            && cursor_pos.y >= self.position.y
            && cursor_pos.y <= (self.position.y + self.size.y)
        {
            true
        } else {
            false
        }
    }
}
