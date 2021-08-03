#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataLoading<Data> {
    Loading,
    Loaded(Data),
    Error,
}
