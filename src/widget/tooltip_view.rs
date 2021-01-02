use crate::event;
use crate::event::Event;
use crate::renderer::{
    AbsoluteWidgetCollection, Message, RenderInstruction, RenderInstructionCollection,
};
use crate::util::{Queue, Vector2D};
use crate::widget::{Axis, Layout, Widget};

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Clone)]
pub struct TooltipViewWidget {
    id: usize,
    size: Vector2D,
    original_size: Vector2D,
    layout: Layout,
    cursor_pos: Vector2D, // State
    dirty: bool,
    children: Vec<Weak<RefCell<dyn Widget>>>,
    render_instruction_collection_ptr: Weak<RefCell<RenderInstructionCollection>>,
    absolute_widget_collection_ptr: Weak<RefCell<AbsoluteWidgetCollection>>,
    tooltip_widget: Weak<RefCell<dyn Widget>>,
}

impl TooltipViewWidget {
    pub fn new(
        size: Vector2D,
        render_instruction_collection_ptr: Weak<RefCell<RenderInstructionCollection>>,
        absolute_widget_collection_ptr: Weak<RefCell<AbsoluteWidgetCollection>>,
        tooltip_widget: Weak<RefCell<dyn Widget>>,
    ) -> TooltipViewWidget {
        TooltipViewWidget {
            id: 0,
            size: size,
            original_size: size,
            cursor_pos: Vector2D::new(-1., -1.),
            layout: Layout::Box(Axis::Vertical),
            dirty: true,
            children: Vec::<Weak<RefCell<dyn Widget>>>::new(),
            render_instruction_collection_ptr: render_instruction_collection_ptr,
            absolute_widget_collection_ptr: absolute_widget_collection_ptr,
            tooltip_widget: tooltip_widget,
        }
    }

    fn is_mouse_inside(&mut self) -> bool {
        if self.cursor_pos.x >= self.position().x
            && self.cursor_pos.x <= (self.position().x + self.size().x)
            && self.cursor_pos.y >= self.position().y
            && self.cursor_pos.y <= (self.position().y + self.size().y)
        {
            true
        } else {
            false
        }
    }
}

impl Widget for TooltipViewWidget {
    fn on_event(&mut self, event: Event, messages: &mut Queue<Box<dyn Message>>) {
        match event {
            event::Event::Mouse(event::Mouse::CursorMoved { x: x_pos, y: y_pos }) => {
                self.cursor_pos.x = x_pos as f64;
                self.cursor_pos.y = y_pos as f64;

                if self.is_mouse_inside() {
                    if let Some(absolute_widget_collection) =
                        self.absolute_widget_collection_ptr.upgrade()
                    {
                        if let Some(widget) = self.tooltip_widget.upgrade() {
                            if widget.borrow_mut().id() == 0 {
                                widget.borrow_mut().set_dirty(true);
                                let size = widget.borrow_mut().original_size();
                                absolute_widget_collection.borrow_mut().insert(
                                    Rc::downgrade(&widget),
                                    self.cursor_pos,
                                    size,
                                );
                            }
                        }
                    }
                } else {
                    if let Some(render_instruction_collection) =
                        self.render_instruction_collection_ptr.upgrade()
                    {
                        if let Some(absolute_widget_collection) =
                            self.absolute_widget_collection_ptr.upgrade()
                        {
                            if let Some(widget) = self.tooltip_widget.upgrade() {
                                render_instruction_collection
                                    .borrow_mut()
                                    .remove(widget.borrow_mut().id());
                                absolute_widget_collection
                                    .borrow_mut()
                                    .remove(widget.borrow_mut().id());
                                widget.borrow_mut().set_id(0);
                            }
                        }
                    }
                }

                for value in self.children.iter_mut() {
                    if let Some(child) = value.upgrade() {
                        child.borrow_mut().on_event(event, messages);
                    }
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
