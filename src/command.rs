use async_trait::async_trait;

#[async_trait]
pub trait Command<Message> {
    async fn run(&self) -> Message;

    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub type Commands<T> = Vec<Box<dyn Command<T>>>;
