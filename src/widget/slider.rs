use crate::event;
use crate::event::Event;
use crate::renderer::{Message, RenderInstruction};
use crate::util::{Color, Queue, Vector2D};
use crate::widget::{Layout, Widget};

use std::cell::RefCell;
use std::rc::Weak;

#[derive(Clone)]
pub struct Position {
    pub slider_value: i32,
    pub x_coordinate: f64,
}

#[derive(Clone)]
pub struct SliderWidget {
    id: usize,
    background_color: Color,
    button_color: Color,
    button_size: Vector2D,
    range: (i32, i32),
    step: i32,
    slider_value: i32,
    on_slide: Option<Box<dyn Message>>,
    slider_positions: Vec<Position>,
    is_pressed: bool,     //State
    cursor_pos: Vector2D, //State
    slider_index: usize,  //State
    dirty: bool,
    children: Vec<Weak<RefCell<dyn Widget>>>,
    position: Vector2D,
    size: Vector2D,
    original_size: Vector2D,
    layout: Layout,
    offset: Vector2D,
}

impl SliderWidget {
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

    pub fn set_message(&mut self, on_slide: Option<Box<dyn Message>>) {
        self.on_slide = on_slide;
    }

    pub fn get_slider_value(&self) -> i32 {
        self.slider_value
    }

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
