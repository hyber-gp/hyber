use crate::event;
use crate::event::Event;
use crate::renderer::{Message, RenderInstruction};
use crate::util::{Color, Queue, Vector2D};
use crate::widget::{Layout, Widget};

use std::cell::RefCell;
use std::rc::Weak;
use std::time::Instant;

/// Time required to press be considered long
const ON_LONG_PRESS_TIME: u128 = 300;

/// Tab is a component that lets the user switch between a group
/// of components by clicking on a tab with a given title.
#[derive(Clone)]
pub struct TabWidget {
    /// The tab's identifier
    id: usize,
    
    /// The tab's background color
    background_color: Color,
    
    /// The dirty flag (i.e., flag used to mark the widgets needed to be rebuilt)
    dirty: bool,
    
    /// The tab's children (i.e., his widgets tree)
    children: Vec<Weak<RefCell<dyn Widget>>>,
    
    /// The tab's position, on a two-dimensional space (x-coordinate and y-coordinate) 
    /// relative to the top left corner
    position: Vector2D,
    
    /// The tab's current size (width and height)
    size: Vector2D,
    
    /// The tab's original size (width and height)
    original_size: Vector2D,
    
    /// The tab's layout
    layout: Layout,

    /// The tab's offset vector coordinates
    offset: Vector2D,
    
    /// The message to be handled when a user press 
    on_press: Option<Box<dyn Message>>,
    
    /// The message to be handled when a user long press (i.e., a user drag some tab)
    tab_moved: Option<Box<dyn Message>>,
    
    /// Wheter the tab is pressed
    is_pressed: bool,
    
    /// The instant when the tab was clicked
    click_time: Instant,
    
    /// The current cursor's position
    cursor_pos: Vector2D,
    
    /// The cursor's position where the mouse button was released after a long press (i.e., drag)
    moved_cursor_pos: Vector2D,
}
impl TabWidget {
    /// Creates a new `TabWidget`
    ///
    /// # Returns
    /// The tab created
    ///
    /// # Arguments
    /// * `size` - the size (width and height) to be assigned to the tab
    /// * `background_color` - the color to be assigned to the tab's background
    /// * `on_press` - the message to be handled when the tab is pressed
    /// * `tab_moved` - the message to be handled when the tab is moved/dragged (long pressed)
    /// or held for at least `ON_LONG_PRESS_TIME`
    pub fn new(
        size: Vector2D,
        background_color: Color,
        on_press: Option<Box<dyn Message>>,
        tab_moved: Option<Box<dyn Message>>,
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
            tab_moved: tab_moved,
            is_pressed: false,
            click_time: Instant::now(),
            cursor_pos: Vector2D::new(-1., -1.),
            moved_cursor_pos: Vector2D::new(-1., -1.),
        }
    }

    /// Sets the message to be handled when the tab is 
    /// held for at least the `ON_LONG_PRESS_TIME`
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `new_message` - the new message to be handled when the tab is 
    /// held for at least the `ON_LONG_PRESS_TIME`
    pub fn set_new_message_move(&mut self, new_message: Option<Box<dyn Message>>) {
        self.tab_moved = new_message;
    }

    /// Gets the cursor's position where the mouse button was
    /// released after a long press (i.e., drag)
    ///
    /// # Returns
    /// The cursor's position at the end of the last long press (i.e., drag)
    ///
    /// # Arguments
    /// No arguments
    pub fn get_moved_cursor_pos(&mut self) -> Vector2D {
        self.moved_cursor_pos
    }
}

impl Widget for TabWidget {
    fn on_event(&mut self, event: Event, messages: &mut Queue<Box<dyn Message>>) {
        match event {
            event::Event::Mouse(event::Mouse::CursorMoved {
                x: x_mouse,
                y: y_mouse,
            }) => {
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
                if self.is_cursor_inside(self.cursor_pos) {
                    self.is_pressed = true;
                    self.click_time = Instant::now();
                }
            }
            event::Event::Mouse(event::Mouse::ButtonReleased(event::MouseButton::Left)) => {
                if self.is_pressed {
                    if self.is_cursor_inside(self.cursor_pos) {
                        //Tab pressed
                        if self.click_time.elapsed().as_millis() < ON_LONG_PRESS_TIME {
                            if let Some(mut message) = self.on_press.clone() {
                                message.set_event(event);
                                messages.enqueue(message);
                            }
                        }
                    }
                    //TAB MOVED
                    if self.click_time.elapsed().as_millis() > ON_LONG_PRESS_TIME {
                        self.moved_cursor_pos.x = self.cursor_pos.x;
                        self.moved_cursor_pos.y = self.cursor_pos.y;
                        if let Some(mut message) = self.tab_moved.clone() {
                            message.set_event(event);
                            messages.enqueue(message);
                        }
                    }
                    self.is_pressed = false;
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
                clip_point: self.position,
                clip_size: self.size,
            },
        ]
    }

    fn set_dirty(&mut self, value: bool) {
        if value {
            self.dirty = true;
            for value in self.get_children() {
                if let Some(child) = value.upgrade() {
                    if child.borrow_mut().is_dirty() {
                        break;
                    } else {
                        child.borrow_mut().set_dirty(true);
                    }
                }
            }
        } else {
            self.dirty = false;
        }
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
