use crate::event::Event;
use crate::renderer::{Message, RenderInstruction};
use crate::util::{Color, Queue, Vector2D};
use crate::widget::{Layout, Widget};

use std::cell::RefCell;
use std::rc::Weak;

/// Label is a widget that displays a short text string. Does not react to input events. 
/// As a result, it cannot get the keyboard focus. A label can, however, display a keyboard
/// alternative as a convenience for a nearby component that has a keyboard alternative 
/// but can't display it.
#[derive(Clone)]
pub struct LabelWidget {
    /// The label's identifier
    id: usize,
    
    /// The label's text
    text: String,
    
    /// The label's font size
    font_size: usize,
    
    /// The label's background color
    background_color: Color,
    
    /// The label's foreground color (i.e., text color)
    foreground_color: Color,
    
    /// The dirty flag (i.e., flag used to mark the widgets needed to be rebuilt)
    dirty: bool,
    
    /// The label's children (i.e., his widgets tree)
    children: Vec<Weak<RefCell<dyn Widget>>>,
    
    /// The label's position, on a two-dimensional space (x-coordinate and y-coordinate) 
    /// relative to the top left corner
    position: Vector2D,
    
    /// The label's current size (width and height)
    size: Vector2D,
    
    /// The label's original size (width and height)
    original_size: Vector2D,
    
    /// The label's layout
    layout: Layout,
    
    /// The label's offset vector coordinates
    offset: Vector2D,

    /// TODO: documentar
    clip_point: Option<Vector2D>,

    /// TODO: documentar
    clip_size: Option<Vector2D>,
}

impl LabelWidget {
    /// Creates a new `LabelWidget`
    ///
    /// # Returns
    /// The label created
    ///
    /// # Arguments
    /// * `text` - the text to be assigned to the label
    /// * `size` - the size (width and height) to be assigned to the label
    /// * `font_size` - the font size to be assigned to the label's text
    /// * `background_color` - the color to be assigned to the icon's background
    /// * `foreground_color` - the color to be assigned to the icon's text
    pub fn new(
        text: String,
        size: Vector2D,
        font_size: usize,
        background_color: Color,
        foreground_color: Color,
    ) -> LabelWidget {
        LabelWidget {
            id: 0,
            text: text,
            font_size: font_size,
            background_color: background_color,
            foreground_color: foreground_color,
            dirty: true,
            children: Vec::<Weak<RefCell<dyn Widget>>>::new(),
            position: Vector2D::new(0., 0.),
            size: size,
            original_size: size,
            layout: Layout::None,
            offset: Vector2D::new(0., 0.),
            clip_point: None,
            clip_size: None,
        }
    }

    /// Sets label's text
    ///
    /// # Returns
    /// No returns
    ///
    /// # Arguments
    /// * `text` - the text to be assigned to the label
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.dirty = true;
    }
}

impl Widget for LabelWidget {
    fn on_event(&mut self, _event: Event, _messages: &mut Queue<Box<dyn Message>>) {}

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn id(&self) -> usize {
        self.id
    }

    fn recipe(&self) -> Vec<RenderInstruction> {
        let clip_point = if let Some(clip_point) = self.clip_point {clip_point} else {self.position};
        let clip_size = if let Some(clip_size) = self.clip_size {clip_size} else {self.size};

        vec![
            // Label rectangle.
            RenderInstruction::DrawRect {
                point: self.position,
                color: self.background_color.clone(),
                size: self.size,
                clip_point: clip_point,
                clip_size: clip_size,
            },
            // Label Text
            RenderInstruction::DrawText {
                point: Vector2D::new(self.position.x, self.position.y + self.size.y),
                color: self.foreground_color,
                font_size: self.font_size,
                string: self.text.clone(),
                clip_point: clip_point,
                clip_size: clip_size,
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

    fn is_cursor_inside(&mut self, _cursor_pos : Vector2D) -> bool {
        false
    }

    fn set_clip_point(&mut self, clip_point: Option<Vector2D>) {
        self.clip_point = clip_point;
    }

    fn set_clip_size(&mut self, clip_size: Option<Vector2D>) {
        self.clip_size = clip_size;
    }
}
