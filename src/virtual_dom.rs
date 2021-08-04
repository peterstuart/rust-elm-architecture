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

    pub fn div(attributes: Vec<Attribute<Message>>, children: Vec<Node<Message>>) -> Node<Message> {
        Node::Element(Element {
            name: "div".into(),
            attributes,
            children,
        })
    }

    pub fn span(
        attributes: Vec<Attribute<Message>>,
        children: Vec<Node<Message>>,
    ) -> Node<Message> {
        Node::Element(Element {
            name: "span".into(),
            attributes,
            children,
        })
    }

    pub fn p(attributes: Vec<Attribute<Message>>, children: Vec<Node<Message>>) -> Node<Message> {
        Node::Element(Element {
            name: "p".into(),
            attributes,
            children,
        })
    }

    pub fn button(
        attributes: Vec<Attribute<Message>>,
        children: Vec<Node<Message>>,
    ) -> Node<Message> {
        Node::Element(Element {
            name: "button".into(),
            attributes,
            children,
        })
    }

    pub fn input(
        attributes: Vec<Attribute<Message>>,
        children: Vec<Node<Message>>,
    ) -> Node<Message> {
        Node::Element(Element {
            name: "input".into(),
            attributes,
            children,
        })
    }

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
