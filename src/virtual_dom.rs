#[derive(Clone, Debug)]
enum AttributeValue<Message> {
    Text(String),
    MessageCallback(Message),
}

#[derive(Clone, Debug)]
pub struct Attribute<Message> {
    name: String,
    value: AttributeValue<Message>,
}

impl<Message: Clone> Attribute<Message> {
    pub fn on_click(message: &Message) -> Self {
        Self {
            name: "onClick".into(),
            value: AttributeValue::MessageCallback(message.clone()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tag<Message> {
    name: String,
    attributes: Vec<Attribute<Message>>,
    children: Vec<Node<Message>>,
}

#[derive(Clone, Debug)]
pub enum Node<Message> {
    Tag(Tag<Message>),
    Text(String),
}

impl<Message: Clone> Node<Message> {
    pub fn text(text: &str) -> Node<Message> {
        Node::Text(text.into())
    }

    pub fn div(attributes: &[Attribute<Message>], children: &[Node<Message>]) -> Node<Message> {
        Node::Tag(Tag {
            name: "div".into(),
            attributes: attributes.to_vec(),
            children: children.to_vec(),
        })
    }

    pub fn button(attributes: &[Attribute<Message>], children: &[Node<Message>]) -> Node<Message> {
        Node::Tag(Tag {
            name: "button".into(),
            attributes: attributes.to_vec(),
            children: children.to_vec(),
        })
    }
}

pub type Html<Message> = Node<Message>;
