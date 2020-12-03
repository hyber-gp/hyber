use crate::event::Event;
use crate::util::Queue;

//struct BoxLayout {
// min_x: unimplemented!(),
// max_x: unimplemented!(),
// min_y: unimplemented!(),
// max_y: unimplemented!()
//}

pub trait Widget<Message> {
    /// This function is needed to detect if the event is being done on this widget, update the state of
    /// the widget based on event and place a message in the message queue.
    ///
    /// # Returns
    /// An hyber Event
    ///
    /// # Arguments
    /// * `event` - an hyber event
    /// * `messages` - queue of messages
    fn on_event(event: Event, messages: &Queue<Message>);
}
