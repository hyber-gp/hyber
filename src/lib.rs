use std::collections::BTreeMap;

/// Enumeration with the Render Instructions @joaosantos
pub enum RenderInstruction {
    /// Instruction to the Render that a point needs to be drawn on the next Clipping
    /// The point should be rendered on absolute coordinates (x,y)
	DrawPoint,
    /// Instruction to the Render that a line needs to be drawn on the next Clipping
    /// The line should be rendered on absolute coordinates from (x1, y1) to (x2, y2)
	DrawLine,
    /// Instruction to the Render that an arc needs to be drawn on the next Clipping
    /// The arc should be rendered with center on absolute coordinates (x, y), 'r'
    /// radius, 'sang' start angle and 'eang' end angle
    DrawArc,
    /// Instruction to the Render that a circle needs to be drawn on the next Clipping
    /// The circle should be rendered with center on absolute coordinates (x, y) and 'r'
    /// radius
    DrawCircle,
    /// Instruction to the Render that a rectangle needs to be drawn on the next Clipping
    /// The rectangle should be rendered on absolute coordinates (x, y) with 'l' length
    /// and 'w' width
	DrawRect,
    /// Instruction to the Render that a triangle needs to be drawn on the next Clipping
    /// The triangle should be rendered between the absolute coordinates (x1, y1),
    /// (x2, y2) and (x3, y3)
    DrawTriangle,
    /// Instruction to the Render that an image needs to be drawn on the next Clipping
    /// [Doubt] The image should be rendered with center on the absolute coordinates (x, y)
    /// and with 'w' width and 'l' length
    DrawImage,
    /// Instruction to the Render that some text needs to be drawn on the next Clipping
    /// [Doubt] The text should be rendered according to the text_alignment
    DrawText,
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

/// Example:
///
/// Criar:
///     (-> BTreeMap<K, V>)
///     - makes a new empty BTreeMap.
///
///     let mut map = BTreeMap::new();
///
/// Limpar:
///     - clears the map, removing all elements
///
///     map.clear();
///
/// Get Value:
///     (-> Option<&V>)
///     - returns a reference to the value corresponding to the key
///
///     map.get(&1);
///
/// Get Key-Value:
///     (-> Option<(&K, &V)>)
///     - returns the key-value pair corresponding to the supplied key
///
///     map.get_key_value(&1);
///
/// Get Mutable Value:
///     (-> Option<&mut V>)
///     - returns a mutable reference to the value corresponding to the key.
///
///     map.get_mut(&1);
///
/// Contains Key:
///     (bool)
///     - returns true if the map contains a value for the specified key.
///
///     map.contains_key(&1);
///
/// First Key-Value:
///     (-> Option<(&K, &V)>)
///     - to obtain the first key-value pair in the map
///
///     map.first_key_value();
///
/// Insertion:
///     (-> Option<V>)
///     - inserts a key-value pair into the map
///
///     map.insert(1, RenderInstruction::DrawPoint);
///
/// Remove:
///     (-> Option<V>)
///     - removes a key from the map, returning the value at the key
///     - if the key was previously in the map
///
///     map.remove(&1);
///
/// Remove Entry:
///     (-> Option<(K, V)>)
///     - removes a key from the map, returning the stored key and value
///     - if the key was previously in the map
///
///     map.remove_entry(&1);

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
