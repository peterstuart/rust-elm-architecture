#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Attribute<Message> {
    OnClick(Message),
    Other(String, String),
}

impl<Message: Clone> Attribute<Message> {
    pub fn on_click(message: &Message) -> Self {
        Self::OnClick(message.clone())
    }

    pub fn class(name: &str) -> Self {
        Self::Other("class".into(), name.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Element<Message> {
    pub name: String,
    pub attributes: Vec<Attribute<Message>>,
    pub children: Vec<Node<Message>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Node<Message> {
    Element(Element<Message>),
    Text(String),
}

impl<Message: Clone> Node<Message> {
    pub fn text(text: &str) -> Node<Message> {
        Node::Text(text.into())
    }

    pub fn div(attributes: &[Attribute<Message>], children: &[Node<Message>]) -> Node<Message> {
        Node::Element(Element {
            name: "div".into(),
            attributes: attributes.to_vec(),
            children: children.to_vec(),
        })
    }

    pub fn span(attributes: &[Attribute<Message>], children: &[Node<Message>]) -> Node<Message> {
        Node::Element(Element {
            name: "span".into(),
            attributes: attributes.to_vec(),
            children: children.to_vec(),
        })
    }

    pub fn button(attributes: &[Attribute<Message>], children: &[Node<Message>]) -> Node<Message> {
        Node::Element(Element {
            name: "button".into(),
            attributes: attributes.to_vec(),
            children: children.to_vec(),
        })
    }
}

pub type Html<Message> = Node<Message>;
