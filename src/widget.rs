use crate::event::Event;
use crate::util::Queue;

//struct BoxLayout {
// min_x: unimplemented!(),
// max_x: unimplemented!(),
// min_y: unimplemented!(),
// max_y: unimplemented!()
//}


/// TODO: DOCUMENTAR ISTO
/// D é Display
/// M é Message
pub trait Widget<D,M> {

    /// @joaosantos @diogosemedo @samuelouteiro @pedrosilva
    /// Facam o que for necessario para o construtor do widget.
    /// Um widget deve saber a que display pertence, bem como
    /// os seus pais e filhos.
    fn new(display: &mut D);

    /// @diogosemedo
    /// This function is needed to detect if the event is being done on this widget, update the state of
    /// the widget based on event and place a message in the message queue.
    ///
    /// # Returns
    /// An hyber Event
    ///
    /// # Arguments
    /// * `event` - an hyber event
    /// * `messages` - queue of messages
    fn on_event(event: Event, messages: &Queue<M>);

    /// @joaosantos
    /// this returns the "recipe" of the widget. In other words,
    /// it returns the collection of Instructions that tell the
    /// renderer how to draw this widget.
    fn recipe();

    /// @joaosantos
    /// recursive function
    /// 3 steps:
    /// - add recipe() instructions to display's tree view
    /// - mark_as_clean()
    /// - call build() on children
    fn build();

    /// @joaosantos
    /// marks widget and its children as dirty - they need to be rebuilt!
    fn mark_as_dirty();

    /// @joaosantos
    /// For internal use only. Called by build(). marks widget as clean - no need to be rebuilt!
    fn mark_as_clean();

    /// @samuelouteiro
    /// adds the given widget as a child of this widget
    fn add_as_child(&mut self, child: Self);

    /// @samuelouteiro
    /// returns collection of its children
    fn children(&mut self);

    /// @samuelouteiro
    /// For internal use only. Sets given widget as parent of this widget
    fn set_as_parent(&mut self, parent: Self);

    /// @samuelouteiro
    /// pass layout constraints to children. May be BoxLayout or SliverLayout -> match should be used!
    fn pass_layout_to_children();
    
    /// @samuelouteiro
    /// set layout constraints. Usually called by parent. May be BoxLayout or SliverLayout -> match should be used!
    fn set_layout();
}
