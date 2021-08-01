pub trait Command<Message> {
    fn run(&self);

    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub type Commands<T> = Vec<Box<dyn Command<T>>>;
