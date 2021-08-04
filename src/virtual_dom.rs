use macros::element;
use std::{fmt, rc::Rc};

pub enum Event<Message> {
    Click(Message),
    Input(Rc<dyn Fn(&str) -> Message>),
}

impl<Message> Event<Message> {
    fn map<OtherMessage, F>(self, f: F) -> Event<OtherMessage>
    where
        Message: 'static,
        F: 'static + Copy + Fn(Message) -> OtherMessage,
    {
        match self {
            Event::Click(message) => Event::Click(f(message)),
            Event::Input(handler) => Event::Input(Rc::new(move |input| f(handler(input)))),
        }
    }
}

pub enum Attribute<Message> {
    On(Event<Message>),
    Other(String, String),
}

impl<Message> Attribute<Message> {
    pub fn on_click(message: Message) -> Self {
        Self::On(Event::Click(message))
    }

    pub fn on_input<F: 'static + Fn(&str) -> Message>(handler: F) -> Self {
        Self::On(Event::Input(Rc::new(handler)))
    }

    pub fn class(name: &str) -> Self {
        Self::Other("class".into(), name.into())
    }

    pub fn type_(type_: &str) -> Self {
        Self::Other("type".into(), type_.into())
    }

    pub fn value(value: &str) -> Self {
        Self::Other("value".into(), value.into())
    }

    fn map<OtherMessage, F>(self, f: F) -> Attribute<OtherMessage>
    where
        Message: 'static,
        F: 'static + Copy + Fn(Message) -> OtherMessage,
    {
        match self {
            Attribute::On(event) => Attribute::On(event.map(f)),
            Attribute::Other(name, value) => Attribute::Other(name, value),
        }
    }
}

pub struct Element<Message> {
    pub name: String,
    pub attributes: Vec<Attribute<Message>>,
    pub children: Vec<Node<Message>>,
}

impl<Message> Element<Message> {
    fn map<OtherMessage, F>(self, f: F) -> Element<OtherMessage>
    where
        Message: 'static,
        F: 'static + Copy + Fn(Message) -> OtherMessage,
    {
        Element {
            name: self.name,
            attributes: self
                .attributes
                .into_iter()
                .map(|attribute| attribute.map(f))
                .collect(),
            children: self
                .children
                .into_iter()
                .map(|child| child.map(f))
                .collect(),
        }
    }
}

impl<Message> fmt::Debug for Element<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)
    }
}

#[derive(Debug)]
pub enum Node<Message> {
    Element(Element<Message>),
    Text(String),
}

impl<Message> Node<Message> {
    pub fn text(text: &str) -> Node<Message> {
        Node::Text(text.into())
    }

    // From https://developer.mozilla.org/en-US/docs/Web/HTML/Element

    // Content Sectioning
    element!("address");
    element!("article");
    element!("aside");
    element!("footer");
    element!("header");
    element!("h1");
    element!("h2");
    element!("h3");
    element!("h4");
    element!("h5");
    element!("h6");
    element!("main");
    element!("nav");
    element!("section");

    // Text Content
    element!("blockquote");
    element!("dd");
    element!("div");
    element!("d1");
    element!("dt");
    element!("figcaption");
    element!("figure");
    element!("hr");
    element!("li");
    element!("ol");
    element!("p");
    element!("pre");
    element!("ul");

    // Inline Text Semantics
    element!("a");
    element!("abbr");
    element!("b");
    element!("bdi");
    element!("bdo");
    element!("br");
    element!("cite");
    element!("code");
    element!("data");
    element!("dfn");
    element!("em");
    element!("i");
    element!("kbd");
    element!("mark");
    element!("q");
    element!("rp");
    element!("rt");
    element!("ruby");
    element!("s");
    element!("samp");
    element!("small");
    element!("span");
    element!("strong");
    element!("sub");
    element!("sup");
    element!("time");
    element!("u");
    element!("var");
    element!("wbr");

    // Image and Multimedia
    element!("area");
    element!("audio");
    element!("img");
    // element!("map");
    element!("track");
    element!("video");

    // Embedded Content
    element!("embed");
    element!("iframe");
    element!("object");
    element!("param");
    element!("picture");
    element!("portal");
    element!("source");

    // SVG and MathML
    element!("svg");
    element!("math");

    // Scripting
    element!("canvas");
    element!("noscript");
    element!("script");

    // Demarcating Edits
    element!("del");
    element!("ins");

    // Table Content
    element!("caption");
    element!("col");
    element!("colgroup");
    element!("table");
    element!("tbody");
    element!("td");
    element!("tfoot");
    element!("th");
    element!("thead");
    element!("tr");

    // Forms
    element!("button");
    element!("datalist");
    element!("fieldset");
    element!("form");
    element!("input");
    element!("label");
    element!("legend");
    element!("meter");
    element!("optgroup");
    element!("option");
    element!("output");
    element!("progress");
    element!("select");
    element!("textarea");

    // Interactive Elements
    element!("details");
    element!("dialog");
    element!("menu");
    element!("summary");

    // Web Components
    element!("slot");
    element!("template");

    pub fn map<OtherMessage, F>(self, f: F) -> Node<OtherMessage>
    where
        Message: 'static,
        F: 'static + Copy + Fn(Message) -> OtherMessage,
    {
        match self {
            Node::Element(element) => Node::Element(element.map(f)),
            Node::Text(text) => Node::Text(text),
        }
    }
}

pub type Html<Message> = Node<Message>;
