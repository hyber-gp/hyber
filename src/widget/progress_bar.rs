use crate::event;
use crate::event::Event;
use crate::key_code::KeyCode;
use crate::renderer::{Message, RenderInstruction};
use crate::util::{Color, Queue, Vector2D};
use crate::widget::{Layout, Widget};

use std::cell::RefCell;
use std::rc::Weak;

#[derive(Clone)]
pub struct ProgressBarWidget {
    id: usize,
    progress: f64,
    font_size: usize,
    background_color: Color,
    foreground_color: Color,
    dirty: bool,
    children: Vec<Weak<RefCell<dyn Widget>>>,
    position: Vector2D,
    size: Vector2D,
    original_size: Vector2D,
    layout: Layout,
    offset: Vector2D,
}

impl ProgressBarWidget {
    pub fn new(
        size: Vector2D,
        font_size: usize,
        progress: f64,
        background_color: Color,
        foreground_color: Color,
    ) -> ProgressBarWidget {
        ProgressBarWidget {
            id: 0,
            font_size: font_size,
            progress: progress,
            background_color: background_color,
            foreground_color: foreground_color,
            dirty: true,
            children: Vec::<Weak<RefCell<dyn Widget>>>::new(),
            position: Vector2D::new(0., 0.),
            size: size,
            original_size: size,
            layout: Layout::None,
            offset: Vector2D::new(0., 0.),
        }
    }

    pub fn set_progress(&mut self, value: f64)
    {
        self.progress = value;
        self.dirty = true;
    }
}

impl Widget for ProgressBarWidget {
    fn on_event(&mut self, event: Event, messages: &mut Queue<Box<dyn Message>>) {}

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn id(&self) -> usize {
        self.id
    }

    fn recipe(&self) -> Vec<RenderInstruction> {
        let progress_perc = Vector2D::new(self.original_size.x * (self.progress/100.0),self.original_size.y);
        
        vec![
            // Progress bar rectangle.
            RenderInstruction::DrawRect {
                point: self.position,
                color: self.background_color.clone(),
                size: self.original_size,
                clip_point: self.position,
                clip_size: self.size,
            },
            // Background progress bar rectangle.
            RenderInstruction::DrawRect {
                point: self.position,
                color: self.foreground_color.clone(),
                size: progress_perc,
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

    fn is_cursor_inside(&mut self, _cursor_pos: Vector2D) -> bool {
        false
    }
}
