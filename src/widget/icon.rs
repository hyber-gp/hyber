use crate::event::Event;
use crate::renderer::{DrawImageOptions, Message, RenderInstruction};
use crate::util::{Color, Queue, Vector2D};
use crate::widget::{Layout, Widget};

use std::cell::RefCell;
use std::rc::Weak;

#[derive(Clone)]
pub struct IconWidget {
    id: usize,
    path: String,
    options: DrawImageOptions,
    background_color: Color,
    dirty: bool,
    children: Vec<Weak<RefCell<dyn Widget>>>,
    position: Vector2D,
    size: Vector2D,
    original_size: Vector2D,
    layout: Layout,
    offset: Vector2D,
}

impl IconWidget {
    pub fn new(
        path: String,
        size: Vector2D,
        options: DrawImageOptions,
        background_color: Color,
    ) -> IconWidget {
        IconWidget {
            id: 0,
            path: path,
            options: options,
            background_color: background_color,
            dirty: true,
            children: Vec::<Weak<RefCell<dyn Widget>>>::new(),
            position: Vector2D::new(0., 0.),
            size: size,
            original_size: size,
            layout: Layout::None,
            offset: Vector2D::new(0., 0.),
        }
    }
}

impl Widget for IconWidget {
    fn on_event(&mut self, event: Event, messages: &mut Queue<Box<dyn Message>>) {}

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn id(&self) -> usize {
        self.id
    }

    fn recipe(&self) -> Vec<RenderInstruction> {
        vec![
            // Icon rectangle.
            RenderInstruction::DrawRect {
                point: self.position,
                color: self.background_color.clone(),
                size: self.size,
                clip_point: self.position,
                clip_size: self.size,
            },
            // Icon Image
            RenderInstruction::DrawImage {
                point: self.position, // todo: CHANGE after testing
                path: self.path.clone(),
                options: self.options.clone(),
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
}
