use crate::{command::Commands, renderer::Renderer, virtual_dom::Html};
use log::info;
use std::{cell::Ref, cell::RefCell, fmt, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

struct State<Model, Message> {
    model: Model,
    html: Html<Message>,
}

impl<Model, Message> State<Model, Message>
where
    Model: Clone,
{
    fn new(model: Model, html: Html<Message>) -> Self {
        Self { model, html }
    }
}

type InitFn<Model, Message> = dyn Fn() -> (Model, Commands<Message>);
type UpdateFn<Model, Message> = dyn Fn(&Message, &mut Model) -> Commands<Message>;
type ViewFn<Model, Message> = dyn Fn(&Model) -> Html<Message>;

#[derive(Clone)]
pub struct App<Model, Message> {
    init: Rc<InitFn<Model, Message>>,
    update: Rc<UpdateFn<Model, Message>>,
    view: Rc<ViewFn<Model, Message>>,
    root_id: String,
    state: Rc<RefCell<Option<State<Model, Message>>>>,
}

impl<Model, Message> App<Model, Message>
where
    Model: 'static + Clone + fmt::Debug + Eq,
    Message: 'static + Clone + fmt::Debug,
{
    pub fn new<Init, Update, View>(init: Init, update: Update, view: View, root_id: &str) -> Self
    where
        Init: 'static + Fn() -> (Model, Commands<Message>),
        Update: 'static + Fn(&Message, &mut Model) -> Commands<Message>,
        View: 'static + Fn(&Model) -> Html<Message>,
    {
        Self {
            init: Rc::new(init),
            update: Rc::new(update),
            view: Rc::new(view),
            root_id: root_id.into(),
            state: Rc::new(RefCell::new(None)),
        }
    }

    pub fn start(&self) {
        let (model, commands) = (self.init)();

        self.update(model, commands);
    }

    pub fn handle_message(&self, message: &Message) {
        info!("message: {:#?}", message);

        let mut new_model = self.state().as_ref().unwrap().model.clone();
        let commands = (self.update)(message, &mut new_model);

        if new_model == self.state().as_ref().unwrap().model {
            return;
        }

        self.update(new_model, commands);
    }

    fn update(&self, model: Model, commands: Commands<Message>) {
        info!("model: {:#?}", model);

        let new_html = (self.view)(&model);
        self.render_app(&new_html).unwrap();

        self.handle_commands(commands);
        self.set_state(State::new(model, new_html));
    }

    fn handle_commands(&self, commands: Commands<Message>) {
        for command in commands {
            let app = self.clone();

            spawn_local(async move {
                let message = command.run().await;
                app.handle_message(&message);
            });
        }
    }

    fn state(&self) -> Ref<Option<State<Model, Message>>> {
        (self.state).borrow()
    }

    fn set_state(&self, state: State<Model, Message>) {
        self.state.replace(Some(state));
    }

    fn render_app(&self, html: &Html<Message>) -> Result<(), JsValue> {
        let renderer = Renderer::new(self);
        renderer.render(
            self.state().as_ref().map(|state| &state.html),
            html,
            &self.root_id,
        )
    }
}
