use crate::event::Event;
use crate::renderer::{Message, RenderInstruction};
use crate::util::{Queue, Vector2D};
use crate::widget::{Axis, Layout, Widget};

use std::cell::RefCell;
use std::rc::Weak;

#[derive(Clone)]
pub struct ListViewWidget {
    id: usize,
    size: Vector2D,
    original_size: Vector2D,
    layout: Layout,
    dirty: bool,
    children: Vec<Weak<RefCell<dyn Widget>>>,
}

impl ListViewWidget {
    pub fn new(size: Vector2D, axis: Axis) -> ListViewWidget {
        ListViewWidget {
            id: 0,
            size: size,
            original_size: size,
            layout: Layout::Box(axis),
            dirty: true,
            children: Vec::<Weak<RefCell<dyn Widget>>>::new(),
        }
    }
}

impl Widget for ListViewWidget {
    fn on_event(&mut self, event: Event, messages: &mut Queue<Box<dyn Message>>) {
        for value in self.children.iter_mut() {
            if let Some(child) = value.upgrade() {
                child.borrow_mut().on_event(event, messages);
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
        // TODO: Debater se isto deve ser usado como clear do ecrã.
        vec![]
    }

    fn set_dirty(&mut self, value: bool) {
        if value {
            self.dirty = true;
            for value in self.get_children() {
                if let Some(child) = value.upgrade() {
                    if child.borrow_mut().is_dirty() {
                        break;
                    }
                    else {
                        child.borrow_mut().set_dirty(true);
                    }
                }
            }
        } else {self.dirty = false;}
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
        self.set_dirty(true);
        self.size = size;
    }

    fn set_original_size(&mut self, size: Vector2D) {
        self.set_dirty(true);
        self.original_size = size;
    }

    fn set_offset(&mut self, _offset: Vector2D) {}
}
