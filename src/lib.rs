#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct Key{
    key_code: i8,
    modifiers: i8
}

pub enum Keyboard{
    KeyPressed(Key),
    KeyReleased(Key)
}

pub enum Mouse{
    CursorEntered,
    CursorLeft,
    CursorMoved {
        x: f32,
        y: f32,
    },
    WheelScroolled{
        value: f32
    },
    ButtonPressed(MouseButton),
    ButtonReleased(MouseButton)
}

pub enum MouseButton{
    Left,
    Right,
    Middle
}


pub enum EventType{
    Mouse(Mouse),
    Keyboard(Keyboard)
}


pub trait EventSystem{
    fn new_event() -> EventType;
}


pub trait Renderer {
    
}



