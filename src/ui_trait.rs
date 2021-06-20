use crate::backend::Doer;

pub trait UI {
    fn new(doer : Doer) -> Self;
    fn main_loop(&mut self) -> Result<!, Box<dyn std::error::Error>>;
}
