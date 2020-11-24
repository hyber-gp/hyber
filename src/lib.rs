/// Enumeration with the Render Instructions
pub enum RenderInstruction {
    /// Instruction to the Render that a point needs to be drawn on the next Clipping
    /// This instruction
	DrawPoint,
    /// Instruction to the Render that a line needs to be drawn on the next Clipping
	DrawLine,
	// Draw a line relative to current position
	DrawLineRel,
	// Draw a line from current position to absolute position
	DrawLineTo,
	// Draw a path
	DrawPath,
	// Draw a rect
	DrawRect,
	// Draw a triangle
	DrawTriangle,
	// Draw a polygon
	DrawPolygon,
	// Draw an arc
	DrawArc,
	// Draw a complete circle
	DrawCircle,
	// Draw an ellipse
	DrawEllipse,
	// Draw text at coordinates X, Y
	DrawTextXY,
	// Move cursor to absolute position
	MoveTo,
	// Move cursor relative to current position
	MoveRel,
	// Clear some widget from the screen 
	Clear,
}

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
