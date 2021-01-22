use crate::event;
use crate::event::Event;
use crate::renderer::{Message, RenderInstruction};
use crate::util::{Color, Queue, Vector2D};
use crate::widget::{Layout, Widget};

use std::cell::RefCell;
use std::rc::Weak;
use std::time::Instant;

/// Minimum time to be considered a long press
const ON_LONG_PRESS_TIME: u128 = 500;

/// Button is a widget that reacts to touches.
/// It can be used as a container to allow child to be clickable,
/// for example, an icon or a lable
#[derive(Clone)]
pub struct ButtonViewWidget {
    /// The button's identifier
    id: usize,

    /// Whether the button is clickable
    is_clickable: bool,

    /// The button's background color
    background_color: Color,

    /// The message to be handled when a user press
    on_press: Option<Box<dyn Message>>,
    
    /// The message to be handled when a user long press
    on_long_press: Option<Box<dyn Message>>,

    /// Whether the button state is pressed
    is_pressed: bool,

    /// The instant when the button was clicked
    click_time: Instant,

    /// The cursor's position
    cursor_pos: Vector2D,

    /// The dirty flag (i.e., flag used to mark the widgets needed to be rebuilt)
    dirty: bool,

    /// The button's children (i.e., his widgets tree)
    children: Vec<Weak<RefCell<dyn Widget>>>,

    /// The button's position, on a two-dimensional space (x-coordinate and y-coordinate) 
    /// relative to the top left corner
    position: Vector2D,

    /// The button's current size (width and height)
    size: Vector2D,

    /// The button's original size (width and height)
    original_size: Vector2D,

    /// The button's layout
    layout: Layout,

    /// The button's offset vector coordinates
    offset: Vector2D,
}

impl ButtonViewWidget {
    /// Creates a new `ButtonViewWidget`
    ///
    /// # Returns
    /// The button created
    ///
    /// # Arguments
    /// * `size` - the size (width and height) to be assigned to the button
    /// * `is_clickable` - boolean indicating if button is clickable
    /// * `background_color` - the color to be assigned to the button's background
    /// * `on_press` - the message to be handled when the button is pressed
    /// * `on_long_press` - the message to be handled when the button is pressed 
    /// and held for at least the `ON_LONG_PRESS_TIME`
    pub fn new(
        size: Vector2D,
        is_clickable: bool,
        background_color: Color,
        on_press: Option<Box<dyn Message>>,
        on_long_press: Option<Box<dyn Message>>,
    ) -> ButtonViewWidget {
        ButtonViewWidget {
            id: 0,
            background_color: background_color,
            is_clickable: is_clickable,
            on_press: on_press,
            on_long_press: on_long_press,
            is_pressed: false,
            click_time: Instant::now(),
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

    /// Sets button's clickable attribute
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `value` - boolean indicating if button is clickable
    pub fn set_is_clickable(&mut self, value: bool) {
        self.is_clickable = value;
    }
}

impl Widget for ButtonViewWidget {
    fn on_event(&mut self, event: Event, messages: &mut Queue<Box<dyn Message>>) {
        match event {
            event::Event::Mouse(event::Mouse::CursorMoved { x: x_pos, y: y_pos }) => {
                //update cursor_pos on mouse move
                self.cursor_pos = Vector2D::new(x_pos as f64, y_pos as f64);
                for value in self.children.iter_mut() {
                    if let Some(child) = value.upgrade() {
                        child.borrow_mut().on_event(event, messages);
                    }
                }
            }
            event::Event::Mouse(event::Mouse::ButtonPressed(event::MouseButton::Left)) => {
                //when left mouse button is pressed do something if button is clickable and if messages aren't empty
                if self.is_clickable && (self.on_press.is_some() || self.on_long_press.is_some()) {
                    //check if cursor is inside button area
                    if self.is_cursor_inside(self.cursor_pos) {
                        self.is_pressed = true;
                        self.click_time = Instant::now();
                    }
                }
            }
            event::Event::Mouse(event::Mouse::ButtonReleased(event::MouseButton::Left)) => {
                //when left mouse button is released do something if button state is pressed
                if self.is_pressed {
                    self.is_pressed = false;
                    //check if cursor is inside button area
                    //if the release it's outside we don't consider it as a click event
                    if self.is_cursor_inside(self.cursor_pos) {
                        if self.click_time.elapsed().as_millis() < ON_LONG_PRESS_TIME {
                            if let Some(mut message) = self.on_press.clone() {
                                message.set_event(event);
                                messages.enqueue(message);
                            }
                        } else {
                            if let Some(mut message) = self.on_long_press.clone() {
                                message.set_event(event);
                                messages.enqueue(message);
                            }
                        }
                    }
                }
            }
            _ => {
                //call on_event to button children
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
        vec![]
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
