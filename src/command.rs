use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Command<Message> {
    async fn run(&self) -> Message;

    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub fn map<Message, OtherMessage, F>(command: Boxed<Message>, f: F) -> Boxed<OtherMessage>
where
    Message: 'static,
    OtherMessage: 'static,
    F: 'static + Fn(Message) -> OtherMessage,
{
    Mapped {
        command,
        f: Box::new(f),
    }
    .boxed()
}

pub type Boxed<T> = Box<dyn Command<T>>;
pub type Commands<T> = Vec<Boxed<T>>;

struct Mapped<Message, MappedMessage> {
    command: Boxed<Message>,
    f: Box<dyn Fn(Message) -> MappedMessage>,
}

#[async_trait(?Send)]
impl<Message, MappedMessage> Command<MappedMessage> for Mapped<Message, MappedMessage> {
    async fn run(&self) -> MappedMessage {
        (self.f)(self.command.run().await)
    }
}
