use crate::backend::Doer;

pub trait UI {
    fn new(doer: Doer) -> Self
    where
        Self: Sized;
    fn main_loop(&mut self) -> Result<!, Box<dyn std::error::Error>>;
}
