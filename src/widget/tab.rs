use crate::event;
use crate::event::Event;
use crate::renderer::{DrawImageOptions, Message, RenderInstruction};
use crate::util::{Color, Queue, Vector2D};
use crate::widget::{Layout, Widget};

use std::cell::RefCell;
use std::rc::Weak;
use std::time::Instant;

const ON_LONG_PRESS_TIME: u128 = 500;

#[derive(Clone)]
pub struct TabWidget {
    id: usize,
    background_color: Color,
    dirty: bool,
    children: Vec<Weak<RefCell<dyn Widget>>>,
    position: Vector2D,
    size: Vector2D,
    original_size: Vector2D,
    layout: Layout,
    offset: Vector2D,
    on_press: Option<Box<dyn Message>>,
    tab_moved_left: Option<Box<dyn Message>>,
    tab_moved_right: Option<Box<dyn Message>>,
    is_pressed: bool, 
    click_time: Instant, 
    cursor_pos: Vector2D, 
    moved_cursor_pos: Vector2D,
}
impl TabWidget {
    pub fn new(
        size: Vector2D,
        background_color: Color,
        on_press: Option<Box<dyn Message>>,
        tab_moved_left: Option<Box<dyn Message>>,
        tab_moved_right: Option<Box<dyn Message>>,
    ) -> TabWidget {
        TabWidget {
            id: 0,
            background_color: background_color,
            dirty: true,
            children: Vec::<Weak<RefCell<dyn Widget>>>::new(),
            position: Vector2D::new(0., 0.),
            size: size,
            original_size: size,
            layout: Layout::None,
            offset: Vector2D::new(0., 0.),
            on_press: on_press,
            tab_moved_left: tab_moved_left,
            tab_moved_right:tab_moved_right,
            is_pressed: false,
            click_time: Instant::now(),
            cursor_pos: Vector2D::new(-1.,-1.),
            moved_cursor_pos: Vector2D::new(-1.,-1.),
        }
    }
    fn is_mouse_inside(&mut self) -> bool {
        if (self.position.x + self.size.x) >= self.cursor_pos.x && (self.position.y + self.size.y) >= self.cursor_pos.y && self.position.x <=self.cursor_pos.x && self.position.y <=self.cursor_pos.y  {
            true
        } else{
            false
        }
       
    }
    pub fn setNewMessageMoveLeft(&mut self, newMessage:Option<Box<dyn Message>>){
        self.tab_moved_left = newMessage;
    }
    pub fn setNewMessageMoveRight(&mut self, newMessage:Option<Box<dyn Message>>){
        self.tab_moved_right = newMessage;
    }

    pub fn get_moved_cursor_pos(&mut self) -> Vector2D{
        self.moved_cursor_pos
    }
}

impl Widget for TabWidget {
    fn on_event(&mut self, event: Event, messages: &mut Queue<Box<dyn Message>>) {
        match event {
            event::Event::Mouse(event::Mouse::CursorMoved { x: x_mouse, y: y_mouse }) => {
                self.cursor_pos.x = x_mouse as f64;
                self.cursor_pos.y = y_mouse as f64;
                for value in self.children.iter_mut() {
                    if let Some(child) = value.upgrade() {
                        child.borrow_mut().on_event(event, messages);
                    }
                }
            }
            event::Event::Mouse(event::Mouse::ButtonPressed(event::MouseButton::Left)) => {
                //CHECK IF INSIDE THE TAB
                if self.is_mouse_inside(){
                    self.is_pressed= true;
                    self.click_time  = Instant::now();
                }
            }
            event::Event::Mouse(event::Mouse::ButtonReleased(event::MouseButton::Left)) => {
                if self.is_pressed{
                    if self.is_mouse_inside(){
                        //Tab pressed
                        if self.click_time.elapsed().as_millis() < ON_LONG_PRESS_TIME {
                            if let Some(mut message) = self.on_press.clone() {
                                message.set_event(event);
                                messages.enqueue(message);
                                println!("Press");
                            }
                        }    
                    }
                    //TAB MOVED
                    if self.click_time.elapsed().as_millis() > ON_LONG_PRESS_TIME{
                        self.moved_cursor_pos.x = self.cursor_pos.x;
                        self.moved_cursor_pos.y = self.cursor_pos.y;
                        //Check if moved to the left or to the right
                        if self.moved_cursor_pos.x < self.position().x{
                            if let Some(mut message) = self.tab_moved_left.clone() {
                                message.set_event(event);
                                messages.enqueue(message);
                                println!("Tab Moved left!");
                            }
                        }else{
                            if let Some(mut message) = self.tab_moved_right.clone() {
                                message.set_event(event);
                                messages.enqueue(message);
                                println!("Tab Moved Right!");
                            }
                        }
                    }  
                    self.is_pressed= false;
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
                // Tab rectangle
                RenderInstruction::DrawRect {
                    point: self.position,
                    color: self.background_color.clone(),
                    size: self.size,
                }
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


}
