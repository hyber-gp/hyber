use crate::event;
use crate::event::Event;
use crate::renderer::{Message, RenderInstruction};
use crate::util::{Color, Queue, Vector2D};
use crate::widget::{Layout, Widget};

use std::cell::RefCell;
use std::rc::Weak;

#[derive(Clone)]
pub struct TextBoxWidget {
    id: usize,
    background_color: Color,
    text_color: Color,
    border_thickness: f64,
    text: String,
    on_text_change: Option<Box<dyn Message>>,
    is_focused: bool,     //State
    cursor_pos: Vector2D, //State
    dirty: bool,
    children: Vec<Weak<RefCell<dyn Widget>>>,
    position: Vector2D,
    size: Vector2D,
    original_size: Vector2D,
    layout: Layout,
    offset: Vector2D,
}

impl TextBoxWidget {
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
