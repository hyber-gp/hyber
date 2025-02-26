use crate::event::Event;
use crate::renderer::{Message, RenderInstruction};
use crate::util::{Color, Queue, Vector2D};
use crate::widget::{Layout, Widget};

use std::cell::RefCell;
use std::rc::Weak;

/// 
#[derive(Clone)]
pub struct RootWidget {
    /// The list's identifier
    id: usize,
    
    /// The list's identifier
    size: Vector2D,
    
    /// The list's identifier
    original_size: Vector2D,
    
    /// The list's identifier
    background_color: Color,

    /// The list's identifier
    layout: Layout,
    
    /// The list's identifier
    dirty: bool,
    
    /// The list's identifier
    children: Vec<Weak<RefCell<dyn Widget>>>,
}

impl RootWidget {
    /// Creates a new `RootWidget`
    ///
    /// # Returns
    /// The root created
    ///
    /// # Arguments
    /// * `size` - the size (width and height) to be assigned to the root
    /// * `background_color` - the color to be assigned to the root's background
    /// * `layout` - the layout type to be assigned to the root
    /// * `message_incremented` - Not documented, check Drive.
    /// * `message_decremented` - Not documented, check Drive.
    pub fn new(
        size: Vector2D,
        background_color: Color,
        layout: Layout,
    ) -> RootWidget {
        RootWidget {
            id: 0,
            size: size,
            original_size: size,
            background_color: background_color,
            layout: layout,
            dirty: true,
            children: Vec::<Weak<RefCell<dyn Widget>>>::new(),
        }
    }
}

impl Widget for RootWidget {
    fn on_event(&mut self, event: Event, messages: &mut Queue<Box<dyn Message>>) {
        match event {
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
        vec![RenderInstruction::Clear {
            color: self.background_color,
        }]
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
        Vector2D::new(0., 0.)
    }

    fn size(&mut self) -> Vector2D {
        self.size
    }

    fn original_size(&mut self) -> Vector2D {
        self.original_size
    }

    fn layout(&mut self) -> &Layout {
        // TODO: Ver se faz sentido ser só vertical
        &self.layout
    }

    fn offset(&mut self) -> Vector2D {
        Vector2D::new(0., 0.)
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
            Vector2D::new(0., 0.),
            self.size,
            self.original_size,
            &self.layout,
            Vector2D::new(0., 0.),
        )
    }

    fn set_position(&mut self, _position: Vector2D) {}

    fn set_size(&mut self, size: Vector2D) {
        self.dirty = true;
        self.size = size;
    }

    fn set_original_size(&mut self, size: Vector2D) {
        self.dirty = true;
        self.original_size = size;
    }

    fn set_offset(&mut self, _offset: Vector2D) {}

    fn set_clip_point(&mut self, _clip_point: Option<Vector2D>) {}

    fn set_clip_size(&mut self, _clip_size: Option<Vector2D>) {}

    fn is_cursor_inside(&mut self, _cursor_pos: Vector2D) -> bool {
        false
    }
}
