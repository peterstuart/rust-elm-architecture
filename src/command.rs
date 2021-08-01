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

pub type Boxed<T> = Box<dyn Command<T>>;
pub type Commands<T> = Vec<Boxed<T>>;
