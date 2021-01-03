use crate::event;
use crate::event::Event;
use crate::renderer::{Message, RenderInstruction};
use crate::util::{Color, Queue, Vector2D};
use crate::widget::{Layout, Widget};

use std::cell::RefCell;
use std::rc::Weak;

#[derive(Clone)]
pub struct CheckBoxWidget {
    id: usize,
    background_color: Color,
    selected_color: Color,
    on_change: Option<Box<dyn Message>>,
    border_size: f64,
    selected_relative_size: f64,
    cursor_pos: Vector2D, //State
    is_checked: bool, //State
    dirty: bool,
    children: Vec<Weak<RefCell<dyn Widget>>>,
    position: Vector2D,
    size: Vector2D,
    original_size: Vector2D,
    layout: Layout,
    offset: Vector2D,
}

impl CheckBoxWidget {
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
            cursor_pos: Vector2D::new(-1.,-1.),
            dirty: true,
            children: Vec::<Weak<RefCell<dyn Widget>>>::new(),
            position: Vector2D::new(0., 0.),
            size: size,
            original_size: size,
            layout: Layout::None,
            offset: Vector2D::new(0., 0.),
        }
    }

    fn is_mouse_inside(&mut self) -> bool {
        if self.cursor_pos.x>=self.position().x && self.cursor_pos.x<=(self.position().x+self.size().x) && self.cursor_pos.y>=self.position().y && self.cursor_pos.y <=(self.position().y+self.size().y) {
            true
        }else{
            false
        }
        
    }

    pub fn set_message(&mut self, on_change: Option<Box<dyn Message>>){
        self.on_change = on_change;
    }

    pub fn get_is_checked(&self) -> bool {
        self.is_checked
    }
}

impl Widget for CheckBoxWidget {
    fn on_event(&mut self, event: Event, messages: &mut Queue<Box<dyn Message>>) {
        match event{
            event::Event::Mouse(event::Mouse::CursorMoved {x: x_pos, y: y_pos}) =>{
                self.cursor_pos = Vector2D::new(x_pos as f64,y_pos as f64);
                for value in self.children.iter_mut() {
                    if let Some(child) = value.upgrade() {
                        child.borrow_mut().on_event(event, messages);
                    }
                }
            },
            event::Event::Mouse(event::Mouse::ButtonPressed(event::MouseButton::Left)) => {
                if self.is_mouse_inside(){
                    if let Some(mut message) = self.on_change.clone(){
                        message.set_event(event);
                        messages.enqueue(message);
                    }
                    self.is_checked = !self.is_checked;
                    self.set_dirty(true);
                }
            },
            _ =>{
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
        if self.is_checked{
            vec![
                RenderInstruction::DrawRect{
                    point: self.position,
                    color: self.selected_color,
                    size: self.size,
                },
                RenderInstruction::DrawRect{
                    point: Vector2D::new(self.position.x+self.size.x*self.selected_relative_size, self.position.y+self.size.y*self.selected_relative_size),
                    color: self.background_color,
                    size: Vector2D::new(self.size.x-(2.*(self.size.x*self.selected_relative_size)), self.size.y-(2.*(self.size.y*self.selected_relative_size)))
                }
            ] 
        }else{
            vec![
                RenderInstruction::DrawRect{
                    point: self.position,
                    color: Color::from_hex(0xFF000000),
                    size: self.size,
                },
                RenderInstruction::DrawRect{
                    point: Vector2D::new(self.position.x+self.border_size, self.position.y+self.border_size),
                    color: self.background_color,
                    size: Vector2D::new(self.size.x-(2.*self.border_size), self.size.y-(2.*self.border_size))
                }
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

}
