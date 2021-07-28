use super::virtual_dom::Html;
use std::sync::mpsc;

pub struct App<Model, Message, Init, Update, View>
where
    Message: Clone,
    Init: Fn() -> Model,
    Update: Fn(&Message, &Model) -> Model,
    View: Fn(&Model) -> Html<Message>,
{
    pub init: Init,
    pub update: Update,
    pub view: View,
}

impl<Model, Message, Init, Update, View> App<Model, Message, Init, Update, View>
where
    Message: Clone,
    Init: Fn() -> Model,
    Update: Fn(&Message, &Model) -> Model,
    View: Fn(&Model) -> Html<Message>,
{
    pub fn run(&self) {
        let (_message_sender, message_receiver) = mpsc::channel();
        // TODO: use message_sender for on_click, etc. to send messages to the run loop

        let mut model = (self.init)();

        let html = (self.view)(&model);
        Self::render(&html);

        while let Ok(message) = message_receiver.recv() {
            model = (self.update)(message, &model);

            let html = (self.view)(&model);
            Self::render(&html);
        }
    }

    fn render(_html: &Html<Message>) {
        // TODO: render
    }
}
