//! Hyber is a renderer-agnostic GUI library.
//!
//! # Usage:
//!
//! To use it, programmers need to create a specific implementation of this crate using a renderer of their choice. That crate must implement the following traits:
//! - [`renderer::Renderer`]
//! - [`display::Display`]
//!
//! Afterwards, programmers that wish to use that specific crate should implement the following trait:
//! - [`renderer::Message`]
//!
//! # Widgets
//! Hyber has a number of basic widgets implemented, which can be found in module [`widget`].
//!
//! Programmers may implement their own custom widget by creating a struct that implements the [`widget::Widget`] trait.
//!
//! # Basic example
//!
//! Here follows a simple example of a program that has a [`widget::button_view::ButtonViewWidget`] with a [`widget::label::LabelWidget`] inside it that counts the number of clicks, acting as a counter. In this example, the aforementioned crate using a renderer to implement [`hyber`](`self`) is called `hyber_renderer`.
//!
//! ```
//! // Import needed widgets and structs from [`hyber`]
//! use hyber::display::Display;
//! use hyber::event::Event;
//! use hyber::event::Mouse::CursorMoved;
//! use hyber::renderer::{AbsoluteWidgetCollection, Message, RenderInstructionCollection, Renderer};
//! use hyber::util::{Color, IDMachine, Vector2D};
//! use hyber::widget::button_view::ButtonViewWidget;
//! use hyber::widget::grid_view::GridViewWidget;
//! use hyber::widget::label::LabelWidget;
//! use hyber::widget::root::RootWidget;
//! use hyber::widget::{Axis, Layout, Widget};
//!
//! // Import needed widgets and structs from [`std`]
//! use std::cell::RefCell;
//! use std::rc::Rc;
//! use std::rc::Weak;
//!
//! // Import our renderer
//! use hyber_renderer;
//!
//! const WIDTH: f64 = 640.;
//! const HEIGHT: f64 = 360.;
//!
//! // To deal with events, we need an enum that implements [`Message`].
//! // In this case, we have two messages, one to increment and another to decrement.
//! #[derive(Clone)]
//! pub enum MessageXPTO {
//!     Increment {
//!         label_ptr: Weak<RefCell<LabelWidget>>,
//!         num_ptr: Weak<RefCell<i64>>,
//!         event: Option<Event>,
//!     },
//!     Decrement {
//!         label_ptr: Weak<RefCell<LabelWidget>>,
//!         num_ptr: Weak<RefCell<i64>>,
//!         event: Option<Event>,
//!     },
//! }
//!
//! impl Message for MessageXPTO {
//!     fn update(&self) {
//!         match self {
//!             MessageXPTO::Increment {
//!                 label_ptr,
//!                 num_ptr,
//!                 event: _,
//!             } => {
//!                 if let Some(label) = label_ptr.upgrade() {
//!                     if let Some(num) = num_ptr.upgrade() {
//!                         *num.borrow_mut() += 1;
//!                         label
//!                         .borrow_mut()
//!                             .set_text(String::from(format!("{}", *num.borrow())));
//!                     }
//!                 }
//!             }
//!             MessageXPTO::Decrement {
//!                 label_ptr,
//!                 num_ptr,
//!                 event: _,
//!             } => {
//!                 if let Some(label) = label_ptr.upgrade() {
//!                     if let Some(num) = num_ptr.upgrade() {
//!                         *num.borrow_mut() -= 1;
//!                         label
//!                         .borrow_mut()
//!                             .set_text(String::from(format!("{}", *num.borrow())));
//!                     }
//!                 }
//!             }
//!         }
//!     }
//!
//!     fn set_event(&mut self, new_event: Event) {
//!         match self {
//!             MessageXPTO::Increment {
//!                 label_ptr: _,
//!                 num_ptr: _,
//!                 event,
//!             } => {
//!                 *event = Some(new_event);
//!             }
//!             MessageXPTO::Decrement {
//!                 label_ptr: _,
//!                 num_ptr: _,
//!                 event,
//!             } => {
//!                 *event = Some(new_event);
//!             }
//!             MessageXPTO::Resize { grid_ptr: _, event } => {
//!                 *event = Some(new_event);
//!             }
//!         }
//!     }
//! }
//!
//! fn main() {
//!     // Every [`hyber`] program needs the following items
//!     let mut display = hyber_renderer::DisplayXPTO::new(
//!         "Test - ESC to exit",
//!         WIDTH as usize,
//!         HEIGHT as usize,
//!         hyber::display::DisplayDescritor {
//!             resizable: true,
//!             ..hyber::display::DisplayDescritor::default()
//!         },
//!     );
//! 
//!     let mut id_machine = IDMachine::new();
//!
//!     let collection = Rc::new(RefCell::new(RenderInstructionCollection::new()));
//!
//!     let absolute_collection = Rc::new(RefCell::new(AbsoluteWidgetCollection::new()));
//!
//!     let mut renderer = hyber_renderer::RendererXPTO::new(WIDTH as i32, HEIGHT as i32);
//!
//!     let events = renderer.create_events_queue();
//!     let messages = renderer.create_message_queue();
//!
//!     let root = Rc::new(RefCell::new(RootWidget::new(
//!         display.get_size(),
//!         Color::new(0xff, 0xff, 0xff, 0xff),
//!         Layout::Box(Axis::Horizontal),
//!     )));
//!
//!     // Shared objects, such as widgets and other variables need to be
//!     // encapsulated inside a Reference Counter pointer when created as follows
//!     let counter = Rc::new(RefCell::new(0));
//!
//!     let label_1 = Rc::new(RefCell::new(LabelWidget::new(
//!         String::from("Teste1!"),
//!         Vector2D::new(200f64, 200f64),
//!         33,
//!         Color::from_hex(0xffff8026),
//!         Color::from_hex(0xff004dff),
//!     )));
//!
//!     // When pressing the button, the counter increments. When long pressing the
//!     // button, the counter decrements.
//!     let button = Rc::new(RefCell::new(ButtonViewWidget::new(
//!         Vector2D::new(200f64, 200f64),
//!         true,
//!         Color::from_hex(0x36bd2b00),
//!         Some(Box::new(MessageXPTO::Increment {
//!             label_ptr: Rc::downgrade(&label_1),
//!             num_ptr: Rc::downgrade(&counter),
//!             event: None,
//!         })),
//!         Some(Box::new(MessageXPTO::Decrement {
//!             label_ptr: Rc::downgrade(&label_1),
//!             num_ptr: Rc::downgrade(&counter),
//!             event: None,
//!         })),
//!     )));
//!
//!     // In this example, we use a grid with column and row sizes of 1 (the grid
//!     // only has one child, the button)
//!     let grid = Rc::new(RefCell::new(GridViewWidget::new(
//!         Vector2D::new(WIDTH, HEIGHT),
//!         Axis::Vertical,
//!         1,
//!     )));
//!
//!     // With the necessary widgets created, we just need to specify their
//!     // child-parent relationships.
//!     button
//!     .borrow_mut()
//!     .add_as_child(Rc::downgrade(&label_1) as Weak<RefCell<dyn Widget>>);
//! 
//!     grid
//!     .borrow_mut()
//!     .add_as_child(Rc::downgrade(&button) as Weak<RefCell<dyn Widget>>);
//! 
//!     root
//!     .borrow_mut()
//!     .add_as_child(Rc::downgrade(&grid) as Weak<RefCell<dyn Widget>>);
//!         
//!     // Finally, the event loop of the renderer runs the program.
//!     renderer.event_loop(
//!         events,
//!         messages,
//!         Rc::downgrade(&root) as Weak<RefCell<dyn Widget>>,
//!         &mut display,
//!         Vector2D::new(WIDTH, HEIGHT),
//!         &mut id_machine,
//!         Rc::downgrade(&collection),
//!         Rc::downgrade(&absolute_collection),
//!     );
//! }
//! ```
//! 
//! # Known issues and planned features to be implemented
//! 
//! Known issues are marked as `TODO` in the code. Here follows a list of planned features that are not implemented yet:
//! 
//! - `SliverLayout` for scrollables and infinite scrolling
//! - Animation system
//! - Flex system

pub mod display;
pub mod event;
pub mod key_code;
pub mod renderer;
pub mod util;
pub mod widget;
