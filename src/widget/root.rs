use crate::event;
use crate::event::Event;
use crate::key_code::KeyCode;
use crate::renderer::{Message, RenderInstruction};
use crate::util::{Color, Queue, Vector2D};
use crate::widget::{Layout, Widget};

use std::cell::RefCell;
use std::rc::Weak;

#[derive(Clone)]
pub struct RootWidget {
    id: usize,
    size: Vector2D,
    original_size: Vector2D,
    background_color: Color,
    message_incremented: Box<dyn Message>,
    message_decremented: Box<dyn Message>,
    layout: Layout,
    dirty: bool,
    children: Vec<Weak<RefCell<dyn Widget>>>,
}

impl RootWidget {
    pub fn new(
        size: Vector2D,
        background_color: Color,
        layout: Layout,
        message_incremented: Box<dyn Message>,
        message_decremented: Box<dyn Message>,
    ) -> RootWidget {
        RootWidget {
            id: 0,
            size: size,
            original_size: size,
            background_color: background_color,
            layout: layout,
            message_incremented: message_incremented,
            message_decremented: message_decremented,
            dirty: true,
            children: Vec::<Weak<RefCell<dyn Widget>>>::new(),
        }
    }
}
impl Widget for RootWidget {
    fn on_event(&mut self, event: Event, messages: &mut Queue<Box<dyn Message>>) {
        match event {
            event::Event::Keyboard(event::Keyboard::KeyPressed {
                key_code: KeyCode::I,
                modifiers:
                    event::ModifiersState {
                        shift: false,
                        control: false,
                        alt: false,
                        logo: false,
                    },
            }) => {
                let mut message = self.message_incremented.clone();
                message.set_event(event);
                messages.enqueue(message);
            }
            event::Event::Keyboard(event::Keyboard::KeyPressed {
                key_code: KeyCode::D,
                modifiers:
                    event::ModifiersState {
                        shift: false,
                        control: false,
                        alt: false,
                        logo: false,
                    },
            }) => {
                let mut message = self.message_decremented.clone();
                message.set_event(event);
                messages.enqueue(message);
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
        // TODO: Debater se isto deve ser usado como clear do ecrã.
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
}
