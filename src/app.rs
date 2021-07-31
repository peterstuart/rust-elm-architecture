use crate::{renderer::Renderer, virtual_dom::Html};
use std::{cell::Ref, cell::RefCell, fmt, rc::Rc};
use wasm_bindgen::prelude::*;

struct State<Model, Message> {
    model: Model,
    html: Option<Html<Message>>,
}

impl<Model, Message> State<Model, Message>
where
    Model: Clone,
{
    fn new(model: Model, html: Html<Message>) -> Self {
        Self {
            model,
            html: Some(html),
        }
    }

    fn initial(model: Model) -> Self {
        Self { model, html: None }
    }
}

pub struct App<Model, Message, Init, Update, View> {
    init: Rc<Init>,
    update: Rc<Update>,
    view: Rc<View>,
    root_id: String,
    state: Rc<RefCell<State<Model, Message>>>,
}

impl<Model, Message, Init, Update, View> App<Model, Message, Init, Update, View>
where
    Model: 'static + Clone + fmt::Debug + Eq,
    Message: 'static + Clone + fmt::Debug,
    Init: 'static + Fn() -> Model,
    Update: 'static + Fn(&Message, &Model) -> Model,
    View: 'static + Fn(&Model) -> Html<Message>,
{
    pub fn new(init: Init, update: Update, view: View, root_id: &str) -> Self {
        let state = State::initial(init());

        Self {
            init: Rc::new(init),
            update: Rc::new(update),
            view: Rc::new(view),
            root_id: root_id.into(),
            state: Rc::new(RefCell::new(state)),
        }
    }

    pub fn start(&self) {
        let model = self.state().model.clone();
        let html = (self.view)(&model);
        self.render_app(&html).unwrap();
        self.set_state(State::new(model, html));
    }

    pub fn handle_message(&self, message: &Message) {
        let new_model = (self.update)(message, &self.state().model);
        if new_model == self.state().model {
            return;
        }

        let new_html = (self.view)(&new_model);
        self.render_app(&new_html).unwrap();

        self.set_state(State::new(new_model, new_html));
    }

    fn state(&self) -> Ref<State<Model, Message>> {
        (*self.state).borrow()
    }

    fn set_state(&self, state: State<Model, Message>) {
        self.state.replace(state);
    }

    fn render_app(&self, html: &Html<Message>) -> Result<(), JsValue> {
        let renderer = Renderer::new(self);
        renderer.render(&self.state().html, html, &self.root_id)
    }
}

impl<Model, Message, Init, Update, View> Clone for App<Model, Message, Init, Update, View> {
    fn clone(&self) -> Self {
        Self {
            init: self.init.clone(),
            update: self.update.clone(),
            view: self.view.clone(),
            root_id: self.root_id.clone(),
            state: self.state.clone(),
        }
    }
}
