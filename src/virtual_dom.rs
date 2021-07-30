use std::rc::Rc;

pub enum Event<Message> {
    Click(Message),
    Input(Rc<dyn Fn(&str) -> Message>),
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
}

pub struct Element<Message> {
    pub name: String,
    pub attributes: Vec<Attribute<Message>>,
    pub children: Vec<Node<Message>>,
}

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
}

pub type Html<Message> = Node<Message>;
