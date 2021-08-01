use crate::{
    app::App,
    command::Commands,
    virtual_dom::{self, Html},
};
use log::trace;
use std::fmt;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::{Document, Element, HtmlElement, InputEvent, Node, Text};

pub struct Renderer<Model, Message, Init, Update, View> {
    app: App<Model, Message, Init, Update, View>,
    document: Document,
}

impl<Model, Message, Init, Update, View> Renderer<Model, Message, Init, Update, View>
where
    Model: 'static + Clone + fmt::Debug + Eq,
    Message: 'static + Clone + fmt::Debug,
    Init: 'static + Fn() -> (Model, Commands<Message>),
    Update: 'static + Fn(&Message, &Model) -> (Model, Commands<Message>),
    View: 'static + Fn(&Model) -> Html<Message>,
{
    pub fn new(app: &App<Model, Message, Init, Update, View>) -> Self {
        Self {
            app: app.clone(),
            document: web_sys::window().unwrap().document().unwrap(),
        }
    }

    pub fn render(
        &self,
        old: Option<&Html<Message>>,
        new: &Html<Message>,
        root_id: &str,
    ) -> Result<(), JsValue>
    where
        Message: 'static + Clone + fmt::Debug,
    {
        let root = self.document.get_element_by_id(root_id).unwrap();
        self.render_node(&old, &Some(new), &root, 0)?;

        Ok(())
    }

    fn render_node(
        &self,
        old: &Option<&Html<Message>>,
        new: &Option<&Html<Message>>,
        parent: &Element,
        index: u32,
    ) -> Result<(), JsValue>
    where
        Message: 'static + Clone + fmt::Debug,
    {
        match (old, new) {
            (None, None) => panic!("don't call render_element with no old or new html"),
            // remove old element
            (Some(old), None) => {
                trace!("remove old element: {:?} {}", old, index);
                Self::remove_child(parent, index)?
            }
            // insert new element
            (None, Some(new)) => {
                trace!("insert new element: {:?}", new);
                Self::append_child(parent, &self.create_node(new)?)?
            }
            // leave text unchanged
            (Some(virtual_dom::Node::Text(old_text)), Some(virtual_dom::Node::Text(new_text)))
                if old_text == new_text =>
            {
                trace!("leave text unchanged: {:?}", new_text);
            }
            // update text
            (Some(virtual_dom::Node::Text(old_text)), Some(virtual_dom::Node::Text(new_text))) => {
                trace!("update text: {:?} -> {:?}", old_text, new_text);
                Self::update_text(
                    &Self::get_child(parent, index)?.dyn_into::<Text>()?,
                    new_text,
                )
            }
            // update element
            (Some(virtual_dom::Node::Element(old)), Some(virtual_dom::Node::Element(new)))
                if old.name == new.name =>
            {
                trace!("update element: {:?}", new.name);
                self.update_element(old, new, &Self::get_child(parent, index)?.dyn_into()?)?;
            }
            // replace node
            (Some(old), Some(new)) => {
                trace!("replace node: {:?} -> {:?} {}", old, new, index);
                Self::replace_child(parent, index, &self.create_node(new)?)?
            }
        }

        Ok(())
    }

    fn get_child(element: &Element, index: u32) -> Result<Node, JsValue> {
        Ok(element
            .dyn_ref::<Node>()
            .unwrap()
            .child_nodes()
            .item(index)
            .unwrap())
    }

    fn remove_child(element: &Element, index: u32) -> Result<(), JsValue> {
        let child = Self::get_child(element, index)?;
        element.remove_child(&child)?;

        Ok(())
    }

    fn replace_child(element: &Element, index: u32, new: &Node) -> Result<(), JsValue> {
        let old = Self::get_child(element, index)?;
        element.replace_child(new, &old)?;

        Ok(())
    }

    fn append_child(element: &Element, child: &Node) -> Result<(), JsValue> {
        element.append_child(child)?;

        Ok(())
    }

    fn create_node(&self, node: &Html<Message>) -> Result<Node, JsValue> {
        Ok(match node {
            virtual_dom::Node::Element(element) => self.create_element(element)?.dyn_into()?,
            virtual_dom::Node::Text(text) => self.create_text(text).dyn_into()?,
        })
    }

    fn create_element(
        &self,
        element: &virtual_dom::Element<Message>,
    ) -> Result<HtmlElement, JsValue> {
        let dom_element = self
            .document
            .create_element(&element.name)?
            .dyn_into::<HtmlElement>()?;

        self.set_attributes(element, &dom_element)?;

        for child in &element.children {
            let dom_child = self.create_node(child)?;
            dom_element.append_child(&dom_child)?;
        }

        Ok(dom_element)
    }

    fn update_element(
        &self,
        old: &virtual_dom::Element<Message>,
        new: &virtual_dom::Element<Message>,
        dom_element: &HtmlElement,
    ) -> Result<(), JsValue> {
        self.clear_attributes(old, dom_element)?;
        self.set_attributes(new, dom_element)?;

        let max_children = old.children.len().max(new.children.len());

        for index in (0..max_children).rev() {
            let old_child = old.children.get(index);
            let new_child = new.children.get(index);

            self.render_node(&old_child, &new_child, dom_element, index as u32)?;
        }

        Ok(())
    }

    fn clear_attributes(
        &self,
        element: &virtual_dom::Element<Message>,
        dom_element: &HtmlElement,
    ) -> Result<(), JsValue> {
        for attribute in &element.attributes {
            match attribute {
                virtual_dom::Attribute::On(event) => match event {
                    virtual_dom::Event::Click(_) => dom_element.set_onclick(None),
                    virtual_dom::Event::Input(_) => dom_element.set_oninput(None),
                },
                virtual_dom::Attribute::Other(name, _) => dom_element.set_attribute(name, "")?,
            }
        }

        Ok(())
    }

    fn set_attributes(
        &self,
        element: &virtual_dom::Element<Message>,
        dom_element: &HtmlElement,
    ) -> Result<(), JsValue> {
        for attribute in &element.attributes {
            match attribute {
                virtual_dom::Attribute::On(event) => {
                    let app = self.app.clone();

                    match event {
                        virtual_dom::Event::Click(message) => {
                            let message = message.clone();

                            let callback = Closure::wrap(Box::new(move || {
                                app.handle_message(&message);
                            })
                                as Box<dyn FnMut()>);

                            dom_element.set_onclick(Some(callback.as_ref().unchecked_ref()));

                            // TODO: this is leaking memory
                            callback.forget();
                        }
                        virtual_dom::Event::Input(handler) => {
                            let handler = handler.clone();

                            let callback = Closure::wrap(Box::new(move |event: InputEvent| {
                                let value = event
                                    .target()
                                    .unwrap()
                                    .dyn_into::<web_sys::HtmlInputElement>()
                                    .unwrap()
                                    .value();
                                let message = handler(&value);
                                app.handle_message(&message);
                            })
                                as Box<dyn Fn(_)>);

                            dom_element.set_oninput(Some(callback.as_ref().unchecked_ref()));

                            // TODO: this is leaking memory
                            callback.forget();
                        }
                    }
                }
                virtual_dom::Attribute::Other(name, value) => {
                    dom_element.set_attribute(name, value)?
                }
            }
        }

        Ok(())
    }

    fn create_text(&self, text: &str) -> Text {
        self.document.create_text_node(text)
    }

    fn update_text(text_node: &Text, text: &str) {
        text_node.set_text_content(Some(text));
    }
}
