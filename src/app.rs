use std::{cell::RefCell, rc::Rc};

use crate::virtual_dom::{self, Attribute, Html, Node};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Document, Element};

#[derive(Clone, Debug, Eq, PartialEq)]
struct State<Model, Message>
where
    Model: Clone + Eq,
    Message: Clone + Eq,
{
    model: Model,
    html: Option<Html<Message>>,
}

impl<Model, Message> State<Model, Message>
where
    Model: Clone + Eq,
    Message: Clone + Eq,
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

pub struct App<Model, Message, Init, Update, View>
where
    Model: 'static + Clone + Eq,
    Message: Clone + Eq,
    Init: 'static + Fn() -> Model,
    Update: 'static + Fn(&Message, &Model) -> Model,
    View: 'static + Fn(&Model) -> Html<Message>,
{
    init: Rc<Init>,
    update: Rc<Update>,
    view: Rc<View>,
    root_id: String,
    state: Rc<RefCell<State<Model, Message>>>,
}

impl<Model, Message, Init, Update, View> App<Model, Message, Init, Update, View>
where
    Model: Clone + Eq,
    Message: 'static + Clone + Eq,
    Init: Fn() -> Model,
    Update: Fn(&Message, &Model) -> Model,
    View: Fn(&Model) -> Html<Message>,
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
        let html = (self.view)(&self.state().model);

        self.render_app(&html).unwrap();
    }

    pub fn handle_message(&self, message: &Message) {
        let state = self.state();

        let new_model = (self.update)(message, &state.model);

        if new_model == state.model {
            return;
        }

        let new_html = (self.view)(&new_model);

        self.render_app(&new_html).unwrap();
        self.set_state(State::new(new_model, new_html));
    }

    fn state(&self) -> State<Model, Message> {
        self.state.borrow().clone()
    }

    fn set_state(&self, state: State<Model, Message>) {
        let mut old_state = self.state.borrow_mut();
        *old_state = state;
    }

    fn render_app(&self, html: &Html<Message>) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let root = document.get_element_by_id(&self.root_id).unwrap();
        Self::remove_children(&root)?;
        self.create_node(&document, html, &root)?;

        Ok(())
    }

    fn remove_children(element: &Element) -> Result<(), JsValue> {
        while let Some(child) = element.last_child() {
            element.remove_child(&child)?;
        }

        Ok(())
    }

    fn create_node(
        &self,
        document: &Document,
        node: &Html<Message>,
        parent: &Element,
    ) -> Result<(), JsValue> {
        match node {
            Node::Element(element) => self.create_element(document, element, parent),
            Node::Text(text) => Self::create_text(document, text, parent),
        }
    }

    fn create_element(
        &self,
        document: &Document,
        element: &virtual_dom::Element<Message>,
        parent: &Element,
    ) -> Result<(), JsValue> {
        let dom_element = document
            .create_element(&element.name)?
            .dyn_into::<web_sys::HtmlElement>()?;

        for attribute in &element.attributes {
            match attribute {
                Attribute::OnClick(message) => {
                    let app = self.clone();
                    let message = message.clone();

                    let callback = Closure::wrap(Box::new(move || {
                        app.handle_message(&message);
                    }) as Box<dyn FnMut()>);

                    dom_element.set_onclick(Some(callback.as_ref().unchecked_ref()));

                    callback.forget();
                }
                Attribute::Other(name, value) => dom_element.set_attribute(name, value)?,
            }
        }

        for child in &element.children {
            self.create_node(document, child, &dom_element)?;
        }

        parent.append_child(&dom_element)?;

        Ok(())
    }

    fn create_text(document: &Document, text: &str, parent: &Element) -> Result<(), JsValue> {
        let text_node = document.create_text_node(text);
        parent.append_child(&text_node)?;

        Ok(())
    }
}

impl<Model, Message, Init, Update, View> Clone for App<Model, Message, Init, Update, View>
where
    Model: Clone + Eq,
    Message: 'static + Clone + Eq,
    Init: Fn() -> Model,
    Update: Fn(&Message, &Model) -> Model,
    View: Fn(&Model) -> Html<Message>,
{
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
