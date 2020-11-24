use std::collections::BTreeMap;

/// Enumeration with the Render Instructions @joaosantos
/// <name>Abs for Instructions to be drawn on absolute positions
/// <name>Rel for Instructions to be drawn on relative positions
pub enum RenderInstruction {
    /// Instruction to the Render that a point needs to be drawn on the next Clipping
    /// The point should be rendered on absolute coordinates (x,y)
	DrawPoint,
    /// Instruction to the Render that a line needs to be drawn on the next Clipping
    /// The line should be rendered on absolute coordinates from (x1, y1) to (x2, y2)
	DrawLineAbs,
	/// Instruction to the Render that a line needs to be drawn on the next Clipping
    DrawLineRel,
    /// Instruction to the Render that an arc needs to be drawn on the next Clipping
    /// The arc should be rendered with center on absolute coordinates (x, y), 'r'
    /// radius, 'sang' start angle and 'eang' end angle
    DrawArc,
    /// Instruction to the Render that a circle needs to be drawn on the next Clipping
    /// The circle should be rendered with center on absolute coordinates (x, y) and 'r'
    /// radius
    DrawCircleAbs,
    /// Instruction to the Render that a rectangle needs to be drawn on the next Clipping
    /// The rectangle should be rendered on absolute coordinates (x, y) with 'l' length
    /// and 'w' width
	DrawRectAbs,
    /// Instruction to the Render that a rectangle needs to be drawn on the next Clipping
    DrawRectRel,
    /// Instruction to the Render that a triangle needs to be drawn on the next Clipping
    /// The triangle should be rendered between the absolute coordinates (x1, y1),
    /// (x2, y2) and (x3, y3)
    DrawTriangleAbs,
    /// Instruction to the Render that a triangle needs to be drawn on the next Clipping
    DrawTriangleRel,
    /// Instruction to the Render that an image needs to be drawn on the next Clipping
    /// [Doubt] The image should be rendered with center on the absolute coordinates (x, y)
    /// and with 'w' width and 'l' length
    DrawImageAbs,
    /// Instruction to the Render that an image needs to be drawn on the next Clipping
    DrawImageRel,
    /// Instruction to the Render that some text needs to be drawn on the next Clipping
    /// [Doubt] The text should be rendered according to the text_alignment
    DrawTextAbs,
    /// Instruction to the Render that some text needs to be drawn on the next Clipping
    DrawTextRel
}
/// Assumptions:
///     - 2D Meshes are compounded by a list of triangles so the instructions are gonna be
///     multiple DrawTriangleAbs intructions
///     Reference: https://github.com/hecrj/iced/blob/master/graphics/src/triangle.rs
///     - Based on: https://en.wikipedia.org/wiki/Geometric_primitive
///     - And on:   https://www.freepascal.org/docs-html/current/rtl/graph/funcdrawing.html

// Structure of an Instruction to be on the Render Instrucstions Collection
pub struct Instruction {
    pub id: u32,
    pub instruction: RenderInstruction,
}

/// Implements the method for a new Instruction
impl Instruction {
    pub fn new(id: u32, instruction: RenderInstruction) -> Instruction {
        Instruction {
            id,
            instruction,
        }
    }
}

/// Example
/// 
/// Insertion:
///     map.insert(1, RenderInstruction::DrawPoint);
/// Get Key-Value:
///     map.get(&1);

let mut instruction = BTreeMap::new();

/// Assumptions for the map:
///  - Need to have a key-value pair of <u32, RenderInstruction>/<id, RenderInstruction>
/// Requirements:
///  - Fast iterator, due to client requirements of rendering
/// 
/// BTreeMap is the choice because of our use case:
///     - You want a map sorted by its keys.
///
/// References: 
///     - https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.remove
///     - https://doc.rust-lang.org/std/collections/index.html#use-a-btreemap-when


pub enum Event {
    //TODO: Rato, teclado, window
}

impl Event {
    // TODO: funcoes
    fn mouse_click() {
        unimplemented!();
    }

}

pub trait Display {
    
}

struct BoxLayout {
    min_x: unimplemented!(),
    max_x: unimplemented!(),
    min_y: unimplemented!(),
    max_y: unimplemented!()
}

struct SliverLayout {
}

pub trait Widget {

}

pub trait Renderer {
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
