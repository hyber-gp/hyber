use crate::event::Event;
use crate::renderer::{DrawImageOptions, Message, RenderInstruction};
use crate::util::{Color, Queue, Vector2D};
use crate::widget::{Layout, Widget};

use std::cell::RefCell;
use std::rc::Weak;

/// Icon is a widget that provides the ability to display an image,
/// a fixed-sized picture.
#[derive(Clone)]
pub struct IconWidget {
    /// The icon's identifier
    id: usize,

    /// The icon's picture absolute path
    path: String,

    /// The icon's draw settings
    options: DrawImageOptions,

    /// The icon's brackground color
    /// TODO: There is an issue regarding the background colour.
    /// When the bg colour contains red, at least when using
    /// the library raqote to render the icon widget,
    /// the system panics with an overflown exception.
    background_color: Color,

    /// The dirty flag (i.e., flag used to mark the widgets needed to be rebuilt)
    dirty: bool,

    /// The icon's children (i.e., his widgets tree)
    children: Vec<Weak<RefCell<dyn Widget>>>,

    /// The icon's position, on a two-dimensional space (x-coordinate and y-coordinate)
    /// relative to the top left corner
    position: Vector2D,

    /// The icon's current size (width and height)
    size: Vector2D,

    /// The icon's original size (width and height)
    original_size: Vector2D,

    /// The icon's layout
    layout: Layout,

    /// The icon's offset vector coordinates
    offset: Vector2D,
}

impl IconWidget {
    /// Creates a new `IconWidget`
    ///
    /// # Returns
    /// The icon created
    ///
    /// # Arguments
    /// * `path` - the absolute path of the picture to be assigned to the icon
    /// * `size` - the size (width and height) to be assigned to the icon
    /// * `options` - the draw settings to be used when drawing the icon
    /// * `background_color` - the color to be assigned to the icon's background
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
    fn on_event(&mut self, _event: Event, _messages: &mut Queue<Box<dyn Message>>) {}

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

    fn set_clip_point(&mut self, _clip_point: Option<Vector2D>) {
        unimplemented!();
    }

    fn set_clip_size(&mut self, _clip_size: Option<Vector2D>) {
        unimplemented!();
    }

    fn is_cursor_inside(&mut self, _cursor_pos : Vector2D) -> bool {
        false
    }
}
