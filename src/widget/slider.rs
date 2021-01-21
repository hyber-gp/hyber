use crate::event;
use crate::event::Event;
use crate::renderer::{Message, RenderInstruction};
use crate::util::{Color, Queue, Vector2D};
use crate::widget::{Layout, Widget};

use std::cell::RefCell;
use std::rc::Weak;

/// Current slider position
#[derive(Clone)]
pub struct Position {
    /// The current value of the slider
    pub slider_value: i32,
    /// The current x-coordinate position of the slider
    pub x_coordinate: f64,
}

/// Slider is a component that lets the user graphically select a value 
/// by sliding a button within a bounded interval. The button 
/// is always positioned at the points that match integer values
/// within the specified interval.
#[derive(Clone)]
pub struct SliderWidget {
    /// The slider's identifier
    id: usize,
    
    /// The slider background color
    background_color: Color,
    
    /// The slider button color
    button_color: Color,
    
    /// The slider button size
    button_size: Vector2D,
    
    /// The slider's range (minimum and maximum)
    range: (i32, i32),
    
    /// The slider's step
    step: i32,
    
    /// The current value of slider step
    slider_value: i32,
    
    /// The message to be handled when a user slide the slider button
    on_slide: Option<Box<dyn Message>>,
    
    /// The possible positions for the slider button
    slider_positions: Vec<Position>,
    
    /// Whether the slider is pressed
    is_pressed: bool,
    
    /// The cursor's position
    cursor_pos: Vector2D,
    
    /// The current slider's index on the `slider_positions`
    slider_index: usize,
    
    /// The dirty flag (i.e., flag used to mark the widgets needed to be rebuilt)
    dirty: bool,
    
    /// The slider's children (i.e., his widgets tree)
    children: Vec<Weak<RefCell<dyn Widget>>>,
    
    /// The slider's position, on a two-dimensional space (x-coordinate and y-coordinate) 
    /// relative to the top left corner
    position: Vector2D,
    
    /// The slider's current size (width and height)
    size: Vector2D,
    
    /// The slider's original size (width and height)
    original_size: Vector2D,
    
    /// The slider's layout
    layout: Layout,
    
    /// The slider's offset vector coordinates
    offset: Vector2D,
}

impl SliderWidget {
    /// Creates a new `SliderWidget`
    ///
    /// # Returns
    /// The slider created
    ///
    /// # Arguments
    /// * `size` - the size (width and height) to be assigned to the slider
    /// * `background_color` - the color to be assigned to the slider's background
    /// * `button_color` - the color to be assigned to the slider button
    /// * `button_size` - the size to be assigned to the slider button
    /// * `range` - the range to be assigned to the slider
    /// * `step` - the step to be assigned to the slider
    /// * `slider_value` - the initial value to be assigned to the slider
    /// * `on_slide` - the message to be handled when the user slides the slider button 
    pub fn new(
        size: Vector2D,
        background_color: Color,
        button_color: Color,
        button_size: Vector2D,
        range: (i32, i32),
        step: i32,
        slider_value: i32,
        on_slide: Option<Box<dyn Message>>,
    ) -> SliderWidget {
        let slider_positions =
            SliderWidget::get_slider_positions(range.0, range.1, step, Vector2D::new(0., 0.), size);
        SliderWidget {
            id: 0,
            background_color: background_color,
            button_color: button_color,
            button_size: button_size,
            slider_value: slider_value,
            range: range,
            step: step,
            on_slide: on_slide,
            slider_index: SliderWidget::get_slider_index(slider_value, &slider_positions),
            slider_positions: slider_positions,
            is_pressed: false,
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

    /// Sets the message to be handled when the user slides the slider button
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `on_slide` - the message to be handled when the user slides the slider button
    pub fn set_message(&mut self, on_slide: Option<Box<dyn Message>>) {
        self.on_slide = on_slide;
    }

    /// Gets the current slider value
    ///
    /// # Returns
    /// The current slider value
    ///
    /// # Arguments
    /// No arguments
    pub fn get_slider_value(&self) -> i32 {
        self.slider_value
    }

    /// Gets all the possible slider positions for a given configuration
    ///
    /// # Returns
    /// A vector with all the possible slider positions for the given configuration
    ///
    /// # Arguments
    /// * `start` - the minimum range value to be considered
    /// * `end` - the maximum range value to be considered
    /// * `step` - the step to be considered
    /// * `position` - the current slider's position
    /// * `size` - the current slider's size
    fn get_slider_positions(
        start: i32,
        end: i32,
        step: i32,
        position: Vector2D,
        size: Vector2D,
    ) -> Vec<Position> {
        let limit = end - start;
        let mut slider_positions: Vec<Position> = Vec::new();
        let step_size = (step as f64 * size.x) / limit as f64;
        let mut incremental_size = position.x;
        for i in (start..end + 1).step_by(step as usize) {
            slider_positions.push(Position {
                slider_value: i,
                x_coordinate: incremental_size,
            });
            incremental_size = incremental_size + step_size;
        }
        slider_positions
    }

    /// Gets the slider's index based on a value and a vector with all his possible positions
    ///
    /// # Returns
    /// The slider's index within the `vector`
    ///
    /// # Arguments
    /// * `value` - the value to be considered
    /// * `vector` - a vector with all slider's possible positions
    fn get_slider_index(value: i32, vector: &Vec<Position>) -> usize {
        if let Ok(result) = vector.binary_search_by_key(
            &value,
            |&Position {
                 slider_value,
                 x_coordinate,
             }| slider_value,
        ) {
            return result;
        }
        0
    }
}

impl Widget for SliderWidget {
    fn on_event(&mut self, event: Event, messages: &mut Queue<Box<dyn Message>>) {
        match event {
            event::Event::Mouse(event::Mouse::CursorMoved { x: x_pos, y: y_pos }) => {
                self.cursor_pos = Vector2D::new(x_pos as f64, y_pos as f64);
                if self.is_pressed {
                    if self.cursor_pos.x > self.position.x + self.size.x {
                        self.cursor_pos.x = self.position.x + self.size.x
                    } else if self.cursor_pos.x < self.position.x {
                        self.cursor_pos.x = self.position.x;
                    }
                    self.set_dirty(true);
                } else {
                    for value in self.children.iter_mut() {
                        if let Some(child) = value.upgrade() {
                            child.borrow_mut().on_event(event, messages);
                        }
                    }
                }
            }
            event::Event::Mouse(event::Mouse::ButtonPressed(event::MouseButton::Left)) => {
                if self.is_cursor_inside(self.cursor_pos) {
                    self.is_pressed = true;
                }
            }
            event::Event::Mouse(event::Mouse::ButtonReleased(event::MouseButton::Left)) => {
                if self.is_pressed {
                    let half_step_size = (self.slider_positions[1].x_coordinate
                        - self.slider_positions[0].x_coordinate)
                        * 0.5;
                    if self.cursor_pos.x
                        > self.slider_positions[self.slider_index].x_coordinate + half_step_size
                    {
                        if self.slider_index != self.slider_positions.len() - 1 {
                            self.slider_index = self.slider_index + 1;
                            while self.slider_positions[self.slider_index].x_coordinate
                                < self.cursor_pos.x
                            {
                                self.slider_index = self.slider_index + 1;
                            }
                            self.slider_value =
                                self.slider_positions[self.slider_index].slider_value;
                            if let Some(mut message) = self.on_slide.clone() {
                                message.set_event(event);
                                messages.enqueue(message);
                            }
                        }
                    } else if self.cursor_pos.x
                        < self.slider_positions[self.slider_index].x_coordinate - half_step_size
                    {
                        if self.slider_index != 0 {
                            self.slider_index = self.slider_index - 1;
                            while self.slider_positions[self.slider_index].x_coordinate
                                > self.cursor_pos.x
                            {
                                self.slider_index = self.slider_index - 1;
                            }
                            self.slider_value =
                                self.slider_positions[self.slider_index].slider_value;
                            if let Some(mut message) = self.on_slide.clone() {
                                message.set_event(event);
                                messages.enqueue(message);
                            }
                        }
                    }
                    self.set_dirty(true);
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
        if self.is_pressed {
            vec![
                RenderInstruction::DrawRect {
                    point: self.position,
                    color: self.background_color,
                    size: self.size,
                    clip_point: self.position,
                    clip_size: self.size,
                },
                RenderInstruction::DrawRect {
                    point: Vector2D::new(
                        self.cursor_pos.x - (self.button_size.x * 0.5),
                        self.position.y + (self.size.y * 0.5) - (self.button_size.y * 0.5),
                    ),
                    color: self.button_color,
                    size: self.button_size,
                    clip_point: self.position,
                    clip_size: self.size,
                },
            ]
        } else {
            vec![
                RenderInstruction::DrawRect {
                    point: self.position,
                    color: self.background_color,
                    size: self.size,
                    clip_point: self.position,
                    clip_size: self.size,
                },
                RenderInstruction::DrawRect {
                    point: Vector2D::new(
                        self.slider_positions[self.slider_index].x_coordinate
                            - (self.button_size.x * 0.5),
                        self.position.y + (self.size.y * 0.5) - (self.button_size.y * 0.5),
                    ),
                    color: self.button_color,
                    size: self.button_size,
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
        self.slider_positions = SliderWidget::get_slider_positions(
            self.range.0,
            self.range.1,
            self.step,
            self.position(),
            size,
        );
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
        let button_upper_left_corner_x =
            self.slider_positions[self.slider_index].x_coordinate - (self.button_size.x * 0.5);
        let button_upper_left_corner_y =
            self.position.y + (self.size.y * 0.5) - (self.button_size.y * 0.5);
        if cursor_pos.x >= button_upper_left_corner_x
            && cursor_pos.x <= (button_upper_left_corner_x + self.button_size.x)
            && cursor_pos.y >= button_upper_left_corner_y
            && cursor_pos.y <= (button_upper_left_corner_y + self.button_size.y)
        {
            true
        } else {
            false
        }
    }
}
